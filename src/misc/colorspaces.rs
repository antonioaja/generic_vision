use num_traits::cast::AsPrimitive;
use num_traits::Bounded;
use rgb::RGB;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(
    Copy, Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize, Eq, Hash, Ord,
)]
/// The HSV pixel
pub struct HSV<T> {
    /// Hue (in degrees)
    pub h: T,
    /// Saturation (between 0 and 1)
    pub s: T,
    /// Value (between 0 and 1)
    pub v: T,
}
impl<N: std::convert::Into<f64> + Copy + 'static> HSV<N> {
    /// Converts an rgb pixel into an hsv pixel
    pub fn from_rgb<T: std::convert::Into<f64> + Bounded>(rgb_pixel: RGB<T>) -> HSV<N>
    where
        f64: AsPrimitive<N>,
    {
        let r_prime: f64 = rgb_pixel.r.into() / T::max_value().into();
        let g_prime: f64 = rgb_pixel.g.into() / T::max_value().into();
        let b_prime: f64 = rgb_pixel.b.into() / T::max_value().into();

        let c_max: f64 = r_prime.max(g_prime.max(b_prime));
        let c_min: f64 = r_prime.min(g_prime.min(b_prime));

        // Account for floating point error (avoids divide by zero error)
        let delta: f64 = if c_max - c_min == 0.0 {
            f64::MIN_POSITIVE
        } else {
            c_max - c_min
        };

        let mut hue: f64 = if delta == 0.0 {
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

        let saturation: f64 = if c_max == 0.0 { 0.0 } else { delta / c_max };

        HSV {
            h: hue.as_(),
            s: saturation.as_(),
            v: c_max.as_(),
        }
    }

    pub fn new(h: N, s: N, v: N) -> HSV<N> {
        HSV { h, s, v }
    }

    /// Converts an hsv pixel to an rbg pixel
    pub fn to_rgb<T: std::marker::Copy + 'static + Bounded + std::convert::Into<f64>>(
        hsv_pixel: HSV<N>,
    ) -> Option<RGB<T>>
    where
        f64: AsPrimitive<T>,
    {
        let c: f64 = hsv_pixel.v.into() * hsv_pixel.s.into();
        let x: f64 = c * (1.0 - ((hsv_pixel.h.into() / 60.0) % 2.0 - 1.0).abs());
        let m: f64 = hsv_pixel.v.into() - c;

        let (r_prime, g_prime, b_prime) = match hsv_pixel.h.into() as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => return None,
        };

        // Account for overflow
        let r: f64 = if (r_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (r_prime + m) * T::max_value().into()
        };
        let g: f64 = if (g_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (g_prime + m) * T::max_value().into()
        };
        let b: f64 = if (b_prime + m) * T::max_value().into() > T::max_value().into() {
            0.0
        } else {
            (b_prime + m) * T::max_value().into()
        };

        Some(RGB {
            r: r.round().as_(),
            g: g.round().as_(),
            b: b.round().as_(),
        })
    }
}

#[derive(
    Copy, Clone, Debug, Default, PartialEq, PartialOrd, Deserialize, Serialize, Eq, Hash, Ord,
)]
pub struct Luma<T> {
    pub luminance: T,
}
impl<T> Luma<T> {
    /// Converts an RGB pixel into a Luma pixel
    pub fn from_rgb<N: std::convert::Into<f64>>(rgb_pixel: RGB<N>) -> Luma<T>
    where
        T: std::convert::Into<f64> + std::marker::Copy + 'static,
        f64: AsPrimitive<T>,
    {
        Luma {
            luminance: ((0.299 * (rgb_pixel.r.into()).powi(2)
                + 0.587 * (rgb_pixel.g.into()).powi(2)
                + 0.114 * (rgb_pixel.b.into()).powi(2))
            .sqrt()
            .round())
            .as_(),
        }
    }

    /// Creates a Luma pixel
    pub fn new(luminance: T) -> Luma<T> {
        Luma { luminance }
    }

    /// Converts a luma pixel into an RGB pixel
    pub fn to_rgb<N: std::convert::From<T>>(luma_pixel: Luma<T>) -> RGB<N>
    where
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