use std::sync::Mutex;
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
struct ControllersData
{

    left_stick_analog: [i16; 2],
    left_stick_press: bool,
    left_trigger: bool,
    left_grip: bool,

    left_face1: bool,
    left_face2: bool,
    left_special: bool,
    
    left_orientation: [f32; 4],
    left_accel: [f32; 3],

    right_stick_analog: [i16; 2],
    right_stick_press: bool,
    right_trigger: bool,
    right_grip: bool,

    right_face1: bool,
    right_face2: bool,
    right_special: bool,

    right_orientation: [f32; 4],
    right_accel: [f32; 3],
    
    timestamp: u64
}

lazy_static!
{
    static ref CONTROLLERS: Mutex<ControllersData> = Mutex::new(ControllersData
    {
        left_stick_analog: [0, 0],
        left_stick_press: false,
        left_trigger: false,
        left_grip: false,

        left_face1: false,
        left_face2: false,
        left_special: false,
    
        left_orientation: [0.0, 0.0, 0.0, 0.0],
        left_accel: [0.0, 0.0, 0.0],

        right_stick_analog: [0, 0],
        right_stick_press: false,
        right_trigger: false,
        right_grip: false,

        right_face1: false,
        right_face2: false,
        right_special: false,

        right_orientation: [0.0, 0.0, 0.0, 0.0],
        right_accel: [0.0, 0.0, 0.0],

        timestamp: 0
    });
}