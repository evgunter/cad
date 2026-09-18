---
id: extrude-strut-rule-has-three-homes-and-fixed-context-argument-lists
kind: issue
title: extrude's strut rule is spelled in three places and three functions carry the same fixed context as loose arguments
status: open
opened: 2026-09-08
---


Found by the BLEND unit K style review (PR 2149), which read the extrude
bundle while the cap-rim arm was being rewritten. Two duplications in one
file, filed together because both are about `extrude.rs`'s strut lane and a
fix would move the same code.

## 1. The strut (profile-corner join) rule is spelled in three places

The rule — identical side-surface keys keep the conventional description
structurally, otherwise `classify_dihedral` at the strut midpoint decides:
Transverse upgrades to `Intersection`, Smooth descends one order through
`geom_brep::tangent_second_order` (jet-determinate ⇒ `TangentIntersection`,
under-determined ⇒ an image at rest in a wall's chart), Indeterminate is
`SliverJoin` — is written out three times:

- `crates/sweep/src/extrude.rs:30-38` — the module docs' step 4.
- `crates/sweep/src/lib.rs:70-82` — the crate docs' profile-corner bullet.
- `crates/sweep/src/extrude.rs:941-988` — the strut arm's own comment, the
  longest of the three.

Same shape as `extrude-cap-rim-argument-and-k-star-have-five-homes` (closed
on PR 2149 when its subject was deleted): the crate's own convention is
`lib.rs:23`, "stated once — owned here".

**Half of the arm's comment is history rather than invariant**, which is what
makes it long:

- `:941-951` — the invariant, plus the row that demonstrates it. Trimmed on
  PR 2149; the rest of the block was outside that fence and was not read.
- `:953-969` — "**Why `k_prev`, and why the pick is free**" argues a decision
  rather than stating it, and names the row that demonstrates it.
- `:971-978` — the FILLET-strut contrast (`#1116`) is an argument about a
  different verb's refusal, kept here because the word "strut" is shared.
- `:980-988` — "The asymmetry is real even so" restates the certification
  meter, which `topo::Body::describe_at_rest`'s own doc already carries.

The cap-rim arm's equivalent block was collapsed to four lines on PR 2149;
this one was left because the fence was the cap rim.

## 2. Three functions carry the same fixed context as loose arguments

Each silences `clippy::too_many_arguments` with a note that the arguments are
"the sweep's fixed context, not a configuration surface" — which is the
observation that they should be one value:

| site | arity | the shared context |
| --- | --- | --- |
| `crates/sweep/src/extrude.rs:738-752` (`sweep_loop`) | 12 | `place, top_place, normal, w, w_norm, band, tol` |
| `crates/sweep/src/extrude.rs:1037-1050` (`side_surface`) | 11 | `place, normal, w, band` |
| `crates/sweep/src/extrude.rs:1126-1139` (`upgrade_rim`) | 10 | `band, tol` |

`sweep_loop` computes `top_place` and `w_norm` from `place` and `w`, and
passes the whole set down. A `SweepContext { place, top_place, normal, w,
w_norm, band, tol }` built once at the door would let all three drop the
`allow`, and the two derived fields would be derived in one place instead of
travelling as arguments beside their sources.

## Why unit K did not fix it

Both halves are outside that unit's fence, which was the cap-rim smooth arm
and its prose (`work/blend/ambiguity-k-below-the-cap-rim-crossover.md`). The
strut lane is untouched behaviour: neither half changes what `extrude`
builds, and a context struct is a signature change across the file that wants
its own review.
