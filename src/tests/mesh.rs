#[test]
fn parse_mesh() {
    use crate::world::mesh::Mesh;

    let data_str = "
            {\
				\"type\": \"sphere\",\
				\"data\": {\
					\"center\": [255,10,88],\
                    \"radius\": 0.2\
				}\
			}";
    let mesh: Box<dyn Mesh> = serde_json::from_str(data_str).unwrap();
}
