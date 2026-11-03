use anyhow::Result;
use clap::Parser;
use redis::redis;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_hw_accel: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = redis::Config { 
        hardware_acceleration: !cli.no_hw_accel, 
        ..Default::default() 
    };
    let mut component = redis::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-redis ready");
    Ok(())
}
