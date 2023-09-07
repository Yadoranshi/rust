use std::io;

fn main() {
    // println!("Input n");

    let mut n = String::new();

    io::stdin().read_line(&mut n)
        .expect("failed to read");

    let n: u32 = n.trim().parse()
        .expect("please type a number");

    let ans: u32 = get_fibo(n);

    println!("{ans}");
}

fn get_fibo(n : u32) -> u32 {

    if n <= 1 {
        n
    }
    else {

        let mut a: u32 = 0;
        let mut b: u32 = 1;
        let mut ret = 0;
        for _ in 2..=n {
            let ret = a + b;
            a = b;
            b = ret;
        }

        ret
    }
            
}