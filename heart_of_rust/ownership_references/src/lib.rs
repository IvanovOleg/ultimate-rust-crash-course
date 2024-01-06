pub fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("String is plural")
    } else {
        println!("String is singular")
    }
}

pub fn change (s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s")
    } else {
        println!("Stings already has an s")
    }
}

pub fn eat(s: String) -> bool {
    // if s.starts_with("b") && s.contains("a") {
    //     true
    // } else {
    //     false
    // }
    s.starts_with("b") && s.contains("a")
}

pub fn bedazzle(s: &mut String) {
    *s = String::from("sparkly");
}
