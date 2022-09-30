use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::Write; // bring flush() into scope

const IP_ADDRESS: &str = "localhost:7878";

fn return_user_input() -> String {
    let mut input = String::new();

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}

fn send_wait_response(mut stream: TcpStream, mut str: &str) {
    let writeBuffer = str.as_bytes();
    stream.write(writeBuffer);

    let mut readBuffer = [0; 512];
    stream.read(&mut readBuffer).unwrap();

    println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
}

fn main() -> std::io::Result<()> {
    let mut user_name;


    println!("==================== WELCOME TO VCTG ====================");
    loop {
        let mut stream = TcpStream::connect(IP_ADDRESS)?;

        println!("--------------------- Select Action ----------------------");
        println!("| [1] Register VCTG                                      |");
        println!("| [2] Sign In VCTG                                       |");
        println!("----------------------------------------------------------");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim()=="1" {
            println!("You Selected Register VCTG");
            print!("Enter ID : ");

            user_name = return_user_input();
            let str = format!("register|{}|{}|", user_name, "void");

            send_wait_response(stream, &str);

            break;
        }

        else if guess.trim()=="2" {
            println!("You Selected Sign In VCTG");
            print!("Enter ID : ");

            user_name = return_user_input();
            let str = format!("signin|{}|{}|", user_name, "void");

            send_wait_response(stream, &str);

            break;
        }

        else {
            println!("Enter Again");
        }
    }


    loop {
        let mut stream = TcpStream::connect(IP_ADDRESS)?;
        println!("---------- Select Action (Press Q or q to quit) ----------");
        println!("| [1] Check Wallet                                       |");
        println!("| [2] Mine Coin                                          |");
        println!("| [3] Sell Coin                                          |");
        println!("| [4] Buy Coin                                           |");
        println!("----------------------------------------------------------");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");


        if guess.trim()=="Q" || guess.trim()=="q" {
            println!("Bye!");
            println!("===============================================");
            break;
        }


        if guess.trim()=="1" {
            println!("You Selected Check Wallet");

            let str = format!("wallet|{}|{}|", user_name, "void");
            send_wait_response(stream, &str);
        }

        else if guess.trim()=="2" {
            println!("You Selected Mine Coin");

            let str = format!("mining|{}|{}|", user_name, "void");
            send_wait_response(stream, &str);
        }

        else if guess.trim()=="3" {
            println!("You Selected Sell Coin");

            print!("Enter Coin Amount : ");
            let mut coin_amount = return_user_input();

            let str = format!("sell|{}|{}|", user_name, coin_amount);
            send_wait_response(stream, &str);
        }

        else if guess.trim()=="4" {
            println!("You selected Buy Coin");

            print!("Enter Coin Amount : ");
            let mut coin_amount = return_user_input();

            let str = format!("buy|{}|{}|", user_name, coin_amount);
            send_wait_response(stream, &str);
        }

        else {
            ;
        }
    }
    Ok(())
}
