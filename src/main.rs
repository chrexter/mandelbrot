fn main() {
    let x = 5;

    square_loop(x);

    println!("{}", x);
}

fn square_loop(mut x: u64) {
    loop {
        x = x * x;
    }
}
