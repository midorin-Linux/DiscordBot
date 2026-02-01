use crate::agent::tools::discord_tools::{
    channel, emoji, guild, invite, member, message, role, schedule, thread, voice,
};

use anyhow::Result;
use async_openai::types::chat::{ChatCompletionTool, ChatCompletionTools, FunctionObjectArgs};
use serde_json::{json, Value};
use serenity::all::Context;

pub fn build_tool(name: &str, description: &str, parameters: Value) -> Result<ChatCompletionTools> {
    Ok(ChatCompletionTools::Function(ChatCompletionTool {
        function: FunctionObjectArgs::default()
            .name(name)
            .description(description)
            .parameters(parameters)
            .build()?,
    }))
}

pub fn tool_definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.extend(channel::definitions()?);
    tools.extend(emoji::definitions()?);
    tools.extend(guild::definitions()?);
    tools.extend(invite::definitions()?);
    tools.extend(member::definitions()?);
    tools.extend(message::definitions()?);
    tools.extend(role::definitions()?);
    tools.extend(schedule::definitions()?);
    tools.extend(thread::definitions()?);
    tools.extend(voice::definitions()?);

    Ok(tools)
}

pub async fn execute_tool_call(ctx: &Context, name: &str, arguments: &str) -> String {
    let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
    tracing::info!("Colling tool: {:#?} with args: {:?}", name, args);

    if let Some(result) = channel::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = emoji::execute(ctx, name, &args).await {
        return result;
    }

    if let Some(result) = guild::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = invite::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = member::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = message::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = role::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = schedule::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = thread::execute(ctx, name, &args).await {
        return result;
    }
    
    if let Some(result) = voice::execute(ctx, name, &args).await {
        return result;
    }

    json!({ "error": format!("unknown tool: {}", name) }).to_string()
}
