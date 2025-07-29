use rand::Rng;
use raster::Color;

pub trait Drawable {
    fn draw<T: Displayable>(&self, display: &mut T);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
            color: Color::rgb(255, 0, 0), // Red by default
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            color: Color::rgb(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ),
        }
    }
}

impl Drawable for Point {
    fn draw<T: Displayable>(&self, display: &mut T) {
        display.display(self.x, self.y, self.color.clone());
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
    color: Color,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: start.clone(),
            end: end.clone(),
            color: Color::rgb(0, 255, 0), // Green by default
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Line {
            start: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            end: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            color: Color::rgb(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ),
        }
    }

    // Bresenham's line algorithm
    fn draw_line<T: Displayable>(&self, display: &mut T) {
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            display.display(x0, y0, self.color.clone());

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}

impl Drawable for Line {
    fn draw<T: Displayable>(&self, display: &mut T) {
        self.draw_line(display);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    color: Color,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: p1.clone(),
            p2: p2.clone(),
            p3: p3.clone(),
            color: Color::rgb(0, 0, 255), // Blue by default
        }
    }
}

impl Drawable for Triangle {
    fn draw<T: Displayable>(&self, display: &mut T) {
        // Draw the three sides of the triangle
        let line1 = Line::new(&self.p1, &self.p2);
        let line2 = Line::new(&self.p2, &self.p3);
        let line3 = Line::new(&self.p3, &self.p1);

        let temp_line1 = Line { start: line1.start, end: line1.end, color: self.color.clone() };
        let temp_line2 = Line { start: line2.start, end: line2.end, color: self.color.clone() };
        let temp_line3 = Line { start: line3.start, end: line3.end, color: self.color.clone() };

        temp_line1.draw(display);
        temp_line2.draw(display);
        temp_line3.draw(display);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        // Determine which point is top-left and which is bottom-right
        let top_left = Point::new(p1.x.min(p2.x), p1.y.min(p2.y));
        let bottom_right = Point::new(p1.x.max(p2.x), p1.y.max(p2.y));
        
        Rectangle {
            top_left,
            bottom_right,
            color: Color::rgb(255, 255, 0), // Yellow by default
        }
    }
}

impl Drawable for Rectangle {
    fn draw<T: Displayable>(&self, display: &mut T) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        // Draw the four sides of the rectangle
        let mut line1 = Line::new(&self.top_left, &top_right);
        let mut line2 = Line::new(&top_right, &self.bottom_right);
        let mut line3 = Line::new(&self.bottom_right, &bottom_left);
        let mut line4 = Line::new(&bottom_left, &self.top_left);

        line1.color = self.color.clone();
        line2.color = self.color.clone();
        line3.color = self.color.clone();
        line4.color = self.color.clone();

        line1.draw(display);
        line2.draw(display);
        line3.draw(display);
        line4.draw(display);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Circle {
    center: Point,
    radius: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
            color: Color::rgb(255, 0, 255), // Magenta by default
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Circle {
            center: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            radius: rng.gen_range(5..50),
            color: Color::rgb(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
            ),
        }
    }

    // Midpoint circle algorithm
    fn draw_circle<T: Displayable>(&self, display: &mut T) {
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = r;
        let mut d = 1 - r;

        while x <= y {
            // Draw 8 octants
            display.display(cx + x, cy + y, self.color.clone());
            display.display(cx - x, cy + y, self.color.clone());
            display.display(cx + x, cy - y, self.color.clone());
            display.display(cx - x, cy - y, self.color.clone());
            display.display(cx + y, cy + x, self.color.clone());
            display.display(cx - y, cy + x, self.color.clone());
            display.display(cx + y, cy - x, self.color.clone());
            display.display(cx - y, cy - x, self.color.clone());

            if d < 0 {
                d += 2 * x + 3;
            } else {
                d += 2 * (x - y) + 5;
                y -= 1;
            }
            x += 1;
        }
    }
}

impl Drawable for Circle {
    fn draw<T: Displayable>(&self, display: &mut T) {
        self.draw_circle(display);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}