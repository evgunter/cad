---
name: Claude Code on the web container provisioning
description: The hosted container's egress policy denies get.nexte.st, so the SessionStart hook takes cargo-nextest from nextest's own GitHub release asset; and no tool install in that hook may be fatal, because `set -e` there costs the session its warm build. Plus what a hosted container does and does not have (FreeCAD absent → matplotlib preview renders only).
type: operational
---

# Claude Code on the web container provisioning

`.claude/hooks/session-start.sh` provisions the hosted container. It is
remote-only (`CLAUDE_CODE_REMOTE`), synchronous, and its output is
snapshotted, so the cold cost is paid once per cache.

## Egress is policy-filtered, per environment

Outbound HTTPS goes through an agent proxy that enforces an allow-list.
On `cloud_default` as of 2026-08-14:

- **DENIED**: `get.nexte.st` — the gateway answers 403 to CONNECT.
- **allowed**: github.com (release assets), crates.io + static.crates.io,
  pypi.org, the Ubuntu apt mirrors.

So `cargo-nextest` is installed from **nextest's own GitHub release
asset**, which is the artifact `get.nexte.st` redirects to — same
publisher, different hostname, not a third-party mirror. The hook tries
the redirector first (hosted-CI parity, ~1s to fail: curl does not retry
a 403) and falls back.

Diagnose a blocked host with `curl -sS "$HTTPS_PROXY/__agentproxy/status"`
— its `recentRelayFailures` names the host and the reason, which curl
itself hides on a failed CONNECT. **Report a denial; do not route around
it** (a different *official source for the same artifact* is not routing
around it).

## Fail-loud, never fail-early

The hook died on that 403 for its first day: `set -euo pipefail` plus a
`curl | tar` pipeline aborted it 14 seconds in, so `cargo fetch` and the
`cargo build --workspace --all-targets` that is the *entire point of the
snapshot* never ran. Every session opened onto an empty `target/`.

The rule that came out of it: the **compiler and the warm build** are the
session and may fail the hook; a **test runner or checker** (nextest,
maturin, ty, admesh, the demo render venv) costs its own gate row and
nothing else, so it warns, records itself in `DEGRADED`, and the hook
replays that list at the end where an agent will actually read it.

## What a hosted container has

Green as of 2026-08-14: `cargo build/fmt/clippy`, `cargo nextest`,
`cargo test --doc`, `crates/pncad-py/run-python-tests.sh` (132 tests, with
`ty` present so the stub lattice is measured, not skipped),
`scripts/check_admesh.sh`.

**FreeCAD is absent on purpose** (~1 GB AppImage whose only job here is
the hosted OCC reference lane). So `demos/render.sh` takes its matplotlib
fallback: preview frames into the gitignored `demos/renders-preview/`,
committed lanes untouched. Renders still need the drift sentence
(`CAD_RENDER_LOCAL_OVERRIDE=i-accept-local-render-drift`), and the tour
must be generated first — `cd demos/tour && cargo run --release -- ../out`
(note `../out`; `out` writes to `demos/tour/out`, where `render.sh` will
not find `scenes.json`). Committed sheets still come from hosted CI.
