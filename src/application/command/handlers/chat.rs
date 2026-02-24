use crate::{
    application::{chat::chat_service::process_message, command::command_registry::Context},
    models::error::AppError,
    shared::discord_utils::split_message,
};

#[poise::command(prefix_command, slash_command)]
pub async fn chat(
    ctx: Context<'_>,
    #[description = "Prompt"] prompt: String,
) -> anyhow::Result<()> {
    let _typing = ctx.channel_id().start_typing(&ctx.serenity_context().http);

    let data = ctx.data();
    let channel_id = ctx.channel_id().get();
    let user_id = ctx.author().id.get();

    let reply = match process_message(
        data.ai_client.as_ref(),
        data.short_term_store.as_ref(),
        data.long_term_store.as_ref(),
        channel_id,
        user_id,
        prompt,
    )
    .await
    {
        Ok(response) => response,
        Err(err) => {
            let user_message = match &err {
                AppError::AIGeneration(_) => "AI応答の生成に失敗しました。",
                AppError::Embedding(_) => "テキストの処理に失敗しました。",
                AppError::Store(_) => "記憶の検索に失敗しました。",
                _ => "予期しないエラーが発生しました。",
            };
            tracing::error!(
                channel_id,
                user_id,
                error = %err,
                "Failed to process message"
            );
            user_message.to_string()
        }
    };

    let chunks = split_message(&reply);
    for (i, chunk) in chunks.iter().enumerate() {
        if i == 0 {
            ctx.say(*chunk).await?;
        } else {
            ctx.channel_id()
                .say(&ctx.serenity_context().http, *chunk)
                .await?;
        }
    }

    Ok(())
}
