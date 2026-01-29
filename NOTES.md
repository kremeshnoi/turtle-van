# Notes

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
