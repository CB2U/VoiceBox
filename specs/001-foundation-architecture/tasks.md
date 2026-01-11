# Epic 1.0: Foundation & Architecture - Tasks

## Setup
- [ ] **T1: Python Environment Setup** <!-- id: 0 -->
  - **Goal:** Create reproducible python env.
  - **Steps:** Create `backend/` dir, init `venv`, create `requirements.txt` (fastapi, uvicorn), install.
  - **Done when:** `source venv/bin/activate` works and packages are installed.
  - **Verify:** `pip list` shows fastapi.
  - **Files:** `backend/requirements.txt`, `backend/.gitignore`.

- [ ] **T2: Rust Environment Setup** <!-- id: 1 -->
  - **Goal:** Create Rust crate.
  - **Steps:** Init `frontend/` (or root) with `cargo init`, add dependencies (`dioxus`, `dioxus-desktop`, `reqwest`, `serde`, `tokio`).
  - **Done when:** `cargo check` passes.
  - **Verify:** `Content of Cargo.toml` includes deps.
  - **Files:** `Cargo.toml`.

## Core Implementation
- [ ] **T3: Backend Health Endpoint** <!-- id: 2 -->
  - **Goal:** Serve a JSON response.
  - **Steps:** Create `backend/src/main.py`. Define `app = FastAPI()`. Add `@app.get("/health")`.
  - **Done when:** `uvicorn backend.src.main:app` starts.
  - **Verify:** `curl localhost:8000/health` returns `{"status": "ok"}`.
  - **Files:** `backend/src/main.py`.

- [ ] **T4: Frontend Basic UI** <!-- id: 3 -->
  - **Goal:** Render a window.
  - **Steps:** `src/main.rs` with `fn app(cx: Scope)`. Use `dioxus_desktop::launch(app)`. Render simple "Voice Box" header.
  - **Done when:** Window opens smoothly.
  - **Verify:** Visual inspect.
  - **Files:** `src/main.rs`.

- [ ] **T5: Frontend Polling Logic** <!-- id: 4 -->
  - **Goal:** Periodically check backend.
  - **Steps:** Add `use_future` or `use_coroutine` in Dioxus. Loop every 2s. Call `reqwest::get`. Update state variable `is_online`.
  - **Done when:** State toggles based on backend availability.
  - **Verify:** Add `println!` debugging to see poll events.
  - **Files:** `src/main.rs` (or `src/api.rs`).

## Verification
- [ ] **T6: Manual Verification (AC-1, AC-6)** <!-- id: 5 -->
  - **Goal:** Verify full loop.
  - **Steps:**
    1. Start python backend.
    2. Start rust frontend.
    3. Check Green status.
    4. Stop backend.
    5. Check Red status.
  - **Done when:** ACs match reality.
  - **Evidence:** Screenshot or log capture describing the test.

## Tracking
- [ ] **T7: Update SPECS Index** <!-- id: 6 -->
  - **Goal:** Keep index current.
  - **Steps:** Update `SPECS.md` status if needed.
  - **Done when:** SPECS.md is accurate.

- [ ] **T8: Final Spec Update** <!-- id: 7 -->
  - **Goal:** Document evidence.
  - **Steps:** Update `spec.md` with summary of verification from T6.
  - **Done when:** `spec.md` has filled `## EVIDENCE` section.
