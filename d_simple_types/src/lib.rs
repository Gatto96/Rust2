
pub fn print_coordinates(coords: (f64, f64)) {
    println!("The coordinates are ({}, {})", coords.0, coords.1);
}

pub fn print_array(coord_array: [f64; 2]) {
    println!("The coordinates are ({}, {})", coord_array[0], coord_array[1]);
}

pub fn ding(x: i32) {
    if x == 13 {
        println!("Ding, you found 13!");
    }
}

pub fn on_off(val: bool) {
    if val {
        println!("Lights are on!");
    }
}