use database::Database;

//add arguments
fn main() {
    println!("hello");

    let mut database = Database::new();
    let _ = database.add("example");
    let _ = database.save();
}
