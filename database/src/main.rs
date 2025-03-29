use database::Database;
fn main() {
    println!("hello");
    let _ = Database::add("html/urls.txt");
}
