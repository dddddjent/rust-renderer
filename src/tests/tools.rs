#[test]
fn rgb_conversion() {
    use crate::util::tools::string2rgb;
    assert_eq!(string2rgb("#daa520"), [218, 165, 32]);
    assert_eq!(string2rgb(&String::from("#ff69b4")), [255, 105, 180]);
}
