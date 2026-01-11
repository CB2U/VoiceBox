# Epic 1.0: Foundation & Architecture - Implementation Plan

## 1. Architecture Overview
The system follows a "Backend for Frontend" pattern, but locally hosted.
- **Frontend (Rust):** Dioxus app running in a native WebView. It manages the UI state and orchestrates requests.
- **Backend (Python):** FastAPI app running as a subprocess (manually managed for now). It handles heavy lifting (future: AI, Audio).
- **Communication:** HTTP REST over `http://127.0.0.1:8000`.

**Module Boundaries:**
- `frontend/` (Rust crate)
- `backend/` (Python package)

## 2. Data Contracts
### Health Check
**Request:**
`GET /health`

**Response:**
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

## 3. Storage and Persistence
- None for this epic.

## 4. External Integrations
- None.

## 5. UX and Operational States
- **State: Initializing**: App just opened, checking backend.
- **State: Online**: Backend responded. Green indicator.
- **State: Offline**: Backend timed out/refused connection. Red indicator.

## 6. Testing Plan
- **Unit (Python):** `pytest` for `test_health.py` to ensure JSON response format.
- **Unit (Rust):** Basic tests for the API client struct (mocking the HTTP request).
- **Manual:** Full end-to-end launch test.

## 7. AC Verification Mapping
| AC ID | Requirement | Verification Method |
|:----- |:----------- |:------------------- |
| AC-1 | Basic UI Scaffold | Launch `cargo run`, check window visibility. |
| AC-6 | Backend Status Monitor | Kill/Start backend script, watch UI update. |

## 8. Risks and Mitigations
- **Risk:** `reqwest` blocks the UI thread.
  - **Mitigation:** Use `reqwest` async client and spawn Tokio tasks for polling to ensure non-blocking UI.

## 9. Rollout and Migration Notes
- Initial setup. No migration needed.

## 10. Observability and Debugging
- **Backend Logs:** Uvicorn access logs (stdout).
- **Frontend Logs:** `println!` or `log` crate output (stdout).
