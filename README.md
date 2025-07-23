# Solana_Health_Insurance

# Overview : 
The program manages health insurance claims on the Solana blockchain. 
It allows: Patients to submit claims with treatment details.
Hospitals to verify claims.
Insurers to approve claims (and transfer funds) or reject them.

It uses the Anchor framework to simplify Solana development and the SPL token program to handle USDC (a stablecoin) transfers. The contract stores claim data in a ClaimAccount and tracks the claim status (Submitted, Verified, Approved, or Rejected).


