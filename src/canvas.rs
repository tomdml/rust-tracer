use crate::color::Color;

pub struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Color>,
}

impl Canvas {

    pub fn new(width: i32, height: i32) -> Canvas {
        let pixels = vec![ Color { r: 0., g: 0., b: 0. }; (width * height) as usize];

        Canvas { width, height, pixels}
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixels[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x:i32, y: i32, c: Color) {
        assert!(x < self.width);
        assert!(y < self.height);

        self.pixels[(y * self.width + x) as usize] = c;
    }

    fn ppm_header(&self) -> String {
        format!("P3\n{} {}\n255", self.width, self.height)
    }

    fn ppm_body(&self) -> String {
        (0..self.height).map(|row| {
            (0..self.width).map(|col| {
                let Color { r, g, b } = self.get_pixel(col, row);
                format!(
                    "{} {} {}",
                    (r * 255.).ceil() as u8, (g * 255.).ceil() as u8, (b * 255.).ceil() as u8
                )
            }).collect::<Vec<String>>().join(" ")
        }).collect::<Vec<String>>().join("\n")
    }

    #[allow(dead_code)]
    pub fn to_ppm(&self) -> String {
        format!("{}\n{}\n", self.ppm_header(), self.ppm_body())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        assert!(c.pixels.iter().all(|&item| item == c.pixels[0]));

    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        c.set_pixel(2, 3, red);

        assert_eq!(c.get_pixel(2, 3), red);

    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let header = c.ppm_header();

        assert_eq!(header, "P3\n5 3\n255");

    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        let p1 = Color::new(1.5, 0., 0.);
        let p2 = Color::new(0., 0.5, 0.);
        let p3 = Color::new(-0.5, 0., 1.);

        c.set_pixel(0, 0, p1);
        c.set_pixel(2, 1, p2);
        c.set_pixel(4, 2, p3);

        let contents =
"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255";

        let body = c.ppm_body();

        assert_eq!(body, contents);

    }



}
