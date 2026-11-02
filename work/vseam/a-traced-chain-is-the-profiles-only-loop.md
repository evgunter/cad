---
id: a-traced-chain-is-the-profiles-only-loop
kind: issue
title: The traced-path door authors one loop, so a profile traced along a face cannot carry a hole
status: open
opened: 2026-11-02
priority: P2
cost: M
refs: [the-gui-cannot-author-a-profile-of-several-loops]
---

Found by the lane that closed
`the-gui-cannot-author-a-profile-of-several-loops`.

`SessionOp::AddDrawnProfile` (`crates/viewer/src/session/op.rs`)
takes `chains: Vec<pncad::trace::Chain>`, "the loops, outer first",
but the add-profile form's traced door builds exactly one:
`Drafts::profile_op` (`crates/viewer/src/drafts.rs`) lowers
`profile_path` through `sketch::traced_loop(.., 0, ..)` and sends
`chains: vec![chain]`. Loops added to the form
(`Drafts::profile_more`) have no lowering into a `Chain`, so
`ViewerBehavior::added_loops_ui` (`crates/viewer/src/pane/create.rs`)
withholds the commit while the chain is traced and loops are added,
and offers no add-loop row while it traces.

A profile drawn tangent to a face's trace therefore cannot carry a
hole from the GUI. A fix lowers each added loop as an untraced chain
of the drawn profile (or says why the trace door cannot take one).
The edit door does not hold a traced program at all
(`sketch::held_loops` refuses it), which is its own limit.
