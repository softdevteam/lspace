pub const BUTTON1: u16 =    0b0000000000000001;
pub const BUTTON2: u16 =    0b0000000000000010;
pub const BUTTON3: u16 =    0b0000000000000100;
pub const BUTTON4: u16 =    0b0000000000001000;
pub const BUTTON5: u16 =    0b0000000000010000;
pub const BUTTON6: u16 =    0b0000000000100000;
pub const BUTTON7: u16 =    0b0000000001000000;
pub const BUTTON8: u16 =    0b0000000010000000;
const BUTTONS_MASK: u16 =   0b0000000011111111;

pub const KEY_CTRL: u16 =   0b0000000100000000;
pub const KEY_SHIFT: u16 =  0b0000001000000000;
pub const KEY_ALT: u16 =    0b0000010000000000;
pub const KEY_META: u16 =   0b0000100000000000;
const KEYS_MASK: u16 =      0b0000111100000000;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InputModifierState {
    value: u16,
}

impl InputModifierState {
    pub fn new() -> InputModifierState {
        return InputModifierState{value: 0};
    }

    pub fn from_values(value: u16) -> InputModifierState {
        return InputModifierState{value: value};
    }

    pub fn from_buttons_and_keys(buttons: u8, keys: u8) -> InputModifierState {
        let value = (buttons as u16) | ((keys as u16) << 8);
        return InputModifierState{value: value};
    }


    pub fn buttons(&self) -> u8 {
        return (self.value & BUTTONS_MASK) as u8;
    }

    pub fn keys(&self) -> u8 {
        return ((self.value & KEYS_MASK) >> 8) as u8;
    }


    pub fn test(&self, mask: u16) -> bool {
        return (self.value & mask) != 0;
    }

    pub fn test_buttons(&self, mask: u8) -> bool {
        return (self.buttons() & mask) != 0;
    }

    pub fn test_keys(&self, mask: u8) -> bool {
        return (self.keys() & mask) != 0;
    }


    pub fn button1(&self) -> bool {
        return self.test(BUTTON1);
    }

    pub fn button2(&self) -> bool {
        return self.test(BUTTON2);
    }

    pub fn button3(&self) -> bool {
        return self.test(BUTTON3);
    }

    pub fn button4(&self) -> bool {
        return self.test(BUTTON4);
    }

    pub fn button5(&self) -> bool {
        return self.test(BUTTON5);
    }

    pub fn button6(&self) -> bool {
        return self.test(BUTTON6);
    }

    pub fn button7(&self) -> bool {
        return self.test(BUTTON7);
    }

    pub fn button8(&self) -> bool {
        return self.test(BUTTON8);
    }

    pub fn button(&self, button_index: u32) -> bool {
        assert!(button_index > 0 && button_index < 8);
        return self.test(BUTTON1 << (button_index - 1));
    }

    pub fn control(&self) -> bool {
        return self.test(KEY_CTRL);
    }

    pub fn shift(&self) -> bool {
        return self.test(KEY_SHIFT);
    }

    pub fn alt(&self) -> bool {
        return self.test(KEY_ALT);
    }

    pub fn meta(&self) -> bool {
        return self.test(KEY_META);
    }
}
