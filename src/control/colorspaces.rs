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

    pub fn from_rgb(rgb_pixel: RGB<u8>) -> HSV {
        let r_prime = rgb_pixel.r as f64 / 255.0;
        let g_prime = rgb_pixel.g as f64 / 255.0;
        let b_prime = rgb_pixel.b as f64 / 255.0;

        let c_max = r_prime.max(g_prime.max(b_prime));
        let c_min = r_prime.min(g_prime.min(b_prime));

        let delta = c_max - c_min;

        let hue = if c_max == r_prime {
            60.0 * (((g_prime - b_prime) / delta) % 6.0)
        } else if c_max == g_prime {
            60.0 * (((b_prime - r_prime) / delta) + 2.0)
        } else if c_max == b_prime {
            60.0 * (((r_prime - g_prime) / delta) + 4.0)
        } else {
            0.0
        };

        let saturation = if c_max == 0.0 { 0.0 } else { delta / c_max };

        HSV {
            h: hue,
            s: saturation,
            v: c_max,
        }
    }
}
