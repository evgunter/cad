---
id: wire-profile-spells-a-missing-precompute-as-a-missing-slot
kind: issue
title: wire_profile answers a missing profile precompute with MissingSlot naming a slot that is not missing
status: open
opened: 2026-09-28
priority: P4
cost: E
---

Found by the GATHER editorial pass over `eval/wire.rs`
(`wire-rs-accumulation-residue-comment-ratio-and-wire-sweep`).

## Finding

`crates/editor-core/src/eval/wire.rs`, `wire_profile`:

```
let Some(pre) = pre else {
    // Unreachable by eval_node's stage order; typed, never a panic.
    return Err(NodeErrorKind::MissingSlot {
        slot: SlotId::Profile { loop_: 0, step: 0, arg: crate::node::StepArg::PointX },
    });
};
```

`run_op` takes `profile_pre: Option<&ProfilePre>`, documented as
"present exactly for `Node::Profile`". That pairing is held by
`eval_node`'s stage order and a comment. When it breaks, the refusal
names loop 0, step 0, argument `PointX` as a missing slot — a specific,
false statement about the document, pointing a caller at a slot that is
there.

## Direction

Either carry the precompute in the type that selects the arm (so a
`Node::Profile` cannot reach `run_op` without one), or refuse the miss
as the internal break it is rather than as a document fault.
