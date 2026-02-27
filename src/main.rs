use std::io;

fn add_item(shop_list: &mut Vec<String>){
    loop{
        println!("Type the item you want to add:");
        let mut new_item = String::new();
        io::stdin()
            .read_line(&mut new_item)
            .expect("Failed to read line");

        shop_list.push(new_item.trim().to_string());
        for _ in 0..10 {
            println!(" ")
        }
        println!("Added item: {} to the list", new_item.trim());
        println!("Type q to exit to main menu, or Enter to continue adding items");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim(){
            "q" => return,
            _  => continue,
        }
    }
}

fn remove_item(selection: &str, shop_list: &mut Vec<String>){
    if let Some(index) = shop_list.iter().position(|x| x.trim() == selection.trim()){
        println!("Item removed {}", selection);
        shop_list.remove(index);
    }
    else {
        println!("Item not found");
    }
}

fn print_shopping_list(shop_list: &Vec<String>){
    for _ in 0..10 {
        println!(" ")
    }
    for(index, item) in shop_list.iter().enumerate(){
        println!("{}: {} ",index+1, item);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match input.trim(){
        "q" => return,
         _  => {},
    }
}

fn call_main_menu(shop_list: &mut Vec<String>){
    println!("--- Shoping List ---");

    println!("Select an option:");
    println!("1. Add a item to the list");
    println!("2. Remove a item from the list");
    println!("3. Print Shopping List");
    println!("4. Exit");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" =>{
            add_item(shop_list);

        } ,
        "2" => {
            println!("Type the item you want to remove:");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");

            remove_item(selection.trim(), shop_list);
        },
        "3" => {print_shopping_list(&shop_list);},
        "4" => {},
        _ => {
            println!("Invalid choice");
            return;
        }
    }
}

fn main() {

    let mut shop_list: Vec<String> = Vec::new();

    loop{
        call_main_menu(&mut shop_list);
    }
}
