---
id: a-fourth-withdrawal-kind-is-forced-at-one-of-its-three-sites
kind: issue
title: PruneReport's kinds are re-declared on OpOutcome and hand-fanned into notices; only the copy between them is exhaustive
status: closed
opened: 2026-09-11
closed: 2026-09-11
branch: view/gesture-doors
---


Found by `view/silent-withdrawals` while adding the THIRD kind
(`prune-kills-a-gesture-and-reports-nothing`), which is the unit that
made the shape visible: adding one kind of withdrawal touched three
places and exactly one of them refuses to compile when it is missed.

## The three places

- **Declared** on `PruneReport`
  (`crates/viewer/src/display.rs:547-573`): `superseded`,
  `dropped_hides`, `killed_gesture`.
- **Re-declared** on `OpOutcome`
  (`crates/viewer/src/session/op.rs:713-772`): the same three fields,
  with the same element type, carried for the same reason.
- **Fanned out** by hand in `app.rs`
  (`crates/viewer/src/app.rs:939-964`): three `notices.extend` calls,
  one per kind, each naming its own `frame::Withdrawal` constructor.

`OpOutcome::from_prune` (`crates/viewer/src/session/op.rs:782-803`)
destructures the report, so the COPY between the first two is E0027 on
a fourth field — that is the one site that holds. Neither of the other
two does: a fourth field added to `PruneReport` and to `OpOutcome`
compiles with no fourth `extend`, and the withdrawal reaches the
chrome's channel and is never worded. That is the defect
`prune-drops-a-hidden-instance-silently` and
`prune-kills-a-gesture-and-reports-nothing` each were, one kind at a
time, arriving by a different route.

`frame::WithdrawalKind` is exhaustive at the `Display` — a fourth
variant reds there — but nothing forces a fourth variant to EXIST for
a fourth report field, and nothing forces the call that renders it.

## What resolving it looks like

Candidates, none obviously right and the choice is the work: `OpOutcome`
holds a `PruneReport` rather than re-declaring its fields; or the fan-out
becomes one iteration over the report's kinds, which needs the report to
be enumerable rather than three named fields. Either would put the
exhaustiveness where the omission actually happens, which is the
rendering call and not the copy.

## Home

VIEW's: `crates/viewer/src/display.rs`, `crates/viewer/src/session/op.rs`,
`crates/viewer/src/app.rs`, `crates/viewer/src/frame.rs`.

## Closed — the exhaustiveness moved to the rendering call

Answered by `view/gesture-doors`, taking both candidates this item
named, because neither alone is the fix.

`OpOutcome` now HOLDS the report (`pub withdrawn: PruneReport`) instead
of re-declaring its three fields, so the second declaration and the
copy between them are both gone — and with them `from_prune`'s
destructure, which was the one site that held and the wrong one.

The fan-out is one call. `frame::Withdrawal::all(&PruneReport)`
destructures the report and yields one `Withdrawal` per non-empty kind,
so a fourth field is E0027 *at the call that words it*. The three
constructors are private, which is the half that makes it stick: a
caller outside `frame` cannot fan out by hand again. `app.rs`'s three
`notices.extend` calls are one.

The reverse direction — a `WithdrawalKind` with no producer — is held
by `every_withdrawal_kind_has_a_producer` against
`WithdrawalKind::ALL`, which made `WithdrawalKind` the crate's tenth
`vocabulary!` declaration (the README's nine-counts moved with it).

**The defect had a fourth instance and it was in a test.**
`frame_policy.rs`'s hand-written mirror of the `app`-gated loop — whose
own comment says *"it has to model every producer that feeds the
notices there … A half-mirror would pass while the real loop dropped
the other"* — listed two producers after #2348 added the third beside
it. It calls `Withdrawal::all` now, so there is no list there to fall
behind.
