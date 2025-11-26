use wgpu::{
    Adapter, DeviceDescriptor, Face, Features, FrontFace, Limits, MemoryHints, MultisampleState,
    PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureFormat, TextureUsages, Trace,
};
use winit::dpi::PhysicalSize;

pub struct DefaultConf;
pub trait Configuration {
    fn surface_config(
        adapter: &Adapter,
        surface: &Surface,
        size: &PhysicalSize<u32>,
    ) -> SurfaceConfiguration;
    fn primitives() -> PrimitiveState;
    fn multisample() -> MultisampleState;
    fn device_dec(limits: Limits) -> DeviceDescriptor<'static>;
    fn adapter<'a>(surface: &'a Surface) -> RequestAdapterOptions<'a, 'a>;
}

impl Configuration for DefaultConf {
    fn primitives() -> PrimitiveState {
        PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: Some(Face::Back),
            polygon_mode: PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        }
    }

    fn adapter<'a>(surface: &'a Surface) -> RequestAdapterOptions<'a, 'a> {
        RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            compatible_surface: Some(surface.to_owned()),
            force_fallback_adapter: false,
        }
    }

    fn multisample() -> MultisampleState {
        MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }
    }

    fn device_dec(limits: Limits) -> DeviceDescriptor<'static> {
        DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: limits,
            memory_hints: MemoryHints::Performance,
            trace: Trace::Off,
        }
    }

    fn surface_config(
        adapter: &Adapter,
        surface: &Surface,
        size: &PhysicalSize<u32>,
    ) -> SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);

        #[expect(clippy::unwrap_used, reason = "unrecoverable")]
        let format = surface_caps
            .formats
            .iter()
            .copied()
            .find(TextureFormat::is_srgb)
            .unwrap_or(*surface_caps.formats.first().unwrap());

        #[expect(clippy::expect_used, reason = "unrecoverable")]
        SurfaceConfiguration {
            format,
            width: size.width,
            height: size.height,
            present_mode: *surface_caps
                .present_modes
                .first()
                .expect("no surface present modes available"),
            alpha_mode: *surface_caps
                .alpha_modes
                .first()
                .expect("no alpha_modes availables"),
            usage: TextureUsages::RENDER_ATTACHMENT,
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        }
    }
}
