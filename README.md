# Turtle Van

Discord music bot

## Features

- YouTube playback (search, URL, playlists)
- Queue management with shuffle support
- Background playlist loading with cancellation
- Slash commands
- Built with Rust 2024 edition

---

## Commands

| Command | Description |
|:--------|:------------|
| `/join` | Join your voice channel |
| `/leave` | Leave current voice channel |
| `/play <query>` | Play YouTube content (URL, playlist, or search) |
| `/pause` | Toggle pause/resume |
| `/next` | Skip to the next track |
| `/clear` | Clear entire queue and stop playlist loading |
| `/shuffle` | Shuffle the queue |
| `/now` | Show current track info |

---

## Quick Start

### Prerequisites

- Rust 2024 edition
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed via pip (not apt):
  
  ```bash
  pip3 install "yt-dlp[default] @ https://github.com/yt-dlp/yt-dlp/archive/master.tar.gz"
  ```
- [Deno](https://deno.land) in PATH — required by yt-dlp for YouTube JS challenges
- Discord bot token with voice permissions

### Installation

```bash
git clone https://github.com/nickshelmet/turtle-van.git
cd turtle-van

echo "DISCORD_TOKEN=your_token_here" > .env

cargo build --release
./target/release/turtle-van
```

---

## Tech Stack

- [Serenity](https://github.com/serenity-rs/serenity) - Discord API
- [Poise](https://github.com/serenity-rs/poise) - Slash command framework
- [Songbird](https://github.com/serenity-rs/songbird) - Voice and audio
- [Tokio](https://tokio.rs) - Async runtime
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) - YouTube extraction
