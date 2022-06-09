use std::{thread, time::Duration};

fn cls(){
    print!("{esc}c", esc = 27 as char); 
}
fn main() {
    cls();
    println!("space is cool :O");
    thread::sleep(Duration::from_secs(5));
}
