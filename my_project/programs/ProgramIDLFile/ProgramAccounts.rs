#[program]

pub mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<initialize>, data: u64) -> Result<()> {
        Ok(())
    }
}
