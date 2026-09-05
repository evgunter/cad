---
id: prune-discards-the-fault-that-explains-the-supersession
kind: issue
title: prune tests the free-move fault with is_ok and drops it, so the supersession notice names the instance but not the cause
status: closed
opened: 2026-09-04
branch: view/prune-report
pr: 1886
closed: 2026-09-05
refs: [opoutcome-superseded-has-no-production-reader, prune-drops-a-hidden-instance-silently]
---

Found by the VIEW-6 review (2026-09-04), which is the reason the
sentence `frame::supersession_notice` produces is thinner than it
should be.

## What happens

`DisplayState::prune` (`crates/viewer/src/display.rs`, `prune`) decides
what to supersede with

```rust
let keep = free_move_check(doc, instance).is_ok();
```

`free_move_check` does not answer a bool. It answers
`Result<(), DisplayFault>`, and the `Err` it builds for the commonest
arm is `DisplayFault::MateConstrained { instance, mates }`, whose
`Display` reads:

> instance 3 is mate-constrained (mate node(s) 5): its pose is
> mate-derived, so the free-move probe refuses — delete the mate(s) if
> free relative motion is intended

`is_ok()` throws that away at the instant it is computed. What survives
into `OpOutcome::superseded` is a bare `Vec<RecipeNodeId>`, so the
notice the user reads is

> free move: the placement on instance 3 was discarded — the document
> no longer admits one there

— the fact, without the cause and without the thing to do about it.
The typed value with the better sentence existed one call frame away.

**This is the same defect `opoutcome-superseded-has-no-production-reader`
closed, one layer down**: a value the code computes correctly and drops
before the user sees it. That item's fix carried the report from the
session to the chrome; this one is about what the report is allowed to
say when it gets there.

## Why VIEW-6 did not fix it

The change is `prune` returning its faults rather than its ids, which
moves:

- `crates/viewer/src/display.rs` — `prune`'s body and signature,
- `crates/viewer/src/session.rs` — both call sites (`step`,
  `commit_action`),
- `crates/viewer/src/session/op.rs` — `OpOutcome::superseded`'s type,
- nine assertion sites across seven test files, all spelled
  `vec![bench.post_b]`.

`display.rs` and `session.rs` were both under an open sibling lane
(`view/two-gestures`, PR #1873, which renames `DisplayState::gesture`
to `free_move` inside `prune` itself), so VIEW-6 recorded the honest
version at the type instead: `frame::supersession_notice`'s docs now
say the payload is bare **because of this discard**, and name this
file.

## What a fix looks like

`prune` returns `Vec<(RecipeNodeId, DisplayFault)>`, or a small named
struct; `supersession_notice` renders each fault through its own
`Display` — which is the rule the rest of the crate follows and the one
the current wording has to state an exception to. That also removes
`supersession_notice`'s composed prose, so `four-badges-five-spellings`
gains a notice producer that looks like the other three.

**It also fixes the delete arm**, which is the notice's other weak
spot: `free_move_check` fails through `display_check` when the instance
is GONE, and today the sentence names an id the tree no longer draws
without saying that is why.

## Closed by PR #1886

`prune` returns a `PruneReport` whose entries are
`display::Withdrawn { instance, cause }`; `OpOutcome::superseded`
carries them; `frame::supersession_notice` counts and lets each fault
render itself, so the mates and the remedy reach the line. The delete
arm is fixed by splitting `DisplayFault::NoSuchNode` off
`NotAnInstance` in `drawn_targets` — an id the tree no longer draws
says *node 4 is not in the document* rather than being named as if it
were still there.

**Corrected in place**: the blast radius above names the right files,
but "nine assertion sites across seven test files, all spelled
`vec![bench.post_b]`" was inherited from
`opoutcome-superseded-has-no-production-reader`'s own correction
against a pre-#1872 tree. At the merge base there are **eleven**
assertion sites across **eight** test files, plus one non-assertion use
in `frame_policy`, in six spellings — `vec![bench.post_b]` five times,
`vec![shelf_i]`, `vec![hub_i]`, `vec![sail_a]`, `vec![sail_b]`, and
`superseded.is_empty()` twice. The two `is_empty` rows are
type-agnostic and did not move.
