<div align="center">
  <img width="768" height="254" alt="afa5fdfb536a4fb6b48da95d47e35d44 - Copy" src="https://github.com/user-attachments/assets/45c89ffe-1d82-425a-805d-a751942f51f7" />
  <p><b>Turtle Van — Discord music bot built with Rust</b></p>
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

---

<img width="160" alt="Turtle Van" src="https://github.com/user-attachments/assets/4da2f4b8-420c-4513-b87c-47aa26d1b34f" />
