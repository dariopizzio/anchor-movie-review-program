mod error;

use anchor_lang::prelude::*;

declare_id!("4wBvMoA1GG9gXDP4JJvrN1fhDyJYSRC5KHK26oGscJys");

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
    use error::MovieReviewError;
    use super::*;

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

        Ok(())
    }


    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, _title: String, description: String, rating: u8) -> Result<()> {
        let movie_review = &mut ctx.accounts.pda_movie_review;
        movie_review.description = description;
        movie_review.rating = rating;
        
        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, _title: String) -> Result<()>{
        // No code needed
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
    title: String,      // 4 + len()
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
    pda_movie_review : Account<'info, MovieAccountState>,
    #[account(mut)] // It needs to be mut to pay the realloc
    initializer: Signer<'info>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_title: String)]
pub struct DeleteMovieReview<'info>{
    #[account(
        mut,
        seeds = [_title.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer // Who will receive the rent
    )]
    pda_movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    initializer: Signer<'info>,
    system_program: Program<'info, System>

}