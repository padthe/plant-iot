
fn main(){
    let arguments: Vec<String> = std::env::args().collect();

    let command = arguments.get(1);


    match command { //check which option command is in 
        Some(value) => match value.as_str(){
            "on" => println!("Turning Led on"),
            "off" => println!("Turning Led off"),
            _ => println!("Unknown command: {value}")
        },
        None => println!("Usage: controller <on|off>")
    }
}