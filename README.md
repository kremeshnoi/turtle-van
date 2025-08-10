# Turtle Van

A Discord music bot built with Rust, featuring YouTube playback capabilities.

## Features

- Join/leave voice channels
- Play music from YouTube
- Playback controls (pause, skip)
- Track information display

## Requirements

- Rust (2024 edition)
- Discord bot token
- Voice channel permissions

## Setup

1. Clone the repository
2. Create a `.env` file with your `DISCORD_TOKEN`
3. Run `cargo build --release`
4. Execute `./target/release/turtle-van`

## Commands

- Join voice channel
- Leave voice channel  
- Play YouTube content
- Pause/resume playback
- Skip tracks
- Show current track

Built with Serenity, Songbird, and Poise.