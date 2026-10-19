---
id: the-viewer-cannot-mate-two-freshly-inserted-parts
kind: issue
title: The viewer cannot mate two freshly inserted parts: every insert is world-pinned, so the mate tool's AddConstraint lands on a pinned bundle and refuses Overconstrained
status: open
opened: 2026-10-17
priority: P1
cost: M
---


Found by INTENT stage 3 B's review (PR 5232).

**What happens.**
- The viewer inserts every instance pinned to the world at the identity
  (`crates/viewer/src/session.rs`'s part insert,
  `Node::instantiate_part`; `crates/viewer/src/pane/create.rs`).
- The mate tool commits `DocEdit::AddConstraint` on the placement it
  places (`session.rs`'s `mate_placement`). For two parts just
  inserted, that placement's bundle already pins its copy, so the door
  refuses `Overconstrained`.
- A user must remove the world pin by hand first (the tree's remove
  door), which is what `crates/viewer/tests/mate_tool_flow.rs` does.

**Why it is filed, not fixed in B.** Spec §3 (Viewer) says the mate tool
commits into a `Place`'s bundle, and "across two unrelated spaces it
first places one copy relative to the other". The open question is how
the tool should treat a placed copy's lone world pin. It could:
- replace the pin in the one action;
- offer to;
- or insert parts bare and pin only on request.

That is a design choice for the viewer's assembly workflow, not a
mechanical follow-on of B.
