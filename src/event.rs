/// Crate enum for the pressed hotkeys
#[derive(Debug, Clone, Copy)]
pub enum Hotkey {
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Exclaim,
    Quotedbl,
    Hash,
    Dollar,
    Percent,
    Ampersand,
    Quote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Delete,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLockClear,
    KpDivide,
    KpMultiply,
    KpMinus,
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0,
    KpPeriod,
    Application,
    Power,
    KpEquals,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo,
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    KpComma,
    KpEqualsAS400,
    AltErase,
    Sysreq,
    Cancel,
    Clear,
    Prior,
    Return2,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,
    Kp00,
    Kp000,
    ThousandsSeparator,
    DecimalSeparator,
    CurrencyUnit,
    CurrencySubUnit,
    KpLeftParen,
    KpRightParen,
    KpLeftBrace,
    KpRightBrace,
    KpTab,
    KpBackspace,
    KpA,
    KpB,
    KpC,
    KpD,
    KpE,
    KpF,
    KpXor,
    KpPower,
    KpPercent,
    KpLess,
    KpGreater,
    KpAmpersand,
    KpDblAmpersand,
    KpVerticalBar,
    KpDblVerticalBar,
    KpColon,
    KpHash,
    KpSpace,
    KpAt,
    KpExclam,
    KpMemStore,
    KpMemRecall,
    KpMemClear,
    KpMemAdd,
    KpMemSubtract,
    KpMemMultiply,
    KpMemDivide,
    KpPlusMinus,
    KpClear,
    KpClearEntry,
    KpBinary,
    KpOctal,
    KpDecimal,
    KpHexadecimal,
    LCtrl,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui,
    Mode,
    AudioNext,
    AudioPrev,
    AudioStop,
    AudioPlay,
    AudioMute,
    MediaSelect,
    Www,
    Mail,
    Calculator,
    Computer,
    AcSearch,
    AcHome,
    AcBack,
    AcForward,
    AcStop,
    AcRefresh,
    BrightnessDown,
    BrightnessUp,
    DisplaySwitch,
    KbdIllumToggle,
    KbdIllumDown,
    KbdIllumUp,
    Eject,
    Sleep,
    AcBookmarks
}

/// Crate enum for the pressed mouse buttons
#[derive(Debug, Clone, Copy)]
pub enum MouseClick {
    Left,
    Middle,
    Right,
    Unknown
}

/// Mouse down button with coordinates
#[derive(Debug, Clone, Copy)]
pub struct Mouse {
    /// Mouse button type
    pub click: MouseClick,
    /// x-axis
    pub x: i32,
    /// y-axis
    pub y: i32
}

impl Mouse {
    pub fn new<T: Into<MouseClick>>(click: T, x: i32, y: i32) -> Self {
        Self { click: click.into(), x, y }
    }
}

/// Inputs
#[derive(Debug, Clone, Copy)]
pub enum Input {
    Hotkey(Hotkey),
    Mouse(Mouse)
}

impl From<Input> for Option<usize> {
    fn from(input: Input) -> Self {
        let key_index = match input {
            Input::Hotkey(key) => {
                match key {
                    Hotkey::Num0 | Hotkey::Kp0 => 0,
                    Hotkey::Num1 | Hotkey::Kp1 => 1,
                    Hotkey::Num2 | Hotkey::Kp2 => 2,
                    Hotkey::Num3 | Hotkey::Kp3 => 3,
                    Hotkey::Num4 | Hotkey::Kp4 => 4,
                    Hotkey::Num5 | Hotkey::Kp5 => 5,
                    Hotkey::Num6 | Hotkey::Kp6 => 6,
                    Hotkey::Num7 | Hotkey::Kp7 => 7,
                    Hotkey::Num8 | Hotkey::Kp8 => 8,
                    Hotkey::Num9 | Hotkey::Kp9 => 9,
                    Hotkey::A => 0xa,
                    Hotkey::B => 0xb,
                    Hotkey::C => 0xc,
                    Hotkey::D => 0xd,
                    Hotkey::E => 0xe,
                    Hotkey::F => 0xf,
                    _ => return None
                }
            },
            Input::Mouse(_) => return None,
        };

        Some(key_index)
    }
}

impl Input {
    /// Convert a vector or `Input` into a vector of `usize`
    pub fn to_keys(inputs: Vec<Input>) -> Vec<usize> {
        let mut ret = Vec::new();

        for input in inputs {
            if let Some(value) = Option::<usize>::from(input) {
                ret.push(value);
            }
        }

        ret 
    }
}