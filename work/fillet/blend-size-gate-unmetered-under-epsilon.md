---
id: blend-size-gate-unmetered-under-epsilon
kind: issue
title: blend size gate: a positive size under the band's zero still reaches a false-fact refusal at both doors
status: open
opened: 2026-09-04
refs: [fillet-nonpositive-radius-false-fact-refusal]
---

Both blend doors share `nonpositive_size_gate`
(`crates/sweep/src/blend/build.rs`), and its rule is an unmetered
bracket read: `size.lo().partial_cmp(&0.0)` must be `Greater`. The
band's zero is `1e-9`, so every size in `(0, 1e-9]` — positive, but
indistinguishable from zero to the tolerance the rest of the request is
judged by — passes the gate and reaches the battery.

What the caller then reads is the refusal the gate exists to prevent.
At `r = 1e-12` on a cube:

- **fillet** — predicate 1's margin is `radius - radius²/arm`, written
  so a plane's unbounded lever arm saturates it at `radius`
  (`crates/sweep/src/blend/battery.rs:355-357`). At `1e-12` that margin
  classifies `Zero`, and the consumer reads "radius 1e-12 m exceeds the
  curvature headroom of [a plane] — reduce the fillet radius": false in
  both halves, and a recourse with nowhere useful to go.
- **chamfer** — `fillet3_corner_independence`'s `|det(n1,n2,n3)|·d` is
  levered by `d` itself, so the consumer reads
  `UnsupportedCorner { corner: DependentNormals }` about a cube corner
  whose normals are exactly orthonormal.

That is the same levering `BlendError::NonpositiveSize`'s own doc
(`crates/sweep/src/blend/mod.rs`) says the door check prevents — "a
false fact about the BODY is worse than no diagnosis". The gate keeps
that promise at zero and below, and does not keep it just above zero.

**The blend's is the one unmetered spelling of three.** The other size
doors in the kernel decide their positivity against the band:

- `shell` — `decide("shell_thickness", Margin::of(thickness), band)`
  (`crates/topo/src/shell.rs:525`), refusing `ShellError::Thickness`
  on anything not `Sign::Positive`;
- `revolve::tube` — `decide("tube_wall", Margin::of(wall), band)`
  (`crates/sweep/src/revolve/tube.rs:373-379`), refusing
  `TubeError::NonpositiveWall { eps }` and naming the eps it judged by.

**The decision is this unit's, not the gate's.** Two shapes, and they
differ in what they promise rather than only in code:

1. **Meter the size against the band**, as shell and tube do — the size
   becomes a two-tolerance question (a `Zero` classification refuses as
   invalid input, an escalation escalates), and `NonpositiveSize` grows
   an eps in its payload the way `TubeError::NonpositiveWall` has one,
   so the refusal says what it judged by.
2. **Keep `> 0` and narrow the promise** — the gate's doc and the
   variant's doc stop claiming to prevent the levered false fact in
   general, and say plainly that they screen a size that is not
   positive at all; the sub-band case is then a known, stated gap in
   predicate 1's and corner-independence's own sentences, and the fix
   belongs to those sentences instead.

**The witness is committed and green.**
`crates/sweep/tests/review_fillet_e1_probes.rs::a_positive_size_under_epsilon_reads_a_false_fact_at_both_doors_today`
pins today's behaviour at `1e-12` as a characterization — fillet
`RadiusHeadroom` with the headroom sentence, chamfer
`DependentNormals` — and is the row that goes red whichever way this
unit decides.

Filed out of FILLET-E1's review round (PR 1743), which landed the
shared gate; the review's own probe row for this class was `#[ignore]`d
and has been converted to the characterization above rather than left
skipped.
