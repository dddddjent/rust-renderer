#[test]
fn deserialize_camera() {
    use crate::world::camera::Camera;
    use simple_math::Vector3f;

    let camera_string = "
        {
            \"position\": [20.0, 5.0, 5.0],\
            \"direction\": [-1.0, 0.0, 0.0],\
            \"up\": [0.0, 0.0, 1.0],\
            \"distance\": 1.0,\
            \"fov\": 60.0,\
            \"size\": [1024, 1024]
        }
    ";

    let camera: Camera = serde_json::from_str(camera_string).unwrap();
    let camera_result = Camera {
        position: Vector3f::new([20f32, 5f32, 5f32]),
        direction: Vector3f::new([-1f32, 0f32, 0f32]),
        up: Vector3f::new([0f32, 0f32, 1f32]),
        distance: 1f32,
        fov: 60f32,
        size: (1024, 1024),
    };
    assert_eq!(camera.position, camera_result.position);
    assert_eq!(camera.direction, camera_result.direction);
    assert_eq!(camera.up, camera_result.up);
    assert_eq!(camera.distance, camera_result.distance);
    assert_eq!(camera.fov, camera_result.fov);
    assert_eq!(camera.size, camera_result.size);
}
