// ejample adivinar numero aleatorio
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("¡Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Por favor, ingresa tu número.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Error al leer la línea");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingresa un número válido!");
            return;
        }
    };

    println!("Has adivinado: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("¡Muy pequeño!"),
        Ordering::Greater => println!("¡Muy grande!"),
        Ordering::Equal => println!("¡Has ganado!"),
    }
}