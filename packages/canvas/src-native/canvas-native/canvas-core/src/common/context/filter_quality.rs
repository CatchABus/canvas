use skia_safe::{CubicResampler, FilterMode, MipmapMode, SamplingOptions};

#[derive(Copy, Clone)]
pub enum FilterQuality {
    None,
    Low,
    Medium,
    High,
}

impl Into<SamplingOptions> for FilterQuality {
    fn into(self) -> SamplingOptions {
        match self {
            FilterQuality::None => SamplingOptions::default(),
            FilterQuality::Low => SamplingOptions {
                use_cubic: false,
                cubic: CubicResampler { b: 0.0, c: 0.0 },
                filter: FilterMode::Linear,
                mipmap: MipmapMode::Nearest,
            },
            FilterQuality::Medium => SamplingOptions {
                use_cubic: true,
                cubic: CubicResampler::mitchell(),
                filter: FilterMode::Nearest,
                mipmap: MipmapMode::Nearest,
            },
            FilterQuality::High => SamplingOptions {
                use_cubic: true,
                cubic: CubicResampler::catmull_rom(),
                filter: FilterMode::Nearest,
                mipmap: MipmapMode::Linear,
            },
        }
    }
}
