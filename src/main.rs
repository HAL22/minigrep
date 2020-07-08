use std::env;
use std::fs;
use minigrep::Config;
use minigrep::run;


fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let config = match config {

        Ok(con) => con,
        Err(err) => {
            panic!(err)
        }
    };

    let result = run(config);

    







    
}
