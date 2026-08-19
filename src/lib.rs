mod config;
use std::ptr::NonNull;

use aviutl2::{lprintln, module::ScriptModuleFunctions};
use config::{AlphaMode, ParamsJson};
use ntscrs::{
    settings::standard::NtscEffect,
    yiq_fielding::{Rgb, Rgbx},
};

fn multiply_channel_by_alpha(channel: u8, alpha: u8) -> u8 {
    ((u16::from(channel) * u16::from(alpha) + 127) / 255) as u8
}

fn multiply_rgb_by_alpha(pixel: &mut [u8]) {
    let alpha = pixel[3];
    for channel in &mut pixel[..3] {
        *channel = multiply_channel_by_alpha(*channel, alpha);
    }
}

fn grayscale_alpha(rgb: &[u8]) -> u8 {
    ((u32::from(rgb[0]) * 299 + u32::from(rgb[1]) * 587 + u32::from(rgb[2]) * 114 + 500) / 1000)
        as u8
}

fn alpha_mask(image_data: &[u8]) -> Vec<u8> {
    let mut mask = Vec::with_capacity(image_data.len() / 4 * 3);
    for pixel in image_data.chunks_exact(4) {
        mask.extend_from_slice(&[pixel[3]; 3]);
    }
    mask
}

fn apply_effect(
    effect: &NtscEffect,
    alpha_mode: AlphaMode,
    dimensions: (usize, usize),
    image_data: &mut [u8],
    current_frame: usize,
) {
    match alpha_mode {
        AlphaMode::Multiply => {
            for pixel in image_data.chunks_exact_mut(4) {
                multiply_rgb_by_alpha(pixel);
            }
            effect.apply_effect_to_buffer::<Rgbx, _>(
                dimensions,
                image_data,
                current_frame,
                [1.0, 1.0],
            );
        }
        AlphaMode::Grayscale => {
            let mut mask = alpha_mask(image_data);
            effect.apply_effect_to_buffer::<Rgbx, _>(
                dimensions,
                image_data,
                current_frame,
                [1.0, 1.0],
            );
            effect.apply_effect_to_buffer::<Rgb, _>(
                dimensions,
                &mut mask,
                current_frame,
                [1.0, 1.0],
            );

            for (pixel, mask_pixel) in image_data.chunks_exact_mut(4).zip(mask.chunks_exact(3)) {
                let alpha = grayscale_alpha(mask_pixel);
                pixel[3] = alpha;
                multiply_rgb_by_alpha(pixel);
            }
        }
    }
}

#[aviutl2::plugin(ScriptModule)]
struct NtscModule;

impl aviutl2::module::ScriptModule for NtscModule {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        Ok(Self)
    }
    fn plugin_info(&self) -> aviutl2::module::ScriptModuleTable {
        aviutl2::module::ScriptModuleTable {
            information: format!(
                "VHS-like effect, powered by ntsc-rs, written in Rust / v{version} / https://github.com/sevenc-nanashi/ntsc-rs.anm2",
                version = env!("CARGO_PKG_VERSION")
            ),
            functions: NtscModule::functions(),
        }
    }
}

#[aviutl2::module::functions]
impl NtscModule {
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn process(
        &self,
        image: NonNull<u8>,
        width: usize,
        height: usize,
        current_frame: usize,
        params: String,
    ) -> aviutl2::AnyResult<()> {
        let current = std::time::Instant::now();
        let params = serde_json::from_str::<ParamsJson>(&params)
            .map_err(|e| aviutl2::anyhow::anyhow!("Failed to parse params JSON: {e}"))?;
        let effect = params.to_ntsc_effect();
        let image_data =
            unsafe { std::slice::from_raw_parts_mut(image.as_ptr(), width * height * 4) };
        apply_effect(
            &effect,
            params.alpha_mode,
            (width, height),
            image_data,
            current_frame,
        );
        lprintln!(
            verbose,
            "[ntsc-rs] Processed frame {} in {:?}",
            current_frame,
            current.elapsed()
        );

        Ok(())
    }
}

aviutl2::register_script_module!(NtscModule);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplies_rgb_by_alpha_with_rounding() {
        let mut transparent = [255, 128, 1, 0];
        multiply_rgb_by_alpha(&mut transparent);
        assert_eq!(transparent, [0, 0, 0, 0]);

        let mut semitransparent = [255, 128, 1, 128];
        multiply_rgb_by_alpha(&mut semitransparent);
        assert_eq!(semitransparent, [128, 64, 1, 128]);

        let mut opaque = [255, 128, 1, 255];
        multiply_rgb_by_alpha(&mut opaque);
        assert_eq!(opaque, [255, 128, 1, 255]);
    }

    #[test]
    fn creates_a_grayscale_mask_from_alpha() {
        let image = [10, 20, 30, 0, 40, 50, 60, 128, 70, 80, 90, 255];
        assert_eq!(alpha_mask(&image), [0, 0, 0, 128, 128, 128, 255, 255, 255]);
    }

    #[test]
    fn converts_processed_mask_to_grayscale() {
        assert_eq!(grayscale_alpha(&[0, 0, 0]), 0);
        assert_eq!(grayscale_alpha(&[255, 255, 255]), 255);
        assert_eq!(grayscale_alpha(&[255, 0, 0]), 76);
        assert_eq!(grayscale_alpha(&[0, 255, 0]), 150);
        assert_eq!(grayscale_alpha(&[0, 0, 255]), 29);
    }

    #[test]
    fn multiply_mode_outputs_opaque_alpha() {
        let effect = NtscEffect::default();
        let mut image = (0..16 * 16)
            .flat_map(|index| [255, 128, 64, index as u8])
            .collect::<Vec<_>>();

        apply_effect(&effect, AlphaMode::Multiply, (16, 16), &mut image, 42);

        assert!(image.chunks_exact(4).all(|pixel| pixel[3] == 255));
    }

    #[test]
    fn grayscale_mode_is_deterministic_and_returns_premultiplied_rgba() {
        let effect = NtscEffect::default();
        let mut first = (0..16 * 16)
            .flat_map(|index| {
                let alpha = index as u8;
                [index as u8, (index * 3) as u8, (index * 7) as u8, alpha]
            })
            .collect::<Vec<_>>();
        let mut second = first.clone();

        apply_effect(&effect, AlphaMode::Grayscale, (16, 16), &mut first, 42);
        apply_effect(&effect, AlphaMode::Grayscale, (16, 16), &mut second, 42);

        assert_eq!(first, second);
        assert!(first.chunks_exact(4).any(|pixel| pixel[3] != 255));
        assert!(
            first
                .chunks_exact(4)
                .all(|pixel| pixel[..3].iter().all(|channel| *channel <= pixel[3]))
        );
    }
}
