# Source Directory Overview

This directory contains the core implementation of the Discord bot, structured according to a **Feature-Based Command Architecture**.

## Architectural Principles

- **Feature-driven design**: Commands and logic are grouped by feature domains (e.g., music, moderation, utilities).
- **Provider isolation**: Each feature may define multiple providers (e.g., YouTube, Spotify) with their own command sets and shared logic.
- **Modular command organization**: Commands are scoped per provider and follow a consistent naming and layout strategy.
- **Separation of concerns**: Reusable logic, cross-cutting concerns (e.g., logging), and shared utilities are centralized under `shared/`.

## Directory Layout

```
src/
├── features/
│   ├── music/
│   │   ├── youtube/
│   │   │   └── commands/
│   │   │       ├── shared/
│   │   │       │   └── etc.  
│   │   │       ├── play.rs
│   │   │       ├── skip.rs
│   │   │       ├── stop.rs
│   │   │       └── etc.
│   │   ├── spotify/
│   │   │   └── commands/
│   │   │       ├── shared/
│   │   │       │   └── etc.  
│   │   │       ├── play.rs
│   │   │       ├── skip.rs
│   │   │       ├── stop.rs
│   │   │       └── etc.
│   │   └── etc./
│   ├── moderation/
│   │   └── commands/
│   │       ├── ban.rs
│   │       ├── kick.rs
│   │       └── etc.
│   ├── utils/
│   │   └── commands/
│   │       ├── ping.rs
│   │       ├── uptime.rs
│   │       └── etc.
│   └── etc./
├── shared/
│   ├── logger.rs
│   └── etc.
└── main.rs
```