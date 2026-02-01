use crate::agent::tools::tools::build_tool;
use crate::agent::tools::discord::{
    err, get_bool, get_channel_id, get_guild_id_default, get_string, get_u32, get_u8, ok, to_value
};

use anyhow::Result;
use async_openai::types::chat::ChatCompletionTools;
use serde_json::{json, Value};
use serenity::all::{Context, CreateInvite};

pub fn definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.push(build_tool(
        "get_invite_list",
        "List guild invites.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." }
            },
            "required": ["guild_id"]
        }),
    )?);

    tools.push(build_tool(
        "create_invite",
        "Create a channel invite.",
        json!({
            "type": "object",
            "properties": {
                "channel_id": { "type": "integer", "description": "Channel id." },
                "max_age": { "type": "integer", "description": "Max age in seconds." },
                "max_uses": { "type": "integer", "description": "Max uses." },
                "temporary": { "type": "boolean", "description": "Temporary membership." },
                "unique": { "type": "boolean", "description": "Unique invite." }
            },
            "required": ["channel_id"]
        }),
    )?);

    tools.push(build_tool(
        "delete_invite",
        "Delete an invite by code.",
        json!({
            "type": "object",
            "properties": {
                "code": { "type": "string", "description": "Invite code." }
            },
            "required": ["code"]
        }),
    )?);

    Ok(tools)
}

async fn get_invite_list(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };

    match guild_id.invites(&ctx.http).await {
        Ok(invites) => ok(to_value(&invites)),
        Err(error) => err(format!("Failed to fetch invites: {error}")),
    }
}

async fn create_invite(ctx: &Context, args: &Value) -> String {
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };

    let mut builder = CreateInvite::new();
    if let Some(max_age) = get_u32(args, "max_age") {
        builder = builder.max_age(max_age);
    }
    if let Some(max_uses) = get_u8(args, "max_uses") {
        builder = builder.max_uses(max_uses);
    }
    if let Some(temporary) = get_bool(args, "temporary") {
        builder = builder.temporary(temporary);
    }
    if let Some(unique) = get_bool(args, "unique") {
        builder = builder.unique(unique);
    }

    match channel_id.create_invite(ctx, builder).await {
        Ok(invite) => ok(to_value(&invite)),
        Err(error) => err(format!("Failed to create invite: {error}")),
    }
}

async fn delete_invite(ctx: &Context, args: &Value) -> String {
    let Some(code) = get_string(args, "code") else {
        return err("code is required");
    };

    match ctx.http.delete_invite(code.as_str(), None).await {
        Ok(invite) => ok(to_value(&invite)),
        Err(error) => err(format!("Failed to delete invite: {error}")),
    }
}

pub async fn execute(ctx: &Context, name: &str, args: &Value) -> Option<String> {
    match name {
        "get_invite_list" => Some(get_invite_list(ctx, args).await),
        "create_invite" => Some(create_invite(ctx, args).await),
        "delete_invite" => Some(delete_invite(ctx, args).await),
        _ => None,
    }
}
