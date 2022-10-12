use anchor_lang::prelude::*;

declare_id!("BJvFZwdNnCLmBUAGmDwL5Yy4pgwXMCxtZ3ScDQmm2vP4");

#[program]
pub mod storage_program_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input:i32) -> Result<()> {
        let data_account = &mut ctx.accounts.data_account;
        data_account.value = input;
        let i = ctx.accounts.data_account.to_account_info();

        msg!("initialized");
        Ok(())
    }
}
// #[account]
// pub struct SecondStorage{
//     pub keys: [StorageKeys; 10]
// }
// #[account]
// pub struct StorageKeys {
//     pub public_key: Pubkey,
//     pub permissions: [u64; 10],
// }
#[account]
pub struct SecondStorage{
    pub value: i32,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer=owner,
        space=8+std::mem::size_of::<SecondStorage>()
    )]

    data_account:Account<'info,SecondStorage>,

    pub system_program: Program<'info,System>
}
