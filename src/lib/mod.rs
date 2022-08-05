pub mod bill_operations {

    
    use std::{collections::HashMap, convert::TryInto};

    #[derive(Debug)]
    pub struct Bill {
        pub exp_name: String,
        pub amount: f32,
    }


    pub fn add_bill(storage: &mut HashMap<i32, Bill>, bill: Bill) {
        let index = (storage.len() as i32) + 1;
        storage.insert(index, bill);
    }

    pub fn view_bills(storage: &HashMap<i32, Bill>) {
        println!("\n\nThis is the list of recorded bills:\n");
        /*for i in storage.iter() {
            println!("{:?}", i);
        }*/

        let mut storage_iterator = storage.iter();
        while let Some(i) = storage_iterator.next() {
            println!("{:?}", i);
        }
    }

    pub fn remove_bill(storage: &mut HashMap<i32, Bill>, id: &i32) {
        let index = id;
        if storage.contains_key(id) {
            storage.remove(id);
        } else {
            println!("non-existent id!")
        }
    }

    pub fn edit_bill(storage: &mut HashMap<i32, Bill>, id: &i32, bill: Bill) {
        let index = id;
        if storage.contains_key(id) {
            storage.remove(id);
            storage.insert(*id, bill);
        } else {
            println!("non-existent id!")
        }
    }

    pub fn calculate_bills_total(storage: &mut HashMap<i32, Bill>) {
        let mut running_sum = 0.0;
        for (i, j) in storage.iter() {
            running_sum += j.amount;
        }

        println!("\n\nThe TOTAL EXPENSES so far amount to {:?}", running_sum);
    }
}
