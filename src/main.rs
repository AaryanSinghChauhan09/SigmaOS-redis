use ::redis::{redis, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = redis::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    component.run().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-redis operational");
    Ok(())
}
