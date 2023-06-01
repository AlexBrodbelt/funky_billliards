#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
    CueBallSetUp,
}


