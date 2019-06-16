use std::ffi::OsStr;
use std::path::Path;
use std::process::{Command, exit};

use inotify::{Event, EventMask, Inotify, WatchMask};
use systray::Application;

mod model;

pub(crate) fn create_app() -> Application {
    let mut app = match systray::Application::new() {
        Ok(w) => w,
        Err(_) => panic!("Can't create window!")
    };

    if Path::new(&get_path()).exists() {
        if check_used() {
            set_status(&app, model::Status::Active);
        } else {
            set_status(&app, model::Status::On);
        }
    } else {
        set_status(&app, model::Status::Off);
    }

    let message = format!("Video device: {}", &get_path().to_string());

    app.add_menu_item(&message, |_| ()).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
        exit(0);
    }).ok();

    app.wait_for_message();

    app
}

pub(crate) fn create_inotify() -> Inotify {
    let mut inotify = Inotify::init()
        .expect("Failed to initialize inotify");

    inotify
        .add_watch(
            &"/dev".to_string(),
            WatchMask::CREATE | WatchMask::DELETE | WatchMask::MODIFY | WatchMask::ALL_EVENTS,
        )
        .expect("Failed to add inotify watch");

    inotify
}

pub(crate) fn watch_events(inotify: &mut Inotify, app: &Application) {
    let mut buffer = [0u8; 4096];

    loop {
        let events = inotify
            .read_events_blocking(&mut buffer)
            .expect("Failed to read inotify events");

        for event in events {
            if filter_event(&event, Option::Some(EventMask::CREATE)) {
                set_status(&app, model::Status::On);
            } else if filter_event(&event, Option::Some(EventMask::DELETE)) {
                set_status(&app, model::Status::Off);
            } else if filter_event(&event, Option::Some(EventMask::CLOSE_WRITE)) {
                set_status(&app, model::Status::On);
            } else if filter_event(&event, Option::None) && check_used() {
                set_status(&app, model::Status::Active);
            }
        }
    }
}

fn set_status(app: &Application, s: model::Status) {
    let icon = model::get_icon(s);
    app.set_icon_from_file(&icon.to_string()).ok();
}

fn check_used() -> bool {
    let output = Command::new("fuser")
        .arg(&get_path())
        .output()
        .expect("no fuser?");
    output.status.success() && !output.stdout.is_empty()
}

fn filter_event(event: &Event<&OsStr>, event_mask: Option<EventMask>) -> bool {
    event_mask.map(|v| event.mask.contains(v)).unwrap_or_else(|| true) &&
        event.name.filter(|s| s.to_os_string() == model::PATH_FILE).is_some()
}

fn get_path() -> String {
    format!("{}{}", model::PATH_PREFIX, model::PATH_FILE)
}
