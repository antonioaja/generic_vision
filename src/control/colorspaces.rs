use rgb::RGB;

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
/// The HSV pixel
pub struct HSV<T> {
    /// Hue (in degrees)
    pub h: T,
    /// Saturation (between 0 and 1)
    pub s: T,
    /// Value (between 0 and 1)
    pub v: T,
}
impl HSV<f64> {
    /// Converts an 8-bit rgb pixel into an hsv pixel
    pub fn from_rgb8(rgb_pixel: RGB<u8>) -> HSV<f64> {
        let r_prime = rgb_pixel.r as f64 / 255.0;
        let g_prime = rgb_pixel.g as f64 / 255.0;
        let b_prime = rgb_pixel.b as f64 / 255.0;

        let c_max = r_prime.max(g_prime.max(b_prime));
        let c_min = r_prime.min(g_prime.min(b_prime));

        // Account for floating point error (avoids divide by zero error)
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

        // Accounts for negative degree wrap around
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

    pub fn new(h: f64, s: f64, v: f64) -> HSV<f64> {
        HSV { h, s, v }
    }

    /// Converts an hsv pixel to an 8-bit rbg pixel
    pub fn to_rgb8(hsv_pixel: HSV<f64>) -> Option<RGB<u8>> {
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
            _ => return None,
        };

        // Account for u8 overflow
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

        Some(RGB {
            r: r.round() as u8,
            g: g.round() as u8,
            b: b.round() as u8,
        })
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Luma<T> {
    pub luminance: T,
}
impl<T> Luma<T> {
    /// Converts an 8-bit RGB pixel into a Luma pixel
    pub fn from_rgb8(rgb_pixel: RGB<u8>) -> Luma<T>
    where
        T: std::convert::From<f64>,
    {
        Luma {
            luminance: ((0.299 * (rgb_pixel.r as f64).powi(2)
                + 0.587 * (rgb_pixel.g as f64).powi(2)
                + 0.114 * (rgb_pixel.b as f64).powi(2))
            .sqrt()
            .round())
            .into(),
        }
    }

    /// Creates a Luma pixel
    pub fn new(luminance: T) -> Luma<T> {
        Luma { luminance }
    }

    /// Converts a luma pixel into an 8-bit RGB pixel
    pub fn to_rgb8(luma_pixel: Luma<T>) -> RGB<u8>
    where
        u8: From<T>,
        T: Copy,
    {
        RGB {
            r: luma_pixel.luminance.into(),
            g: luma_pixel.luminance.into(),
            b: luma_pixel.luminance.into(),
        }
    }
}

pub trait Pixel<T> {
    /// Interpret slice as a 2d coordinate system, returning a specific pixel
    fn get_pixel(&self, x: u32, y: u32, w: u32) -> T;
}
impl<T> Pixel<HSV<T>> for Vec<HSV<T>>
where
    T: Copy,
{
    fn get_pixel(&self, x: u32, y: u32, w: u32) -> HSV<T> {
        let one = y * w;
        self[(one + x) as usize]
    }
}
impl<T> Pixel<Luma<T>> for Vec<Luma<T>>
where
    T: Copy,
{
    fn get_pixel(&self, x: u32, y: u32, w: u32) -> Luma<T> {
        let one = y * w;
        self[(one + x) as usize]
    }
}
impl<T> Pixel<RGB<T>> for Vec<RGB<T>>
where
    T: Copy,
{
    fn get_pixel(&self, x: u32, y: u32, w: u32) -> RGB<T> {
        let one = y * w;
        self[(one + x) as usize]
    }
}
