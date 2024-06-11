mod todo;

use todo::Todo;

fn get_nth_arg(n: usize, msg: &str) -> String {
    std::env::args().nth(n).expect(&msg)
}

fn main() {
    let action = get_nth_arg(1, "Please specify an action");
    let item = get_nth_arg(2, "Please specify an item");

    println!("{:?} {:?}", action, item);

    let mut todo = Todo::new().expect("Initialisation of db failed");
    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Saved"),
            Err(why) => println!("An Error occurred: {:?}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("{} is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Saved"),
                Err(why) => println!("An Error occurred: {:?}", why),
            },
        }
    }
}
