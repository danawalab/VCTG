use std::io;
use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    println!("test stream");

    let mut stream = TcpStream::connect("192.168.0.105:7878").await.unwrap();
    println!("created stream");

    let result = stream.write(b"register|helloworld").await;
    println!("wrote to stream {:?}", result);
    println!("wrote to stream; success={:?}", result.is_ok());

    println!("==================== WELCOME TO VCTG ====================");
    loop {
        println!("---------- Select Action (Press Q or q to quit) ----------");
        println!("[1] Register VCTG");
        println!("[2] Sign In VCTG");
        println!("[3] Check Wallet");
        println!("[4] Mine Coin");
        println!("[5] Sell Coin");
        println!("[6] Buy Coin");
        println!("----------------------------------------------------------");

        let mut guess = String::new(); // new 함수는 새로운 빈 String을 생성합니다
        // 이 라인은 새로운 빈 String 인스턴스와 연결된 가변변수를 생성합니다.

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        if guess.trim()=="Q" || guess.trim()=="q" {
            println!("Bye!");
            println!("===============================================");
            break;
        }

        if guess.trim()=="1" {
            println!("You selected Register VCTG");
        } else if guess.trim()=="2" {
            println!("You selected Sign In VCTG");
        } else if guess.trim()=="3" {
            println!("You selected Check Wallet");
        } else if guess.trim()=="4" {
            println!("You selected Mine Coin");
        } else if guess.trim()=="5" {
            println!("You selected Sell Coin");
        } else if guess.trim()=="6" {
            println!("You selected Buy Coin");
        } // TODO : TCP 연결 체크

    }

}
