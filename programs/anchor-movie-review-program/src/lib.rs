mod error;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

declare_id!("DdfERqFmyJXMTv9j6sCJzrtHjmaNKW4WRaf9ig83fhL1");

const ANCHOR_DISCRIMINATOR: usize = 8;
const PUBKEY_SIZE: usize = 32;
const U8_SIZE: usize = 1;
const STRING_LENGTH_PREFIX: usize = 4;

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;

#[program]
pub mod anchor_movie_review_program {
    use super::*;
    use error::MovieReviewError;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // Input validations
        require!(
            rating >= MIN_RATING && rating <= MAX_RATING,
            MovieReviewError::InvalidRating
        );

        require!(
            title.len() <= MAX_TITLE_LENGTH,
            MovieReviewError::TitleTooLong
        );

        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            MovieReviewError::DescriptionTooLong
        );

        let movie_review = &mut ctx.accounts.pda_movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.pda_mint.to_account_info(),
                    to: ctx.accounts.pda_token_account.to_account_info(),
                    mint: ctx.accounts.pda_mint.to_account_info(),
                },
                &[&["mint".as_bytes(), &[ctx.bumps.pda_mint]]],
            ),
            10 * 10 ^ 6,
        )?;

        Ok(())
    }

    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        _title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        let movie_review = &mut ctx.accounts.pda_movie_review;
        movie_review.description = description;
        movie_review.rating = rating;

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, _title: String) -> Result<()> {
        // No code needed
        Ok(())
    }

    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        // The initialization is entirely handled by Anchor
        Ok(())
    }
}

#[account]
// This attribute macro implements various traits that help with serialization and deserialization
// of the account, set the discriminator for the account, and set the owner of a new account as the
// program ID defined in the declare_id! macro.
pub struct MovieAccountState {
    reviewer: Pubkey,    // 32
    rating: u8,          // 1
    title: String,       // 4 + len()
    description: String, // 4 + len()
}

// For the MovieAccountState account, since it is dynamic, we implement the Space trait to calculate
// the space required for the account.
// We add the STRING_LENGTH_PREFIX twice to the space to account for the title and description string prefix.
// We need to add the length of the title and description to the space upon initialization.
impl Space for MovieAccountState {
    const INIT_SPACE: usize =
        ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + U8_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}

#[derive(Accounts)]
#[instruction(_title: String, description: String)] // To reference them in this struct
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [
        _title.as_bytes(),
        initializer.key().as_ref()],
        bump, payer=initializer,
        space = MovieAccountState::INIT_SPACE + _title.len() + description.len()
    )]
    pda_movie_review: Account<'info, MovieAccountState>,
    #[account(mut)] // It needs to be mut to pay the rent
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>, // Token Program to mint tokens
    #[account(
        mut,
        seeds = ["mint".as_bytes()],
        bump
    )]
    pda_mint: Account<'info, Mint>, // the mint account for the tokens that we'll mint to users when they add a movie review
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = pda_mint,
        associated_token::authority = initializer,
    )]
    pda_token_account: Account<'info, TokenAccount>, // the associated token account for the afforementioned mint and reviewer (ATA)
    associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        realloc  = MovieAccountState::INIT_SPACE + title.len() + description.len(),
        realloc::payer = initializer,
        realloc::zero = true // zero initialized
    )]
    pda_movie_review: Account<'info, MovieAccountState>,
    #[account(mut)] // It needs to be mut to pay the realloc
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [_title.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer // Who will receive the rent
    )]
    pda_movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = pda_mint // We could create another PDA
    )]
    pda_mint: Account<'info, Mint>,
    #[account(mut)]
    user: Signer<'info>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    system_program: Program<'info, System>,
}
