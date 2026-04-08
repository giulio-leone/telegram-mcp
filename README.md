# telegram-mcp

Pure Rust Telegram client with MCP integration — stealth mode, broadcast, tracking.

Built on [grammers](https://github.com/Lonami/grammers) (pure Rust MTProto implementation).

## Setup

1. Get API credentials from [https://my.telegram.org](https://my.telegram.org)
2. Set environment variables:
   ```bash
   export TG_API_ID=12345678
   export TG_API_HASH=abcdef0123456789abcdef0123456789
   ```
3. Login:
   ```bash
   cargo run --bin tg-pair
   ```

## CLI Tools

| Command | Description |
|---------|-------------|
| `tg-pair` | Phone login with SMS code + 2FA |
| `tg-send` | Send message to user/group |
| `tg-broadcast` | Send to multiple recipients |
| `tg-track` | Online presence tracker (WIP) |
| `tg-poll` | Event polling daemon (WIP) |

## Stealth Mode

Suppress read receipts and online status:
```bash
tg-send --stealth @username "hello"
# or
TG_STEALTH=1 tg-send @username "hello"
```

## Architecture

```
crates/
├── tg-client/     — Core MTProto client (grammers wrapper)
├── domain/        — Ports + domain models
└── mcp-server/    — CLI binaries + future MCP bridge
```

## License

MIT
