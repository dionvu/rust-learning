use restaurant::front_of_house;

fn main() {
    let mut wait_list: Vec<front_of_house::Guest> = vec![
        front_of_house::Guest {
            name: String::from("Ben"),
            amount: 3,
        },
        front_of_house::Guest {
            name: String::from("Jake"),
            amount: 7,
        },
        front_of_house::Guest {
            name: String::from("Dave"),
            amount: 1,
        },
        front_of_house::Guest {
            name: String::from("Tony"),
            amount: 2,
        },
    ];

    front_of_house::add_to_wait_list(&mut wait_list);

    front_of_house::print_wait_list(&wait_list);

    front_of_house::estimate_wait_time(&wait_list, "Dion");
}