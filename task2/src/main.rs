use std::io;
const NOMINALS: [f64; 8] = [2.0, 1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01];
//use std::io;
use rand::Rng;
fn main() {
    println!("welcome to your Vending Machine");
    let price: f64 = rand::thread_rng().gen_range(1.00..10.00); // generate random number between (1.00..10.00)
    let y = (price * 100.0).round() / 100.0; // tansform the number into a readble f64  // example : from 1.0634272732823 -> 1.06
    println!("\nyou have Selected Product#12, Price : {}€", y);
    println!("\nplease enter the amount you wish to pay :");
    let mut coin_counters = [20_usize; 8];
    let mut user_input_string = String::new(); // take string input from user
    io::stdin()
        .read_line(&mut user_input_string)
        .expect("please enter a valid amount");
    let user_input_int: f64 = user_input_string.trim().parse().unwrap(); // trim then parse the input to f64
    loop {
        if !((1.00..100.01).contains(&user_input_int) && user_input_int > y) {
            println!("pls enter the correct amount");
            break;
        } else {
            println!("great, thanks");

            //Task 2- Start

            let mut payback: f64 = ((user_input_int - y) * 100.0).floor() / 100.0;
            println!("the rest is : {payback}");

            let change_table_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01]; // list of changes available

            let mut number_of_change_array = [0, 0, 0, 0, 0, 0, 0, 0]; // list that tracks how many times the coins have been used

            for i in 0..change_table_array.len() {
                if payback >= change_table_array[i] {
                    let index = payback / change_table_array[i];

                    payback -= change_table_array[i] * index.floor();
                    payback = (payback * 100.0).round() / 100.0;

                    number_of_change_array[i] += index.floor() as u64; // track the use of the coins in the table
                }
            }
            for i in 0..number_of_change_array.len() {
                if number_of_change_array[i] != 0 {
                    println!(
                        "\nyou will receive {0}  {1}€",
                        number_of_change_array[i], NOMINALS[i]
                    );
                    coin_counters[i] -= 1;
                }
            }

            println!("The Transaction is over !");
        }

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
    }
}
