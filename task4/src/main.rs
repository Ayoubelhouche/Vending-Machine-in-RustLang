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
        let y: f64 = list_of_prices[user_selection_int];
        if (0..11).contains(&user_selection_int) {
            condition = true;
            println!("\n you have selected Product number : {user_selection_int} with the price of : {y}€");
        } else {
            println!("Please enter The number of one of the items you see on the list");
            condition = false;
        }

        println!("\nplease enter the amount you wish to pay with by inputing the number of the coin desired if done type 44 :");
        let list_of_coins = [
            " 1- 2.00€ ",
            " 2- 1.00€ ",
            " 3- 0.50€ ",
            " 4- 0.20€ ",
            " 5- 0.10€ ",
            " 6- 0.05€ ",
            " 7- 0.02€ ",
            " 8- 0.01€ ",
        ];
        println!("\n{:?}", list_of_coins);
        let condition1 = false;
        let mut ammount: f64 = 0.0;

        let mut coin_counters = [20_usize; 8];
        while !condition1 {
            let mut user_input_string = String::new(); // take string input from user

            io::stdin()
                .read_line(&mut user_input_string)
                .expect("please enter a correct choice from 1-8 or if done type 44");

            let user_input_int: usize = user_input_string.trim().parse().unwrap(); // trim then parse the input to f64
            if user_input_int == 44 {
                break;
            }
            // this section is to detect the user's input then adding it to the total and update the coins counters

            ammount += NOMINALS[user_input_int - 1];
            coin_counters[user_input_int - 1] += 1;
            println!(" {}€ is added to the balance", NOMINALS[user_input_int - 1]);
        }

        println!("\nCalculating ..");
        println!("Your Total Input is :{ammount}€");

        let mut payback: f64 = ((ammount - y) * 100.0).floor() / 100.0;
        println!("\nthe rest is after the transaction is : {payback}€");

        let change_table_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01]; // list of changes available

        let mut number_of_change_array = [0, 0, 0, 0, 0, 0, 0, 0]; // list that tracks how many times the coins have been used

        for i in 0..change_table_array.len() {
            if payback >= change_table_array[i] {
                let index = payback / change_table_array[i];

                payback -= change_table_array[i] * index.floor();
                payback = (payback * 100.0).round() / 100.0;

                number_of_change_array[i] += index.floor() as isize;
                // track the use of the coins in the table
            }
        }
        // ! this section is for how many coins are going to be returned to the customer
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

        // Optional for user .. this is just to check if the coin calculations are correct
        for i in 0..coin_counters.len() {
            println!(
                " the number of {0}€ coins in the vending machine after the transaction is :  {1}",
                NOMINALS[i], coin_counters[i]
            );
        }

        // Warning system of overflow or shortage in coins
        for i in 0..coin_counters.len() {
            if coin_counters[i] == 0 || coin_counters[i] > 50 {
                println!("there was an Error in the system, you will receive the ammount you entered back to you");
            }
        }
        // exit instructions
        println!("\nplease input 'q' to leave");
        let mut user_input_string2: String = String::new();
        io::stdin()
            .read_line(&mut user_input_string2)
            .expect("please enter the correct character");
        let character: char = user_input_string2.trim().parse().expect("error");
        if character == 'q' {
            println!("\nthank you for using our Vending machine, Goodbye !");
        } else {
            println!("please input the correct character to proceed");
        }
    }
}
