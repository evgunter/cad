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
`seg::read_back_arc` reads it against what was authored:
- its start on its carrier (`path_arc_start_on_carrier`, the stored radius);
- its landing on its end vertex (`path_arc_landing`). For a
  `Facts::Decided` arc the radial part its inline decision admitted is
  left out, so what is read is the sweep;
- a `Via` arc's carrier against `q` (`path_arc_through_point`). `q`
  rides `BuiltArc::through` to the door and is never stored;
- its branch (`path_arc_branch`): the stored r·Δθ signed by the turn
  sense the construction authored from data other than its sweep. That
  is the side of the chord `q` or a tangent's target lies on, a
  winding, or a side.

Every arc is read except a `Bulge` leg whose authored bulge reads as a
line. All reads go through `k_stats::check_unlogged` (audit row F23).
Exact scalars do not run any of it.

**Outcomes.** Zero passes. A miss clear of the read's own rounding at
its magnitude by the band is `PathError::ArcMissesAuthoredPoint`, a
kernel defect. A miss within that rounding is
`PathError::ArcMissBelowSceneResolution`, with the recourse "move the
profile nearer the origin, or author the arc with a smaller radius …;
a coarser tolerance also reads it", shared with validation's
`ArcBelowSceneResolution`. Both name the step and its role, the verb,
the read, the authored point, the miss, the magnitude and the rounding.
A read in the band, or poisoned, escalates, for every arc.

**Teeth** (kept: `path::stored_arc_landing`, `tests/arc_read_back_sweep.rs`
and `tangent_arc_near_a_full_turn`). Each mutant below was run, and
these went red:

| mutant | rows that went red |
|---|---|
| the guard always passes | 10 `stored_arc_landing` rows |
| the clearance gate inverted | 8 rows |
| a decided arc's landing skipped | 2 rows |
| a decided arc's in-band landing passed | 1 row |
| the flat skip applied to every arc | 2 rows |
| `split_half_tan`'s X → −1/X | every far-turn row, on `path_arc_branch` (tangent, `Center`, `Via`) |
| the tangent arc's radius × (1 + 1e-9) | refuses on `path_arc_start_on_carrier` |
| the cancelling first arm | tangent, `Center` and `Via` refuse at f64 |

**The sweep found a live defect, fixed here.** A counterexample search
over every arc mode at `f64` (`tests/arc_read_back_sweep.rs`) found 229
`Via` legs near a full turn whose carrier sat off `q`, up to 4e-5 m at
R = 1e6, on scenes that resolve the read. The cause was that
`bulge_from_via` took its turn's cross term as `(q − a) × (b − q)`,
which cancels. It now reads `(q − a) × (b − a)`, which is equal over
the reals. The first counterexample is pinned
(`the_sweeps_via_arc_near_a_full_turn_holds_its_through_point`).

**What moved.** Sound arcs on scenes past the read's resolution now
refuse at `f64` instead of storing. `tangent_arc_near_a_full_turn`
pins which rows refuse at each ε (`REFUSES_PAST_RESOLUTION`). Three
viewer rows at 1e307 to 1.6e308 m now refuse at the door, before the
flattener (filed
`issues/viewer-flattens-arc-arms-have-no-path-witness-after-the-door-read-back`).
No symbolic receipt moves (`m10_9_pins_interval`, `m10_10`, `sym11`,
`sym_9`, `m10_bulge`).

**Not covered.** A `Via` arrival or fused incoming whose fillet fits
exactly stores no arc on the `Via` carrier, so there is nothing stored
to read against `q`. That residue stays on
`a-via-arcs-carrier-is-never-checked-against-its-authored-through-point`.
Arc fillets carry no authored sense yet, so their branch is not read.
A line×line fillet's branch is not read either; its declared
tangencies are, in the fillet re-read.
