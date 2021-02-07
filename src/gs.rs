mod server;
mod configuration;

//pub static configuration: configuration::Configuration = configuration::Configuration::new();
// Add global for config 
fn main() { 
    let config: configuration::Configuration = configuration::Configuration::new();
    let pool = server::threading::ThreadPool::new(16);
    // TODO: Graceful Shutdown implementation
    // Shutdown without error
    ctrlc::set_handler(|| std::process::exit(0)).expect("Error setting Ctrl-C handler");
    // Initiate and run server
    server::HTTP::bind(config).run(server::handle_connection, &pool);
}