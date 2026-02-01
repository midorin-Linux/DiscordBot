use crate::agent::tools::tools::build_tool;
use crate::agent::tools::discord::{
    err, get_bool, get_channel_id, get_guild_id_default, get_user_id, ok, to_value
};

use anyhow::Result;
use async_openai::types::chat::ChatCompletionTools;
use serde_json::{json, Value};
use serenity::all::{Context, EditMember};

pub fn definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.push(build_tool(
        "move_member_voice",
        "Move a member to a voice channel.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "user_id": { "type": "integer", "description": "User id." },
                "channel_id": { "type": "integer", "description": "Target voice channel id." }
            },
            "required": ["guild_id", "user_id", "channel_id"]
        }),
    )?);

    tools.push(build_tool(
        "disconnect_member_voice",
        "Disconnect a member from voice.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "user_id": { "type": "integer", "description": "User id." }
            },
            "required": ["guild_id", "user_id"]
        }),
    )?);

    tools.push(build_tool(
        "mute_member",
        "Server mute a member.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "user_id": { "type": "integer", "description": "User id." },
                "mute": { "type": "boolean", "description": "Mute flag." }
            },
            "required": ["guild_id", "user_id", "mute"]
        }),
    )?);

    tools.push(build_tool(
        "deafen_member",
        "Server deafen a member.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "user_id": { "type": "integer", "description": "User id." },
                "deafen": { "type": "boolean", "description": "Deafen flag." }
            },
            "required": ["guild_id", "user_id", "deafen"]
        }),
    )?);

    Ok(tools)
}

async fn move_member_voice(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(user_id) = get_user_id(args, "user_id") else {
        return err("user_id is required");
    };
    let Some(channel_id) = get_channel_id(args, "channel_id") else {
        return err("channel_id is required");
    };

    let builder = EditMember::new().voice_channel(channel_id);
    match guild_id.edit_member(ctx, user_id, builder).await {
        Ok(member) => ok(to_value(&member)),
        Err(error) => err(format!("Failed to move member: {error}")),
    }
}

async fn disconnect_member_voice(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(user_id) = get_user_id(args, "user_id") else {
        return err("user_id is required");
    };

    let builder = EditMember::new().disconnect_member();
    match guild_id.edit_member(ctx, user_id, builder).await {
        Ok(member) => ok(to_value(&member)),
        Err(error) => err(format!("Failed to disconnect member: {error}")),
    }
}

async fn mute_member(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(user_id) = get_user_id(args, "user_id") else {
        return err("user_id is required");
    };
    let Some(mute) = get_bool(args, "mute") else {
        return err("mute is required");
    };

    let builder = EditMember::new().mute(mute);
    match guild_id.edit_member(ctx, user_id, builder).await {
        Ok(member) => ok(to_value(&member)),
        Err(error) => err(format!("Failed to mute member: {error}")),
    }
}

async fn deafen_member(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(user_id) = get_user_id(args, "user_id") else {
        return err("user_id is required");
    };
    let Some(deafen) = get_bool(args, "deafen") else {
        return err("deafen is required");
    };

    let builder = EditMember::new().deafen(deafen);
    match guild_id.edit_member(ctx, user_id, builder).await {
        Ok(member) => ok(to_value(&member)),
        Err(error) => err(format!("Failed to deafen member: {error}")),
    }
}

pub async fn execute(ctx: &Context, name: &str, args: &Value) -> Option<String> {
    match name {
        "move_member_voice" => Some(move_member_voice(ctx, args).await),
        "disconnect_member_voice" => Some(disconnect_member_voice(ctx, args).await),
        "mute_member" => Some(mute_member(ctx, args).await),
        "deafen_member" => Some(deafen_member(ctx, args).await),
        _ => None,
    }
}
