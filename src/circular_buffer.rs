pub struct CircularBuffer {
    points: [usize; 480],
    start: usize,
    end: usize,
    size: usize,
}

impl CircularBuffer {
    pub fn new() -> CircularBuffer {
        CircularBuffer {
            points: [0; 480],
            start: 0,
            end: 0,
            size: 0,
        }
    }

    pub fn push_back(&mut self, point: usize) {
        assert!(self.len() < self.points.len());
        self.points[self.end % self.points.len()] = point;
        self.end = (self.end + 1) % self.points.len();
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> usize {
        assert!(self.len() > 0);
        let index = self.start;
        self.start = (self.start + 1) % self.points.len();
        self.size -= 1;
        self.points[index]
    }

    pub fn len(&self) -> usize {
        self.size
    }
}
