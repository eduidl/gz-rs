// Exexute with gz sim Quadcopter Teleop

use std::time::Duration;

use gz::{
    msgs::{
        boolean::Boolean, pose::Pose, protobuf::MessageField, quaternion::Quaternion,
        vector3d::Vector3d,
    },
    transport::Node,
};

fn main() {
    let mut node = Node::new().unwrap();

    let x = 0.;
    let y = 1.5;
    let z = 0.;

    let (res, result) = node
        .request::<_, Boolean>(
            "/world/quadcopter_teleop/set_pose",
            &Pose {
                name: "X3".to_string(),
                position: MessageField::some(Vector3d {
                    x,
                    y,
                    z,
                    ..Default::default()
                }),
                orientation: MessageField::some(Quaternion {
                    w: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            },
            Duration::from_secs(1),
        )
        .unwrap();

    assert!(result);

    dbg!(res);
}
