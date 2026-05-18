
# HeyReach API
This is a Rust WebAssembly component that provides the HeyReach API integration functionality.

## Functionality

### Auth
- check-api-key: Validate an API key against the HeyReach API.

### Campaigns
- campaigns-get-all: Get all campaigns with optional filtering by status, keyword, and account.
- campaigns-get-by-id: Get a specific campaign by its ID.
- campaigns-resume: Resume a paused campaign.
- campaigns-pause: Pause an active campaign.
- campaigns-add-leads: Add leads to a campaign (returns count).
- campaigns-add-leads-v2: Add leads to a campaign (returns added/updated/failed counts).

### Lists
- lists-get-all: Get all lead lists with optional keyword filtering.
- lists-get-by-id: Get a specific list by its ID.
- lists-get-leads: Get paginated leads from a list.
- lists-add-leads: Add leads to a list.
- lists-add-leads-v2: Add leads to a list (returns added/updated/failed counts).
- lists-delete-leads: Delete leads from a list by membership IDs.
- lists-delete-leads-by-profile-url: Delete leads from a list by LinkedIn profile URLs.

### Lead and Tags
- lead-get: Get a lead by LinkedIn profile URL.
- lead-get-lists: Get all lists a lead belongs to.
- lead-get-tags: Get tags assigned to a lead.
- lead-replace-tags: Replace all tags on a lead.

### Inbox
- inbox-get-conversations-v2: Get paginated inbox conversations with filtering.
- inbox-send-message: Send a message in an inbox conversation.

### LinkedIn Accounts
- li-account-get-all: Get all linked LinkedIn accounts with optional keyword filtering.

### Webhooks
- webhooks-create: Create a new webhook for campaign events.
- webhooks-get-by-id: Get a specific webhook by its ID.
- webhooks-get-all: Get all webhooks with pagination.
- webhooks-delete: Delete a webhook by its ID.

## Building
You can build this component by running the following command in the project in your terminal:
```Bash
wkg wit fetch
cargo build --target=wasm32-wasip2 --release
```

## Interfacing
To use this WebAssembly component in your own WebAssembly component, simply import this interface into your component like so:
```WIT
world your-world {
    import wasco-dev:heyreach-api@0.1.1/heyreach-api;

    // Your world definition.
}
```
