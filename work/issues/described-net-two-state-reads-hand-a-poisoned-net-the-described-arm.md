---
id: described-net-two-state-reads-hand-a-poisoned-net-the-described-arm
kind: issue
title: Thirteen placeholder-or-described reads outside tier-3 check 1 hand a described net carrying poison to the described arm
status: open
opened: 2026-09-05
---


## What

TOPO's S330 (PR 1923) gave tier-3 check 1 a third answer for a
`Surface::Nurbs` payload — placeholder, described, or described-and-
carrying-poison — because `geom`'s rule (`crates/geom/src/net.rs:128-131`)
says a net poisoned in some channel is corrupt DESCRIBED geometry that
must fail at each consumer's described arm. The dual review swept the
tree for the other consumers that ask "placeholder or described?" and
hand everything else the described arm, and found thirteen, none of
which S330 touched (other programs' files):

- `crates/topo/src/pcurves.rs:273` (TRIM)
- `crates/topo/src/props.rs:1531` (code-quality Track M / S-CERT)
- `crates/topo/src/replace_face.rs:1211`, `crates/topo/src/transform.rs:370, :495` (SHELL)
- `crates/topo/src/census.rs:1741, :2284, :3488` (CURVED; `:2272-2275`'s
  argument that no public-door body carries a placeholder is now
  narrower than it reads, since check 1 also bars poisoned nets — S350's
  live path is the in-src rows and second callers)
- `crates/mesh/src/chords.rs:508`, `crates/mesh/src/trimmed.rs:200` (S-MESH)
- `crates/step-import/src/adopt.rs:530, :710, :952` (EXCH)

The reviewer did not establish that any is WRONG — several refuse
downstream by escalation — only that the invariant now protects one
door and the sweep was never run. The tool is the door S330's fix pass
lands: `NurbsSurface::net_state() -> NetState` (`crates/geom/src/surfaces/nurbs.rs`),
one method answering the three states, so a consumer's match is
exhaustive over them instead of over a guard pair.

Two notes from the same review, same rule, for whoever takes this:
`Real::is_poison` is NaN-only at `f64` (`crates/geom/src/lib.rs:92-95`),
so a net of `+∞` control points is DESCRIBED and passes check 1 while
describing no locus — geom-core's policy, not S330's; and
`crates/topo/src/r2_probes.rs:1-2` says "committed to the reviewer's
own branch only" while sitting on main.

## Home

`work/issues/`: the thirteen sites span six programs and no one owns
the class; a taker claims it by moving this file.
