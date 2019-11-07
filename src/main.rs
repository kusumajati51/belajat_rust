use std::io::*;
use std::cmp::*;
use rand::*;
fn main(){
    let secret_number = rand::thread_rng().gen_range(1,150);
    loop{
        println!("Silahakan Masukan nomer :");
        let mut guess = String::new();
        stdin().read_line(&mut guess)
        .expect("Failed to read Line");
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number");

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("To Big"),
            Ordering::Equal => {
                println!("You Win 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️ 🌎️  😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻 😻");
                break;
            }
        }
    }
}
