use std::io;

pub struct Guest {
    pub name: String,
    pub amount: i8,
}

pub fn add_to_wait_list(wait_list: &mut Vec<Guest>) -> &mut Vec<Guest> {

    let mut guest_name: String = String::new();
    let mut guest_amount = String::new();

    println!("Welcome to our restaurant, what is your name?"); 

    io::stdin().read_line(&mut guest_name).expect("Failed to read input.");
    guest_name = guest_name.trim().to_string(); // trim() returns a &str, to_string converts to String

    println!("How many people will be dining tonight?");

    io::stdin().read_line(&mut guest_amount).expect("failed to read number of guests."); 
    
    let guest_amount: i8 = match guest_amount.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Failed to read number of guests.");
            0
        },
    };

    wait_list.push(Guest {
        name: guest_name,
        amount: guest_amount,
    }); 
    
    wait_list 
}

pub fn print_wait_list(wait_list: &Vec<Guest>) {
    let mut cur_guest_num = 1;

    for i in 0..wait_list.len() {
        println!("{}. {}, {}", cur_guest_num, wait_list[i].name, wait_list[i].amount);

        cur_guest_num += 1;
    }
}

pub fn estimate_wait_time(wait_list: &Vec<Guest>, name: &str) {
    let mut time: i32 = 0;

    for i in 0..wait_list.len() {
        time += (wait_list[i].amount * 3) as i32;
        if wait_list[i].name == name {
            println!("Wait time is approximately {} minutes.", time);
        }
    }
}