//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    match ctx.accounts.tweet_reaction.reaction {
        ReactionType::Like => {
            ctx.accounts.tweet.likes = ctx.accounts.tweet.likes.checked_sub(1).ok_or(TwitterError::MinLikesReached)?;
        },
        ReactionType::Dislike => {
            ctx.accounts.tweet.dislikes = ctx.accounts.tweet.dislikes.checked_sub(1).ok_or(TwitterError::MinDislikesReached)?;
        },
    }

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub reaction_author: Signer<'info>,
    #[account(
        mut,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        close = reaction_author,
        bump,
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
}
