use std::io;


fn add_item(new_item : &str, shop_list: &mut Vec<String>){
    shop_list.push(new_item.to_string());
    println!("Added item: {} to the list", new_item);
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

fn print_shopping_list(shop_list: &mut Vec<String>){
    for(index, item) in shop_list.iter().enumerate(){
        println!("{}: {} in shopping_list",index+1, item);
    }
}

fn main() {
    println!("--- Shoping List ---");
    let mut shop_list: Vec<String> = Vec::new();

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
            println!("Type the item you want to add:");
            let mut new_item = String::new();
            io::stdin()
            .read_line(&mut new_item)
            .expect("Failed to read line");

            add_item(new_item.trim(), &mut shop_list);

        } ,
        "2" => {
            println!("Type the item you want to remove:");
            let mut selection = String::new();
            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line");

            remove_item(selection.trim(), &mut shop_list);
        },
        "3" => {},
        "4" => {},
         _ => {
             println!("Invalid choice");
             return;
         }
    }


}
