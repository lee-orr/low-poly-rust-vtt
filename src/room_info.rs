pub struct RoomInfo {
    room_name: String,
}

impl RoomInfo {
    pub fn new(name: &str) -> Self {
        Self {
            room_name: name.to_owned(),
        }
    }

    pub fn main_channel_name(&self) -> String {
        format!("main::{}", &self.room_name)
    }
}
