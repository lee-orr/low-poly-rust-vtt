use naia_shared::{Protocolize};

pub mod chat;
pub mod channels;
pub mod auth;

use chat::Chat;
use auth::Auth;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Chat(Chat),
}