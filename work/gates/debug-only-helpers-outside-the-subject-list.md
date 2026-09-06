---
id: debug-only-helpers-outside-the-subject-list
kind: issue
title: Five debug-only helpers under cfg(debug_assertions) in mesh and topo are on no gate's subject list
status: open
opened: 2026-09-06
---


## Finding

Filed by the GATES orchestrator from the `gates/debug-only-subjects`
lane's sweep (PR 2030), which generalised
`scripts/gates/bit-identity-debug-only.sh` to a (subject, symbol)
list and then swept `crates/*/src` for other `#[cfg(debug_assertions)]`
item heads. Five are gated by nothing:

- `crates/mesh/src/curved.rs:868` and `:889`
- `crates/mesh/src/tessellate.rs:298`
- `crates/mesh/src/walk.rs:310`
- `crates/topo/src/euler.rs:981`

Each is a debug-only mechanism whose source shape (every site of the
symbol under the attribute, or inside a `debug_assert!`) nothing in CI
holds, the state `debug-only-counters-have-no-gate` found for
`product.rs`'s gather counter. The candidate fix is a row per helper in
the gate's `SUBJECTS` list with a symbol pattern unique to its subject
(the gate's KNOWN GAP 3: a symbol that is also an ordinary word
over-counts), which is this program's edit; the helpers themselves are
S-MESH's and TOPO's files and are read only. Whether every one WANTS
gating — a `debug_assert!`-shaped helper with one caller is a different
case from a counter read by tests — is decided per row when it is
taken.

**What the sweep could not match** (the lane's own statement):
`#[cfg(all(debug_assertions, …))]` heads, feature-gated mechanisms, an
item head more than three lines below its attribute, and anything under
`tools/`, `demos/` or `crates/*/tests`.
