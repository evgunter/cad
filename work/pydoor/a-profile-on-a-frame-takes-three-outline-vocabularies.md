---
id: a-profile-on-a-frame-takes-three-outline-vocabularies
kind: issue
title: Doc.draw_on_face, Node.drawn_profile and Node.profile each accept a different outline vocabulary, and the mixed-list convention lives only in Python
status: open
opened: 2026-11-22
priority: P3
cost: M
---

Found by VSEAM's review of PR 7465 (`vseam/py-sketch-on-face-retires`, 2026-11-22). Filed by the VSEAM orchestrator.

`crates/pncad-py/src/py/doc.rs` has three outline parsers, and each takes a different vocabulary:

| Parser | Accepts | Used by |
|---|---|---|
| `loops_from_outline` | `ClosedLoop` only | `Node.profile` |
| `pncad::trace::Outline` | `Chain` only | `Node.drawn_profile` |
| `loops_on_face` | a list mixing both, outer first | `Doc.draw_on_face` |

So two doors that both mean "a profile drawn on a frame", `Node.drawn_profile` and `Doc.draw_on_face`, accept different outlines.

`loops_on_face` also rewrites a façade error's `SlotId::Profile.loop_` to the chain's place in the list. That gives chain addressing a second home in the binding, beside the one `pncad::trace::drawn_profile` owns. The mixed-outline convention therefore exists only in Python: no Rust or viewer caller gets it. That sits uneasily with `work/bind/the-python-facade-holds-code-that-is-not-a-call-to-the-rust-facade`.

Decide:
- whether the Rust façade should own one outline type (closed loops and traced chains, in order) that all three Python doors pass through;
- or whether the vocabularies are distinct on purpose.
