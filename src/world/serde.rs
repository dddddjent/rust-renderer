use serde::de;
use serde::Deserializer;
use simple_math::Vector3;

use crate::util::tools::string2rgb;

pub fn deserialize_color<'de, D>(deserializer: D) -> Result<Vector3<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ColorVisitor;

    impl<'de> serde::de::Visitor<'de> for ColorVisitor {
        type Value = Vector3<u8>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(
                formatter,
                "a string representing a color in the format '#rrggbb' or an array of u8"
            )
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Vector3::new(string2rgb(value)))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            let mut v = Vector3::<u8>::zeros();
            for val in &mut v {
                if let Some(seq_value) = seq.next_element::<u8>()? {
                    *val = seq_value;
                }
            }
            Ok(v)
        }
    }
    deserializer.deserialize_any(ColorVisitor)
}
