use naia_shared::Protocolize;

pub mod auth;
pub mod channels;
pub mod chat;

use auth::Auth;
use chat::Chat;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    Chat(Chat),
}
