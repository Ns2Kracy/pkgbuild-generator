pub mod consts;
pub mod generators;
pub mod utils;

#[tokio::main]
async fn main() {
    generators::generate_packages().await.unwrap();
}
