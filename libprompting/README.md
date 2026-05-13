# libprompting

`libprompting` is a library built on `snapd-rs` which implements the core logic of what it means to be a prompting client.

At a high level, the library should be an async, event-driven layer with ID-based prompt tracking and separate ingress/egress paths.

The core flow is:

1. Query `GET /v2/notices?types=interfaces-requests-prompt&timeout=1m` to get prompt notices. If there are no outstanding prompts, snapd waits up to the timeout and returns an empty list; if a prompt appears first, it is returned.
2. Once a prompt notice is received, retrieve full prompt details via `GET /v2/interfaces/requests/prompts/{id}`.
3. Emit the prompt to the consuming application.
4. When the application replies, send that reply to snapd via `POST /v2/interfaces/requests/prompts/{id}`.

## Recommended architecture

Use internal background tasks and a central outstanding-prompt map keyed by prompt ID:

1. **Ingress task** watches notices and fetches prompt details.
2. **Dispatcher task** emits prompts to the application and inserts them into `HashMap<prompt_id, PromptState>`.
3. **Egress task** accepts replies from the application and forwards them to snapd.

Because replies are keyed by `prompt_id`, replies can be sent in any order for any outstanding prompt.

## Application-facing API model

Expose:

- a stream/subscription for prompt lifecycle events (new prompt + resolution),
- a `send_reply` method for replies.

`send_reply` should return snapd's outcome, including all prompts satisfied by that reply:

```rust
pub async fn send_reply(&self, reply: PromptReply) -> Result<ReplyOutcome, Error>;

pub struct ReplyOutcome {
    pub replied_prompt_id: PromptId,
    pub satisfied_prompt_ids: Vec<PromptId>,
}
```

After a successful reply, the library should remove all `satisfied_prompt_ids` from the outstanding map and emit resolution events for them.

## Resolution and prompt removal

Prompts may disappear because they timed out or were handled by another prompting client. snapd indicates this with a `"resolved"` key in notice data.

When `"resolved"` is observed for a prompt ID, the library should:

1. remove the prompt from the outstanding map (idempotently),
2. emit a resolution event to the application so it can stop any in-progress handling for that prompt.

A single event stream works well for this:

```rust
pub enum PromptEvent {
    PromptRaised(PromptRequest),
    PromptResolved { prompt_id: PromptId, reason: ResolvedReason },
}
```

This keeps application state synchronized with snapd even when prompt lifecycles change externally.
