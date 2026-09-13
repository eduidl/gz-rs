//! Regression tests for channel and direct callback context lifetimes.
//!
//! A decoder or direct user callback parks inside the FFI callback. Its sender belongs
//! to the callback closure, so disconnection at this point exposes premature
//! destruction of that closure. Each probe runs in a child process and exits
//! without resuming the decoder on failure, avoiding a return through freed
//! user_data. On success it resumes delivery and checks eventual reclamation.

use std::{
    process::{Command, Stdio},
    sync::OnceLock,
    thread,
    time::{Duration, Instant},
};

use async_channel::TryRecvError;
use crossbeam_channel::{Receiver, Sender, bounded};
use gz_msgs::stringmsg::StringMsg;
use gz_msgs_common::{
    GzMessage,
    protobuf::{self, CodedInputStream, CodedOutputStream, Message, SpecialFields},
};
use gz_transport::Node;
use uuid::Uuid;

const CHILD_MODE: &str = "GZ_RS_CALLBACK_LIFETIME_PROBE";
const TIMEOUT: Duration = Duration::from_secs(10);
#[derive(Debug)]
struct DecoderGate {
    in_user_callback: bool,
    entered: Sender<()>,
    resume: Receiver<()>,
}

static DECODER_GATE: OnceLock<DecoderGate> = OnceLock::new();

#[derive(Clone, Debug, Default, PartialEq)]
struct PausedStringMsg(StringMsg);

impl GzMessage for PausedStringMsg {
    const GZ_TYPE_NAME: &'static str = StringMsg::GZ_TYPE_NAME;
}

impl Message for PausedStringMsg {
    const NAME: &'static str = StringMsg::NAME;

    fn is_initialized(&self) -> bool {
        self.0.is_initialized()
    }

    fn merge_from(&mut self, input: &mut CodedInputStream<'_>) -> protobuf::Result<()> {
        self.0.merge_from(input)
    }

    fn write_to_with_cached_sizes(
        &self,
        output: &mut CodedOutputStream<'_>,
    ) -> protobuf::Result<()> {
        self.0.write_to_with_cached_sizes(output)
    }

    fn compute_size(&self) -> u64 {
        self.0.compute_size()
    }

    fn special_fields(&self) -> &SpecialFields {
        self.0.special_fields()
    }

    fn mut_special_fields(&mut self) -> &mut SpecialFields {
        self.0.mut_special_fields()
    }

    fn new() -> Self {
        Self::default()
    }

    fn default_instance() -> &'static Self {
        static INSTANCE: OnceLock<PausedStringMsg> = OnceLock::new();
        INSTANCE.get_or_init(Self::default)
    }

    fn parse_from_bytes(bytes: &[u8]) -> protobuf::Result<Self> {
        let message = StringMsg::parse_from_bytes(bytes)?;
        let gate = DECODER_GATE.get().unwrap();
        if !gate.in_user_callback {
            gate.entered.send(()).unwrap();
            // Resume only after proving that teardown retained the context.
            gate.resume.recv().unwrap();
        }
        Ok(Self(message))
    }
}

#[test]
fn unsubscribe_keeps_in_flight_native_callback_alive() {
    run_probe(
        "unsubscribe_keeps_in_flight_native_callback_alive",
        "unsubscribe",
    );
}

#[test]
fn node_drop_keeps_in_flight_native_callback_alive() {
    run_probe("node_drop_keeps_in_flight_native_callback_alive", "drop");
}

#[test]
fn unsubscribe_keeps_direct_callback_alive() {
    run_probe(
        "unsubscribe_keeps_direct_callback_alive",
        "unsubscribe_direct",
    );
}

#[test]
fn node_drop_keeps_direct_callback_alive() {
    run_probe("node_drop_keeps_direct_callback_alive", "drop_direct");
}

fn run_probe(test_name: &str, mode: &str) {
    if std::env::var(CHILD_MODE).as_deref() == Ok(mode) {
        probe_native_callback_lifetime(mode);
    }

    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test_name, "--nocapture"])
        .env(CHILD_MODE, mode)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let output = child.wait_with_output().unwrap();
            panic!(
                "Native callback lifetime probe timed out:\n{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
        thread::sleep(Duration::from_millis(10));
    }
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "Native callback lifetime probe failed ({mode}, {}):\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn probe_native_callback_lifetime(mode: &str) -> ! {
    let partition = Uuid::new_v4().to_string();
    let mut node = Node::with_partition(&partition).unwrap();
    // Gazebo snapshots both handlers before local delivery. While the first
    // decoder is parked, the other handler has been acquired but not invoked.
    // Both contexts must remain live, regardless of handler iteration order.
    let direct = mode.ends_with("_direct");
    let receivers: [async_channel::Receiver<PausedStringMsg>; 2] = std::array::from_fn(|_| {
        if direct {
            let (tx, rx) = async_channel::bounded(1);
            assert!(node.subscribe("lifetime", move |msg: PausedStringMsg| {
                let gate = DECODER_GATE.get().unwrap();
                gate.entered.send(()).unwrap();
                gate.resume.recv().unwrap();
                tx.try_send(msg).unwrap();
            }));
            rx
        } else {
            node.subscribe_channel::<PausedStringMsg>("lifetime", 1)
                .unwrap()
        }
    });
    let (entered_tx, entered_rx) = bounded(2);
    let (resume_tx, resume_rx) = bounded(2);
    DECODER_GATE
        .set(DecoderGate {
            in_user_callback: direct,
            entered: entered_tx,
            resume: resume_rx,
        })
        .unwrap();

    // Keep Node and Publisher on their creating thread. Local publication
    // enters the native callback synchronously and parks in our decoder.
    let publisher = thread::spawn(move || {
        let mut node = Node::with_partition(&partition).unwrap();
        let mut publisher = node.advertise::<StringMsg>("lifetime").unwrap();
        assert!(publisher.publish(&StringMsg {
            data: "pause during decoding".into(),
            ..Default::default()
        }));
    });
    entered_rx.recv_timeout(TIMEOUT).unwrap();
    assert!(
        receivers
            .iter()
            .all(|rx| matches!(rx.try_recv(), Err(TryRecvError::Empty))),
        "The native callback must still own its sender before teardown"
    );

    match mode {
        "unsubscribe" | "unsubscribe_direct" => assert!(node.unsubscribe("lifetime")),
        "drop" | "drop_direct" => drop(node),
        _ => unreachable!(),
    }

    for rx in &receivers {
        let result = rx.try_recv();
        if !matches!(result, Err(TryRecvError::Empty)) {
            eprintln!(
                "{mode} destroyed a native callback's sender while a decoder \
                 was still running: expected Empty (sender retained), got {result:?}"
            );
            // Do not resume a callback whose user_data has already been freed.
            std::process::exit(1);
        }
    }

    for _ in &receivers {
        resume_tx.send(()).unwrap();
    }
    for rx in receivers {
        assert_eq!(rx.recv_blocking().unwrap().0.data, "pause during decoding");
        assert!(
            matches!(rx.recv_blocking(), Err(async_channel::RecvError)),
            "The native callback's sender was retained after delivery finished"
        );
    }
    publisher.join().unwrap();
    std::process::exit(0);
}
