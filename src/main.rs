use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(CORNFLOWERBLUE);
    Segment::new(vec2(app.time, 0.0), 100.0, app.time).render(&draw);
    draw.to_frame(app, &frame).unwrap();
}

trait Render {
    fn render(&self, draw: &Draw);
}

struct Segment {
    start: Vec2,
    length: f32,
    angle: f32,
}

impl Segment {
    fn new(start: Vec2, length: f32, angle: f32) -> Self {
        Self {
            start,
            length,
            angle,
        }
    }
}

impl Render for Segment {
    fn render(&self, draw: &Draw) {
        let x = self.length * self.angle.cos() + self.start.x;
        let y = self.length * self.angle.sin() + self.start.y;
        draw.line().start(self.start).end(vec2(x, y));
    }
}
