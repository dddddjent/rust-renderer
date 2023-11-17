pub trait Renderer {
    fn new() -> Self;
    fn set_args();
    fn set_camera();
    fn set_world();
    fn step();
}
