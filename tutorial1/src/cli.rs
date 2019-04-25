pub fn run () {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    let command = args[1].clone();
    let name = "Nate";
    let status = 100;
    println!("{}", command);

    if command == "hello" {
        println!("hello, {}, how are you?", name);
    } else if command == "status" {
        println!("Status: {}%", status);
    }
}
