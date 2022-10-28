use sdl2::{Sdl, AudioSubsystem};
use sdl2::mouse::MouseButton;
use sdl2::video::Window;
use sdl2::{pixels::Color, render::Canvas, rect::Rect};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

use std::collections::HashMap;

use crate::models::audio::Audio;
use crate::{
    models::api::Api,
    event::{
        Hotkey,
        MouseClick,
        Mouse,
        Input
    },
    properties::{
        rectangle::Rectangle,
        color
    },
    apis::api::{
        WINDOW_MAX_H,
        WINDOW_MAX_W,
        WINDOW_MIN_H,
        WINDOW_MIN_W
    }
};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

/// SDL2 implementation
pub struct SdlApi {
    /// Graphic context
    context: Sdl,
    /// Audio device
    audio_device: AudioDevice<SquareWave>,
    /// Interacting with the window
    canvas: Canvas<Window>,
    /// Used to keep the window open
    is_open: bool,
    /// Used to handle multiple pressed keys continously
    key_pressed: HashMap<sdl2::keyboard::Keycode, bool>,
    /// Window size
    window_size: (u32, u32)
}

impl SdlApi {
    pub fn new(title: String, w: u32, h: u32) -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        
        // Init window
        let mut window = video_subsystem.window(&title, w, h)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        
        // Set window size limits
        window.set_minimum_size(WINDOW_MIN_W, WINDOW_MIN_H).unwrap();
        window.set_maximum_size(WINDOW_MAX_W, WINDOW_MAX_H).unwrap();
    
        let canvas = window
            .into_canvas()
            .build()
            .unwrap();
        
        let window_size = canvas.window().size();
        let audio_device = SdlApi::build_audio(&context);
    
        Self {
            context,
            audio_device,
            canvas,
            is_open: true,
            key_pressed: HashMap::new(),
            window_size
        }
    }

    /// Init audio
    fn build_audio(context: &Sdl) -> AudioDevice<SquareWave> {
        let audio_subsystem = context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),  // mono
            samples: None       // default sample size
        };

        audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap()
    }
}

impl Api for SdlApi {
    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn draw_rect(&mut self, rect: Rectangle, color: color::Color) {
        let filled = Some(rect.into());

        self.canvas.set_draw_color(color);
        self.canvas.fill_rect(filled).unwrap();
    }

    fn is_window_open(&self) -> bool {
        self.is_open
    }

    fn display(&mut self) {
        self.canvas.present();
    }

    fn events(&mut self) -> Vec<Input> {
        let mut inputs = Vec::<Input>::new();
        let mut event_pump = self.context.event_pump().unwrap();

        // Events handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_open = false,

                // Hotkeys pressed
                Event::KeyDown { keycode, .. } => {
                    if keycode.is_some() {
                        self.key_pressed.insert(
                            keycode.unwrap(),
                            true
                        );
                    }
                },

                // Hotkeys released
                Event::KeyUp { keycode, .. } => {
                    if keycode.is_some() {
                        self.key_pressed.insert(
                            keycode.unwrap(),
                            false
                        );
                    }
                },

                // Handle the window events
                Event::Window { win_event, .. } => {
                    if let WindowEvent::Resized(w, h) = win_event {
                        self.window_size = (w as u32, h as u32);
                    }
                },

                // Mouse buttons
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    let mouse = Mouse::new(mouse_btn, x, y);

                    inputs.push(Input::Mouse(mouse));
                },

                _ => {}
            }
        }

        // Add pressed hotkeys
        for kp in self.key_pressed.iter() {
            if *kp.1 == true {
                let hotkey = Hotkey::from(*kp.0);
    
                inputs.push(Input::Hotkey(hotkey));
            }
        }

        inputs
    }

    fn window_size(&self) -> (u32, u32) {
        self.window_size
    }

}

impl Audio for SdlApi {
    fn resume_beep(&mut self) {
        self.audio_device.resume();
    }

    fn pause_beep(&mut self) {
        self.audio_device.pause();
    }
}

impl From<Rectangle> for Rect {
    fn from(r: Rectangle) -> Self {
        Self::new(r.x, r.y, r.w, r.h)
    }
}

impl From<color::Color> for Color {
    fn from(c: color::Color) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<sdl2::keyboard::Keycode> for Hotkey {
    fn from(keycode: sdl2::keyboard::Keycode) -> Self {
        match keycode {
            Keycode::Backspace => Self::Backspace,
            Keycode::Tab => Self::Tab,
            Keycode::Return => Self::Return,
            Keycode::Escape => Self::Escape,
            Keycode::Space => Self::Space,
            Keycode::Exclaim => Self::Exclaim,
            Keycode::Quotedbl => Self::Quotedbl,
            Keycode::Hash => Self::Hash,
            Keycode::Dollar => Self::Dollar,
            Keycode::Percent => Self::Percent,
            Keycode::Ampersand => Self::Ampersand,
            Keycode::Quote => Self::Quote,
            Keycode::LeftParen => Self::LeftParen,
            Keycode::RightParen => Self::RightParen,
            Keycode::Asterisk => Self::Asterisk,
            Keycode::Plus => Self::Plus,
            Keycode::Comma => Self::Comma,
            Keycode::Minus => Self::Minus,
            Keycode::Period => Self::Period,
            Keycode::Slash => Self::Slash,
            Keycode::Num0 => Self::Num0,
            Keycode::Num1 => Self::Num1,
            Keycode::Num2 => Self::Num2,
            Keycode::Num3 => Self::Num3,
            Keycode::Num4 => Self::Num4,
            Keycode::Num5 => Self::Num5,
            Keycode::Num6 => Self::Num6,
            Keycode::Num7 => Self::Num7,
            Keycode::Num8 => Self::Num8,
            Keycode::Num9 => Self::Num9,
            Keycode::Colon => Self::Colon,
            Keycode::Semicolon => Self::Semicolon,
            Keycode::Less => Self::Less,
            Keycode::Equals => Self::Equals,
            Keycode::Greater => Self::Greater,
            Keycode::Question => Self::Question,
            Keycode::At => Self::At,
            Keycode::LeftBracket => Self::LeftBracket,
            Keycode::Backslash => Self::Backslash,
            Keycode::RightBracket => Self::RightBracket,
            Keycode::Caret => Self::Caret,
            Keycode::Underscore => Self::Underscore,
            Keycode::Backquote => Self::Backquote,
            Keycode::A => Self::A,
            Keycode::B => Self::B,
            Keycode::C => Self::C,
            Keycode::D => Self::D,
            Keycode::E => Self::E,
            Keycode::F => Self::F,
            Keycode::G => Self::G,
            Keycode::H => Self::H,
            Keycode::I => Self::I,
            Keycode::J => Self::J,
            Keycode::K => Self::K,
            Keycode::L => Self::L,
            Keycode::M => Self::M,
            Keycode::N => Self::N,
            Keycode::O => Self::O,
            Keycode::P => Self::P,
            Keycode::Q => Self::Q,
            Keycode::R => Self::R,
            Keycode::S => Self::S,
            Keycode::T => Self::T,
            Keycode::U => Self::U,
            Keycode::V => Self::V,
            Keycode::W => Self::W,
            Keycode::X => Self::X,
            Keycode::Y => Self::Y,
            Keycode::Z => Self::Z,
            Keycode::Delete => Self::Delete,
            Keycode::CapsLock => Self::CapsLock,
            Keycode::F1 => Self::F1,
            Keycode::F2 => Self::F2,
            Keycode::F3 => Self::F3,
            Keycode::F4 => Self::F4,
            Keycode::F5 => Self::F5,
            Keycode::F6 => Self::F6,
            Keycode::F7 => Self::F7,
            Keycode::F8 => Self::F8,
            Keycode::F9 => Self::F9,
            Keycode::F10 => Self::F10,
            Keycode::F11 => Self::F11,
            Keycode::F12 => Self::F12,
            Keycode::PrintScreen => Self::PrintScreen,
            Keycode::ScrollLock => Self::ScrollLock,
            Keycode::Pause => Self::Pause,
            Keycode::Insert => Self::Insert,
            Keycode::Home => Self::Home,
            Keycode::PageUp => Self::PageUp,
            Keycode::End => Self::End,
            Keycode::PageDown => Self::PageDown,
            Keycode::Right => Self::Right,
            Keycode::Left => Self::Left,
            Keycode::Down => Self::Down,
            Keycode::Up => Self::Up,
            Keycode::NumLockClear => Self::NumLockClear,
            Keycode::KpDivide => Self::KpDivide,
            Keycode::KpMultiply => Self::KpMultiply,
            Keycode::KpMinus => Self::KpMinus,
            Keycode::KpPlus => Self::KpPlus,
            Keycode::KpEnter => Self::KpEnter,
            Keycode::Kp1 => Self::Kp1,
            Keycode::Kp2 => Self::Kp2,
            Keycode::Kp3 => Self::Kp3,
            Keycode::Kp4 => Self::Kp4,
            Keycode::Kp5 => Self::Kp5,
            Keycode::Kp6 => Self::Kp6,
            Keycode::Kp7 => Self::Kp7,
            Keycode::Kp8 => Self::Kp8,
            Keycode::Kp9 => Self::Kp9,
            Keycode::Kp0 => Self::Kp0,
            Keycode::KpPeriod => Self::KpPeriod,
            Keycode::Application => Self::Application,
            Keycode::Power => Self::Power,
            Keycode::KpEquals => Self::KpEquals,
            Keycode::F13 => Self::F13,
            Keycode::F14 => Self::F14,
            Keycode::F15 => Self::F15,
            Keycode::F16 => Self::F16,
            Keycode::F17 => Self::F17,
            Keycode::F18 => Self::F18,
            Keycode::F19 => Self::F19,
            Keycode::F20 => Self::F20,
            Keycode::F21 => Self::F21,
            Keycode::F22 => Self::F22,
            Keycode::F23 => Self::F23,
            Keycode::F24 => Self::F24,
            Keycode::Execute => Self::Execute,
            Keycode::Help => Self::Help,
            Keycode::Menu => Self::Menu,
            Keycode::Select => Self::Select,
            Keycode::Stop => Self::Stop,
            Keycode::Again => Self::Again,
            Keycode::Undo => Self::Undo,
            Keycode::Cut => Self::Cut,
            Keycode::Copy => Self::Copy,
            Keycode::Paste => Self::Paste,
            Keycode::Find => Self::Find,
            Keycode::Mute => Self::Mute,
            Keycode::VolumeUp => Self::VolumeUp,
            Keycode::VolumeDown => Self::VolumeDown,
            Keycode::KpComma => Self::KpComma,
            Keycode::KpEqualsAS400 => Self::KpEqualsAS400,
            Keycode::AltErase => Self::AltErase,
            Keycode::Sysreq => Self::Sysreq,
            Keycode::Cancel => Self::Cancel,
            Keycode::Clear => Self::Clear,
            Keycode::Prior => Self::Prior,
            Keycode::Return2 => Self::Return2,
            Keycode::Separator => Self::Separator,
            Keycode::Out => Self::Out,
            Keycode::Oper => Self::Oper,
            Keycode::ClearAgain => Self::ClearAgain,
            Keycode::CrSel => Self::CrSel,
            Keycode::ExSel => Self::ExSel,
            Keycode::Kp00 => Self::Kp00,
            Keycode::Kp000 => Self::Kp000,
            Keycode::ThousandsSeparator => Self::ThousandsSeparator,
            Keycode::DecimalSeparator => Self::DecimalSeparator,
            Keycode::CurrencyUnit => Self::CurrencyUnit,
            Keycode::CurrencySubUnit => Self::CurrencySubUnit,
            Keycode::KpLeftParen => Self::KpLeftParen,
            Keycode::KpRightParen => Self::KpRightParen,
            Keycode::KpLeftBrace => Self::KpLeftBrace,
            Keycode::KpRightBrace => Self::KpRightBrace,
            Keycode::KpTab => Self::KpTab,
            Keycode::KpBackspace => Self::KpBackspace,
            Keycode::KpA => Self::KpA,
            Keycode::KpB => Self::KpB,
            Keycode::KpC => Self::KpC,
            Keycode::KpD => Self::KpD,
            Keycode::KpE => Self::KpE,
            Keycode::KpF => Self::KpF,
            Keycode::KpXor => Self::KpXor,
            Keycode::KpPower => Self::KpPower,
            Keycode::KpPercent => Self::KpPercent,
            Keycode::KpLess => Self::KpLess,
            Keycode::KpGreater => Self::KpGreater,
            Keycode::KpAmpersand => Self::KpAmpersand,
            Keycode::KpDblAmpersand => Self::KpDblAmpersand,
            Keycode::KpVerticalBar => Self::KpVerticalBar,
            Keycode::KpDblVerticalBar => Self::KpDblVerticalBar,
            Keycode::KpColon => Self::KpColon,
            Keycode::KpHash => Self::KpHash,
            Keycode::KpSpace => Self::KpSpace,
            Keycode::KpAt => Self::KpAt,
            Keycode::KpExclam => Self::KpExclam,
            Keycode::KpMemStore => Self::KpMemStore,
            Keycode::KpMemRecall => Self::KpMemRecall,
            Keycode::KpMemClear => Self::KpMemClear,
            Keycode::KpMemAdd => Self::KpMemAdd,
            Keycode::KpMemSubtract => Self::KpMemSubtract,
            Keycode::KpMemMultiply => Self::KpMemMultiply,
            Keycode::KpMemDivide => Self::KpMemDivide,
            Keycode::KpPlusMinus => Self::KpPlusMinus,
            Keycode::KpClear => Self::KpClear,
            Keycode::KpClearEntry => Self::KpClearEntry,
            Keycode::KpBinary => Self::KpBinary,
            Keycode::KpOctal => Self::KpOctal,
            Keycode::KpDecimal => Self::KpDecimal,
            Keycode::KpHexadecimal => Self::KpHexadecimal,
            Keycode::LCtrl => Self::LCtrl,
            Keycode::LShift => Self::LShift,
            Keycode::LAlt => Self::LAlt,
            Keycode::LGui => Self::LGui,
            Keycode::RCtrl => Self::RCtrl,
            Keycode::RShift => Self::RShift,
            Keycode::RAlt => Self::RAlt,
            Keycode::RGui => Self::RGui,
            Keycode::Mode => Self::Mode,
            Keycode::AudioNext => Self::AudioNext,
            Keycode::AudioPrev => Self::AudioPrev,
            Keycode::AudioStop => Self::AudioStop,
            Keycode::AudioPlay => Self::AudioPlay,
            Keycode::AudioMute => Self::AudioMute,
            Keycode::MediaSelect => Self::MediaSelect,
            Keycode::Www => Self::Www,
            Keycode::Mail => Self::Mail,
            Keycode::Calculator => Self::Calculator,
            Keycode::Computer => Self::Computer,
            Keycode::AcSearch => Self::AcSearch,
            Keycode::AcHome => Self::AcHome,
            Keycode::AcBack => Self::AcBack,
            Keycode::AcForward => Self::AcForward,
            Keycode::AcStop => Self::AcStop,
            Keycode::AcRefresh => Self::AcRefresh,
            Keycode::BrightnessDown => Self::BrightnessDown,
            Keycode::BrightnessUp => Self::BrightnessUp,
            Keycode::DisplaySwitch => Self::DisplaySwitch,
            Keycode::KbdIllumToggle => Self::KbdIllumToggle,
            Keycode::KbdIllumDown => Self::KbdIllumDown,
            Keycode::KbdIllumUp => Self::KbdIllumUp,
            Keycode::Eject => Self::Eject,
            Keycode::Sleep => Self::Sleep,
            Keycode::AcBookmarks => Self::AcBookmarks,
        }
    }
}

impl From<MouseButton> for MouseClick {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Unknown => Self::Unknown,
            MouseButton::Left => Self::Left,
            MouseButton::Middle => Self::Middle,
            MouseButton::Right => Self::Right,
            MouseButton::X1 => Self::Unknown,
            MouseButton::X2 => Self::Unknown,
        }
    }
}
