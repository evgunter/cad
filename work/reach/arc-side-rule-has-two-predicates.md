---
id: arc-side-rule-has-two-predicates
kind: issue
title: The chord's arc-side rule is the azimuth window on a monotone section and the run side on a tilted sphere section, where the chart-free one could serve every conic
status: closed
opened: 2026-10-02
priority: P1
cost: M
pr: 3985
branch: reach/arc-from-pairing
closed: 2026-10-04
---


## What

Found by the `reach/tilted-sphere-pair` lane, which added the second.
`chord_join` selects a chord's arc of the section conic two ways:

- `select_arc` — azimuth-window containment (M5 S9), where the conic's
  chart azimuth is monotone (`SectionConic::azimuth_monotone`: every
  cylinder conic the table admits, and a polar sphere section);
- `select_arc_by_run_side` — the arc leaves each run end on the run's
  left under the face's outward normal, for a sphere section tilted
  against the chart, whose azimuth doubles back.

The second reads no chart, so it would select the arc on every conic
the wall lane mints. Two predicates for one rule is the P1 shape. The
boolean's planar side (`JoinLane::BoolPlanar`) reads the mate wall's
window by value and has no run-side form yet
(`planar-side-of-a-tilted-plane-sphere-cut-has-no-arc-cue`).

## The question

Fold the window rule into the run-side one (every cylinder cut and
polar sphere cut then re-selects by the run; bits should not move, but
the `split_arc_window` and `bool_between_arc_window` rungs and the
`BoolPlanar` window plumbing lose their consumers), or keep the window
rule where its premise holds and say what it buys. The planar side's
cue is the same question one lane over. The run-side rule's
own open premise is `run-side-arc-rule-reads-only-the-run-at-each-end`.

## A third reading, on the split lane (CLEAVE, PR 3718)

CLEAVE's conic pairing (`splitting::join`'s `conic_pairs`) walks a
curved face's section conic in the direction `n_plane × n_out` and
pairs each entering crossing with the next one. The arc from an entry
to its exit in walk order is the arc inside the face. So on the split
lane the pairing already knows which arc each chord should take, and
`chord_spec` then derives it a second time, by azimuth window or by
run side. The three readings agree wherever each one reads: each names
the arc that leaves its ends into the face, and the run-side rule
refuses rather than choose at a corner where it cannot see that.

Today they never meet on one face. The split lane refuses sphere faces
at its reduce, and only tilted sphere sections take the run-side rule
(pinned by `crates/sweep/tests/tilted_sphere_pair.rs`,
`a_tilted_split_of_a_sphere_body_refuses_before_either_arc_rule`).
When the split lane admits sphere faces, the unification this item asks
for could hand the walk's arc to `chord_spec` instead of re-deriving it.

## Answered (PR 3985, `reach/arc-from-pairing`)

Neither predicate: the arc is the pairing's datum. The designer pair
(`analysis/design-fork/arc-side-rule-d1` and `-d2`) converged on it and
the orchestrator adopted it. The join hands `chord_spec` the section's
direction of departure at each site (`chord_join::Leave`): the boolean
germ's `dir`, the split's `±(n_plane × n_out)` (`splitting::join`'s
`split_leave`, which `conic_pairs` walks by too). The chord takes, of
the conic's two arcs, the one leaving its start along it
(`chord_arc_leave_germ`, `chord_arc_leave_section`). `select_arc`, `select_arc_by_run_side`,
`SectionConic::azimuth_monotone`, the window handed to the planar side,
`ArcWindowCase`'s chord cases, `ArcSideCase`, `SectionArcSide` and
`SectionNotPolar` are gone; the cone apex's window case survives as
`SplitJoinError::ApexUnlifted` for the window walk's other readers.

Landed behind a cross-check: both selections side by side, refusing on
any disagreement, over the full suite at three ε. The PR body has the
counts. In-suite, the two disagreed in two classes: the run-side rule
on a ball whose seam lies in the other operand's plane face (the die
pips; a `y`-poled ball on a cylinder cap), where the datum's chords
build and the op then stops at the role read (`SectionLoopUndecided`)
with no body; and the window rule on a cylinder wall
(`germ_coplanar_conic`'s tube strut), where the datum's body meets its
closed form. The phase-1 commit as written
paired by chord length; run that way, a third class shows, a slab
across a round boss, which the walk-order pairing removed.

## Closed (2026-10-04)

Merged by PR 3985. A chord takes the arc its pairing named, read from
one datum minted with the crossing; `select_arc` and the run-side
selector are retired. After the merge with JOIN's chord ranking
(PR 4008), the walk-order filter pinned nothing and was removed
(orchestrator's ruling (a)); main's `wall_region` and
`ChordJoiner::fragments`, which fed only the retired azimuth window,
went with it. A second independent verifier
(`analysis/reach-verify2/3985`) found the final head VERIFIED: of
26,499 chords main mints across topo, sweep, mesh, editor-core and
step-import, and 10,830 in the tour, the head mints every one; all
four mutants are killed; every ε red is red on main too.
