use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

const IP_ADDRESS: &str = "localhost:7878";

fn return_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input
}
fn main() -> std::io::Result<()> {
    println!("==================== WELCOME TO VCTG ====================");
    loop {
        let mut stream = TcpStream::connect(IP_ADDRESS)?;
        println!("---------- Select Action (Press Q or q to quit) ----------");
        println!("| [1] Register VCTG                                      |");
        println!("| [2] Sign In VCTG                                       |");
        println!("| [3] Check Wallet                                       |");
        println!("| [4] Mine Coin                                          |");
        println!("| [5] Sell Coin                                          |");
        println!("| [6] Buy Coin                                           |");
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
            println!("You selected Register VCTG");
            println!("Enter ID : ");

            let id = return_user_input();
            let str = format!("register|{}|", id);

            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="2" {
            println!("You selected Sign In VCTG");
            println!("Enter ID : ");

            let id = return_user_input();
            let str = format!("signin|{}|", id);

            let writeBuffer = str.as_bytes();
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="3" {
            println!("You selected Check Wallet");
            let writeBuffer = b"wallet|user|";
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="4" {
            println!("You selected Mine Coin");
            let writeBuffer = b"mining|user|";
            stream.write(writeBuffer)?;

            let mut readBuffer = [0; 512];
            stream.read(&mut readBuffer).unwrap();

            println!("Response: {}", String::from_utf8_lossy(&readBuffer[..]));
        }

        else if guess.trim()=="5" {
            println!("You selected Sell Coin");
        }

        else if guess.trim()=="6" {
            println!("You selected Buy Coin");
        } // TODO : TCP 연결 체크
    }
    Ok(())
}
