---
name: demo-purpose
description: The demos' binding purpose (Evan, 2026-08-09) — demonstrate REAL natural library usage; awkwardness is a library finding to record, never to hide; byte-identity soft for demo improvements, kept for mechanical migrations
metadata:
  type: feedback
---

**The demos exist to demonstrate real, natural library usage** —
the way a user would actually write the model (Evan, 2026-08-09,
PR #289/#290 threads + in-chat). The always-seen copy of this rule
lives in `demos/tour/src/main.rs`'s crate-root doc ("The demos'
purpose") — keep that block and this memory in sync.

**Why:** demos are the library's usage oracle. If they're written
in a contorted way (to preserve bytes, to dodge a gap), they stop
measuring what using the library is actually like — and the gap
they dodge goes unrecorded.

**How to apply:**
- Demo edits that improve authoring naturalness may break
  byte-identity freely; mechanical migrations (imports, plumbing)
  still prove byte-identity, because there the diff proves
  nothing changed. Specs say which contract applies per unit.
- Awkwardness met while writing a demo is a LIBRARY FINDING:
  gap-comment it at the site and record it in the orchestrator
  log — never quietly work around it.
- Standing goal: every demo authorable
  through the Python bindings; the tour corpus doubles as the
  bindings' coverage oracle.
