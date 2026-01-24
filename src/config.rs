// NOTE: 設定項目が被ると動かないので\u{200b}（ゼロ幅スペース）を入れて回避する

#[aviutl2::filter::filter_config_items]
#[derive(Debug)]
pub struct Config {
    #[group(name = "入力リサイズ", opened = false)]
    input_resize: group! {
        #[check(
            name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}",
            default = true
        )]
        input_resize_enabled: bool,
        #[track(name = "縦サイズ", range = 144..=1080, default = 480, step = 1.0)]
        input_resize_height: i32,
    },
    #[track(name = "シード", range = 0..=10000, default = 0, step = 1.0)]
    seed: i32,

    #[select(name = "フィールド", items = Field, default = Field::InterleavedUpper)]
    field: Field,

    #[select(name = "ローパスフィルター", items = FilterType, default = FilterType::Butterworth)]
    filter_type: FilterType,

    #[select(
        name = "入力輝度フィルター",
        items = LumaLowpass,
        default = LumaLowpass::Notch
    )]
    input_luma_filter: LumaLowpass,

    #[select(
        name = "入力クロマローパス",
        items = ChromaLowpass,
        default = ChromaLowpass::Full
    )]
    chroma_lowpass_in: ChromaLowpass,

    #[track(
        name = "コンポジット信号シャープ",
        range = -1.0..=2.0,
        default = 1.0,
        step = 0.001
    )]
    composite_sharpening: f32,

    #[group(name = "コンポジットノイズ", opened = false)]
    composite_noise: group! {
        #[check(name = "有効", default = true)]
        composite_noise_enabled: bool,
        #[track(name = "強度", range = 0.0..=1.0, default = 0.05, step = 0.001)]
        composite_noise_intensity: f32,
        #[track(name = "周波数", range = 0.0..=1.0, default = 0.5, step = 0.001)]
        composite_noise_frequency: f32,
        #[track(name = "細かさ\u{200b}\u{200b}\u{200b}\u{200b}", range = 1..=5, default = 1, step = 1.0)]
        composite_noise_detail: i32,
    },

    #[track(name = "スノーノイズ", range = 0.0..=10000.0, default = 0.025, step = 0.001)]
    snow: f32,

    #[track(name = "スノーノイズ：異方性", range = 0.0..=1.0, default = 0.5, step = 0.001)]
    snow_anisotropy: f32,

    #[select(
        name = "走査線位相シフト",
        items = PhaseShift,
        default = PhaseShift::Degrees180
    )]
    video_scanline_phase_shift: PhaseShift,

    #[track(
        name = "走査線位相シフトオフセット",
        range = 0..=3,
        default = 0,
        step = 1.0
    )]
    video_scanline_phase_shift_offset: i32,

    #[select(name = "クロマ復調フィルター", items = ChromaDemodulationFilter, default = ChromaDemodulationFilter::Notch)]
    chroma_demodulation_filter: ChromaDemodulationFilter,

    #[track(name = "明度のにじみ", range = 0.0..=1.0, default = 0.5, step = 0.001)]
    luma_smear: f32,

    #[group(name = "ヘッド切り替え", opened = false)]
    head_switching: group! {
        #[check(name = "有効\u{200b}", default = true)]
        head_switching_enabled: bool,
        #[track(name = "高さ", range = 0..=24, default = 8, step = 1.0)]
        head_switching_height: i32,
        #[track(name = "オフセット", range = 0..=24, default = 3, step = 1.0)]
        head_switching_offset: i32,
        #[track(name = "水平シフト", range = -100.0..=100.0, default = 72.0, step = 0.001)]
        head_switching_horizontal_shift: f32,
    },
    #[group(name = "ヘッド切り替え：途中開始", opened = false)]
    head_switching_mid_line: group! {
        #[check(name = "有効\u{200b}\u{200b}", default = true)]
        head_switching_mid_line_enabled: bool,
        #[track(name = "位置", range = 0.0..=1.0, default = 0.95, step = 0.001)]
        head_switching_mid_line_position: f32,
        #[track(name = "揺れ", range = 0.0..=1.0, default = 0.03, step = 0.001)]
        head_switching_mid_line_jitter: f32,
    },

    #[group(name = "トラッキングノイズ", opened = false)]
    tracking_noise: group! {
        #[check(name = "有効\u{200b}\u{200b}\u{200b}", default = true)]
        tracking_noise_enabled: bool,
        #[track(name = "高さ\u{200b}", range = 0..=120, default = 12, step = 1.0)]
        tracking_noise_height: i32,
        #[track(name = "波の強さ", range = -50.0..=50.0, default = 15.0, step = 0.001)]
        tracking_noise_wave_intensity: f32,
        #[track(name = "スノーノイズ：強度", range = 0.0..=1.0, default = 0.025, step = 0.001)]
        tracking_noise_snow_intensity: f32,
        #[track(name = "スノーノイズ：異方性\u{200b}", range = 0.0..=1.0, default = 0.25, step = 0.001)]
        tracking_noise_snow_anisotropy: f32,
        #[track(name = "ノイズ強度", range = 0.0..=1.0, default = 0.25, step = 0.001)]
        tracking_noise_noise_intensity: f32,
    },

    #[group(name = "リンギング", opened = false)]
    ringing: group! {
        #[check(name = "有効\u{200b}\u{200b}\u{200b}\u{200b}", default = true)]
        ringing_enabled: bool,
        #[track(name = "周波数\u{200b}", range = 0.0..=1.0, default = 0.45, step = 0.001)]
        ringing_frequency: f32,
        #[track(name = "パワー", range = 1.0..=10.0, default = 4.0, step = 0.001)]
        ringing_power: f32,
        #[track(name = "強度\u{200b}", range = 0.0..=10.0, default = 4.0, step = 0.001)]
        ringing_scale: f32,
    },

    #[group(name = "明度ノイズ", opened = false)]
    luma_noise: group! {
        #[check(name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}", default = true)]
        luma_noise_enabled: bool,
        #[track(name = "強度\u{200b}\u{200b}", range = 0.0..=1.0, default = 0.01, step = 0.001)]
        luma_noise_intensity: f32,
        #[track(name = "周波数\u{200b}\u{200b}", range = 0.0..=1.0, default = 0.5, step = 0.001)]
        luma_noise_frequency: f32,
        #[track(name = "細かさ\u{200b}", range = 1..=5, default = 1, step = 1.0)]
        luma_noise_detail: i32,
    },

    #[group(name = "クロマノイズ", opened = false)]
    chroma_noise: group! {
        #[check(name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}", default = true)]
        chroma_noise_enabled: bool,
        #[track(name = "強度\u{200b}\u{200b}\u{200b}", range = 0.0..=1.0, default = 0.1, step = 0.001)]
        chroma_noise_intensity: f32,
        #[track(name = "周波数\u{200b}\u{200b}\u{200b}", range = 0.0..=0.5, default = 0.05, step = 0.001)]
        chroma_noise_frequency: f32,
        #[track(name = "細かさ\u{200b}\u{200b}", range = 1..=5, default = 2, step = 1.0)]
        chroma_noise_detail: i32,
    },

    #[track(name = "クロマ位相ずれ", range = 0.0..=1.0, default = 0.0, step = 0.001)]
    chroma_phase_error: f32,

    #[track(name = "クロマ位相ノイズ", range = 0.0..=1.0, default = 0.001, step = 0.001)]
    chroma_phase_noise: f32,

    #[track(name = "クロマ遅延(横)", range = -40.0..=40.0, default = 0.0, step = 0.001)]
    chroma_delay_horizontal: f32,

    #[track(name = "クロマ遅延(縦)", range = -20..=20, default = 0, step = 1.0)]
    chroma_delay_vertical: i32,

    #[group(name = "VHSエミュレーション", opened = false)]
    vhs_settings: group! {
        #[check(
            name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}",
            default = true
        )]
        vhs_settings_enabled: bool,
        #[select(name = "テープ速度", items = VhsTapeSpeed, default = VhsTapeSpeed::LP)]
        vhs_tape_speed: VhsTapeSpeed,
        #[track(name = "クロマ欠落", range = 0.0..=1000.0, default = 0.025, step = 0.001)]
        vhs_chroma_loss: f32,
    },
    #[group(name = "VHS：シャープ", opened = false)]
    vhs_sharpen: group! {
        #[check(
            name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}",
            default = true
        )]
        vhs_sharpen_enabled: bool,
        #[track(name = "強度\u{200b}\u{200b}\u{200b}\u{200b}", range = 0.0..=5.0, default = 0.25, step = 0.001)]
        vhs_sharpen_intensity: f32,
        #[track(name = "周波数\u{200b}\u{200b}\u{200b}\u{200b}", range = 0.5..=4.0, default = 1.0, step = 0.001)]
        vhs_sharpen_frequency: f32,
    },
    #[group(name = "VHS：エッジの揺れ", opened = false)]
    vhs_edge_wave: group! {
        #[check(
            name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}",
            default = true
        )]
        vhs_edge_wave_enabled: bool,
        #[track(name = "強度\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}", range = 0.0..=20.0, default = 0.5, step = 0.001)]
        vhs_edge_wave_intensity: f32,
        #[track(name = "速度", range = 0.0..=10.0, default = 4.0, step = 0.001)]
        vhs_edge_wave_speed: f32,
        #[track(name = "周波数\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}", range = 0.0..=0.5, default = 0.05, step = 0.001)]
        vhs_edge_wave_frequency: f32,
        #[track(name = "細かさ\u{200b}\u{200b}\u{200b}", range = 1..=5, default = 2, step = 1.0)]
        vhs_edge_wave_detail: i32,
    },

    #[check(name = "クロマ縦ブレンド", default = true)]
    chroma_vert_blend: bool,

    #[select(
        name = "出力クロマローパス",
        items = ChromaLowpass,
        default = ChromaLowpass::Full
    )]
    chroma_lowpass_out: ChromaLowpass,

    #[group(name = "スケール", opened = false)]
    scale: group! {
        #[check(
            name = "有効\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}",
            default = true
        )]
        scale_enabled: bool,
        #[track(name = "横スケール", range = 0.125..=8.0, default = 1.0, step = 0.001)]
        horizontal_scale: f32,
        #[track(name = "縦スケール", range = 0.125..=8.8, default = 1.0, step = 0.001)]
        vertical_scale: f32,
        #[check(name = "映像サイズに合わせる", default = false)]
        scale_with_video_size: bool,
    },
}

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
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

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
pub enum FilterType {
    #[item(name = "Constant K")]
    ConstantK,
    #[item(name = "Butterworth")]
    Butterworth,
}

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
pub enum LumaLowpass {
    #[item(name = "ノッチ")]
    Notch,
    #[item(name = "ボックス")]
    Box,
    #[item(name = "なし")]
    None,
}

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
pub enum ChromaLowpass {
    #[item(name = "フル")]
    Full,
    #[item(name = "ライト")]
    Light,
    #[item(name = "なし")]
    None,
}

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
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

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
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

#[derive(Debug, Clone, Copy, aviutl2::filter::FilterConfigSelectItems)]
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
