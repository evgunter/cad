---
id: the-witness-slack-is-eps-independent
kind: issue
title: the door's f64 witness is a relative constant, not the run's eps: at a tight eps row it is many band-widths loose
status: closed
opened: 2026-09-06
parent: SYM-6
closed: 2026-09-14
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

## Re-homed at M10's exit sweep (2026-09-13)

Here for the same reason and through the same seam: `WITNESS_REL` is the door's
f64 witness and lives in PROPS' `real.rs`, and what the row is about is what the
tier's door accepts.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## Decision for Ev (2026-09-14, `[ev]` PR from the SYM orchestrator)

**D1 — which route, or neither.** The finding stands: the door's `f64`
witness is `WITNESS_REL = 1e-9` relative, not the run's ε, so at
ε = 1e-12 it accepts a registration whose sides differ by a thousand
band-widths — loosest exactly where the numeric shield is tightest.
The deciding lane (`Interval`) refuses exactly, so soundness is
bounded; what is weak is the cheap check.

- **(1) Thread `Tol` to the door** — `Real::register_equal(self, other, tol: Tol)`,
  `tol` passed to `swept::placed_segment_spec` and the revolve
  builders from their callers (about twenty call sites in `sweep`);
  the slack becomes `max(tol.eps(), WITNESS_REL · scale)` — the
  relative floor stays, measured necessary. **Recommended**: it is how
  every other tolerance in the kernel arrives, and the
  `witness-not-ambient` gate exists for exactly this reason.
- **(2) A `Tol` on the symbolic session**, applied by
  `Sym::register_equal` as a second refusal after the value channel's
  relative one. Confined to `geom-core`; leaves the trait alone; puts
  a tolerance inside a type whose whole argument is that it carries
  none. Not recommended for that reason.
- **(0) Leave it, ratified as not-now** — the row becomes `deferred`
  citing this answer: the cheap check is a cheap check, the exact
  refusal is at the lane that decides, and the M10-9 pins hold
  `registrations_contradicted == 0` on every fixture. Honest, and
  the cheapest; it keeps the asymmetry the review named.

The orchestrator's pick is (1), taken as a unit of its own (M,
STRUCTURAL) in the block after SYM-B1, unless Ev picks (0), in which
case the row is deferred on this section.

**D2 — one refusal arm or two** (`the-span-identity-is-not-a-theorem-of-the-floats`):
`Contradicted` today covers both "the claim is false" and "the
arithmetic could not tell at this scale" (an adversarial torus at a
minor radius of 10¹⁸ with a wall below one ULP). The door has one
witness and cannot know which. **Recommended: keep one arm** and make
the loud channel the drive's receipt (`registrations_refused`, already
counted) plus a document-scale row asserting the count is zero on the
M10 fixtures — the row's own suggestion — and close that row on it.
The alternative, a third arm keyed on the witness's scale, is a second
tolerance and is not recommended.

## Ev's answer (2026-09-14): route (1)

"definitely (1)" on #2552. Taken as SYM-6 (`docs/SYM-6-SPEC.md`,
block SYM-B1 slot 2); this row closes at its merge.

## Taken by SYM-6 (PR #2604), with what departed from this file's words

**Route (1) above is what shipped**, on Ev's D1 pick on `[ev]` #2552:
`Real::register_equal(self, other, tol: Tol)`, `tol` handed down through
`swept::placed_segment_spec` and the revolve/extrude registrants from the
nearest holder, and `WITNESS_REL` deleted.

**The slack is `tol.eps() · max(|a|, |b|, 1)`, and that is NOT the
formula the picked route's own text on #2552 wrote.** That text said
`max(tol.eps(), WITNESS_REL · scale)`, which floors the slack at
`1e-9 · scale` — and a floor at the old constant defeats the whole point
at a tight ε row, where the unit's claim is that the witness TIGHTENS.
What shipped is the spec's formula (`docs/SYM-6-SPEC.md`, "The claim"),
it is strictly better, and the departure from the route's wording is
recorded here because it was disclosed nowhere else.

**The cost, both directions, measured.** At ε = 1e-12 the tube-wall
torus probe's refusals rise 15 → 20 and `mesh`'s far-placed ball picks up
its first; at ε = 1e-6 they fall 46 → 10, which is the honest cost: the
door admits sides a thousand times further apart than the retired 1e-9
constant did. No additional registration is recorded on any measured
document at any row (`registered` is pinned per document by
`m10_9_no_registrant_lies_on_any_measured_document`).

### Absolute-scale limits that sit UNDER this row, noted by both reviews

Neither is a defect of this row and neither is filed as one; both bound
what a far-from-the-origin probe of the door can reach.

- **The residual gates are absolute-ε and refuse first.** At
  ε ∈ {1e-9, 1e-12} a body at coordinates of 10⁹ is refused UPSTREAM of
  the door by the certification residual gates, before the door's answer
  could matter (R2 NOTE-2); R1's far washer at 10⁹ refuses on
  `carrier_endpoint_end`. The registrants still run before the refusal,
  so the door stays testable at that scale — which is how both reviewers
  measured claim 2 there.
- **`Sym<f64>`/`Sym<Probe>` panic far from the origin, off this door**:
  `work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert`
  (R2's row, reproduced at the merge base). It stands in the way of any
  future fixture-scale row that wants to drive `Sym<f64>` far out, e.g.
  one counting `Disputed` on the M10-10 documents.

### Two shapes seen and DECLINED for SYM-6 (R2 Q1)

- `band: Band` and `tol: Tol` now travel side by side through the extrude
  and revolve frames; `Band::linear(tol)` derives one from the other, so
  the pair is redundant at every frame that carries both. Pre-existing and
  wider than this door.
- `SymCounts::registrations_refused` is incremented from two places
  (`Sym::register_equal`'s forwarded refusal arm and the cyclic arm),
  which is why "what the column counts" needs a paragraph rather than a
  line. Pre-existing.
