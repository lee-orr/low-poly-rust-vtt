use bevy::reflect::TypeUuid;
use serde::Deserialize;

#[derive(Deserialize, TypeUuid)]
#[uuid = "413ac251-bfeb-41b3-9db0-4b8b380a2c46"]
pub struct Settings {
    signaling_url: String,
}
