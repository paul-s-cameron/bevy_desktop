mod assets;
mod utils;

#[cfg(feature = "dev")]
mod dev;

use bevy::{
    audio::{AudioPlugin, Volume}, prelude::*, window::{PrimaryWindow, WindowLevel}, winit::WinitWindows
};
#[allow(deprecated)]
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::{Foundation::{COLORREF, HWND}, Graphics::Dwm::{DwmSetWindowAttribute, DWMWA_TRANSITIONS_FORCEDISABLED, DWMWA_USE_IMMERSIVE_DARK_MODE}, UI::WindowsAndMessaging::{GetWindowLongW, SetLayeredWindowAttributes, SetWindowLongW, GWL_EXSTYLE, LWA_ALPHA, WS_EX_LAYERED}};

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::NONE));

    // Order new `AppStep` variants by adding them here:
    app.configure_sets(
        Update,
        (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
    );

    // Spawn the main camera.
    app.add_systems(Startup, spawn_camera);
    app.add_systems(Startup, apply_transparency);

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(
                    Window {
                        transparent: true,
                        decorations: false,
                        present_mode: bevy::window::PresentMode::AutoVsync,
                        window_level: WindowLevel::AlwaysOnBottom,
                        mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        // #[cfg(target_os = "windows")]
                        // composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        #[cfg(target_os = "macos")]
                        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
                        #[cfg(target_os = "linux")]
                        composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
                        // cursor_options: CursorOptions {
                        //     hit_test: false,
                        //     ..default()
                        // },
                        ..default()
                    }
                ),
                ..default()
            })
            .set(AudioPlugin {
                global_volume: GlobalVolume {
                    volume: Volume::new(0.3),
                },
                ..default()
            }),
    );

    // Add other plugins.
    app.add_plugins((
    ));

    // Enable dev tools for dev builds.
    #[cfg(feature = "dev")]
    app.add_plugins(dev::plugin);
    app
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        IsDefaultUiCamera,
    ));
}

fn apply_transparency(
    mut window_query: Query<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    if let Ok(entity) = window_query.get_single_mut() {
        println!("Found Primary Window");
        if let Some(window_wrapper) = winit_windows.get_window(entity) {
            println!("WindowWrapper");
            #[allow(deprecated)]
            if let Ok(window_handle) = window_wrapper.raw_window_handle() {
                println!("RawWindowHandle");
                match window_handle {
                    RawWindowHandle::Win32(w) => {
                        println!("Win32");
                        let hwnd = HWND(w.hwnd.get() as *mut _);
        
                        unsafe {
                            // Enable layered transparency
                            let mut style = GetWindowLongW(hwnd, GWL_EXSTYLE);
                            style |= (WS_EX_LAYERED.0) as i32;
                            SetWindowLongW(hwnd, GWL_EXSTYLE, style);
            
                            // Set the transparency color key (e.g., black pixels will be transparent)
                            _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 0, LWA_ALPHA);
            
                            // Improve compatibility with Windows 11 themes
                            let enable: i32 = 1;
                            _ = DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, enable as *const _, std::mem::size_of::<i32>() as u32);
                            _ = DwmSetWindowAttribute(hwnd, DWMWA_TRANSITIONS_FORCEDISABLED, enable as *const _, std::mem::size_of::<i32>() as u32);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
