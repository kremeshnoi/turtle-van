<div align="center">
  <img src="https://github.com/user-attachments/assets/b259cd71-20af-45af-b088-ef4a9d9cda05" alt="Turtle Van Logo" width="140" height="140" />
  <h1>Turtle Van</h1>
  <p><b>Discord music bot built with Rust</b></p>
  <br/>
</div>

---

## Features

- Slash command support
- YouTube playback (search or URL)
- Queue management
- Voice connection handling
- Built in Rust 2024 for performance and safety

---

## Commands

| Command | Description | Usage |
|:--------|:-------------|:------|
| `/join`  | Join your voice channel | `/join` |
| `/leave` | Leave current voice channel | `/leave` |
| `/play`  | Play YouTube content (URL or search query) | `/play <url or search>` |
| `/pause` | Pause the current track | `/pause` |
| `/skip`  | Skip to the next track | `/skip` |
| `/now`   | Show info about the current track | `/now` |

---

## Quick Start

### Prerequisites
- **Rust 2024 edition**
- **Discord bot token** with voice permissions

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/turtle-van.git
cd turtle-van

# Set up environment
echo "DISCORD_TOKEN=your_token_here" > .env

# Build and run
cargo build --release
./target/release/turtle-van
```

## Architecture Overview

Turtle Van is powered by:

- **[Serenity](https://github.com/serenity-rs/serenity)** – handles Discord API interactions and event dispatching
- **[Songbird](https://github.com/serenity-rs/songbird)** – provides voice connection management and audio streaming
- **Tokio** – async runtime enabling concurrent operations for voice and command handling
- **YouTube extractor** – lightweight module for resolving YouTube URLs and search queries
- **Slash command framework** – structured routing for `/` commands with contextual responses
