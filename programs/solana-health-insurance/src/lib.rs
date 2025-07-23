use anchor_lang::prelude::*;
use anchor_spl::token::{ self, Token, TokenAccount, Transfer };

 
declare_id!("Ct4CqfUt6GMgv5P2NxsmpEN9N6r2Fj2VGv6YadW4znHA");

#[program]
pub mod solana_health_insurance {
    use super::*;
    pub fn submit_claim(ctx: Context<SubmitClaim>, claim_id: u64, treatment_data_hash:[u8; 64],) -> Result<()> {
       let claim = &mut ctx.accounts.claim;
       claim.claim_id = claim_id;
       claim.patient = ctx.accounts.patient.key();
       claim.hospital = ctx.accounts.hospital.key();
       claim.insurer = ctx.accounts.insurer.key();
       claim.treatment_data_hash = treatment_data_hash;
       claim.claim_status = ClaimStatus::Submitted;
       claim.bump = ctx.bumps.claim;        
       Ok(())
    }

    pub fn hospital_verify_claim(ctx:Context<HospitalVerifyClaim>) -> Result<()>{
        let claim = &mut ctx.accounts.claim;
        require!(
            claim.claim_status == ClaimStatus::Submitted,
            CustomError:: InvalidStatus
        );
        claim.claim_status = ClaimStatus:: Verified;
        Ok(())

    }

    pub fn insurer_approve_and_release_funds(ctx:Context<InsurerApprove>, amount: u64,) -> Result<()> {
            let claim = &mut ctx.accounts.claim;
            require!(
                claim.claim_status == ClaimStatus:: Verified,
                CustomError::InvalidStatus
            );
            let cpi_accounts = Transfer {
                from: ctx.accounts.insurer_token.to_account_info(),
                to:ctx.accounts.patient_token.to_account_info(),
                authority:ctx.accounts.insurer.to_account_info(),
            };

            let cpi_program = ctx.accounts.token_program.to_account_info();

            token:: transfer(
                CpiContext::new(cpi_program, cpi_accounts), amount)?;
                Ok(())
         }


     pub fn insurer_reject_claim(ctx:Context<InsurerRejectClaim>) -> Result <()>{
        let claim = &mut ctx.accounts.claim;

        require!(claim.claim_status == ClaimStatus:: Verified || claim.claim_status == ClaimStatus::Submitted,
        CustomError::InvalidStatus
        );
        claim.claim_status = ClaimStatus :: Rejected;
        Ok(())
     }    
}

#[derive(Accounts)]
#[instruction(claim_id: u64)]
pub struct SubmitClaim<'info> {
     
    #[account(init, seeds = [b"claim", patient.key().as_ref(), &claim_id.to_le_bytes()], bump, payer= patient, space = 8 + ClaimAccount::SIZE,)]
    pub claim: Account<'info, ClaimAccount>,
    #[account(mut)]
    pub patient: Signer<'info>,
    pub hospital: AccountInfo<'info>,
    pub insurer:AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct HospitalVerifyClaim <'info>
{
    #[account(mut, has_one = hospital)]
    pub claim: Account<'info, ClaimAccount>,
    pub hospital:Signer<'info>,
}
#[derive(Accounts)]
pub struct InsurerApprove<'info>{
    #[account(mut, has_one = insurer)]
    pub claim:Account<'info, ClaimAccount>,
    pub insurer: Signer<'info>,
    #[account(mut)]
    pub insurer_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub patient_token: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct InsurerRejectClaim<'info>{
    #[account(mut, has_one = insurer)]
    pub claim: Account<'info, ClaimAccount>,
    pub insurer : Signer<'info>,
}

#[account]
pub struct ClaimAccount {
    pub claim_id: u64,
    pub patient : Pubkey,
    pub hospital: Pubkey,
    pub insurer: Pubkey,
    pub treatment_data_hash:[u8; 64],
    pub claim_status:ClaimStatus,
    pub bump:u8,
}
impl ClaimAccount
 {
   pub const SIZE: usize = 8+ 32+ 32+ 32+64+1+1;
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone,PartialEq, Eq)]
pub enum ClaimStatus{
    Submitted,
    Verified,
    Approved,
    Rejected,
}

#[error_code]
pub enum CustomError {
    #[msg("Invalid claim status for this operation")]
    InvalidStatus,
    
}