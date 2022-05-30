fn cls(){
    print!("{esc}c", esc = 27 as char); 
}
fn main() {
    cls();
    println!("space is cool :O");
}
