use std::fs;
use std::io;

pub struct Config {
    pub query:String,
    pub filename:String
}

impl Config{

    pub fn new(args: &[String]) -> Result<Config,& 'static str>{

        if args.len() < 3{

            return Err("not enough parameters");
        }
        else{

            let query = args[1].clone();

            let filename = args[2].clone();

            return Ok(Config{
                query,
                filename
            })

        }


    }

}

pub fn run(config: Config) -> Result<String,io::Error>{

    let contents = fs::read_to_string(config.filename);

    let contents = match contents{

        Err(x) => return Err(x),

        Ok(contents) => contents
      
    };


    return Ok(contents);

}