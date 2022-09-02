use std::io;

fn main() {

    println!("=============== WELCOME TO VCTG ===============");
    loop {
        println!("Select Action (Press Q or q to quit)");
        println!("[1] Register VCTG");
        println!("[2] Sign In VCTG");
        println!("[3] Check Wallet");
        println!("[4] Mine Coin");
        println!("[5] Sell Coin");
        println!("[6] Buy Coin");

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
        }


        //
        // let guess: u32 = match guess.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // }; // trim -> 개행문자 빼기 / parse -> 정수형으로 변환 / _ -> 모든 값과 매칭 됨
        //
        //
        // println!("You guessed: {}", guess);
        //
        // match guess.cmp(&secret_number) {
        //     Ordering::Less    => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal   => { println!("You win!"); break; }
        // }
    }

}
