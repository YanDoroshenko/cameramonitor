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
        Status::On => "img/cameramonitor_on.svg",
        Status::Active => "img/cameramonitor_active.svg"
    }
}