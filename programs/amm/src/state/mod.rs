use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
  pub seed: u64,
  pub authority: Option<Pubkey>,
  pub mint_x: Pubkey,
  pub mint_y: Pubkey,
  pub fee: u16,  // * read below
  pub locked: bool,
  pub config_bump: u8,
  pub lp_bump: u8
}

// most amm fees are small and u16 provides enough room for that amount. It's good for memory and compute efficiency as well as it takes only 2 bytes of storage compared to 4bytes of u32 or 8bytes of u64. 