mod server {
    pub fn listening_for_loop() {
        let listener = TcpListener::bind("localhost:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            handle_connection(stream);
        }
    }

    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);

        handle_request(request);
    }


    pub fn handle_request(request: String){
        let mut splited = request.split("|");

        match splited.next() {
            "register" => println!("register"),
            "mining"   => println!("mining"),
            "wallet"   => println!("wallet"),
        }
    }
}