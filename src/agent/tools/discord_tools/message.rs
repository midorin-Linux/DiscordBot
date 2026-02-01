use crate::agent::tools::tools::build_tool;
use crate::agent::tools::discord::{
    err, get_bool, get_channel_id, get_message_id, get_string, get_u64_list, get_u8, get_user_id, ok, to_value
};
use std::str::FromStr;

use anyhow::Result;
use async_openai::types::chat::ChatCompletionTools;
use serde_json::{json, Value};
use serenity::all::{Context, CreateMessage, EditMessage, GetMessages, MessageId, ReactionType};

pub fn definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.push(build_tool(
        "send_message",
        "Send a message to a channel.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "content": { "type": "string", "description": "Message content." },
                "tts": { "type": "boolean", "description": "Text-to-speech flag." }
            },
            "required": ["channel_id", "content"]
        }),
    )?);

    tools.push(build_tool(
        "edit_message",
        "Edit a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." },
                "content": { "type": "string", "description": "New message content." }
            },
            "required": ["channel_id", "message_id", "content"]
        }),
    )?);

    tools.push(build_tool(
        "delete_message",
        "Delete a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." }
            },
            "required": ["channel_id", "message_id"]
        }),
    )?);

    tools.push(build_tool(
        "get_message",
        "Get a specific message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." }
            },
            "required": ["channel_id", "message_id"]
        }),
    )?);

    tools.push(build_tool(
        "bulk_delete_messages",
        "Bulk delete messages from a channel.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_ids": { "type": "array", "items": { "type": "integer" }, "description": "Message ids to delete." }
            },
            "required": ["channel_id", "message_ids"]
        }),
    )?);

    tools.push(build_tool(
        "get_message_history",
        "Fetch message history from a channel.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "limit": { "type": "integer", "description": "Max messages (1-100)." },
                "before": { "type": "integer", "description": "Fetch messages before this message id." },
                "after": { "type": "integer", "description": "Fetch messages after this message id." },
                "around": { "type": "integer", "description": "Fetch messages around this message id." }
            },
            "required": ["channel_id"]
        }),
    )?);

    tools.push(build_tool(
        "pin_message",
        "Pin a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." }
            },
            "required": ["channel_id", "message_id"]
        }),
    )?);

    tools.push(build_tool(
        "unpin_message",
        "Unpin a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." }
            },
            "required": ["channel_id", "message_id"]
        }),
    )?);

    tools.push(build_tool(
        "add_reaction",
        "Add a reaction to a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." },
                "emoji": { "type": "string", "description": "Emoji to react with." }
            },
            "required": ["channel_id", "message_id", "emoji"]
        }),
    )?);

    tools.push(build_tool(
        "remove_reaction",
        "Remove a reaction from a message.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "message_id": { "type": "integer", "description": "Message id." },
                "emoji": { "type": "string", "description": "Emoji to remove." },
                "user_id": { "type": "integer", "description": "User id to remove reaction for." }
            },
            "required": ["channel_id", "message_id", "emoji"]
        }),
    )?);

    Ok(tools)
}

async fn send_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(content) = get_string(args, "content") else {
        return err("content is required");
    };

    let mut builder = CreateMessage::new().content(content);
    if let Some(tts) = get_bool(args, "tts") {
        builder = builder.tts(tts);
    }

    match channel_id.send_message(ctx, builder).await {
        Ok(message) => ok(to_value(&message)),
        Err(error) => err(format!("Failed to send message: {error}")),
    }
}

async fn edit_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };
    let Some(content) = get_string(args, "content") else {
        return err("content is required");
    };

    let builder = EditMessage::new().content(content);

    match channel_id.edit_message(ctx, message_id, builder).await {
        Ok(message) => ok(to_value(&message)),
        Err(error) => err(format!("Failed to edit message: {error}")),
    }
}

async fn delete_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };

    match channel_id.delete_message(&ctx.http, message_id).await {
        Ok(()) => ok(json!({ "deleted": true })),
        Err(error) => err(format!("Failed to delete message: {error}")),
    }
}

async fn get_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };

    match channel_id.message(&ctx.http, message_id).await {
        Ok(message) => ok(to_value(&message)),
        Err(error) => err(format!("Failed to fetch message: {error}")),
    }
}

async fn bulk_delete_messages(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_ids) = get_u64_list(args, "message_ids") else {
        return err("message_ids is required");
    };

    let message_ids: Vec<MessageId> = message_ids.into_iter().map(MessageId::new).collect();

    match channel_id.delete_messages(&ctx.http, &message_ids).await {
        Ok(()) => ok(json!({ "deleted": message_ids.len() })),
        Err(error) => err(format!("Failed to bulk delete messages: {error}")),
    }
}

async fn get_message_history(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };

    let mut builder = GetMessages::new();
    if let Some(limit) = get_u8(args, "limit") {
        builder = builder.limit(limit);
    }
    if let Some(before) = get_message_id(args, "before") {
        builder = builder.before(before);
    }
    if let Some(after) = get_message_id(args, "after") {
        builder = builder.after(after);
    }
    if let Some(around) = get_message_id(args, "around") {
        builder = builder.around(around);
    }

    match channel_id.messages(ctx, builder).await {
        Ok(messages) => ok(to_value(&messages)),
        Err(error) => err(format!("Failed to fetch message history: {error}")),
    }
}

async fn pin_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };

    match channel_id.pin(&ctx.http, message_id).await {
        Ok(()) => ok(json!({ "pinned": true })),
        Err(error) => err(format!("Failed to pin message: {error}")),
    }
}

async fn unpin_message(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };

    match channel_id.unpin(&ctx.http, message_id).await {
        Ok(()) => ok(json!({ "unpinned": true })),
        Err(error) => err(format!("Failed to unpin message: {error}")),
    }
}

async fn add_reaction(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };
    let Some(emoji) = get_string(args, "emoji") else {
        return err("emoji is required");
    };
    let reaction = match ReactionType::from_str(&emoji) {
        Ok(reaction) => reaction,
        Err(_) => return err("Invalid emoji format"),
    };

    match channel_id.create_reaction(&ctx.http, message_id, reaction).await {
        Ok(()) => ok(json!({ "reacted": true })),
        Err(error) => err(format!("Failed to add reaction: {error}")),
    }
}

async fn remove_reaction(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };
    let Some(message_id) = get_message_id(args, "message_id") else {
        return err("message_id is required");
    };
    let Some(emoji) = get_string(args, "emoji") else {
        return err("emoji is required");
    };
    let reaction = match ReactionType::from_str(&emoji) {
        Ok(reaction) => reaction,
        Err(_) => return err("Invalid emoji format"),
    };
    let user_id = get_user_id(args, "user_id");

    match channel_id
        .delete_reaction(&ctx.http, message_id, user_id, reaction)
        .await
    {
        Ok(()) => ok(json!({ "removed": true })),
        Err(error) => err(format!("Failed to remove reaction: {error}")),
    }
}

pub async fn execute(ctx: &Context, name: &str, args: &Value) -> Option<String> {
    match name {
        "send_message" => Some(send_message(ctx, args).await),
        "edit_message" => Some(edit_message(ctx, args).await),
        "delete_message" => Some(delete_message(ctx, args).await),
        "get_message" => Some(get_message(ctx, args).await),
        "bulk_delete_messages" => Some(bulk_delete_messages(ctx, args).await),
        "get_message_history" => Some(get_message_history(ctx, args).await),
        "pin_message" => Some(pin_message(ctx, args).await),
        "unpin_message" => Some(unpin_message(ctx, args).await),
        "add_reaction" => Some(add_reaction(ctx, args).await),
        "remove_reaction" => Some(remove_reaction(ctx, args).await),
        _ => None,
    }
}
