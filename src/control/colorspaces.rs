use rgb::RGB;

#[derive(Copy, Clone, Debug, Default)]
/// The HSV pixel
pub struct HSV {
    /// Hue (in degrees)
    pub h: f64,
    /// Saturation (between 0 and 1)
    pub s: f64,
    /// Value (between 0 and 1)
    pub v: f64,
}
impl HSV {
    pub fn new(h: f64, s: f64, v: f64) -> HSV {
        HSV { h, s, v }
    }

    pub fn from_rgb8(rgb_pixel: RGB<u8>) -> HSV {
        let r_prime = rgb_pixel.r as f64 / 255.0;
        let g_prime = rgb_pixel.g as f64 / 255.0;
        let b_prime = rgb_pixel.b as f64 / 255.0;

        let c_max = r_prime.max(g_prime.max(b_prime));
        let c_min = r_prime.min(g_prime.min(b_prime));

        let delta = if c_max - c_min == 0.0 {
            f64::MIN_POSITIVE
        } else {
            c_max - c_min
        };

        let mut hue = if delta == 0.0 {
            0.0
        } else if c_max == r_prime {
            60.0 * (((g_prime - b_prime) / delta) % 6.0)
        } else if c_max == g_prime {
            60.0 * (((b_prime - r_prime) / delta) + 2.0)
        } else if c_max == b_prime {
            60.0 * (((r_prime - g_prime) / delta) + 4.0)
        } else {
            0.0
        };

        if hue < 0.0 {
            hue += 360.0;
        }

        let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };

        HSV {
            h: hue,
            s: saturation,
            v: c_max,
        }
    }

    pub fn to_rgb8(hsv_pixel: HSV) -> RGB<u8> {
        let c = hsv_pixel.v * hsv_pixel.s;
        let x = c * (1.0 - ((hsv_pixel.h / 60.0) % 2.0 - 1.0).abs());
        let m = hsv_pixel.v - c;

        let (r_prime, g_prime, b_prime) = match hsv_pixel.h as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0),
        };

        let r = if (r_prime + m) * 255.0 > 255.0 {
            0.0
        } else {
            (r_prime + m) * 255.0
        };

        let g = if (g_prime + m) * 255.0 > 255.0 {
            0.0
        } else {
            (g_prime + m) * 255.0
        };

        let b = if (b_prime + m) * 255.0 > 255.0 {
            0.0
        } else {
            (b_prime + m) * 255.0
        };

        RGB {
            r: r.round() as u8,
            g: g.round() as u8,
            b: b.round() as u8,
        }
    }
}
