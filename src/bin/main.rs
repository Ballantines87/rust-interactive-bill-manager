// Rust Interactive Bill Manager


use std::{collections::HashMap};
use::std::io::stdin;
use::demo::bill_operations::*;



fn main() {

    let mut bills_storage:HashMap<i32, Bill> = HashMap::new();
    

    loop {
        
        let mut input = String::from("");
        
        let mut choice = 0;


        println!("== Manage Bills ==");
        println!("1. Add bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Bill total");
        println!(" ");
        println!("Enter Selection:");

        stdin().read_line(&mut input).expect("failed to read input");
        choice = input.trim().parse::<i32>().expect("failed to parse from String to i32");

        match choice {
            1 => {
                let mut exp_name = String::from(" ");
                let mut exp_amount= String::from(" ");
                println!("Please give a name to your EXPENSE:");
                stdin().read_line(&mut exp_name).expect("failed to read input exp_name");
                println!("Please insert the AMOUNT:");
                stdin().read_line(&mut exp_amount).expect("failed to read input exp_amount");
                let exp_amount = exp_amount.trim().parse::<f32>().expect("failed to parse from String to f32");
                
                let bill = Bill{
                    amount: exp_amount,
                    exp_name: exp_name
                };

                add_bill(&mut bills_storage, bill);
                
                
            }
            2 => {
                view_bills(&bills_storage);

            }
            3 => {
                let mut bill_id = String::from(" ");
                view_bills(&bills_storage);
                println!("Please input the bill's ID corresponding to the record that you want to REMOVE:");
                stdin().read_line(&mut bill_id).expect("failed to read input bill_id");
                let bill_id = bill_id.trim().parse::<i32>().expect("failed to parse from String to i32");
                remove_bill(&mut bills_storage,&bill_id);
            }
            4 => {

                let mut exp_name = String::from(" ");
                let mut exp_amount = String::from(" ");
                let mut bill_id = String::from(" ");

                view_bills(&bills_storage);

                println!("Please input the bill's ID corresponding to the record that you want to UPDATE:\n");
                stdin().read_line(&mut bill_id).expect("failed to read input bill_id");
                println!("Please give a NEW name to your EXPENSE:");
                stdin().read_line(&mut exp_name).expect("failed to read input exp_name");
                println!("Please insert the NEW AMOUNT:");
                stdin().read_line(&mut exp_amount).expect("failed to read input exp_amount");
                let exp_amount = exp_amount.trim().parse::<f32>().expect("failed to parse from String to f32");

                let bill_id = bill_id.trim().parse::<i32>().expect("failed to parse from String to i32");

                let updated_bill = Bill{
                    amount: exp_amount,
                    exp_name: exp_name
                };

                edit_bill(&mut bills_storage, &bill_id, updated_bill);


            }

            5 => {

                calculate_bills_total(&mut bills_storage);
                
            }

            _ => {

                println!("wrong input!");
                
            }

        }
        
        let mut input = String::from("");
        println!("\nDo you want to perform another operation? Y/N");
        stdin().read_line(&mut input).expect("failed to read input");
        let input:&str = &input.trim().to_lowercase();
        match input {
            "y" => continue,
            _ => break
        }
    }



}
