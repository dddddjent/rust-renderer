#[test]
fn parse_material() {
    use crate::world::material::Material;
    let data_str = "
            {\
				\"type\": \"rough_metal\",\
				\"data\": {\
					\"color\": [255,10,88],\
                    \"rough_index\": 0.2\
				}\
			}";
    let mat: Material = serde_json::from_str(data_str).unwrap();
    match mat {
        Material::RoughMetal { data } => {
            assert_eq!(data.color, [255, 10, 88]);
            assert_eq!(data.rough_index, 0.2f32);
        }
        _ => panic!("Not rough metal?"),
    }
}
