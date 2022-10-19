/// Crate enum for the pressed hotkeys
pub enum Hotkey {
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
    Left,
    Right,
    Up,
    Down,
    Unknown
}

/// Crate enum for the pressed mouse buttons
pub enum Mouse {
    Coord(i32, i32),
    Left(i32, i32),
    Middle(i32, i32),
    Right(i32, i32),
    Unknown
}
