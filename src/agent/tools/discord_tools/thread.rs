use crate::agent::tools::tools::build_tool;
use crate::agent::tools::discord::{
    err, get_bool, get_channel_id, get_guild_id_default, get_message_id, get_string, get_u16, get_user_id, ok, parse_auto_archive_duration, parse_thread_type, to_value
};

use anyhow::Result;
use async_openai::types::chat::ChatCompletionTools;
use serde_json::{json, Value};
use serenity::all::{Context, CreateThread};

pub fn definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.push(build_tool(
        "create_thread",
        "Create a thread in a channel.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Parent channel id." },
                "name": { "type": "string", "description": "Thread name." },
                "kind": { "type": "string", "description": "Thread type (public, private, news)." },
                "auto_archive_duration": { "type": "integer", "description": "Auto archive minutes (60, 1440, 4320, 10080)." },
                "rate_limit_per_user": { "type": "integer", "description": "Slowmode in seconds." },
                "invitable": { "type": "boolean", "description": "Allow non-mods to invite." },
                "message_id": { "type": "integer", "description": "Message id to start thread from." }
            },
            "required": ["channel_id", "name"]
        }),
    )?);

    tools.push(build_tool(
        "delete_thread",
        "Delete a thread.",
        json!({
            "type": "object",
            "properties": {
                "thread_id": { "type": "integer", "description": "Thread channel id." }
            },
            "required": ["thread_id"]
        }),
    )?);

    tools.push(build_tool(
        "get_thread_list",
        "List active threads in a guild.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." }
            },
            "required": ["guild_id"]
        }),
    )?);

    tools.push(build_tool(
        "add_thread_member",
        "Add a user to a thread.",
        json!({
            "type": "object",
            "properties": {
                "thread_id": { "type": "integer", "description": "Thread channel id." },
                "user_id": { "type": "integer", "description": "User id." }
            },
            "required": ["thread_id", "user_id"]
        }),
    )?);

    Ok(tools)
}

async fn create_thread(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(name) = get_string(args, "name") else {
        return err("name is required");
    };

    let mut builder = CreateThread::new(name);
    if let Some(kind) = args.get("kind").and_then(parse_thread_type) {
        builder = builder.kind(kind);
    }
    if let Some(duration) = args.get("auto_archive_duration").and_then(parse_auto_archive_duration) {
        builder = builder.auto_archive_duration(duration);
    }
    if let Some(rate_limit) = get_u16(args, "rate_limit_per_user") {
        builder = builder.rate_limit_per_user(rate_limit);
    }
    if let Some(invitable) = get_bool(args, "invitable") {
        builder = builder.invitable(invitable);
    }

    let message_id = get_message_id(args, "message_id");
    let result = match message_id {
        Some(message_id) => channel_id
            .create_thread_from_message(ctx, message_id, builder)
            .await,
        None => channel_id.create_thread(ctx, builder).await,
    };

    match result {
        Ok(thread) => ok(to_value(&thread)),
        Err(error) => err(format!("Failed to create thread: {error}")),
    }
}

async fn delete_thread(ctx: &Context, args: &Value) -> String {
    let Some(thread_id) = get_channel_id(args, "thread_id") else {
        return err("thread_id is required");
    };

    match thread_id.delete(&ctx.http).await {
        Ok(channel) => ok(to_value(&channel)),
        Err(error) => err(format!("Failed to delete thread: {error}")),
    }
}

async fn get_thread_list(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };

    match guild_id.get_active_threads(&ctx.http).await {
        Ok(threads) => ok(to_value(&threads)),
        Err(error) => err(format!("Failed to fetch threads: {error}")),
    }
}

async fn add_thread_member(ctx: &Context, args: &Value) -> String {
    let Some(thread_id) = get_channel_id(args, "thread_id") else {
        return err("thread_id is required");
    };
    let Some(user_id) = get_user_id(args, "user_id") else {
        return err("user_id is required");
    };

    match thread_id.add_thread_member(&ctx.http, user_id).await {
        Ok(()) => ok(json!({ "added": true })),
        Err(error) => err(format!("Failed to add thread member: {error}")),
    }
}

pub async fn execute(ctx: &Context, name: &str, args: &Value) -> Option<String> {
    match name {
        "create_thread" => Some(create_thread(ctx, args).await),
        "delete_thread" => Some(delete_thread(ctx, args).await),
        "get_thread_list" => Some(get_thread_list(ctx, args).await),
        "add_thread_member" => Some(add_thread_member(ctx, args).await),
        _ => None,
    }
}
