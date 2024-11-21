fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    let n = 2;
    // TODO: Fix the function call.
    call_me(n);
}
