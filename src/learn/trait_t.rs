use std::f64::consts;
pub trait HasArea{
    fn area(&self) -> f64;
}
pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}
impl HasArea for Circle {
    pub fn area(&self) -> f64 {
        consts::PI * (self.radius * self.radius)
    }
}
