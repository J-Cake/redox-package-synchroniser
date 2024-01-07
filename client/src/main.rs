pub mod error;

pub use error::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("Hello, world!");
    
    Ok(())
}
