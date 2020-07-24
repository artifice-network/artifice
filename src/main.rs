use manager::{ArtificeDB, Manager};
use manager::database::Database;

fn main(){
    let database = ArtificeDB::default();
    let manager = Manager::load(database, b"example password").unwrap();
    println!("hello world");
}