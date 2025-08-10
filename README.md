<div align="center">
  <img src="https://github.com/user-attachments/assets/b259cd71-20af-45af-b088-ef4a9d9cda05" alt="Turtle Van Logo" width="150" height="150"/>
  
  # Turtle Van
  
  **Discord music bot built with Rust**
  
  ---
</div>

## Commands

| Command | Description | Usage |
|---------|-------------|-------|
| `/join` | Join your voice channel | `!join` |
| `/leave` | Leave current voice channel | `!leave` |
| `/play` | Play YouTube content | `!play <url or search>` |
| `/pause` | Pause current track | `!pause` |
| `/skip` | Skip to next track | `!skip` |
| `/now` | Show current track info | `!now` |

## Quick Start

### Prerequisites
- Rust 2024 edition
- Discord bot token with voice permissions

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
