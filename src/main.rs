use std::env;

use dbus::{BusType, Connection, Message};
use dbus::arg::Array;
use inotify::{EventMask, Inotify, WatchMask, Event};
use std::path::Path;
use std::process::Command;
use systray::Application;
use core::borrow::BorrowMut;
use std::thread;
use std::ffi::OsStr;

fn main() {
    const PATH: &str = "/dev/video0";

    enum Status {
        Off,
        On,
        Active,
    }

    let app: Application = create_app();

    let mut inotify = create_inotify();

    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify
            .read_events(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &Status::On);
            } else if event.mask.contains(EventMask::DELETE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &Status::Off);
            } else if event.mask.contains(EventMask::CLOSE_WRITE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &Status::On);
            } else if event.name.filter(|s| s.to_os_string() == "video0").is_some() && check_used() {
                set_status(&app, &Status::Active);
            }
        }
    }

    fn create_app() -> Application {
        let mut app = match systray::Application::new() {
            Ok(w) => w,
            Err(_) => panic!("Can't create window!")
        };

        if Path::new(PATH).exists() {
            if check_used() {
                set_status(&app, &Status::Active);
            } else {
                set_status(&app, &Status::On);
            }
        } else {
            set_status(&app, &Status::Off);
        }

        app.add_menu_item(&"Print a thing".to_string(), |_| {
            println!("Printing a thing!");
        }).ok();
        app.add_menu_item(&"Add Menu Item".to_string(), |window| {
            window.add_menu_item(&"Interior item".to_string(), |_| {
                println!("what");
            }).ok();
            window.add_menu_separator().ok();
        }).ok();
        app.add_menu_separator().ok();
        app.add_menu_item(&"Quit".to_string(), |window| {
            window.quit();
        }).ok();

        return app;
    }

    fn set_status(app: &Application, s: &Status) {
        let image = match s {
            Status::Off => "/dev/null",
            Status::On => "img/cameramonitor_off.png",
            Status::Active => "img/cameramonitor_on.png"
        };
        app.set_icon_from_file(&image.to_string()).ok();
    };

    fn check_used() -> bool {
        let output = Command::new("fuser")
            .arg(&PATH)
            .output()
            .expect("no fuser?");

        return output.status.success() && !output.stdout.is_empty();
    }

    fn create_inotify() -> Inotify {
        let mut inotify = Inotify::init()
            .expect("Failed to initialize inotify");

        inotify
            .add_watch(
                &"/dev".to_string(),
                WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY | WatchMask::ALL_EVENTS,
            )
            .expect("Failed to add inotify watch");

        return inotify;
    }

    println!("Waiting on message!");
    app.wait_for_message();
}