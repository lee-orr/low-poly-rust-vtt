mod args;
use std::{net::SocketAddr};

use bevy::{prelude::{*}, utils::HashMap};
use clap::Parser;
use naia_bevy_server::{Server, ServerAddrs, RoomKey, UserKey, Plugin as NaiaPlugin, ServerConfig, Stage as NaiaStage, events::{AuthorizationEvent, ConnectionEvent, DisconnectionEvent}};

use crate::shared_lib::{protocol::{Protocol, channels::Channels}, shared_config};

pub struct ServerPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ServerState {
    Idle,
    Listening
}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {

        let initial_state = {
            #[cfg(feature = "client")]
            {
                ServerState::Idle
            }
            #[cfg(not(feature="client"))]
            {
                ServerState::Listening
            }
        };

        app
            .init_resource::<ServerSettings>() 
            .init_resource::<Games>()

            .add_plugin(NaiaPlugin::<Protocol, Channels>::new(
                ServerConfig::default(),
                shared_config(),
            ))
            .add_state(initial_state)
            .add_system_set(SystemSet::on_enter(ServerState::Listening).with_system(init))
            .add_system_to_stage(NaiaStage::ReceiveEvents, authorization)
            .add_system_to_stage(NaiaStage::ReceiveEvents, connection_event)
            .add_system_to_stage(NaiaStage::ReceiveEvents, disconnection);
    }
}

pub struct ServerSettings {
    pub main_port: u16,
    pub webrtc_port: u16,
    pub webrtc_url: String
}

pub struct GameConnections {
    pub game: String,
    pub user_connection: HashMap<UserKey, String>
}

#[derive(Default)]
pub struct Games {
    pub games: HashMap<RoomKey, GameConnections>,
    pub user_rooms: HashMap<UserKey, RoomKey>,
    pub game_names: HashMap<String, RoomKey>
}

impl Default for ServerSettings {
    fn default() -> Self {

        let args : args::Args = args::Args::parse();

        Self { main_port: args.port, webrtc_port: args.webrtc_port, webrtc_url: args.webrtc_url }
    }
}

fn init(mut server: Server<Protocol, Channels>, server_settings: Res<ServerSettings>) {
    info!("starting server");
    if let (Ok(main_addr), Ok(webrtc_addr)) = (format!("0.0.0.0:{}", server_settings.main_port).parse::<SocketAddr>(), format!("0.0.0.0:{}", server_settings.webrtc_port).parse::<SocketAddr>()) {
        let server_addresses = ServerAddrs::new(main_addr, webrtc_addr, &server_settings.webrtc_url);

        info!("listening on {}, webrtc on {} - public {}", main_addr, webrtc_addr, &server_settings.webrtc_url);
        server.listen(&server_addresses);
    } else {
        error!("couldn't parse addresses")
    }
}

fn authorization(mut event_reader: EventReader<AuthorizationEvent<Protocol>>, mut server: Server<Protocol, Channels>, mut games: ResMut<Games>) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(auth)) = event {
            let game = (*auth.game).clone();
            let room_key = if let Some(key) = games.game_names.get(&game) {
                key.to_owned()
            }  else {
                let room = server.make_room();
                let key = room.key();
                games.game_names.insert(game.clone(), key.clone());
                games.games.insert(key.clone(), GameConnections { game: game.clone(), user_connection: HashMap::new()});

                key
            };

            games.user_rooms.insert(user_key.clone(), room_key.clone());
            if let Some(game_connections) = games.games.get_mut(&room_key) {
                let player  = (*auth.player).clone();
                info!("Authorizing {} to {}", &player, &game);
                game_connections.user_connection.insert(user_key.clone(), player);
            }

            server.accept_connection(user_key);
        }
    }
}

fn connection_event<'world, 'state>(
    mut event_reader: EventReader<ConnectionEvent>,
    games: Res<Games>,
    mut server: Server<'world, 'state, Protocol, Channels>) {
        for event in event_reader.iter() {
            let ConnectionEvent(user_key) = event;
            if let Some(game) = games.user_rooms.get(&user_key) {
                let address = server
                    .user_mut(user_key)
                    // Add User to the main Room
                    .enter_room(&game)
                    // Get User's address for logging
                    .address();
        
                info!("Naia Server connected to: {}", address);
            } else {
                server.reject_connection(&user_key);
                info!("Rejected connection");
            }
        }
    }


fn disconnection(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut games: ResMut<Games>) {
        for event in event_reader.iter() {
            let DisconnectionEvent(user_key, user) = event;
            info!("Naia Server disconnected from: {:?}", user.address);

            if let Some(room_key) = games.user_rooms.remove(&user_key) {
                let mut delete_game = false;
                let mut game_name = "".to_owned();
                if let Some(game) = games.games.get_mut(&room_key) {
                    if let Some(_) = game.user_connection.remove(&user_key) {
                        if game.user_connection.len() == 0 {
                            delete_game = true;
                            game_name = game.game.clone();
                        }
                    }
                }
                if delete_game {
                    let _ = games.game_names.remove(&game_name);
                    let _ = games.games.remove(&room_key);
                }
            }
        }
    }