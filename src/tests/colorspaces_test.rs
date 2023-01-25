use anyhow::*;
use rgb::RGB;

use crate::control::colorspaces::HSV;

#[test]
fn rgb2hsv() -> Result<()>{
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let original = RGB::new(r, g, b);
                let intermediate = HSV::from_rgb8(original);
                let final_out = HSV::to_rgb8(intermediate);

                ensure!(original == final_out, format!("Conversion failed!\n{:?} != {:?}",original, final_out));
            }
        }
    }

    Ok(())
}