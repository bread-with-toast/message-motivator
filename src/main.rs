mod sleep;

fn main() {
    sleep::wait(3);
    println!("Hello, world no. 1!");
    sleep::wait(2);
    println!("Hello, world no. 2!");
}