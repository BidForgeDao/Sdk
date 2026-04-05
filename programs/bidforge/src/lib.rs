use anchor_lang::prelude::*;

declare_id!("BidF0rge11111111111111111111111111111111");

#[program]
pub mod bidforge {
    use super::*;

    pub fn initialize_auction(ctx: Context<InitializeAuction>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.highest_bid = 0;
        auction.highest_bidder = Pubkey::default();
        auction.ended = false;
        Ok(())
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;

        require!(!auction.ended, CustomError::AuctionEnded);
        require!(amount > auction.highest_bid, CustomError::BidTooLow);

        auction.highest_bid = amount;
        auction.highest_bidder = *ctx.accounts.bidder.key;

        Ok(())
    }

    pub fn finalize(ctx: Context<Finalize>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.ended = true;
        Ok(())
    }
}

#[account]
pub struct Auction {
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub ended: bool,
}

#[derive(Accounts)]
pub struct InitializeAuction<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 1)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub bidder: Signer<'info>,
}

#[derive(Accounts)]
pub struct Finalize<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
}

#[error_code]
pub enum CustomError {
    #[msg("Auction already ended")]
    AuctionEnded,
    #[msg("Bid too low")]
    BidTooLow,
}
