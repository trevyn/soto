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

    match steamworks::Client::init() {
        Ok(client) => {
            let _cb = client.register_callback(|p: steamworks::PersonaStateChange| {
                println!("Got callback: {:?}", p);
            });

            let utils = client.utils();
            println!("Utils:");
            println!("AppId: {:?}", utils.app_id());
            println!("UI Language: {}", utils.ui_language());

            let apps = client.apps();
            println!("Apps");
            println!(
                "IsInstalled(480): {}",
                apps.is_app_installed(steamworks::AppId(480))
            );
            println!(
                "InstallDir(480): {}",
                apps.app_install_dir(steamworks::AppId(480))
            );
            println!("BuildId: {}", apps.app_build_id());
            println!("AppOwner: {:?}", apps.app_owner());
            println!("Langs: {:?}", apps.available_game_languages());
            println!("Lang: {}", apps.current_game_language());
        }

        Err(e) => println!("Steam client error: {}", e),
    }

    Glass::run(config(), |context| Box::new(FluidSimApp::new(context)))
}
