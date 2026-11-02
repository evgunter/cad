---
id: a-pattern-count-has-no-upper-bound-at-the-loop-that-uses-it
kind: issue
title: A member family's count is admitted up to u32::MAX per index and evaluated inside one node, with the cancel token checked only between nodes
status: closed
opened: 2026-09-21
priority: P1
cost: M
closed: 2026-11-01
refs: [need-count-spells-every-failure-as-a-pattern-count]
---


Filed by the VGEOM orchestrator while re-deriving the fork on
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`
against today's tree. Its sibling is
`need-count-spells-every-failure-as-a-pattern-count`.

## Finding

`crates/editor-core/src/eval/wire.rs`, the pattern node:

```
let n = slots::count(vals, SlotId::Count).ok_or(NodeErrorKind::MissingSlot { ... })?;
if n < 1 {
    return Err(NodeErrorKind::NonPositiveCount { count: n });
}
...
for j in 1..n {
    let map = stepped_map(kind, j, results, vals, tol)?;
    let placement = names::output_body(usize::try_from(j).unwrap_or(usize::MAX)).map_err(naming)?;
    for (i, body) in master.iter().enumerate() { ... instances.push(Arc::new(place(...)?)); }
}
```

**The only guard on `n` is at the bottom.** There is no upper bound
before the loop, so the count's own magnitude is never a refusal; it
is a number of iterations. The same shape is at the stepped node,
which raises the same `n < 1` and no ceiling.

Each iteration allocates and places `master.len()` bodies into
`instances`, so the loop is not merely slow — it grows a `Vec<Arc<Body>>`
without bound.

## The bound exists in this crate, on a different path

`crates/editor-core/src/mate/member.rs` refuses a count that will not
fit the name table's `u32` row width, through the naming layer:

```
let n = usize::try_from(n).ok().and_then(|n| crate::names::output_body(n).ok())
    .ok_or_else(|| ... NamingError::Emission {
        what: "a pattern's count exceeds the table's u32 row width",
    })?;
```

So the tree already knows what a too-large pattern count is and what to
call it. The evaluator's own loop asks the same question **inside** the
loop instead — `names::output_body(usize::try_from(j)...)` is per
iteration — so the refusal arrives after roughly `u32::MAX` placements
rather than before the first. That is the guard sited downstream of the
work it is for, which is the shape
`work/vgeom/`'s `vgeom/refusal-floor` unit named on its own ground
(`work/vgeom/log.md`, 2026-09-21): a guard correct about what it looks
at, sited one step past the thing that hurts.

## And it is not cancellable

`crates/editor-core/src/eval/mod.rs`: *"The cooperative cancel token
(spec D5): checked **BETWEEN nodes**"*. An unbounded loop inside one
node therefore observes no cancel, so the recourse a long evaluation
otherwise has does not apply to this one.

## Reachability, and what was not checked

The producer is established and is not this crate's:
`crates/viewer/src/props.rs`'s `SlotValue::of` casts a typed `f64` to
`i64` with `value as i64`, which **saturates** — `1e30` commits
`i64::MAX` — and `crates/viewer/src/pane/properties.rs`'s `slot_field`
sets no `egui::DragValue` range, so `1e30` is a value the chrome
accepts for a `Count` slot. That chain is
`work/vgeom/a-count-slots-cast-still-saturates-for-a-finite-value-too-large`,
open, and it is why this row was found.

**Stated as a negative result about a search, not as a proof:** no
outer budget, node-size limit or iteration cap was found in
`crates/editor-core/src/eval/`, and the search was a grep of that
module for `budget|deadline|abort|cancel` plus a read of the pattern
and stepped arms. It did not cover the eval SERVICE above the crate
(`crates/viewer/src/session/`, VSEAM's) or any caller-side timeout, so
a reader should not conclude from this row alone that a person's
session hangs — only that nothing inside the evaluator stops it.
Nothing here was driven end to end.

## Fence

`crates/editor-core/src/eval/wire.rs` and `eval/mod.rs` — WIRE's, by
`scripts/work.py territory --files -`. Filed, not fixed: a numeric door
the viewer consumes is a hand-off and never a diff from VGEOM.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. `wire_pattern` still loops `1..n` with no ceiling and no cancel check. `Node::Pattern` and `PlacedUnion` retire into `index(N)`; the bound and cancel question carry over to `N`, so re-read it there rather than closing it. Parked on `patterns-are-index-variables`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## D10 exit walk (2026-10-29)

The scene still stands on main, re-sited. `wire_pattern` and `stepped_map` are gone (stage 3 G, #5954), and an index's count is now range-checked before its loop (`IndexCountOutOfRange`, in `crates/editor-core/src/eval/members.rs`). But the range it admits is `0..=u32::MAX`, and `members` (`eval/members.rs`) yields that many members inside one node. The cancel token is still checked only between nodes. So the bound sits ahead of the loop now but is still the name table's width rather than a size an evaluation can afford, and the loop is still not cancellable. Open.

## Re-checked after D10 (2026-11-01, WIRE orchestrator)

From the read-only `wirehold-triage` against origin/main 8509df2510. INTENT closed on 2026-10-30, and every trigger this row waited on has fired.

**Changed; the defect stands.**
- `wire_pattern` and its `1..n` loop are gone (G, #5954). The count is now range-checked before the loop: `members.rs:120` against `MemberCountFault::ADMITS = 0..=u32::MAX` (`repetition.rs:88`).
- But `evaluate_members` still pushes every member into a `Vec` inside one node (`members.rs:240-290`), and the cancel token is still checked only between nodes (`eval/mod.rs:2634`; DESIGN.md:2163, "yielding between nodes/levels").
- Nested indices multiply, so a family can reach u32::MAX^depth members.
- In the viewer, a count of 1e30 saturates to i64::MAX (`viewer/src/props.rs:192`) and is refused by ADMITS; about 4e9 is still admitted.

What it needs:
- (a) The cancel token checked per member.
- (b) Whether a family has a size budget. That is a policy question.

DESIGN's cancelation sentence says between nodes. Check its provenance (`git log -S`) before treating per-member yielding as a clause change.
## Family members yield (2026-11-01, `wire-members`)

**Built.** `eval::members::eval_family` reads the cancel token before each member, and a canceled family returns `Canceled` instead of a step. `eval_any` passes it up, and both walks in `evaluate_at_descent` treat it as the between-node check does: the outcome is `EvalOutcome::Canceled`, the completed prefix is kept, and the canceled node holds no result. On the level schedule, the level's other nodes that finished are kept. DESIGN's Cancelation bullet now says family members yield.

Witnesses:
- `g_index_families::a_ring_canceled_mid_family_returns_the_completed_prefix`: a ring of `u32::MAX` copies, canceled 150 ms in, serial and parallel. On main it ran 214 s until it was killed.
- `eval::members::tests::a_family_canceled_by_a_member_stops_before_the_next`: the token is set inside member 2's evaluation, and exactly three members evaluate.

**Overflow.** No product of counts is computed anywhere. Swept `members.rs`, `repetition.rs`, `mate/member.rs` and `viewer/src/props.rs` for `product`, `checked_mul`, `saturating_mul` and `as u32`. `members()` lists members lazily from a stack of `(env, next, count)`, one level per index, so the only size is the member `Vec`'s `len()`. That is a `usize`, and memory runs out long before it can wrap. A body family's flat row is bounded by `names::output_body` inside `name_family`, a typed `NamingError` beyond `u32`. `wire_combine`'s `whole` closure saturates `u32::try_from(i)`, but its `Instances` input is only ever built by `name_family`, so `i` fits. No guard was needed, so there is no overflow test.

**What a family costs.** Measured on a debug build on this box, with a ring of `N` copies of one block (`g_index_families::ring`):

| N | time | RSS after (process start ≈ 81 MB, N = 1 → 108 MB) |
|---|---|---|
| 1 | 11 ms | 108 MB |
| 1,000 | 1.09 s | 126 MB |
| 10,000 | 10.5 s | 284 MB |
| 100,000 | 113 s | 1.85 GB |

- That is about 1.1 ms and **about 17.5 KB per member**, linear, with every member sharing the one block body. Most of the 17.5 KB is probably each member's name table and value. A release build would be faster, but not much smaller.
- Two consequences:
  - On this 16 GB box, a family exhausts memory at roughly 0.9 M members, and the allocation failure aborts the process. That is not a typed refusal.
  - `ADMITS` lets one index reach 4.3e9 members, about 75 TB. Nested indices multiply that.
- `overlay` clones the whole results map once per member, so a family also costs O(N × nodes in the document) transiently.

## Size-ceiling recommendation (for a designer pair; not built)

**Recommendation: cancelation is not enough.** A ceiling on members per family is needed.

Cancelation needs someone watching to press it. A headless run has nobody to cancel it: `pncad`, a script, CI, or a save replayed on load. For those, a count `ADMITS` accepts ends in an allocator abort, not a refusal. That breaks D9's "nothing panics on a legal input" in spirit, because the input is legal by `ADMITS`.

- **What the bound is: a total-member count per family**, not a tighter per-index range. A per-index range does not bound nesting, and an inner count can depend on the outer index's value, so the total is not a product known up front. The constant would be `MEMBERS_PER_FAMILY`. At 17.5 KB per member, 2^16 (65,536) members holds about 1.1 GB, and 2^20 holds about 18 GB, which is already past this box. The number should be picked against the target machine. 2^16 is my suggestion.
- **Where it refuses: in `eval_family`**, from a running count as members are listed, beside the cancel check. The refusal arrives before the member past the bound evaluates, not after the `Vec` holds it. `Doc::member_envs` takes the same bound so the edit-side readers agree.
  - Optionally, the edit door could pre-check the plain case where every count is a literal. There the product is known, and it must be taken with `checked_mul` in `u64`, since that is the one place a product would first appear.
- **What recourse it offers:** a typed `NodeErrorKind`, for example `FamilyTooLarge { at_least, bound }`, naming the node and the indices it reaches. The fix it says is to lower a count, or to split the family across operations. That fits the existing `IndexCountOutOfRange` refusal and recourse rather than a new channel.

Out of this unit: the union fold over a family is superlinear and does not yield (filed `wirehold/a-unions-fold-of-a-family-grows-faster-than-its-members`). The viewer's per-member slot reading is eager (filed `chrome/a-per-member-slot-reading-lists-every-member-eagerly`).
