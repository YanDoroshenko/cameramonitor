use systray::Application;
use inotify::Inotify;

mod lib;

fn main() {
    let mut app: Application = lib::create_app();

    let mut inotify: Inotify = lib::create_inotify();

    lib::watch_events(&mut inotify, &app);

    app.wait_for_message();
}
