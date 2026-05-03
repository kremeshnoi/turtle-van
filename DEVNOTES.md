# Notes

## 2026-05-03: YouTube bot-detection — daily redeploy workaround

### Problem
`/play` started failing with yt-dlp errors like:
```
ERROR: [youtube] <id>: Sign in to confirm you're not a bot. Use --cookies-from-browser
or --cookies for the authentication.
```
A manual `railway redeploy` consistently clears the error and playback resumes immediately.

### Working theory (not verified)
We believe YouTube's bot detection is flagging the Railway deployment's egress IP, and
spinning up a new container assigns a fresh IP that hasn't been flagged yet. This is an
inference from the observed behavior — what we've actually confirmed is only that
`railway redeploy` makes the error go away. Other plausible explanations exist
(stale yt-dlp/extractor cache, process-level state, Songbird/HTTP client state) and have
not been ruled out. Treat the IP-rotation explanation as a working hypothesis, not a
root cause.

### Workaround
Scheduled GitHub Actions workflow at `.github/workflows/redeploy.yml` runs
`railway redeploy` every day at 21:00 UTC (00:00 Riga in summer, 23:00 Riga in winter).
The workflow also exposes `workflow_dispatch` so it can be triggered on demand from the
Actions tab if a block hits mid-day.

### Important
- GitHub Actions cron is UTC-only, so the local fire time drifts 1h across DST. This is
  acceptable for a low-traffic music bot; don't try to "fix" it with two cron entries.
- Redeploy disconnects the bot from any active voice channels and drops queues — the
  00:00 Riga schedule was chosen specifically to minimize that impact.
- If redeploys stop clearing the error, escalate to cookie-based auth (`--cookies` via
  a Netscape cookies file mounted as a volume) before investing in heavier fixes like
  a PO Token provider sidecar.

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
