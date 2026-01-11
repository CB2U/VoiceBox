# Tasks - Epic 2.0: YouTube Integration

## Backend
- [ ] Add `yt-dlp` to `backend/requirements.txt` and install. <!-- id: 0 -->
- [ ] Create `backend/api/routers/youtube.py` (or similar). <!-- id: 1 -->
- [ ] Implement `extract_audio` service function wrapping `subprocess.run` for `yt-dlp` and `ffmpeg`. <!-- id: 2 -->
- [ ] Implement `POST /extract-from-youtube` endpoint. <!-- id: 3 -->
- [ ] Test backend with `curl` or manual script. <!-- id: 4 -->

## Frontend
- [ ] Create `YouTubeImport` component in Dioxus. <!-- id: 5 -->
- [ ] Add input fields for URL, Start Time, End Time. <!-- id: 6 -->
- [ ] Add generic `extract_youtube_audio` function to HTTP client. <!-- id: 7 -->
- [ ] Connect internal state (loading, error) to UI. <!-- id: 8 -->
- [ ] Refactor File Import UI to sit alongside YouTube Import (Tabs or Accordion). <!-- id: 9 -->

## Verification
- [ ] Manual Test: Extract a 5s clip from a video. <!-- id: 10 -->
- [ ] Manual Test: Ensure UI handles errors (invalid URL). <!-- id: 11 -->
- [ ] Start/Stop backend and ensure clean integration. <!-- id: 12 -->
