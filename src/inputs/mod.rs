use self::key::Key;

pub mod events;
pub mod key;

pub mod actions;

pub enum InputEvent {
    Input(Key),
    Tick,
}
