---
id: two-green-prs-merge-into-a-red-main
kind: issue
title: Two PRs each green against a base without the other merge into a red main, and nothing re-gates the pair
status: open
opened: 2026-09-04
---


## Finding

**Main has now gone red twice in one day by the same mechanism, and it is not
a coverage hole — every job that should have caught it ran and passed.**

`ci.yml` builds the pull-request TEST-MERGE commit (`ci.yml:475` states this),
so a PR is gated against `main` **as it was when the run started**. Two PRs
whose diffs do not overlap, and which therefore never conflict, can each be
green against a base that lacks the other and be merged minutes apart:

- `#1769` added `crates/viewer/src/tree.rs`'s `blamed_mates`, a deliberately
  exhaustive `match` on `editor_core::MateFault` (merged 22:04:54).
- `#1725` added `MateFault::Unleverable` (merged 22:26:52).

Twenty-two minutes apart, no file in common, both green, and the pair does not
compile: `E0004`, `viewer` fails to build, which reds `clippy`,
`build + archive (default)`, `rustfmt + rustdoc (gate) + wasm32` and the render
lanes on **every** PR opened afterwards. Repaired by PR 1792.

Earlier the same day, `#1756` changed `topo::shell` to return `Shelled<T>` and
left two now-no-op field accesses in `demos/tour`; repaired by PR 1775. That
one is usually told as a sampled-axis story (the `k-lint` job was green over a
`skipped` demos step), and it is — but the shape underneath is the same: a
signature the base did not have when the other side was gated.

**The class is semantic conflict without textual conflict.** Git's merge
detects overlapping TEXT; nothing in this repo's gate re-runs the pair. An
exhaustive `match` in one crate and a new enum variant in another are the
canonical instance, and the kernel is full of the shape — every
`#[non_exhaustive]`-less public enum, every trait with a new required method,
every signature change with an out-of-crate caller.

**What would close it, and why this is a finding rather than a fix.** A merge
queue re-gates each PR against the tip it will actually land on, which is the
mechanism built for exactly this and would have caught both instances. It also
serializes merges and costs a full run per merge, on a repo whose minutes are
already watched (`docs/CI-MINUTES-2026-08.md`). The cheaper half-measures — a
required "branch up to date with main" before merge, or a post-merge canary on
`main` that summons the two lanes rather than gating anyone — trade different
things. Whoever takes this decides which; the point of the file is that the
class is now measured at **twice in one day** rather than argued.

Found by the lane repairing the second instance (PR 1792), which established
the timeline from the merge commits rather than from the PR bodies. Filed here
rather than on a program's slate because the mechanism is the merge gate
itself; `work/ciw/` is the nearest owner if one is wanted.

## Was

`unrowed` — raised while repairing the 2026-09-04 `viewer` main-red.
