pub struct Filter {
    b: [f32; 76],
    x: [f32; 76],
}

impl Filter {
    pub fn new(b: [f32; 76]) -> Filter {
        Filter { b: b, x: [0.0; 76] }
    }

    pub fn get_average(&self) -> f32 {
        self.x.iter().fold(0.0, |sum, x| sum + x) / self.x.len() as f32
    }

    pub fn filter(&mut self, x: f32) -> f32 {
        for i in (0..76 - 1).rev() {
            self.x[i + 1] = self.x[i];
        }
        self.x[0] = x;
        let mut y = 0.0;
        for i in 0..self.b.len() {
            y = y + self.b[i] * self.x[i];
        }
        y
    }
}
