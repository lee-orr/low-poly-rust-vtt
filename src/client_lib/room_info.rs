pub struct RoomInfo {
    pub url: Option<String>,
    pub room_name: String,
    pub player: Option<String>,
}

impl RoomInfo {
    pub fn new(info: &str) -> Option<Self> {
        extract_room_name(info).map(|name| Self {
            room_name: name,
            url: extract_host_name(info),
            player: extract_player_name(info),
        })
    }
}

fn extract_room_name(info: &str) -> Option<String> {
    let split = info.split(&['/', '@']);
    split
        .into_iter()
        .last()
        .map(|room_name| room_name.to_owned())
}

fn extract_host_name(info: &str) -> Option<String> {
    let split = info.split_inclusive(&['/', '@']).collect::<Vec<&str>>();
    if split.len() == 2 && split[0].ends_with('/') {
        Some(split[0].trim_end_matches('/').to_owned())
    } else if split.len() == 3 {
        Some(split[1].trim_end_matches('/').to_owned())
    } else {
        None
    }
}

fn extract_player_name(info: &str) -> Option<String> {
    let split = info.split_inclusive(&['/', '@']).collect::<Vec<&str>>();
    if (split.len() == 2 || split.len() == 3) && split[0].ends_with('@') {
        Some(split[0].trim_end_matches('@').to_owned())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn extracts_room_name_from_str() {
        let room = RoomInfo::new("game_name").unwrap();
        assert_eq!("game_name", room.room_name);
        assert_eq!(None, room.url);
        assert_eq!(None, room.player);
    }

    #[test]
    fn extracts_room_and_host_from_str() {
        let room = RoomInfo::new("localhost:8888/game_name").unwrap();
        assert_eq!("game_name", room.room_name);
        assert_eq!("localhost:8888", room.url.unwrap_or_default());
        assert_eq!(None, room.player);
    }

    #[test]
    fn extracts_room_and_player_from_str() {
        let room = RoomInfo::new("player@game_name").unwrap();
        assert_eq!("game_name", room.room_name);
        assert_eq!("player", room.player.unwrap_or_default());
        assert_eq!(None, room.url);
    }

    #[test]
    fn extracts_room_player_and_host_from_str() {
        let room = RoomInfo::new("player@localhost:8888/game_name").unwrap();
        assert_eq!("game_name", room.room_name);
        assert_eq!("player", room.player.unwrap_or_default());
        assert_eq!("localhost:8888", room.url.unwrap_or_default());
    }
}
