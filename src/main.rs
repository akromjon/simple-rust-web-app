mod config;
use config::App;

fn main() {

    let config: App= App::new();

    println!("App name: {}", config.app);
    println!("Database: {}", config.database);    
}
