use ntscrs::settings::standard as ntsc_settings;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum Field {
    #[item(name = "交互")]
    Alternating,
    #[item(name = "上位のみ")]
    Upper,
    #[item(name = "下位のみ")]
    Lower,
    #[item(name = "両方")]
    Both,
    #[item(name = "交互：上位から")]
    InterleavedUpper,
    #[item(name = "交互：下位から")]
    InterleavedLower,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum FilterType {
    #[item(name = "Constant K")]
    ConstantK,
    #[item(name = "Butterworth")]
    Butterworth,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum LumaLowpass {
    #[item(name = "ノッチ")]
    Notch,
    #[item(name = "ボックス")]
    Box,
    #[item(name = "なし")]
    None,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum ChromaLowpass {
    #[item(name = "フル")]
    Full,
    #[item(name = "ライト")]
    Light,
    #[item(name = "なし")]
    None,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum PhaseShift {
    #[item(name = "0度")]
    Degrees0,
    #[item(name = "90度")]
    Degrees90,
    #[item(name = "180度")]
    Degrees180,
    #[item(name = "270度")]
    Degrees270,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum ChromaDemodulationFilter {
    #[item(name = "ボックス")]
    Box,
    #[item(name = "ノッチ")]
    Notch,
    #[item(name = "1ラインコム")]
    OneLineComb,
    #[item(name = "2ラインコム")]
    TwoLineComb,
}

#[derive(Debug, Clone, Copy, Deserialize, aviutl2::filter::FilterConfigSelectItems)]
pub enum VhsTapeSpeed {
    #[item(name = "SP (標準)")]
    SP,
    #[item(name = "LP (長時間)")]
    LP,
    #[item(name = "EP (延長)")]
    EP,
    #[item(name = "なし")]
    None,
}

impl From<Field> for ntsc_settings::UseField {
    fn from(value: Field) -> Self {
        match value {
            Field::Alternating => ntsc_settings::UseField::Alternating,
            Field::Upper => ntsc_settings::UseField::Upper,
            Field::Lower => ntsc_settings::UseField::Lower,
            Field::Both => ntsc_settings::UseField::Both,
            Field::InterleavedUpper => ntsc_settings::UseField::InterleavedUpper,
            Field::InterleavedLower => ntsc_settings::UseField::InterleavedLower,
        }
    }
}

impl From<FilterType> for ntsc_settings::FilterType {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::ConstantK => ntsc_settings::FilterType::ConstantK,
            FilterType::Butterworth => ntsc_settings::FilterType::Butterworth,
        }
    }
}

impl From<LumaLowpass> for ntsc_settings::LumaLowpass {
    fn from(value: LumaLowpass) -> Self {
        match value {
            LumaLowpass::Notch => ntsc_settings::LumaLowpass::Notch,
            LumaLowpass::Box => ntsc_settings::LumaLowpass::Box,
            LumaLowpass::None => ntsc_settings::LumaLowpass::None,
        }
    }
}

impl From<ChromaLowpass> for ntsc_settings::ChromaLowpass {
    fn from(value: ChromaLowpass) -> Self {
        match value {
            ChromaLowpass::Full => ntsc_settings::ChromaLowpass::Full,
            ChromaLowpass::Light => ntsc_settings::ChromaLowpass::Light,
            ChromaLowpass::None => ntsc_settings::ChromaLowpass::None,
        }
    }
}

impl From<PhaseShift> for ntsc_settings::PhaseShift {
    fn from(value: PhaseShift) -> Self {
        match value {
            PhaseShift::Degrees0 => ntsc_settings::PhaseShift::Degrees0,
            PhaseShift::Degrees90 => ntsc_settings::PhaseShift::Degrees90,
            PhaseShift::Degrees180 => ntsc_settings::PhaseShift::Degrees180,
            PhaseShift::Degrees270 => ntsc_settings::PhaseShift::Degrees270,
        }
    }
}

impl From<ChromaDemodulationFilter> for ntsc_settings::ChromaDemodulationFilter {
    fn from(value: ChromaDemodulationFilter) -> Self {
        match value {
            ChromaDemodulationFilter::Box => ntsc_settings::ChromaDemodulationFilter::Box,
            ChromaDemodulationFilter::Notch => ntsc_settings::ChromaDemodulationFilter::Notch,
            ChromaDemodulationFilter::OneLineComb => {
                ntsc_settings::ChromaDemodulationFilter::OneLineComb
            }
            ChromaDemodulationFilter::TwoLineComb => {
                ntsc_settings::ChromaDemodulationFilter::TwoLineComb
            }
        }
    }
}

impl From<VhsTapeSpeed> for ntsc_settings::VHSTapeSpeed {
    fn from(value: VhsTapeSpeed) -> Self {
        match value {
            VhsTapeSpeed::SP => ntsc_settings::VHSTapeSpeed::SP,
            VhsTapeSpeed::LP => ntsc_settings::VHSTapeSpeed::LP,
            VhsTapeSpeed::EP => ntsc_settings::VHSTapeSpeed::EP,
            VhsTapeSpeed::None => ntsc_settings::VHSTapeSpeed::NONE,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ParamsJson {
    pub seed: i32,
    pub field: Field,
    pub filter_type: FilterType,
    pub input_luma_filter: LumaLowpass,
    pub chroma_lowpass_in: ChromaLowpass,
    pub composite_sharpening: f32,
    pub composite_noise_enabled: bool,
    pub composite_noise_intensity: f32,
    pub composite_noise_frequency: f32,
    pub composite_noise_detail: i32,
    pub snow: f32,
    pub snow_anisotropy: f32,
    pub video_scanline_phase_shift: PhaseShift,
    pub video_scanline_phase_shift_offset: i32,
    pub chroma_demodulation_filter: ChromaDemodulationFilter,
    pub luma_smear: f32,
    pub head_switching_enabled: bool,
    pub head_switching_height: i32,
    pub head_switching_offset: i32,
    pub head_switching_horizontal_shift: f32,
    pub head_switching_mid_line_enabled: bool,
    pub head_switching_mid_line_position: f32,
    pub head_switching_mid_line_jitter: f32,
    pub tracking_noise_enabled: bool,
    pub tracking_noise_height: i32,
    pub tracking_noise_wave_intensity: f32,
    pub tracking_noise_snow_intensity: f32,
    pub tracking_noise_snow_anisotropy: f32,
    pub tracking_noise_noise_intensity: f32,
    pub ringing_enabled: bool,
    pub ringing_frequency: f32,
    pub ringing_power: f32,
    pub ringing_scale: f32,
    pub luma_noise_enabled: bool,
    pub luma_noise_intensity: f32,
    pub luma_noise_frequency: f32,
    pub luma_noise_detail: i32,
    pub chroma_noise_enabled: bool,
    pub chroma_noise_intensity: f32,
    pub chroma_noise_frequency: f32,
    pub chroma_noise_detail: i32,
    pub chroma_phase_error: f32,
    pub chroma_phase_noise: f32,
    pub chroma_delay_horizontal: f32,
    pub chroma_delay_vertical: i32,
    pub vhs_settings_enabled: bool,
    pub vhs_tape_speed: VhsTapeSpeed,
    pub vhs_chroma_loss: f32,
    pub vhs_sharpen_enabled: bool,
    pub vhs_sharpen_intensity: f32,
    pub vhs_sharpen_frequency: f32,
    pub vhs_edge_wave_enabled: bool,
    pub vhs_edge_wave_intensity: f32,
    pub vhs_edge_wave_speed: f32,
    pub vhs_edge_wave_frequency: f32,
    pub vhs_edge_wave_detail: i32,
    pub chroma_vert_blend: bool,
    pub chroma_lowpass_out: ChromaLowpass,
    pub scale_enabled: bool,
    pub horizontal_scale: f32,
    pub vertical_scale: f32,
    pub scale_with_video_size: bool,
}

impl ParamsJson {
    pub fn to_ntsc_effect(&self) -> ntsc_settings::NtscEffect {
        const SNOW_SCALE: f32 = 0.01;
        const VHS_CHROMA_LOSS_SCALE: f32 = 0.001;

        let mut effect = ntsc_settings::NtscEffect::default();
        effect.random_seed = self.seed;
        effect.use_field = self.field.into();
        effect.filter_type = self.filter_type.into();
        effect.input_luma_filter = self.input_luma_filter.into();
        effect.chroma_lowpass_in = self.chroma_lowpass_in.into();
        effect.chroma_demodulation = self.chroma_demodulation_filter.into();
        effect.luma_smear = self.luma_smear;
        effect.composite_sharpening = self.composite_sharpening;
        effect.video_scanline_phase_shift = self.video_scanline_phase_shift.into();
        effect.video_scanline_phase_shift_offset = self.video_scanline_phase_shift_offset;
        effect.head_switching =
            self.head_switching_enabled
                .then(|| ntsc_settings::HeadSwitchingSettings {
                    height: self.head_switching_height,
                    offset: self.head_switching_offset,
                    horiz_shift: self.head_switching_horizontal_shift,
                    mid_line: self.head_switching_mid_line_enabled.then_some({
                        ntsc_settings::HeadSwitchingMidLineSettings {
                            position: self.head_switching_mid_line_position,
                            jitter: self.head_switching_mid_line_jitter,
                        }
                    }),
                });
        effect.tracking_noise =
            self.tracking_noise_enabled
                .then_some(ntsc_settings::TrackingNoiseSettings {
                    height: self.tracking_noise_height,
                    wave_intensity: self.tracking_noise_wave_intensity,
                    snow_intensity: self.tracking_noise_snow_intensity,
                    snow_anisotropy: self.tracking_noise_snow_anisotropy,
                    noise_intensity: self.tracking_noise_noise_intensity,
                });
        effect.composite_noise =
            self.composite_noise_enabled
                .then_some(ntsc_settings::FbmNoiseSettings {
                    frequency: self.composite_noise_frequency,
                    intensity: self.composite_noise_intensity,
                    detail: self.composite_noise_detail,
                });
        effect.ringing = self
            .ringing_enabled
            .then_some(ntsc_settings::RingingSettings {
                frequency: self.ringing_frequency,
                power: self.ringing_power,
                intensity: self.ringing_scale,
            });
        effect.luma_noise = self
            .luma_noise_enabled
            .then_some(ntsc_settings::FbmNoiseSettings {
                frequency: self.luma_noise_frequency,
                intensity: self.luma_noise_intensity,
                detail: self.luma_noise_detail,
            });
        effect.chroma_noise =
            self.chroma_noise_enabled
                .then_some(ntsc_settings::FbmNoiseSettings {
                    frequency: self.chroma_noise_frequency,
                    intensity: self.chroma_noise_intensity,
                    detail: self.chroma_noise_detail,
                });
        effect.snow_intensity = self.snow * SNOW_SCALE;
        effect.snow_anisotropy = self.snow_anisotropy;
        effect.chroma_phase_noise_intensity = self.chroma_phase_noise;
        effect.chroma_phase_error = self.chroma_phase_error;
        effect.chroma_delay_horizontal = self.chroma_delay_horizontal;
        effect.chroma_delay_vertical = self.chroma_delay_vertical;
        effect.vhs_settings = self
            .vhs_settings_enabled
            .then(|| ntsc_settings::VHSSettings {
                tape_speed: self.vhs_tape_speed.into(),
                chroma_loss: self.vhs_chroma_loss * VHS_CHROMA_LOSS_SCALE,
                sharpen: self
                    .vhs_sharpen_enabled
                    .then_some(ntsc_settings::VHSSharpenSettings {
                        intensity: self.vhs_sharpen_intensity,
                        frequency: self.vhs_sharpen_frequency,
                    }),
                edge_wave: self.vhs_edge_wave_enabled.then_some({
                    ntsc_settings::VHSEdgeWaveSettings {
                        intensity: self.vhs_edge_wave_intensity,
                        speed: self.vhs_edge_wave_speed,
                        frequency: self.vhs_edge_wave_frequency,
                        detail: self.vhs_edge_wave_detail,
                    }
                }),
            });
        effect.chroma_vert_blend = self.chroma_vert_blend;
        effect.chroma_lowpass_out = self.chroma_lowpass_out.into();
        effect.scale = self.scale_enabled.then_some(ntsc_settings::ScaleSettings {
            horizontal_scale: self.horizontal_scale,
            vertical_scale: self.vertical_scale,
            scale_with_video_size: self.scale_with_video_size,
        });

        effect
    }
}
