use std::env;
use std::fs;

use openttd_savegame::header::SavegameHeader;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use openttd_video::sdl2::{VideoEvent, VideoMode, VideoSubsystem, WindowOptions};

fn main() {
    let mut args = env::args().skip(1);
    let mut maybe_path = None;
    let mut make_window = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--window" => make_window = true,
            "--help" | "-h" => {
                eprintln!("usage: openttd_cli [--window] <savegame>");
                std::process::exit(2);
            }
            _ => {
                if maybe_path.is_none() {
                    maybe_path = Some(arg);
                }
            }
        }
    }

    let path = match maybe_path {
        Some(path) => path,
        None if make_window => {
            // Allow testing window without savegame
            String::new()
        }
        None => {
            eprintln!("usage: openttd_cli [--window] <savegame>");
            std::process::exit(2);
        }
    };

    if !path.is_empty() {
        let bytes = match fs::read(&path) {
            Ok(bytes) => bytes,
            Err(err) => {
                eprintln!("failed to read {path}: {err}");
                std::process::exit(1);
            }
        };

        match SavegameHeader::parse(&bytes) {
            Ok(result) => {
                println!("{result:?}");
            }
            Err(err) => {
                eprintln!("failed to parse header: {err}");
                std::process::exit(1);
            }
        }
    }

    if make_window {
        let mut video = VideoSubsystem::init().expect("init video");
        if let Some(driver) = video.current_driver() {
            println!("video driver: {driver}");
        }
        let mut window = video
            .create_window(
                "OpenTTD",
                VideoMode {
                    width: 640,
                    height: 480,
                },
                WindowOptions::default(),
            )
            .expect("create window");
        #[cfg(feature = "sdl2")]
        window.show();
        #[cfg(feature = "sdl2")]
        window.render_solid_color(24, 40, 72);
        println!("window created");
        let exit_flag = Arc::new(AtomicBool::new(false));
        let exit_handle = Arc::clone(&exit_flag);
        ctrlc::set_handler(move || {
            exit_handle.store(true, Ordering::SeqCst);
        })
        .expect("set ctrlc handler");
        loop {
            if exit_flag.load(Ordering::SeqCst) {
                println!("exit requested");
                return;
            }
            while let Ok(Some(event)) = video.poll_event() {
                println!("event: {:?}", event);
                if matches!(event, VideoEvent::Quit) {
                    return;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
