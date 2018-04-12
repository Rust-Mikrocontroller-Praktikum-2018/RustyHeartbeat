use circular_buffer::CircularBuffer;
use stm32f7::lcd::{Color, Lcd, WindowLayer};

pub struct HeartBeatWriter {
    points: CircularBuffer,
    layer: WindowLayer,
    width: usize,
}

impl HeartBeatWriter {
    pub fn new(layer: WindowLayer, width: usize) -> HeartBeatWriter {
        let points = CircularBuffer::new();
        HeartBeatWriter {
            layer: layer,
            points: points,
            width: width,
        }
    }

    pub fn add_new_data(&mut self, lcd: &mut Lcd, data: usize) {
        self.layer.next_frame(lcd);

        let position = self.layer.get_position();
        if self.points.len() == self.width {
            let old_data = self.points.pop_front();
            self.layer.print_point_color_absoulte(
                position + self.width,
                old_data,
                Color::from_argb8888(0),
            );
            self.layer
                .print_point_color_absoulte(position, old_data, Color::from_argb8888(0));
        }
        self.points.push_back(data);

        self.layer
            .print_point_color_absoulte(position, data, Color::rgb(255, 255, 255));
        self.layer.print_point_color_absoulte(
            position + self.width,
            data,
            Color::rgb(255, 255, 255),
        );
    }
}
