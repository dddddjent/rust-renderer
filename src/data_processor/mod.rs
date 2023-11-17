pub trait DataProcessor {
    fn new() -> Self;
    fn init(&mut self) -> &mut Self;
    fn read<E>(data_path: &str) -> Result<(), E>;
    fn set_args();
    fn set_buffer();
    fn process<E>() -> Result<(), E>;
    fn get();
    fn write(output_path: &str);
}
