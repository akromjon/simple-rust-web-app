use std::env;
use std::fs;

pub struct App {
   pub app: String,
   pub database: String,
}


impl App {
    
    pub fn new() -> App {

        App::load_env_file();

        let name = env::var("APP_NAME").unwrap_or_else(|_| "app".to_string());
        
        let database = env::var("APP_DATABASE").unwrap_or_else(|_| "localhost".to_string());
               
        App {
            app: name,
            database: database,
        }
    }

    fn load_env_file(){
        
        let env_file = fs::read_to_string(".env").expect("Error reading .env file");
        
        for line in env_file.lines() {
         
            let parts: Vec<&str> = line.split('=').collect();
           
            if parts.len() == 2 {
                
                let key = parts[0];
               
                let value = parts[1];
               
                env::set_var(key, value);
            }
        }
    }
}