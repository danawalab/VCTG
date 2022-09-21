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

    input
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
            println!("You selected Register VCTG");
            println!("Enter ID : ");

            user_name = return_user_input();
            user_name = user_name.trim().to_string();
            let str = format!("register|{}|", user_name);

            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
            break;
        }

        else if guess.trim()=="2" {
            println!("You selected Sign In VCTG");
            println!("Enter ID : ");

            user_name = return_user_input();
            user_name = user_name.trim().to_string();
            let str = format!("signin|{}|", user_name);

            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
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
            println!("You selected Check Wallet");

            let str = format!("wallet|{}|", user_name);
            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="2" {
            println!("You selected Mine Coin");

            let str = format!("mining|{}|", user_name);
            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="3" {
            println!("You selected Sell Coin");

            let str = format!("sell|{}|", user_name);
            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="4" {
            println!("You selected Buy Coin");

            let str = format!("buy|{}|", user_name);
            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else {
            ;
        }
    }
    Ok(())
}
