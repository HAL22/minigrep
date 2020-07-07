use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let query = args.get(1);

    let filename = args.get(2);

    let query = match query{

       None => panic!("The query cannot be null"),
       Some(query) => query

    };

    let filename = match filename{

        None => panic!("The file ename cannot be null"),
        Some(filename) => filename

    };

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");



    println!("Query: {}, and filename: {}",query,filename);
    
}
