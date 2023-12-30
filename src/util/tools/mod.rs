use simple_math::{Vector3f, Vector3u8};

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

#[inline]
pub fn v3u8_to_v3f(v3u8: &Vector3u8) -> Vector3f {
    Vector3f::new([*v3u8.x() as f32, *v3u8.y() as f32, *v3u8.z() as f32])
}

#[inline]
pub fn v3f_to_v3u8(v3f: &Vector3f) -> Vector3u8 {
    Vector3u8::new([*v3f.x() as u8, *v3f.y() as u8, *v3f.z() as u8])
}

#[inline]
pub const fn ray_collision_epsilon() -> f32 {
    1e-4
}
