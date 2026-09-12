---
name: demo-purpose
description: The demos' binding purpose (Ev, 2026-08-09) — demonstrate REAL natural library usage; awkwardness is a library finding, never hidden
metadata:
  type: feedback
---

**The demos exist to demonstrate real, natural library usage** — the
way a user would actually write the model (Ev, 2026-08-09). They are
the library's usage oracle: a demo contorted to preserve bytes or to
dodge a gap stops measuring what using the library is like, and the gap
it dodges goes unrecorded. Source comments across the tree cite this
file; the demo-side elaboration is the crate doc of
`demos/tour/src/main.rs`.

- Awkwardness met while writing a demo is a LIBRARY FINDING: gap-comment
  it at the site and record it in the orchestrator log — never quietly
  work around it.
- Demo edits that improve authoring naturalness may break byte-identity
  freely; mechanical migrations (imports, plumbing) still prove it,
  because there the diff proves nothing changed. Specs say which
  contract applies per unit.
- Standing goal: every demo authorable through the Python bindings; the
  tour corpus doubles as the bindings' coverage oracle.
