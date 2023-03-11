use std::collections::VecDeque;

use songbird::tracks::Queued;

use crate::metadata::{format_metadata, AuxMetadataKey};
use crate::vc::enter_vc;
use crate::{CommandResult, Context};

crate::commands!(skip, mv, swap, remove);

async fn queue_modify<F: FnOnce(&mut VecDeque<Queued>) -> String>(
    ctx: Context<'_>,
    f: F,
) -> CommandResult {
    enter_vc(ctx, false, |handler_lock, c| async move {
        let handler = handler_lock.lock().await;
        let m = handler.queue().modify_queue(f);
        ctx.say(&m).await?;
        Ok(())
    })
    .await
}

#[poise::command(slash_command)]
async fn skip(ctx: Context<'_>) -> CommandResult {
    enter_vc(ctx, false, |handler_lock, c| async move {
        let handler = handler_lock.lock().await;
        if handler.queue().is_empty() {
            ctx.say("queue is empty").await?;
        } else {
            let _ = handler.queue().skip();
            ctx.say("skipped").await?;
        }
        Ok(())
    })
    .await
}

#[poise::command(slash_command)]
async fn mv(
    ctx: Context<'_>,
    #[description = "move from where"] from: usize,
    #[description = "move to where"] to: usize,
) -> CommandResult {
    if from == 0 || to == 0 {
        ctx.say("Cannot move the current song").await?;
        return Ok(());
    }
    queue_modify(ctx, |x| {
        if let Some(song) = x.remove(from) {
            if to > x.len() {
                x.push_back(song);
            } else {
                x.insert(to, song);
            }
            "Success".into()
        } else {
            format!("Failed: index out of bounds for {from}")
        }
    })
    .await
}

#[poise::command(slash_command)]
async fn swap(
    ctx: Context<'_>,
    #[description = "swap from"] a: usize,
    #[description = "swap to"] b: usize,
) -> CommandResult {
    if a == 0 || b == 0 {
        ctx.say("Cannot swap the current song").await?;
        return Ok(());
    }
    queue_modify(ctx, |x| {
        if a >= x.len() {
            format!("F: index out of bounds for {a}")
        } else if b >= x.len() {
            format!("F: index out of bounds for {b}")
        } else {
            x.swap(a, b);
            "Success".into()
        }
    })
    .await
}

#[poise::command(slash_command)]
async fn remove(
    ctx: Context<'_>,
    #[description = "which index to remove"] index: usize,
) -> CommandResult {
    enter_vc(ctx, false, |handler, ctx| async move {
        let handler = handler.lock().await;

        let result = handler.queue().modify_queue(|x| {
            if let Some(track) = x.remove(index) {
                if let Err(e) = track.stop() {
                    Err(format!("Failed to stop track: {:?}", e))
                } else {
                    Ok(track)
                }
            } else {
                Err(format!("No track at index {index}"))
            }
        });

        drop(handler);

        match result {
            Ok(track) => {
                let map = track.typemap().read().await;
                let metadata = map.get::<AuxMetadataKey>().unwrap();
                ctx.say(&format!("Removed: {}", format_metadata(metadata)))
                    .await?;
            }
            Err(e) => {
                ctx.say(&e).await?;
            }
        }

        Ok(())
    })
    .await
}
