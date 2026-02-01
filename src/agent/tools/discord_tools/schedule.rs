use crate::agent::tools::tools::build_tool;
use crate::agent::tools::discord::{
    err, get_bool, get_channel_id, get_guild_id_default, get_string, get_u64, ok, parse_scheduled_event_status, parse_scheduled_event_type, parse_timestamp, to_value
};

use anyhow::Result;
use async_openai::types::chat::ChatCompletionTools;
use serde_json::{json, Value};
use serenity::all::{Context, CreateScheduledEvent, EditScheduledEvent, ScheduledEventId};

pub fn definitions() -> Result<Vec<ChatCompletionTools>> {
    let mut tools = Vec::new();

    tools.push(build_tool(
        "get_scheduled_events",
        "List scheduled events in a guild.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "with_user_count": { "type": "boolean", "description": "Include user counts." }
            },
            "required": ["guild_id"]
        }),
    )?);

    tools.push(build_tool(
        "create_scheduled_event",
        "Create a scheduled event in a guild.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "name": { "type": "string", "description": "Event name." },
                "start_time": { "type": "string", "description": "RFC3339 start timestamp." },
                "kind": { "type": "string", "description": "Event type (voice, stage, external)." },
                "channel_id": { "type": "integer", "description": "Channel id for voice/stage events." },
                "end_time": { "type": "string", "description": "RFC3339 end timestamp." },
                "description": { "type": "string", "description": "Event description." },
                "location": { "type": "string", "description": "Location for external events." }
            },
            "required": ["guild_id", "name", "start_time", "kind"]
        }),
    )?);

    tools.push(build_tool(
        "modify_scheduled_event",
        "Modify a scheduled event.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "event_id": { "type": "integer", "description": "Scheduled event id." },
                "name": { "type": "string", "description": "Event name." },
                "description": { "type": "string", "description": "Event description." },
                "start_time": { "type": "string", "description": "RFC3339 start timestamp." },
                "end_time": { "type": "string", "description": "RFC3339 end timestamp." },
                "channel_id": { "type": "integer", "description": "Channel id for voice/stage events." },
                "location": { "type": "string", "description": "Location for external events." },
                "kind": { "type": "string", "description": "Event type (voice, stage, external)." },
                "status": { "type": "string", "description": "Event status (scheduled, active, completed, canceled)." }
            },
            "required": ["guild_id", "event_id"]
        }),
    )?);

    tools.push(build_tool(
        "delete_scheduled_event",
        "Delete a scheduled event.",
        json!({
            "type": "object",
            "properties": {
                "guild_id": { "type": "integer", "description": "Guild id." },
                "event_id": { "type": "integer", "description": "Scheduled event id." }
            },
            "required": ["guild_id", "event_id"]
        }),
    )?);

    Ok(tools)
}

async fn get_scheduled_events(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let with_user_count = get_bool(args, "with_user_count").unwrap_or(false);

    match guild_id.scheduled_events(&ctx.http, with_user_count).await {
        Ok(events) => ok(to_value(&events)),
        Err(error) => err(format!("Failed to fetch scheduled events: {error}")),
    }
}

async fn create_scheduled_event(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(name) = get_string(args, "name") else {
        return err("name is required");
    };
    let Some(start_time) = args.get("start_time").and_then(parse_timestamp) else {
        return err("start_time is required (RFC3339)");
    };
    let Some(kind) = args.get("kind").and_then(parse_scheduled_event_type) else {
        return err("kind is required");
    };

    let mut builder = CreateScheduledEvent::new(kind, name, start_time);

    if let Some(description) = get_string(args, "description") {
        builder = builder.description(description);
    }
    if let Some(end_time) = args.get("end_time").and_then(parse_timestamp) {
        builder = builder.end_time(end_time);
    }
    if let Some(channel_id) = get_channel_id(args, "channel_id") {
        builder = builder.channel_id(channel_id);
    }
    if let Some(location) = get_string(args, "location") {
        builder = builder.location(location);
    }

    match guild_id.create_scheduled_event(ctx, builder).await {
        Ok(event) => ok(to_value(&event)),
        Err(error) => err(format!("Failed to create scheduled event: {error}")),
    }
}

async fn modify_scheduled_event(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(event_id) = get_u64(args, "event_id").map(ScheduledEventId::new) else {
        return err("event_id is required");
    };

    let mut builder = EditScheduledEvent::new();
    let mut changed = false;

    if let Some(name) = get_string(args, "name") {
        builder = builder.name(name);
        changed = true;
    }
    if let Some(description) = get_string(args, "description") {
        builder = builder.description(description);
        changed = true;
    }
    if let Some(start_time) = args.get("start_time").and_then(parse_timestamp) {
        builder = builder.start_time(start_time);
        changed = true;
    }
    if let Some(end_time) = args.get("end_time").and_then(parse_timestamp) {
        builder = builder.end_time(end_time);
        changed = true;
    }
    if let Some(channel_id) = get_channel_id(args, "channel_id") {
        builder = builder.channel_id(channel_id);
        changed = true;
    }
    if let Some(location) = get_string(args, "location") {
        builder = builder.location(location);
        changed = true;
    }
    if let Some(kind) = args.get("kind").and_then(parse_scheduled_event_type) {
        builder = builder.kind(kind);
        changed = true;
    }
    if let Some(status) = args.get("status").and_then(parse_scheduled_event_status) {
        builder = builder.status(status);
        changed = true;
    }

    if !changed {
        return err("No scheduled event fields provided to modify");
    }

    match guild_id.edit_scheduled_event(ctx, event_id, builder).await {
        Ok(event) => ok(to_value(&event)),
        Err(error) => err(format!("Failed to modify scheduled event: {error}")),
    }
}

async fn delete_scheduled_event(ctx: &Context, args: &Value) -> String {
    let Some(guild_id) = get_guild_id_default(args) else {
        return err("guild_id is required");
    };
    let Some(event_id) = get_u64(args, "event_id").map(ScheduledEventId::new) else {
        return err("event_id is required");
    };

    match guild_id.delete_scheduled_event(&ctx.http, event_id).await {
        Ok(()) => ok(json!({ "deleted": true })),
        Err(error) => err(format!("Failed to delete scheduled event: {error}")),
    }
}

pub async fn execute(ctx: &Context, name: &str, args: &Value) -> Option<String> {
    match name {
        "get_scheduled_events" => Some(get_scheduled_events(ctx, args).await),
        "create_scheduled_event" => Some(create_scheduled_event(ctx, args).await),
        "modify_scheduled_event" => Some(modify_scheduled_event(ctx, args).await),
        "delete_scheduled_event" => Some(delete_scheduled_event(ctx, args).await),
        _ => None,
    }
}
