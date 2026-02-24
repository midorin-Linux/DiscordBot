use crate::application::{chat::chat_service::process_message, command::command_registry::Context};

#[poise::command(prefix_command)]
pub async fn chat(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> anyhow::Result<()> {
    let _typing = ctx.channel_id().start_typing(&ctx.serenity_context().http);

    let data = ctx.data();
    let channel_id = ctx.channel_id().get();
    let user_id = ctx.author().id.get();

    let reply = match process_message(
        data.rig_client.as_ref(),
        &data.in_memory_store,
        &data.vector_store,
        channel_id,
        user_id,
        prompt,
    )
    .await
    {
        Ok(response) => response,
        Err(err) => {
            tracing::error!(
                channel_id,
                user_id,
                error = %err,
                "Failed to process message"
            );
            format!("エラーが発生しました: {err}")
        }
    };

    ctx.say(reply).await?;
    Ok(())
}
