
#[macro_export]
macro_rules! rgb {
    ($r:expr,$g:expr,$b:expr,$s:expr) => {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", $r, $g, $b, $s)
    };
}

#[macro_export]
macro_rules! rgbbg {
    ($r:expr,$g:expr,$b:expr,$s:expr) => {
        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", $r, $g, $b, $s)
    };
}

pub mod colors {
    use std::f64::consts::PI;

    pub struct RGB {
        pub r: u8,
        pub g: u8,
        pub b: u8
    }
    
    pub struct HSV {
        pub h: u16,
        pub s: u8,
        pub v: u8
    }

    fn radians_to_degrees(radians: f64) -> f64 {
        return radians * PI / 180.0;
    }
    
    fn degrees_to_radians(degrees: f64) -> f64 {
        return degrees * 180.0 / PI;
    }

    pub fn rgb_to_hsv(rgb: RGB) -> HSV {
        let rp = rgb.r as f64 / 255.0;
        let gp = rgb.g as f64 / 255.0;
        let bp = rgb.b as f64 / 255.0;

        let c_max = rp.max(gp).max(bp);
        let c_min = rp.min(gp).min(bp);

        let diff = c_max - c_min;

        let sixty_degrees = degrees_to_radians(60.0);

        // get hue
        let hue = if diff == 0.0 {
            0.0
        } else if c_max == rp {
            sixty_degrees * (((gp - bp) / diff) % 6.0)
        } else if c_max == gp {
            sixty_degrees * (((bp - rp) / diff) + 2.0)
        } else {
            sixty_degrees * (((rp - gp) / diff) + 4.0)
        };

        // get saturation
        let sat = if c_max == 0.0 { 0.0 } else { diff / c_max };

        // get value
        let val = c_max;

        return HSV {
            h: radians_to_degrees(hue).round() as u16,
            s: (sat * 100.0).round() as u8,
            v: (val * 100.0).round() as u8
        };
    }

    pub fn hsv_to_rgb(hsv: HSV) -> RGB {
        let hue: f64 = hsv.h as f64;
        let sat: f64 = hsv.s as f64 / 100.0;
        let val: f64 = hsv.v as f64 / 100.0;

        let c = val * sat;

        let x = c * (1.0 - (((degrees_to_radians(hue) / degrees_to_radians(60.0)) % 2.0) - 1.0).abs());

        let m = val - c;

        let (rp, gp, bp) = if 0.0 <= hue && hue < 60.0 {
            (c, x, 0.0)
        } else if 60.0 <= hue && hue < 120.0 {
            (x, c, 0.0)
        } else if 120.0 <= hue && hue < 180.0 {
            (0.0, c, x)
        } else if 180.0 <= hue && hue < 240.0 {
            (0.0, x, c)
        } else if 240.0 <= hue && hue < 300.0 {
            (x, 0.0, c)
        } else if 300.0 <= hue && hue < 360.0 {
            (c, 0.0, x)
        } else {
            panic!("hue must be between 0 and 360");
        };

        return RGB {
            r: ((rp + m) * 255.0).round() as u8,
            g: ((gp + m) * 255.0).round() as u8,
            b: ((bp + m) * 255.0).round() as u8
        };
    }
}
