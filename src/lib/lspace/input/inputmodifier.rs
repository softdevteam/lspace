pub const BUTTON1: u16 =    0b0000000000000001;
pub const BUTTON2: u16 =    0b0000000000000010;
pub const BUTTON3: u16 =    0b0000000000000100;
pub const BUTTON4: u16 =    0b0000000000001000;
pub const BUTTON5: u16 =    0b0000000000010000;
const BUTTONS_MASK: u16 =   0b0000000000011111;

pub const KEY_CTRL: u16 =   0b0000000100000000;
pub const KEY_SHIFT: u16 =  0b0000001000000000;
pub const KEY_ALT: u16 =    0b0000010000000000;
pub const KEY_SUPER: u16 =  0b0000100000000000;
const KEYS_MASK: u16 =      0b0000111100000000;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InputModifierState {
    value: u16,
}

impl InputModifierState {
    pub fn new() -> InputModifierState {
        InputModifierState{value: 0}
    }

    pub fn from_values(value: u16) -> InputModifierState {
        InputModifierState{value: value}
    }

    pub fn from_buttons_and_keys(buttons: u8, keys: u8) -> InputModifierState {
        let value = (buttons as u16) | ((keys as u16) << 8);
        InputModifierState{value: value}
    }

    pub fn buttons(&self) -> u8 {
        (self.value & BUTTONS_MASK) as u8
    }

    pub fn keys(&self) -> u8 {
        ((self.value & KEYS_MASK) >> 8) as u8
    }

    pub fn test(&self, mask: u16) -> bool {
        (self.value & mask) != 0
    }

    pub fn test_buttons(&self, mask: u8) -> bool {
        (self.buttons() & mask) != 0
    }

    pub fn test_keys(&self, mask: u8) -> bool {
        (self.keys() & mask) != 0
    }

    pub fn button1(&self) -> bool {
        self.test(BUTTON1)
    }

    pub fn button2(&self) -> bool {
        self.test(BUTTON2)
    }

    pub fn button3(&self) -> bool {
        self.test(BUTTON3)
    }

    pub fn button4(&self) -> bool {
        self.test(BUTTON4)
    }

    pub fn button5(&self) -> bool {
        self.test(BUTTON5)
    }

    pub fn button(&self, button_index: u32) -> bool {
        assert!(button_index > 0 && button_index < 8);
        self.test(BUTTON1 << (button_index - 1))
    }

    pub fn control_key(&self) -> bool {
        self.test(KEY_CTRL)
    }

    pub fn shift_key(&self) -> bool {
        self.test(KEY_SHIFT)
    }

    pub fn alt_key(&self) -> bool {
        self.test(KEY_ALT)
    }

    pub fn super_key(&self) -> bool {
        self.test(KEY_SUPER)
    }
}
