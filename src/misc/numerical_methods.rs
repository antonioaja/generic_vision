/// A simple & generic bisection method
pub struct BisectionMethod {
    min: [f64; 2],
    max: [f64; 2],
}
impl BisectionMethod {
    pub fn new(min: [f64; 2], max: [f64; 2]) -> Self {
        Self { min, max }
    }

    /// Returns the midpoint of the current min/max values
    pub fn calc_midpoint(&self) -> f64 {
        (self.min[0] + self.max[0]) / 2.0
    }

    /// Does an iteration of the method, returning a point closest to 0
    pub fn iterate(&mut self, input: f64) -> [f64; 2] {
        let midpoint = self.calc_midpoint();

        if self.min[1] * input < 0.0 {
            self.max[0] = midpoint;
            self.max[1] = input;

            if input - self.min[1] > 0.0 {
                return self.min;
            }
        } else if self.max[1] * input < 0.0 {
            self.min[0] = midpoint;
            self.min[1] = input;

            if input - self.max[1] > 0.0 {
                return self.max;
            }
        }

        [midpoint, input]
    }
}
