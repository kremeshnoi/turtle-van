<div align="center">
  <h1>Turtle Van</h1>
  <p><b>Discord music bot built with Rust</b></p>
  <img width="1000" alt="Turtle Van" src="https://github.com/user-attachments/assets/cc131f2c-bd9b-4b21-b17e-517989594d81" />
</div>

---

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
| `/skip` | Skip current track |
| `/skip all` | Clear entire queue and stop playlist loading |
| `/shuffle` | Shuffle the queue |
| `/now` | Show current track info |

---

## Quick Start

### Prerequisites

- Rust 2024 edition
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) installed and in PATH
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
