use std::io;

//use std::io;
use rand::Rng;
fn main() {
    println!("welcome to your Vending Machine");
    let price: f64 = rand::thread_rng().gen_range(1.00..10.00);
    let y = (price * 100.0).round() / 100.0;
    println!("\nyou have Selected Product#12, Price : {}€", y);
    println!("\nplease enter the amount you wish to pay :");

    loop {
        let mut user_input_string = String::new();
        io::stdin()
            .read_line(&mut user_input_string)
            .expect("please enter a valid amount");
        let user_input_int: f64 = user_input_string.trim().parse().unwrap();
        //.expect("please enter a valid ammount");
        if (1.00..100.01).contains(&user_input_int) && user_input_int > y {
            println!("great, thanks");
            let payback: f64 = user_input_int - y;
            println!("your change is : {:.2}€", payback);
            println!("please input 'q' to leave");
            let mut user_input_string2: String = String::new();
            io::stdin()
                .read_line(&mut user_input_string2)
                .expect("please enter the correct character");
            let character: char = user_input_string2.trim().parse().expect("error");
            if character == 'q' {
                println!("thank you for using our Vending machine, Goodbye !");
                break;
            } 
                println!("please input the correct character to proceed");
            
        } else if user_input_int < y {
            println!("the amount you entered is not enough , pls enter the correct amount");
        } else {
            println!("pls enter the correct amount");
        }
    }
}
