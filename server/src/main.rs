pub mod error;

pub use crate::error::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    println!("Hello, world!");
    
    Ok(())
}
