extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1, 100);

    println!("Advinhe o número!");
    println!("Número secreto {}", secret_number);

    loop {
        println!("Digite o seu palpite.");
        let mut hunch = String::new();

        match io::stdin().read_line(&mut hunch) {
            Ok(num) => println!("Você digitou {}\n", num),
            Err(_) => println!("Deu ruim em algo, lol"),
        };

        let hunch: u32 = match hunch.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match hunch.cmp(&secret_number) {
            Ordering::Less => println!("Número muito baixo"),
            Ordering::Greater => println!("Número muito alto"),
            Ordering::Equal => {
                println!("Parabéns! O número secreto é: {}", hunch);
                break;
            },
        }
    }
}
