---
id: curve-kind-placement-disagrees-with-the-ratified-seat-clause
kind: ruling
title: The ratified VERB-SEAT S1 puts CurveKind beside Curve3; the code has kept it in topo::query since SEAT-2
status: open
opened: 2026-09-14
needs_ev: true
refs: [edge-carrier-kind-has-no-readback-door, 2587]
priority: P1
cost: D
---



## What disagrees

`docs/VERB-SEAT-DESIGN.md` §1 S1, the text Ev ratified at PR #1388 and
the text SEAT closed on unchanged (readable at the SHA
`docs/DOC-LEDGER.md` sweep 8 names, `git show
326be1bf4:docs/VERB-SEAT-DESIGN.md`, §1 S1 second bullet):

> `CurveKind` moves down beside `Curve3` (its own doc note at
> `editor-core/src/names/geompred.rs:77` records the move as additive);
> `SurfaceKind` stays in `geom-brep` and is reused (the
> one-fieldless-mirror rule).

"Beside `Curve3`" is `geom`. The code puts it in `crates/topo/src/query.rs`
and has since SEAT-2 (`ab01294ce`, "SEAT-2: the topo query module",
an implementer lane), where it still is — `CurveKind`, `CurveKindSet`
and the EXACT predicates in one place.

`crates/verbs/README.md` §1 S1 — the design page that replaced the doc
when SEAT closed — says something else again:

> `CurveKind` is defined here — the mirror lives where it is used,
> beside the predicates that read it — while `SurfaceKind` stays the
> workspace's one fieldless surface mirror in `geom-brep` and is reused.

That sentence is the CODE's rule, not the ratified one. Its history,
run per CLAUDE.md's "check that Ev ever agreed" (the worktree is
shallow, so through the GitHub API): it first appears at `ab01294ce`
(SEAT-2, agent-authored) and reaches `crates/verbs/README.md` at
`e1bc0f99a`, "SEAT closing sweep: the ratified design moves beside the
code" — PR 2011, agent-authored, not an `[ev]` PR. No commit ratifies
it. The companion table's `Ratified (#1388)` row is about the page's
lineage and does not reach this sentence, which is exactly the case
CLAUDE.md's rule names.

## Why it is Ev's and not a lane's

Two texts and one tree disagree, and which one moves is a design
choice, not a description the code has already moved:

- **the clause moves** — `crates/verbs/README.md` §1 S1 is amended to
  record that the mirror stayed in `topo::query`, and the ratified
  VERB-SEAT sentence is noted as superseded by SEAT-2; or
- **the code moves** — `CurveKind` goes down beside `Curve3` in `geom`
  and `topo::query` re-exports it, which is a kernel-wide rename
  touching every reader of the mirror and its set type.

`edge-carrier-kind-has-no-readback-door` (PR 2587) had to pick a
placement for the answer type of a new door and chose STAY, on reasons
that are the code's own and hold either way: both readers of the mirror
are in `topo`, `CurveKindSet` and the three EXACT predicates sit beside
it, and a door naming an answer type authored elsewhere is what
`readback::face_carrier_kind` already does with `geom_brep::SurfaceKind`.
That PR's `query.rs` placement paragraph states the rule as this
crate's, not as ratified text. Nothing in it forecloses either
disposition above.

## What to do with it

Carry it on the next `[ev]` PR this program opens. Whichever way it
goes, one of the two sentences is edited in that PR, and the
`query.rs` placement paragraph follows it.
