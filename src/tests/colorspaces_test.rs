use anyhow::*;
use rayon::prelude::*;
use rgb::RGB;

use crate::control::colorspaces::HSV;

#[test]
fn rgb8_to_hsv_f64() -> Result<()> {
    (0..=u8::MAX).into_par_iter().for_each(|r| {
        (0..=u8::MAX).for_each(|g| {
            (0..=u8::MAX).for_each(|b| {
                let original = RGB::new(r, g, b);
                let intermediate: HSV<f64> = HSV::from_rgb::<u8>(original);
                let final_out = HSV::<f64>::to_rgb(intermediate)
                    .context("Could not convert from HSV to RGB!")
                    .unwrap();

                assert_eq!(original, final_out);
            });
        });
    });

    Ok(())
}

#[test]
fn rgb8_to_hsv_f32() -> Result<()> {
    (0..=u8::MAX).into_par_iter().for_each(|r| {
        (0..=u8::MAX).for_each(|g| {
            (0..=u8::MAX).for_each(|b| {
                let original = RGB::new(r, g, b);
                let intermediate: HSV<f32> = HSV::from_rgb::<u8>(original);
                let final_out = HSV::<f32>::to_rgb(intermediate)
                    .context("Could not convert from HSV to RGB!")
                    .unwrap();

                assert_eq!(original, final_out);
            });
        });
    });

    Ok(())
}

// Impossibly long to test
// #[test]
// fn rgb16_to_hsv32() -> Result<()> {
//     (0..=u16::MAX).into_par_iter().for_each(|r| {
//         (0..=u16::MAX).for_each(|g| {
//             (0..=u16::MAX).for_each(|b| {
//                 let original = RGB::new(r, g, b);
//                 let intermediate: HSV<f32> = HSV::from_rgb::<u16>(original);
//                 let final_out = HSV::<f32>::to_rgb(intermediate)
//                     .context("Could not convert from HSV to RGB!")
//                     .unwrap();

//                 assert_eq!(original, final_out);
//             });
//         });
//     });

//     Ok(())
// }

// Impossibly long to test
// #[test]
// fn rgb16_to_hsv64() -> Result<()> {
//     (0..=u16::MAX).into_par_iter().for_each(|r| {
//         (0..=u16::MAX).for_each(|g| {
//             (0..=u16::MAX).for_each(|b| {
//                 let original = RGB::new(r, g, b);
//                 let intermediate: HSV<f64> = HSV::from_rgb::<u16>(original);
//                 let final_out = HSV::<f64>::to_rgb(intermediate)
//                     .context("Could not convert from HSV to RGB!")
//                     .unwrap();

//                 assert_eq!(original, final_out);
//             });
//         });
//     });

//     Ok(())
// }