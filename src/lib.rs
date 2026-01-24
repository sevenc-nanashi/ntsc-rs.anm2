mod config;
use aviutl2::filter::{FilterConfigItemSliceExt, FilterConfigItems};
use config::Config;

#[aviutl2::plugin(FilterPlugin)]
struct MyFilterPlugin;

fn resize_rgba_bilinear(
    src: &[u8],
    src_width: usize,
    src_height: usize,
    dst_width: usize,
    dst_height: usize,
) -> Vec<u8> {
    let mut dst = vec![0u8; dst_width * dst_height * 4];
    if src_width == 0 || src_height == 0 || dst_width == 0 || dst_height == 0 {
        return dst;
    }

    let scale_x = if dst_width > 1 {
        (src_width - 1) as f32 / (dst_width - 1) as f32
    } else {
        0.0
    };
    let scale_y = if dst_height > 1 {
        (src_height - 1) as f32 / (dst_height - 1) as f32
    } else {
        0.0
    };

    for y in 0..dst_height {
        let src_y = y as f32 * scale_y;
        let y0 = src_y.floor() as usize;
        let y1 = (y0 + 1).min(src_height - 1);
        let wy = src_y - y0 as f32;
        for x in 0..dst_width {
            let src_x = x as f32 * scale_x;
            let x0 = src_x.floor() as usize;
            let x1 = (x0 + 1).min(src_width - 1);
            let wx = src_x - x0 as f32;

            let idx00 = (y0 * src_width + x0) * 4;
            let idx10 = (y0 * src_width + x1) * 4;
            let idx01 = (y1 * src_width + x0) * 4;
            let idx11 = (y1 * src_width + x1) * 4;
            let out_idx = (y * dst_width + x) * 4;

            for c in 0..4 {
                let p00 = src[idx00 + c] as f32;
                let p10 = src[idx10 + c] as f32;
                let p01 = src[idx01 + c] as f32;
                let p11 = src[idx11 + c] as f32;
                let top = p00 + (p10 - p00) * wx;
                let bottom = p01 + (p11 - p01) * wx;
                let value = top + (bottom - top) * wy;
                dst[out_idx + c] = value.round().clamp(0.0, 255.0) as u8;
            }
        }
    }

    dst
}

impl aviutl2::filter::FilterPlugin for MyFilterPlugin {
    fn new(info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        Ok(Self)
    }
    fn plugin_info(&self) -> aviutl2::filter::FilterPluginTable {
        aviutl2::filter::FilterPluginTable {
            name: "ntsc".to_string(),
            label: Some("加工".to_string()),
            information: format!(
                "NTSC or VHS-like effect, powered by ntsc-rs, written in Rust / v{version} / https://github.com/sevenc-nanashi/ntsc-rs.auf2",
                version = env!("CARGO_PKG_VERSION")
            ),
            flags: aviutl2::bitflag!(aviutl2::filter::FilterPluginFlags {
                video: true,
                as_filter: true
            }),
            config_items: Config::to_config_items(),
        }
    }
    fn proc_video(
        &self,
        config: &[aviutl2::filter::FilterConfigItem],
        video: &mut aviutl2::filter::FilterProcVideo,
    ) -> aviutl2::AnyResult<()> {
        let cfg = config.to_struct::<Config>();
        let effect = cfg.to_ntsc_effect();
        let src_width = video.video_object.width as usize;
        let src_height = video.video_object.height as usize;
        let mut image_data = vec![0u8; src_width * src_height * 4];
        video.get_image_data(&mut image_data);
        let target_height = cfg.input_resize_height().max(1) as usize;
        let (effect_width, effect_height, mut effect_buffer): (usize, usize, Vec<u8>) =
            if cfg.input_resize_enabled() && target_height != src_height {
            let target_width = ((src_width as f32) * (target_height as f32)
                / (src_height as f32))
                .round()
                .max(1.0) as usize;
            let resized = resize_rgba_bilinear(
                &image_data,
                src_width,
                src_height,
                target_width,
                target_height,
            );
            (target_width, target_height, resized)
        } else {
            (src_width, src_height, image_data)
        };

        effect.apply_effect_to_buffer::<ntscrs::yiq_fielding::Rgbx, _>(
            (effect_width, effect_height),
            &mut effect_buffer,
            video.object.frame as _,
            [1.0, 1.0],
        );

        let output = if effect_width != src_width || effect_height != src_height {
            resize_rgba_bilinear(&effect_buffer, effect_width, effect_height, src_width, src_height)
        } else {
            effect_buffer
        };
        video.set_image_data(&output, src_width as _, src_height as _);
        Ok(())
    }
}

aviutl2::register_filter_plugin!(MyFilterPlugin);
