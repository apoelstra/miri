fn main() {
    assert!(std::char::from_u32(-1_i32 as u32).is_none());
    let _ = match unsafe { std::mem::transmute::<i32, char>(-1) } { //~ ERROR encountered 4294967295, but expected something in the range 0..=1114111
        'a' => {true},
        'b' => {false},
        _ => {true},
    };
}
