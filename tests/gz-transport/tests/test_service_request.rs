use std::{
    ffi::{CString, c_uint},
    panic::{AssertUnwindSafe, catch_unwind},
    time::Duration,
};

use gz_msgs::stringmsg::StringMsg;
use gz_msgs_common::{GzMessage, protobuf::Message};
use gz_transport::Node;
use gz_transport_sys as ffi;
use gz_transport_tests::TestService;
use uuid::Uuid;

#[test]
fn request_preserves_protobuf_bytes() {
    let partition = Uuid::new_v4().to_string();
    let service = TestService::new(&partition);
    let mut node = Node::with_partition(&partition).unwrap();

    // Empty serialization is valid, as are NUL bytes inside a string field.
    for data in ["", "ordinary request", "embedded\0NUL", "trailing\0"] {
        let request = StringMsg {
            data: data.into(),
            ..Default::default()
        };
        let (response, result) = node
            .request::<_, StringMsg>("/echo", &request, Duration::from_secs(5))
            .expect("valid Protobuf request must succeed without an appended NUL");
        assert!(result);
        assert_eq!(response, request);
    }
    assert_eq!(service.calls(), 4);
}

#[test]
fn request_accepts_maximum_timeout() {
    let partition = Uuid::new_v4().to_string();
    let service = TestService::new(&partition);
    let mut node = Node::with_partition(&partition).unwrap();
    let request = StringMsg {
        data: "maximum timeout".into(),
        ..Default::default()
    };
    let max_timeout = Duration::from_millis(u64::from(c_uint::MAX));

    // The local service responds synchronously, without waiting for the timeout.
    // Fractional milliseconds retain their existing truncation behavior.
    for timeout in [max_timeout, max_timeout + Duration::from_nanos(999_999)] {
        let (response, result) = node
            .request::<_, StringMsg>("/echo", &request, timeout)
            .expect("maximum representable timeout must be accepted");
        assert!(result);
        assert_eq!(response, request);
    }
    assert_eq!(service.calls(), 2);
}

#[test]
fn request_panics_on_out_of_range_timeout_without_sending() {
    let partition = Uuid::new_v4().to_string();
    let service = TestService::new(&partition);
    let mut node = Node::with_partition(&partition).unwrap();
    let request = StringMsg::default();

    // Establish availability so a missing service cannot mask a wrapped timeout.
    assert!(
        node.request::<_, StringMsg>("/echo", &request, Duration::from_secs(5))
            .unwrap()
            .1
    );
    assert_eq!(service.calls(), 1);

    let max_ms = u64::from(c_uint::MAX);
    for timeout in [
        Duration::from_millis(max_ms + 1),
        Duration::from_millis(max_ms + 5001),
        Duration::MAX,
    ] {
        let panic = catch_unwind(AssertUnwindSafe(|| {
            node.request::<_, StringMsg>("/echo", &request, timeout)
        }))
        .expect_err("out-of-range timeout must panic");
        let message = panic.downcast_ref::<String>().unwrap();
        assert!(message.contains("Timeout in milliseconds exceeds c_uint::MAX"));
        assert_eq!(service.calls(), 1, "invalid timeout reached the service");
    }
}

struct RawClient {
    node: *mut ffi::Node,
    response: *mut ffi::String,
}

impl RawClient {
    fn new(partition: &str) -> Self {
        let partition = CString::new(partition).unwrap();
        // SAFETY: partition is a valid C string for this call. Both allocations
        // are owned by this client and released in Drop, including on panic.
        let client = unsafe {
            Self {
                node: ffi::nodeCreate(partition.as_ptr()),
                response: ffi::stringCreate(),
            }
        };
        assert!(!client.node.is_null());
        assert!(!client.response.is_null());
        client
    }

    fn request(&mut self, bytes: &[u8]) -> bool {
        let type_name = CString::new(StringMsg::GZ_TYPE_NAME).unwrap();
        let mut result = false;
        // SAFETY: the node and response are live and exclusively borrowed.
        // Strings and exactly bytes.len() request bytes remain valid for the call.
        unsafe {
            ffi::nodeRequest(
                &mut *self.node,
                c"/echo".as_ptr(),
                bytes.as_ptr().cast(),
                bytes.len(),
                type_name.as_ptr(),
                type_name.as_ptr(),
                5000,
                &mut *self.response,
                &mut result,
            )
        }
    }
}

impl Drop for RawClient {
    fn drop(&mut self) {
        // SAFETY: this client uniquely owns both allocations; destruction also
        // accepts null pointers if construction failed.
        unsafe {
            ffi::stringDestroy(&mut self.response);
            ffi::nodeDestroy(&mut self.node);
        }
    }
}

#[test]
fn malformed_requests_never_reach_service() {
    let partition = Uuid::new_v4().to_string();
    let service = TestService::new(&partition);
    let mut client = RawClient::new(&partition);
    let valid = StringMsg {
        data: "valid prefix".into(),
        ..Default::default()
    }
    .write_to_bytes()
    .unwrap();

    // Establish that the service is available, so a timeout cannot hide a
    // parser failure. C++ can partially populate this message before failing.
    assert!(client.request(&valid));
    assert_eq!(service.calls(), 1);

    let mut nul_appended = valid.clone();
    nul_appended.push(0);
    let truncated = &valid[..valid.len() - 1];
    for (case, bytes) in [
        ("appended NUL", nul_appended.as_slice()),
        ("truncated field", truncated),
        ("zero tag", &[0][..]),
    ] {
        let executed = client.request(bytes);
        assert_eq!(service.calls(), 1, "{case} reached the service");
        assert!(!executed, "{case} must fail");
    }

    assert!(client.request(&valid));
    assert_eq!(service.calls(), 2);
}
