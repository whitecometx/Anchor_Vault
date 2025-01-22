use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

declare_id!("769rvc3M29X4F35pAeYP9CVwhTAPdkVycM1fFWYB36Ma");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;
        Ok(())
    }
    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }
    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'Info> {
    #[account (mut)]
    pub user: Signer<'Info>,
    
    #[account ( 
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE, // how much space will this account actually takeup. 
        seeds =[b"state", user.key().as_ref()], // user.key - so that unique and makesur eonly this user can create
        bump
    )]
    pub state: Account<'Info, VaultState>,
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'Info>,

    pub system_program: Program<'Info, System>
}

impl<'Info> Initialize<'Info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        self.state.vault_bump = bumps.vault;
        self.state.state_bump = bumps.state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Payment<'Info> {

    #[account(mut)]
    pub user: Signer<'Info>,

    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'Info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'Info>,
    pub system_program: Program<'Info, System>
}

impl<'Info> Payment<'Info>{
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_program: AccountInfo<'_> = self.system_program.to_account_info();
        let cpi_accounts: Transfer<'_> = Transfer {
            from: self.user.to_account_info(),
            to: self.user.to_account_info(),
        };
        //let cpi_ctx: CpiContext<'_, '_, '_, '_, ...> = CpiContext::new(cpi_program, cpi_accounts);
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to : self.user.to_account_info(),
        };
        let vault_state_key = self.state.key();
    
        let seeds = &[b"vault", vault_state_key.as_ref(), &[self.state.vault_bump]];
        let signer_seeds = &[&seeds[..]];

        //let cpi_ctx: CpiContext<'_, '_, '_, '_, ...> = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'Info> {

    #[account()]
    pub user: Signer<'Info>,
    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = state.state_bump
    )]
    pub state: Account<'Info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    pub vault: SystemAccount<'Info>,
    pub system_program: Program<'Info, System>
}
impl<'Info> Close<'Info>{
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from : self.vault.to_account_info(),
            to : self.user.to_account_info(),
        };
            let vault_state_key = self.state.key();
        
            let seeds = &[b"vault", vault_state_key.as_ref(), &[self.state.vault_bump]];
            let signer_seeds = &[&seeds[..]];
    
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
            let amount = self.vault.lamports();
            
            transfer(cpi_ctx, amount)?;
            Ok(())
    }
}
#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8, // 
    pub state_bump: u8 // PDA i am deriving

}
