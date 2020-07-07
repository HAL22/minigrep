use std::fs;

pub struct Config {
    pub query:String,
    pub filename:String
};

impl Config{

    fn new(args: &[String]) -> Result<Config,& 'static' str>{

        if args.len() < 3{

            return Err("not enough parameters");
        }
        else{

            let query = args[1];

            let filename = args[2];

            return Ok(Config{
                query,
                filename
            })

        }


    }

}

fn run(config: Config){

    let contents = fs::read_to_string(config.filename);

    let contents = match contents{

        Err()
    }
    

}