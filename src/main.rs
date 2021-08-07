use clap::{App, Arg, ArgMatches};

fn get_address(matches: ArgMatches) {
    if let Some(i) = matches.value_of("address") {
        println!("You ordered for delivery!");
        println!("Delivery address: {}", i);
    }
}

fn main() {
    let matches = App::new("Burger Builder")
        .version("1.0")
        .author("Jordan Burroughs <jordanburr23@gmail.com>")
        .about("Helps you build a burger correctly")
        .arg(
            Arg::new("style")
                .short('s')
                .long("style")
                .value_name("BURGER_STYLE")
                .about("What type of burger do you want?")
                .takes_value(true),
        )
        .arg(
            Arg::new("order_type")
                .required(true)
                .short('o')
                .long("order-type")
                .value_name("ORDER_TYPE")
                .about("What toppings do you want on your burger")
                .takes_value(true),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .required_if_eq("order_type", "delivery")
                .value_name("ADDRESS")
                .about("Where do you live?")
                .takes_value(true),
        )
        .arg(
            Arg::new("toppings")
                .short('t')
                .long("toppings")
                .value_name("TOPPING")
                .about("Dine-in, pickup, or delivery?")
                .multiple_occurrences(true)
                .takes_value(true),
        )
        .get_matches();

    if let Some(i) = matches.value_of("style") {
        println!("You want a {} burger.", i);

        match i {
            "smash" => println!("I'm going to make a smash burger"),
            _ => println!("You said {}, but you meant to say smash burger", i),
        }
    }

    if let Some(t) = matches.values_of("toppings") {
        println!("Toppings:");

        let vals: Vec<&str> = t.collect();

        for val in vals {
            match val {
                "lettuce" => println!("- {}", val),
                "pickles" => println!("- {}", val),
                "tomato" => println!("- {}", val),
                "onions" => println!("- {}", val),
                "cheese" => println!("- {}", val),
                _ => println!("We don't have {}", val),
            }
        }
    }

    if let Some(i) = matches.value_of("order_type") {
        match i {
            "delivery" => get_address(matches),
            _ => println!("You said {}.", i),
        }
    }
}
