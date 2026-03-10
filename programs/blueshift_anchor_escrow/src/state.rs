use anchor_lang::prelude::*;


#[account(discriminator = 1)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive: u64,
    pub bump: u8,
}
impl Escrow {
    /// 返回 Escrow 账户的初始化空间大小
    pub const fn init_space() -> usize {
        8 + size_of::<Self>()
    }
}