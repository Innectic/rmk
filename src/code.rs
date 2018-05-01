
// Keycodes for USB HID
//
// Thanks to this GitHub Gist: https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
//
#[derive(PartialOrd, PartialEq)]
pub enum KeyCode {
	Gone = 0x00,
	RollOver,
	PostFail,
	ErrorUndefined,
	A, B, C, D, E, F, G, H, I, J, K, L, M,
	N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
	N1, N2, N3, N4, N5, N6, N7, N8, N9, N0,
	Enter,
	Escape,
	BackSpace,
	Tab,
	Space,
	Minus,
	Equal,
	LeftBracket,
	RightBracket,
	BackSlash,
	HashTilde,
	SemiColon,
	Apostrophe,
	Grave,
	Comma,
	Dot,
	Slash,
	CapsLock,
	F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, // TODO: Do I actually care enough to allow more F-types? Most oses should still support up to like 24
	PrintScreen,
	ScrollLock,
	Pause,
	Insert,
	Home,
	PageUp,
	Delete,
	End,
	PageDown,
	ArrowRight,
	ArrowLeft,
	ArrowDown,
	ArrowUp,
	NumLock,
	PadSlash,
	PadAsterisk,
	PadMinus,
	PadPlus,
	PadEnter,
	Pad1,
	Pad2,
	Pad3,
	Pad4,
	Pad5,
	Pad6,
	Pad7,
	Pad8,
	Pad9,
	Pad0,
	PadDot,
	NoneUSBlackSlash,
	Application,

	LeftControl = 0xE0,
	LeftShift,
	LeftAlt,
	LeftMeta,
	RightControl,
	RightShift,
	RightAlt,
	RightMeta
}

impl KeyCode {

	pub fn modifier(&self) -> bool {
		self >= KeyCode::LeftControl && self <= &KeyCode::RMeta
	}

	pub fn normal(&self) -> bool {
		self >= KeyCode::A && self <= &KeyCode::Application
	}
}
