mod config;
use aviutl2::filter::FilterConfigItems;
use config::Config;

#[aviutl2::plugin(FilterPlugin)]
struct MyFilterPlugin;

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
}

aviutl2::register_filter_plugin!(MyFilterPlugin);
