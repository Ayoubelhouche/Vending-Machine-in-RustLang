use std::io;
const NOMINALS: [f64; 8] = [2.0, 1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01];

fn main() {
    println!("welcome to your Vending Machine");

    //task 3

    let list_of_items = [
        "Chocolate Bar - 3.0€",
        "Energy Drink -  4.5€",
        "Bag of Chips -  6.5€",
        "Vitamin Water - 3.0€",
        "Soda -          3.5€",
        "Diet Soda -     4.5€",
        "Iced Tea -      5€",
        "Water bottle -  1.5€",
        "Snicker bar -   1€",
        "Ice Cream -     2.5€",
    ];
    let list_of_prices = [3.0, 4.5, 6.5, 3.0, 3.5, 4.5, 5.0, 1.5, 1.0, 2.5];
    let mut coin_counters = [20_usize; 8];
    for i in 0..list_of_items.len() {
        println!("{}- {}.", i, list_of_items[i]);
    }
    println!("\n Please Select one of the items before you !");
    // make sure that user inputs a correct
    let mut condition = false;
    while !condition {
        let mut user_selection = String::new();
        io::stdin()
            .read_line(&mut user_selection)
            .expect("please enter a valid choice");
        let user_selection_int: usize = user_selection.trim().parse().expect("error");
        if (0..11).contains(&user_selection_int) {
            condition = true;
            println!("\n you have selected Product number : {user_selection_int}");
        } else {
            println!("Please enter The number of one of the items you see on the list");
            condition = false;
        }
        let y: f64 = list_of_prices[user_selection_int];

        println!("\nplease enter the amount you wish to pay :");
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
                println!("\nCalculating ..");

                let mut payback: f64 = ((user_input_int - y) * 100.0).floor() / 100.0;
                println!("\nthe rest is : {payback}€");

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

                // ! this section is for how many coins are going to be returned to the customer
                for i in 0..number_of_change_array.len() {
                    if number_of_change_array[i] != 0 {
                        println!(
                            "\nyou will receive {0}  {1}€",
                            number_of_change_array[i], NOMINALS[i]
                        );
                        coin_counters[i] -= 1;
                    }
                }

                println!("\nThe Transaction is over !");
            }

            println!("\nplease input 'q' to leave");
            let mut user_input_string2: String = String::new();
            io::stdin()
                .read_line(&mut user_input_string2)
                .expect("please enter the correct character");
            let character: char = user_input_string2.trim().parse().expect("error");
            if character == 'q' {
                println!("\nthank you for using our Vending machine, Goodbye !");
                break;
            } else {
                println!("please input the correct character to proceed");
            }
        }
    }
}
