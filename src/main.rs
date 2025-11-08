use std::io;

fn main() {
    println!("Tebak Angka !");
    println!("cobalah masukkkan angka :");
    let mut tebak = String::new();

    io::stdin()
        .read_line(&mut tebak)
        .expect("Gagal menebak :( ");
    println!("Kamu menebak : {tebak}");
}
