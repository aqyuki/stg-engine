mod core;

pub fn main() -> anyhow::Result<()> {
    let mut engine = core::engine::Engine::new()?;
    engine.run();
    Ok(())
}
