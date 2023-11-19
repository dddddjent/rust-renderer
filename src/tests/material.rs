#[test]
fn deserialize_material() {
    use crate::world::material::Material;
    let data_str = "
            {\
				\"type\": \"rough_metal\",\
				\"data\": {\
					\"color\": \"#ffeedd\",\
                    \"rough_index\": 0.2\
				}\
			}";
    match serde_json::from_str(data_str).unwrap() {
        Material::RoughMetal { data } => {
            assert_eq!(data.color, [255, 238, 221]);
            assert_eq!(data.rough_index, 0.2f32);
        }
        _ => panic!("Not rough metal?"),
    }

    let data_str = "
            {\
				\"type\": \"isotropic_light\",\
				\"data\": {\
					\"color\": [255,238,    221],\
                    \"brightness\": 0.2\
				}\
			}";
    match serde_json::from_str(data_str).unwrap() {
        Material::IsotropicLight { data } => {
            assert_eq!(data.color, [255, 238, 221]);
            assert_eq!(data.brightness, 0.2f32);
        }
        _ => panic!("Not isotropic light?"),
    }
}
