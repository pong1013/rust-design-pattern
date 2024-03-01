pub mod observer;
pub mod subject;
pub trait Observer {
    fn update(&mut self, tmp: f64);
    fn get_id(&self) -> usize;
}
