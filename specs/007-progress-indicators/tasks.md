# Epic 3.1: Progress Indicators - Tasks

## Phase 1: YouTube Backend Support
- [ ] Research/Update `yt-dlp` command to use subprocess piping for real-time progress.
- [ ] Implement progress parsing logic in `YouTubeService`.
- [ ] Add SSE (Server-Sent Events) endpoint in `youtube.py` router.

## Phase 2: Frontend Infrastructure
- [ ] Create `ProgressBar` Dioxus component.
- [ ] Implement SSE client logic in Rust to listen for progress updates.
- [ ] Add progress state to `ScriptEditor` for synthesis loop.

## Phase 3: Integration
- [ ] Wire up `YoutubeExtractor` UI to show progress bar during download.
- [ ] Wire up `ScriptEditor` UI to show progress bar during synthesis.

## Phase 4: Verification
- [ ] Verify progress bar accuracy (0% to 100%).
- [ ] Test error handling during progress (connection drop, etc.).
