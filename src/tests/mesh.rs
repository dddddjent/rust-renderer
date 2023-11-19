#[test]
fn deserialize_mesh() {
    use crate::world::mesh::Mesh;

    let data_str = "
            {\
				\"type\": \"sphere\",\
				\"data\": {\
					\"center\": [255,10,88],\
                    \"radius\": 0.2\
				}\
			}";

    match serde_json::from_str(data_str).unwrap() {
        Mesh::Sphere { data } => {
            assert_eq!(data.center, [255f32, 10f32, 88f32]);
            assert_eq!(data.radius, 0.2f32);
        }
        _ => panic!("Not a sphere?"),
    };
}
