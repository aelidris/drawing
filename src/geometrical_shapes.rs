use rand::Rng;
use raster::{ Color, Image };

pub trait Drawable {
    fn draw(&self, image: &mut Image);
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
            color: Color::rgb(255, 0, 0),
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
                rng.gen_range(0..=255)
            ),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color.clone());
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
            color: Color::rgb(0, 255, 0),
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
                rng.gen_range(0..=255)
            ),
        }
    }

    // Bresenham's line algorithm
    fn draw_line(&self, image: &mut Image) {
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
            image.display(x0, y0, self.color.clone());

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
    fn draw(&self, image: &mut Image) {
        self.draw_line(image);
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
            color: Color::rgb(255, 255, 255),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        // Draw the three sides of the triangle
        let line1 = Line::new(&self.p1, &self.p2);
        let line2 = Line::new(&self.p2, &self.p3);
        let line3 = Line::new(&self.p3, &self.p1);

        let temp_line1 = Line { start: line1.start, end: line1.end, color: self.color.clone() };
        let temp_line2 = Line { start: line2.start, end: line2.end, color: self.color.clone() };
        let temp_line3 = Line { start: line3.start, end: line3.end, color: self.color.clone() };

        temp_line1.draw(image);
        temp_line2.draw(image);
        temp_line3.draw(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Debug)]
pub struct Rectangle {
    point1: Point,
    point2: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let point1 = Point::new(p1.x, p1.y);
        let point2 = Point::new(p2.x, p2.y);

        Rectangle {
            point1,
            point2,
            color: Color::rgb(255, 255, 255),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.point2.x, self.point1.y);
        let bottom_left = Point::new(self.point1.x, self.point2.y);

        // Draw the four sides of the rectangle
        let mut line1 = Line::new(&self.point1, &top_right);
        let mut line2 = Line::new(&top_right, &self.point2);
        let mut line3 = Line::new(&self.point2, &bottom_left);
        let mut line4 = Line::new(&bottom_left, &self.point1);

        line1.color = self.color.clone();
        line2.color = self.color.clone();
        line3.color = self.color.clone();
        line4.color = self.color.clone();

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
        line4.draw(image);
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
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Circle {
            center: Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
            radius: rng.gen_range(50..350),
            color: Color::rgb(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255)
            ),
        }
    }

    // Midpoint circle algorithm
    fn draw_circle(&self, image: &mut Image) {
        let cx = self.center.x;
        let cy = self.center.y;
        let r = self.radius;

        let mut x = 0;
        let mut y = r;
        let mut d = 1 - r;

        while x <= y {
            // Draw 8 octants
            image.display(cx + x, cy + y, self.color.clone());
            image.display(cx - x, cy + y, self.color.clone());
            image.display(cx + x, cy - y, self.color.clone());
            image.display(cx - x, cy - y, self.color.clone());
            image.display(cx + y, cy + x, self.color.clone());
            image.display(cx - y, cy + x, self.color.clone());
            image.display(cx + y, cy - x, self.color.clone());
            image.display(cx - y, cy - x, self.color.clone());

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
    fn draw(&self, image: &mut Image) {
        self.draw_circle(image);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
