mod app;
mod camera;
mod circle_pipeline;
mod color;
mod fluid_sim;
mod post_processing;
mod rectangle_pipeline;
mod simple_vertex;
mod timer;

use glass::{
    device_context::DeviceConfig,
    wgpu::{Backends, Limits, PowerPreference, PresentMode},
    window::WindowConfig,
    Glass, GlassConfig, GlassError,
};
use wgpu::InstanceFlags;

use crate::app::{FluidSimApp, HEIGHT, WIDTH};

fn config() -> GlassConfig {
    GlassConfig {
        device_config: DeviceConfig {
            power_preference: PowerPreference::HighPerformance,
            features: wgpu::Features::PUSH_CONSTANTS
                | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            limits: Limits {
                max_push_constant_size: 128,
                ..Limits::default()
            },
            backends: Backends::all(),
            instance_flags: InstanceFlags::from_build_config(),
        },
        window_configs: vec![WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            exit_on_esc: true,
            present_mode: PresentMode::AutoVsync,
            ..WindowConfig::default()
        }],
    }
}

fn main() -> Result<(), GlassError> {
    eprintln!("Hello world!");

    Glass::new_and_run(config(), |event_loop, context| {
        Box::new(FluidSimApp::new(event_loop, context))
    })
}
