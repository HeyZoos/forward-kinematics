use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(CORNFLOWERBLUE);
    let root = Segment::root(vec2(app.time, 0.0), 100.0, app.time);
    let s1 = Segment::cont(Box::new(root), 100.0, deg_to_rad(45.0));
    s1.render(&draw);
    draw.to_frame(app, &frame).unwrap();
}

trait Render {
    fn render(&self, draw: &Draw);
}

struct Segment {
    start: Vec2,
    length: f32,
    angle: f32,
    parent: Option<Box<Segment>>,
}

impl Segment {
    fn root(start: Vec2, length: f32, angle: f32) -> Self {
        Self {
            start,
            length,
            angle,
            parent: None,
        }
    }

    fn cont(parent: Box<Segment>, length: f32, angle: f32) -> Self {
        Self {
            length,
            angle,
            start: vec2(0.0, 0.0),
            parent: Some(parent),
        }
    }

    fn start(&self) -> Vec2 {
        if let Some(parent) = &self.parent {
            parent.end()
        } else {
            self.start
        }
    }

    fn end(&self) -> Vec2 {
        let x = self.length * self.angle.cos() + self.start().x;
        let y = self.length * self.angle.sin() + self.start().y;
        vec2(x, y)
    }
}

impl Render for Segment {
    fn render(&self, draw: &Draw) {
        let mut drawing = draw.line();

        drawing = if let Some(parent) = &self.parent {
            parent.render(draw);
            drawing.color(YELLOW)
        } else {
            drawing.color(RED)
        };

        drawing.start(self.start()).end(self.end());
    }
}
