pub(crate) const PATH_PREFIX: &str = "/dev/";
pub(crate) const PATH_FILE: &str = "video0";

pub(crate) enum Status {
    Off,
    On,
    Active,
}

pub(crate) fn get_icon(s: Status) -> &'static str {
    match s {
        Status::Off => "/dev/null",
        Status::On => "/usr/share/cameramonitor/img/cameramonitor_on.svg",
        Status::Active => "/usr/share/cameramonitor/img/cameramonitor_active.svg"
    }
}