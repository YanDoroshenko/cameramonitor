use core::borrow::BorrowMut;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::thread;

use dbus::{BusType, Connection, Message};
use dbus::arg::Array;
use inotify::{Event, EventMask, Inotify, WatchMask};
use systray::Application;

mod model;

fn main() {
    let mut app: Application = create_app();

    let mut inotify = create_inotify();

    watch_events(&mut inotify, &app);

    app.wait_for_message();
}

fn create_app() -> Application {
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => panic!("Can't create window!")
    };

    if Path::new(model::PATH).exists() {
        if check_used() {
            set_status(&app, &model::Status::Active);
        } else {
            set_status(&app, &model::Status::On);
        }
    } else {
        set_status(&app, &model::Status::Off);
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

fn set_status(app: &Application, s: &model::Status) {
    let image = match s {
        model::Status::Off => "/dev/null",
        model::Status::On => "img/cameramonitor_off.png",
        model::Status::Active => "img/cameramonitor_on.png"
    };
    app.set_icon_from_file(&image.to_string()).ok();
}

fn check_used() -> bool {
    let output = Command::new("fuser")
        .arg(&model::PATH)
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

fn watch_events(inotify: &mut Inotify, app: &Application) {
    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify
            .read_events(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if event.mask.contains(EventMask::CREATE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &model::Status::On);
            } else if event.mask.contains(EventMask::DELETE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &model::Status::Off);
            } else if event.mask.contains(EventMask::CLOSE_WRITE) && event.name.filter(|s| s.to_os_string() == "video0").is_some() {
                set_status(&app, &model::Status::On);
            } else if event.name.filter(|s| s.to_os_string() == "video0").is_some() && check_used() {
                set_status(&app, &model::Status::Active);
            }
        }
    }
}
