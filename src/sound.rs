
pub fn dog() {
    println!("Dog goes WOOF!");
}

pub fn cat() {
    println!("Cat goes MEOW!");
}

pub fn fox() {
    println!("What does the fox say???");
}

pub mod tame {
    pub fn dog() {
        println!("Dog goes WOOF!");
    }

    pub fn cat() {
        println!("Cat goes MEOW!");
    }
}

pub mod wild {
    pub fn fox() {
        println!("What does the fox say???");
    }
}