pub struct CliConfig {
    query: String,
    filename: String
}

impl CliConfig {
    pub fn new(args: &[String]) -> Result<CliConfig, &str> {
        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();
    
            Ok(Self {
                query,
                filename
            })
        }

    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }
}
