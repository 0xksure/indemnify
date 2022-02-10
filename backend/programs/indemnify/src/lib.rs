use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod indemnify {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }

    // get_quote provides a price for insuring the floor
    pub fn get_quote(ctx: Context<GetQuote>) -> ProgramResult {
        let report_account = &ctx.accounts.report_account;
        let report_info = banksea_oracle::get_report_info(&report_account.to_account_info())?;
        let price = report_info.price;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

// ReportAccount is the account info of the nft
#[account]
pub struct ReportAccount {}

#[derive(Accounts)]
pub struct GetQuote<'info> {
    #[account(mut)]
    pub report_account: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
