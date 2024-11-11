use rmk::action::{KeyAction, Action};
use rmk::keycode::{KeyCode, ModifierCombination};
use rmk::{a, k, layer, mo};

pub(crate) const COL: usize = 7;
pub(crate) const ROW: usize = 12;
pub(crate) const NUM_LAYER: usize = 3;

macro_rules! mt {
    ($a: ident, $b: ident) => {
        KeyAction::ModifierTapHold(Action::Key(KeyCode::$a), Action::Key($b))
    };
}

#[rustfmt::skip]
pub fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            // Left
            [k!(Escape), k!(F1), k!(F2), k!(F3), k!(F4), k!(F5), k!(F6)],
            [k!(Grave), k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4), k!(Kc5), a!(No)],
            [k!(Tab), k!(Q), k!(W), k!(E), k!(R), k!(T), a!(No)],
            [KeyAction::ModifierTapHold(Action::Key(KeyCode::Escape), ModifierCombination::new_from(false,false,false,false,true)),
                 k!(A), k!(S), k!(D), k!(F), k!(G), a!(No)],
            [k!(LShift), k!(Z), k!(X), k!(C), k!(V), k!(B), a!(No)],
            [k!(LGui), k!(Minus), k!(Equal), k!(Quote), mo!(1), k!(Space), k!(Escape)],
            // Right
            [k!(F7), k!(F8), k!(F9), k!(F10), k!(F11), k!(F12), k!(Insert)],
            [a!(No), k!(Kc6), k!(Kc7), k!(Kc8), k!(Kc9), k!(Kc0), k!(Delete)],
            [a!(No), k!(Y), k!(U), k!(I), k!(O), k!(P), k!(LAlt)],
            [a!(No), k!(H), k!(J), k!(K), k!(L), k!(Semicolon), k!(RCtrl)],
            [a!(No), k!(N), k!(M), k!(Comma), k!(Dot), k!(Slash), k!(RShift)],
            [k!(Backspace), k!(Enter), mo!(2), k!(Backslash), k!(LeftBracket), k!(RightBracket), k!(RGui)]
        ]),
        layer!([
            // Left
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), k!(UP), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Left), k!(Down), k!(Right), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Right
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
        layer!([
            // Left
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            // Right
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), k!(Left), k!(Down), k!(UP), k!(Right), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)],
            [a!(No), a!(No), a!(No), a!(No), a!(No), a!(No), a!(No)]
        ]),
    ]
}
