mod config;
use std::ptr::NonNull;

use aviutl2::{lprintln, module::ScriptModuleFunctions};
use config::ParamsJson;

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
        effect.apply_effect_to_buffer::<ntscrs::yiq_fielding::Rgbx, _>(
            (width, height),
            image_data,
            current_frame as _,
            [1.0, 1.0],
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
