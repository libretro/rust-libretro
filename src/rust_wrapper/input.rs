use super::retro_input_state_cb;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_B;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_Y;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_SELECT;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_START;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_UP;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_DOWN;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_LEFT;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_RIGHT;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_A;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_X;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_L;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_R;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_L2;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_R2;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_L3;
use super::libretro::RETRO_DEVICE_ID_JOYPAD_R3;
use super::libretro::RETRO_DEVICE_JOYPAD;
use core::prelude::*;

// WARNING
// Don't change without also changing InputState and static asserts

pub enum ControllerButton
{
    PadB = RETRO_DEVICE_ID_JOYPAD_B as int,
    PadY = RETRO_DEVICE_ID_JOYPAD_Y as int,
    PadSelect = RETRO_DEVICE_ID_JOYPAD_SELECT as int,
    PadStart = RETRO_DEVICE_ID_JOYPAD_START as int,
    PadUp = RETRO_DEVICE_ID_JOYPAD_UP as int,
    PadDown = RETRO_DEVICE_ID_JOYPAD_DOWN as int,
    PadLeft = RETRO_DEVICE_ID_JOYPAD_LEFT as int,
    PadRight = RETRO_DEVICE_ID_JOYPAD_RIGHT as int,
    PadA = RETRO_DEVICE_ID_JOYPAD_A as int,
    PadX = RETRO_DEVICE_ID_JOYPAD_X as int,
    PadL = RETRO_DEVICE_ID_JOYPAD_L as int,
    PadR = RETRO_DEVICE_ID_JOYPAD_R as int,
    PadL2 = RETRO_DEVICE_ID_JOYPAD_L2 as int,
    PadR2 = RETRO_DEVICE_ID_JOYPAD_R2 as int,
    PadL3 = RETRO_DEVICE_ID_JOYPAD_L3 as int,
    PadR3 = RETRO_DEVICE_ID_JOYPAD_R3 as int,
}

#[static_assert]
static _I0: bool = (PadB as int == 0);
#[static_assert]
static _I1: bool = (PadY as int == 1);
#[static_assert]
static _I2: bool = (PadSelect as int == 2);
#[static_assert]
static _I3: bool = (PadStart as int == 3);
#[static_assert]
static _I4: bool = (PadUp as int == 4);
#[static_assert]
static _I5: bool = (PadDown as int == 5);
#[static_assert]
static _I6: bool = (PadLeft as int == 6);
#[static_assert]
static _I7: bool = (PadRight as int == 7);
#[static_assert]
static _I8: bool = (PadA as int == 8);
#[static_assert]
static _I9: bool = (PadX as int == 9);
#[static_assert]
static _I10: bool = (PadL as int == 10);
#[static_assert]
static _I11: bool = (PadR as int == 11);
#[static_assert]
static _I12: bool = (PadL2 as int == 12);
#[static_assert]
static _I13: bool = (PadR2 as int == 13);
#[static_assert]
static _I14: bool = (PadL3 as int == 14);
#[static_assert]
static _I15: bool = (PadR3 as int == 15);

pub struct InputState
{
    // WARNING
    // Don't change size without also changing ControllerButton
    // and static asserts
    pub button: [ButtonState, ..16]
}

pub struct ButtonState
{
    pub pressed: bool,
    pub down: bool,
    pub up: bool
}

impl Index<ControllerButton, ButtonState> for InputState
{
    fn index<'a>(&'a self, index: &ControllerButton) -> &'a ButtonState
    {
        unsafe {self.button.unsafe_get(*index as uint)}
    }
}

impl InputState
{
    pub fn poll(player: u32) -> InputState
    {
        // assert!(player < 16, "Tried to poll input for invalid player number");
        let state: InputState =
        unsafe
        {
            InputState
            {
                button: [
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadB as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadY as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadSelect as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadStart as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadUp as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadDown as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadLeft as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadRight as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadA as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadX as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL2 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR2 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadL3 as u32) != 0, down: false, up: false},
                    ButtonState {pressed: retro_input_state_cb.unwrap()(player, RETRO_DEVICE_JOYPAD, 0, PadR3 as u32) != 0, down: false, up: false},
                    ]
            }
        };
        // TODO track state changes and update down and up fields
        state      
    }
}
