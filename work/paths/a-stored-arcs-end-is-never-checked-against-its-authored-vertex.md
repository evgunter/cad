---
id: a-stored-arcs-end-is-never-checked-against-its-authored-vertex
kind: issue
title: paths: nothing checks a stored arc's end against the end vertex it was authored to, so a construction whose arithmetic misses passes validation silently (path.rs's 'verified-never-trusted' promise)
status: closed
closed: 2026-10-20
branch: paths/stored-arc-landing-check
opened: 2026-10-19
priority: P2
pr: 5449
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

## Outcome

#5449 (`paths/stored-arc-landing-check`), an `[ev]` fork: both
designers recommended reading every constructed arc back at point
scalars, at the door, and Ev ruled on 2026-10-20: "we definitely want
the tripwire. hopefully we can make it so the failure is loud and we
can fix any cases of ill conditioning that come up, even though those
kernel issues will be mixed together with unfixable cases".

**What the door reads.** At a point scalar (`T::WITNESS == Inexact`:
`f64`, `Dual`, `Probe`, `Sym<f64>`), before the chain stores an arc
(`Core::build`, after the fillet tangency re-read, and `circle_loop`),
`path::arc_reads_back` reads, for every arc validation reads as an
arc (`seg::reads_as_arc`, validation's own sagitta question, unlogged;
a segment read as a line keeps no carrier), its landing on its end vertex
(`seg::constructed_landing`, `path_arc_landing`, 16 ulps) and a `Via`
arc's carrier against its authored `q` (`seg::constructed_through_point`,
`path_arc_through_point`, 8 ulps). `q` travels on `BuiltArc::through`
to the door and is not stored. Both go through `k_stats::check_unlogged`
(audit row F23). Exact scalars do not run it.

**Outcomes.** Zero passes. Off at a magnitude the scene resolves:
`PathError::ArcMissesAuthoredPoint`, a kernel defect. Off past the
scene's resolution: `PathError::ArcMissBelowSceneResolution`, with the
recourse "move the shape nearer the origin, or use a coarser
tolerance". Both name the step and its role, the verb, the read, the
authored point, the miss, the magnitude and the rounding there. In the
band or poisoned: `PathError::Escalated`, naming the read, with the same
recourse. One exception: an arc whose end is on its carrier only to the
door's own inline decision (`Facts::Decided`, a `Center` arc or a
decided fillet tangency) passes an in-band landing, which that decision
admitted; reading it again would decide it twice
(`review_s2`'s drawn decided tangencies land 1.1e-9 to 1.3e-9 m off at
ε = 1e-9).

**Teeth** (kept, `path::stored_arc_landing`, every ε row): the
cancelling tangent, `Center` and `Via` arcs, spelled by hand as they
stood before #5403, each refuse `ArcMissesAuthoredPoint` naming their
read; the door's own arcs on the same probes pass; a miss past the
scene's resolution refuses the sibling, naming its step and recourse;
an in-band miss and a poisoned arc escalate. Mutating the guard to
always pass reds the first three; inverting its resolution gate reds
the first three and the sibling row.

**What moved.** Sound arcs on scenes the read cannot resolve now refuse
at `f64` instead of storing: `tangent_arc_near_a_full_turn`'s
1.37e7 and 1.37e8 rows at the default ε, its `Center` 1.37e6 row and its
1e4 m short `Via` row at ε = 1e-12. Those rows now pin the refusal and
its scale. Three viewer rows at 1e307 to 1.6e308 m now refuse at the
door before the flattener (filed
`issues/viewer-flattens-arc-arms-have-no-path-witness-after-the-door-read-back`).
No symbolic receipt moves (`m10_9_pins_interval`, `m10_10`, `sym11`,
`sym_9`, `m10_bulge`).

**The sweep found a live defect, fixed here.** A counterexample search
over every arc mode at `f64` (`tests/arc_read_back_sweep.rs`, about a
million programs at three ε rows) found 229 `Via` legs near a full
turn whose carrier sat off `q` (up to 4e-5 m at R = 1e6, 4.8e-8 m at
R = 1.4e4) on scenes that resolve the read: `bulge_from_via` took its
turn's cross term as `(q − a) × (b − q)`, two long, nearly opposed
vectors whose product cancels to the short chord's scale. It now reads
`(q − a) × (b − a)`, equal over the reals, and the sweep finds none.
The first counterexample is pinned
(`the_sweeps_via_arc_near_a_full_turn_holds_its_through_point`).

**Not covered.** A `Via` arrival or fused incoming whose fillet fits
exactly stores no arc on the `Via` carrier, so there is nothing stored
to read against `q`. That residue stays on
`a-via-arcs-carrier-is-never-checked-against-its-authored-through-point`.
