pub trait Event {
    fn name(&self) -> &str;
    fn payload(&self) -> &str;
    fn version(&self) -> u16;
    fn id(&self) -> &str;
}
