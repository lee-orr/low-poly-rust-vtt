#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum ClientState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}