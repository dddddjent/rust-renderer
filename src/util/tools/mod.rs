pub fn string2rgb(color_str: &str) -> [u8; 3] {
    assert_eq!(color_str.len(), 7);
    assert_eq!(&color_str[0..1], "#");
    let color_parsed: u32 = u32::from_str_radix(&color_str[1..7], 16).unwrap();
    [
        ((color_parsed & 0x00ff0000) >> 16) as u8,
        ((color_parsed & 0x0000ff00) >> 8) as u8,
        (color_parsed & 0x000000ff) as u8,
    ]
}
