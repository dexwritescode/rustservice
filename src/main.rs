use settings::Settings;

mod settings;

fn main() {

    let settings = Settings::new().expect("Error setting up the configurations.");
    println!("Using: server.port: {}", settings.server.port);
}
