use std::time::Duration;

use naia_shared::{LinkConditionerConfig, SharedConfig, SocketConfig};

use self::protocol::channels::{Channels, CHANNEL_CONFIG};

pub mod chat;
pub mod protocol;

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(40));

    let link_condition = Some(LinkConditionerConfig::average_condition());

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}
