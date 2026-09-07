---
id: the-witness-slack-is-eps-independent
kind: issue
title: the door's f64 witness is a relative constant, not the run's eps: at a tight eps row it is many band-widths loose
status: open
opened: 2026-09-06
---

**Raised in review of M10-9** (R1, on the registered-identity door),
**attempted in the fix pass, and REVERTED by two measurements.** Filed
so the limit is on the record with what it would take to remove.

## The finding, which stands

`geom_core::real::WITNESS_REL = 1e-9`, relative to the larger magnitude
and floored at one, is what `f64::register_equal` (and `Probe`'s, which
delegates) compares against. It does not move with
`CAD_TOLERANCE_EPS`. At ε = 1e-12 the funnel calls a length zero at
1e-12 and the door accepts a registration whose two sides differ by
1e-9 — a thousand band-widths. The witness is therefore LOOSEST exactly
where the numeric-first shield is tightest, which is R1's point and it
is correct.

The soundness consequence is bounded and stated at the method: a
registration is an AXIOM whose soundness rests on the registrant's
proof, the witness refuses only a lie visible at the point, and at
`Interval` — the lane that decides — the refusal is the exact
"enclosures must MEET" test, which carries no tolerance at all. So this
is a weakness of the CHEAP check, not of the tier.

## Why the obvious fix does not work

The fix pass made the slack `Tol::witness().eps()` and both halves of
CI said no:

1. **`scripts/gates/witness-not-ambient.sh` fires**: kernel library code
   may not MINT a tolerance witness — the run's ε is an entry-point
   commitment, taken as `tol: Tol` and passed down. That gate exists
   for a stronger reason than this preference and it is right.
   `Real::register_equal(self, other)` has no tolerance parameter, and
   neither do the generic constructor bodies that call it.
2. **An ABSOLUTE slack is wrong far from the origin**, measured rather
   than reasoned: ε is a length in metres, `f64` rounding at
   coordinates of 10⁹ is ~10⁻⁷, so an absolute ε refuses a TRUE
   identity and the registrants' `debug_assert!`s fire. It turned
   `mesh::r1_probes_issue1362::r1_the_ball_tessellates_honestly_at_every_placement_the_doors_admit`
   and
   `sweep::verbs_tubewall_r2_probes::r2_stored_inner_and_outer_radii_are_always_distinct`
   red in the `f64` lane (CI run 34048088597).

So an ε-derived witness has to be RELATIVE as well, and it has to
receive the tolerance rather than read it.

## The two routes, if it is taken

1. **Thread `Tol` to the door.** `Real::register_equal(self, other,
   tol: Tol)`, and `tol` passed to `swept::placed_segment_spec` and
   `revolve`'s spec builders from their callers. Correct and
   conventional — every other tolerance in the kernel arrives this way
   — and it is a signature change through the sweep pipeline (ten-odd
   call sites in `extrude.rs` and `loft.rs` alone).
2. **Put a `Tol` on the symbolic session.** `with_session_rules` is
   called by the drive and by tests, both of which hold the tolerance;
   `Sym::register_equal` could then apply the session's ε as a SECOND
   refusal after the value channel's own relative one. Confined to
   `geom-core` plus its session call sites, and it leaves
   `Real::register_equal`'s signature alone — but it puts a tolerance
   inside a type whose whole argument is that it carries none.

Either is a design decision for the unit that next touches the door.
Whichever is taken, the relative floor stays: the absolute-only
spelling is measured wrong.
