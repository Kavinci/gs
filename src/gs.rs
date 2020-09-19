mod server;
mod configuration;

//pub static config: configuration::Configuration = configuration::Configuration::new();
// Add global for config 
// Add handler for ctrl + c
fn main() { 
    let config: configuration::Configuration = configuration::Configuration::new();
    let pool = server::threading::ThreadPool::new(16);
    server::HTTP::bind(config.port.clone()).run(server::handle_connection, pool);
}