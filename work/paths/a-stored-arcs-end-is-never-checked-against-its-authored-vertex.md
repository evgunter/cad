---
id: a-stored-arcs-end-is-never-checked-against-its-authored-vertex
kind: issue
title: paths: nothing checks a stored arc's end against the end vertex it was authored to, so a construction whose arithmetic misses passes validation silently (path.rs's 'verified-never-trusted' promise)
status: open
opened: 2026-10-19
priority: P2
---


Found by #5403's review (Q5), the class behind both
`tangent-arc-near-a-full-turn-loses-its-end-vertex` and
`a-center-arc-near-a-full-turn-loses-its-end-vertex`.

**The gap.** `path.rs`'s crate docs promise "verified-never-trusted;
nothing is trusted because the algebra produced it". A tangent or
`Via` arc's endpoint facts are `Facts::Registered`. They are theorems
of the reals, which `ConstructedProfile::validate` does not re-read
(D1). A `Center` arc's facts are `Decided`: equidistance is decided at
the door, but nothing reads the stored SWEEP against the end vertex.
So an arithmetic miss in the sweep went through `validate` with no
error at all:
- the tangent arc at R = 1.36e4 missed by 3.4e-8;
- the `Center` arc at R = 1.37e6 missed by 2.1e-4.

Both are fixed at the construction now, but the next construction
whose arithmetic misses will pass the same way.

**Why P2.** Once #5403 lands no silent miss past ε is known: the one
live miss its re-review's generator found is the hairpin fillet's,
about 7e-11 m (`a-hairpin-line-fillets-half-tangent-cancels`, which
carries its own derived worst case). This row is the guard against the
class's next instance, not a fix for a live one, so it sits below a
live silent wrong answer (P1) and above a cosmetic one.

**Shape of the check.** At storage time, decide
`‖landing(a) − b‖` (the `arc_landing` margin that `seg.rs` already
names for table arcs) against the band, and refuse it typed:
- definitely off: refuse;
- in band: escalate;
- past the scene's resolution: `ArcBelowSceneResolution`.

Questions for whoever takes it:
- whether a registered arc's landing is a decision at all, or a
  `debug_assert!`-class witness check (the registrants' pattern);
- what it costs the symbolic tier, since the landing's forms are new
  per arc;
- whether it supersedes `Facts::Registered` for the endpoint at the
  far end.
