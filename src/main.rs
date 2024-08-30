use handler::fetch_github_activities;

mod handler;
mod model;

#[tokio::main]
async fn main() {
    println!("-----------------------------------");
    println!("Welcome to the Github Activity Fetcher!");
    print!("This app will fetch your github activities\n");
    println!("-----------------------------------");
    println!("\n");
    loop {
        let mut s = String::new();
        println!("Enter your github username: ");

        std::io::stdin().read_line(&mut s).unwrap();

        let s = s.trim();

        fetch_github_activities(s).await.unwrap();

        println!("\nDo you want to continue? (y/n or any key to exit): ");

        let mut continue_input = String::new();

        std::io::stdin().read_line(&mut continue_input).unwrap();

        if continue_input.trim() != "y" {
            println!("---------------------------");
            println!("Thanks for using the app!");
            println!("Goodbye! :)");
            println!("---------------------------");
            println!("\n");
            break;
        }
    }
}
