use systray::Application;
use inotify::Inotify;

mod lib;

fn main() {
    let app: Application = lib::create_app();

    let mut inotify: Inotify = lib::create_inotify();

    lib::watch_events(&mut inotify, &app);
}
