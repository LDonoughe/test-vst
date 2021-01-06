pub fn sin(a : f32,f : f32,t : f32) -> f32 {
  a*(2.0*pi()*f*t)
}
pub fn _saw(velocity : f32, x : f32, p: f32) -> f32 {
  (-2.0*(f32::from(velocity)/pi()) * (x*pi()/p).cot().atan())/2.0
}

fn pi() -> f32 {
  std::f32::consts::PI
}

// Cotangent is Cos/Sin
trait Cotangent {
  fn cot(&self) -> f32;
}

impl Cotangent for f32 {
  fn cot(&self) -> f32 {
    self.cos()/self.sin() as f32
  }
}
