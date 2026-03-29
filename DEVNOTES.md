# Notes

## 2026-03-29: Discord DAVE (E2EE) protocol required — voice connection refused

### Problem
Bot could not join voice channels. Songbird failed with `establishing connection failed`. The Discord voice WebSocket closed with code 4017: `E2EE/DAVE protocol required`.

### Cause
Discord began enforcing the DAVE (Discord Audio/Video Encryption) end-to-end encryption protocol for all non-Stage voice channels in March 2026. Songbird 0.4.x does not implement DAVE, so Discord's voice server rejects the connection.

### Solution
1. Updated songbird from `0.4.x` (crates.io) to `0.5.0` from the `next` git branch, which includes DAVE support (PR [serenity-rs/songbird#291](https://github.com/serenity-rs/songbird/pull/291), merged 2026-03-28)
2. Updated reqwest from `0.11` to `0.12` (required by songbird 0.5)
3. Migrated from songbird 0.4 `TrackHandle::typemap()` API to songbird 0.5 `Track::new_with_data()` / `TrackHandle::data::<T>()` API
4. Added `ctx.defer()` to `/play` and `/join` commands to prevent Discord interaction timeouts during voice connection

### Important
- Songbird 0.5.0 with DAVE is not yet published to crates.io — using git dependency until a release is made
- Once songbird publishes a crates.io release with DAVE, switch back to a versioned dependency

## 2026-01-29: YouTube 403 Forbidden — yt-dlp requires JS runtime

### Problem
Bot stopped playing audio. Tracks were being added to the queue, Songbird reported `Track added to queue`, but when attempting to download the audio stream YouTube returned HTTP 403 Forbidden. Commands `/pause` and `/now` reported nothing was playing.

### Cause
YouTube introduced mandatory JavaScript challenges for obtaining audio streams. The old version of yt-dlp (installed via `apt`) could not solve them because:
1. It had no JS runtime (deno/node)
2. It lacked the `yt-dlp-ejs` dependency for built-in JS challenge solving
3. `apt` ships an outdated version that does not support YouTube's new protection

Songbird error in logs:
```
TrackState { playing: Errored(Create(Fail("failed with http status code: 403 Forbidden"))) }
```

### Solution
1. Removed yt-dlp installed via `apt` (`sudo apt remove yt-dlp`)
2. Installed yt-dlp via `pip` from GitHub master + `yt-dlp-ejs` dependency:
   ```bash
   pip3 install "yt-dlp[default] @ https://github.com/yt-dlp/yt-dlp/archive/master.tar.gz"
   ```
3. Installed deno as a JS runtime for yt-dlp:
   ```bash
   curl -fsSL https://deno.land/install.sh | sh
   ```
4. Updated `Dockerfile` — yt-dlp is now installed via pip instead of downloading the binary directly

### Important
- For local development, deno must be in PATH (`~/.deno/bin`)
- yt-dlp must be installed via pip, not apt
- Related issue: https://github.com/yt-dlp/yt-dlp/issues/12482
