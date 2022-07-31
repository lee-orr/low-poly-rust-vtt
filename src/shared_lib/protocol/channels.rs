use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, ReliableSettings
};

#[derive_channels]
pub enum Channels {
    Chat,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::Chat,
        direction: ChannelDirection::Bidirectional,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default())
    }
];