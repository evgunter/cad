# PERF-6 — tier 3's +V check certifies a sign, not a precision

**Status: ratified at dispatch (PERF orchestrator, 2026-09-11).** Binds
the implementer of unit `PERF-6`; deleted at merge per
`docs/DOC-LEDGER.md`. Read `docs/prompts/implementer-discipline.md` in
full first. The item is
`work/perf/tier3-plus-v-needs-a-sign-and-pays-for-a-precision.md`
(Ev's finding, PR 2303); `work/perf/gate-then-measure-pays-two-quadratures.md`
rides with this unit and lands after it.

## 0. The finding this executes

Tier 3's ninth check, the +V global orientation invariant, consumes
the SIGN of the body's volume enclosure and nothing else
(`crates/topo/src/validate.rs`, `plus_v_invariant`: `decide("positive_volume",
Margin::over_lever(v_hi, surface_area), band)`). It obtains it from
`mass_properties_certified` (`validate_geometric_certified`), the full
certified quadrature refined per face to `QUAD_TARGET_LEN_FACTOR · ε`
(`crates/geom-brep/src/props/quad.rs`, factor 1024; `QUAD_INIT_PIECES`
16, `QUAD_MAX_ROUNDS` 12, `last_round_refuses` predicting an
unreachable target after round 0). Two consequences, both measured:

- **A false refusal.** The teapot's canal at ε = 1e-12: round 0's
  enclosure excluded zero by ~5 orders of magnitude, and the body was
  refused `VolumeUncomputable { QuadratureBudget { width_len: 2.53e-8,
  target_len: 1.024e-9, rounds: 1 } }` — a valid solid reported
  unvalidatable for missing a precision the check never reads.
- **A cost.** On a NURBS-walled body (`loft_prism`) tier 3 is 157 ms
  and `mass_properties` 160 ms (PERF kernel lane, release): tier 3 IS
  the reporting quadrature, paid again by every caller that gates and
  then measures.

The coupling was deliberate ("ONE certified quadrature, held and then
handed on") and the unit keeps that property for callers that want
both: what changes is WHEN each caller's certification stops.

## 1. What this unit delivers

**A certification level on the certificate.** `MassProperties` (or the
certified result type the doors hand back) carries what it certifies:
`Sign` — the volume enclosure's sign is definite under the band, the
values are enclosures at whatever round that took; or `Target` — every
face met the reporting target as today. Make the level part of the
type (two types, or an enum the number-reading accessors refuse on —
choose the one that makes reading a `Sign`-level volume as a number
impossible by construction, D9 row 0).

**The sign door.** `validate_geometric` and every tier-3 door that
exists only to run check 7 certify to `Sign`: run round 0 for every
face (exactly today's round 0 — same schedule, same bits), sum, and
decide the margin under the band; if definite, stop. If not definite,
continue refining exactly as today — round by round, every face, the
sum re-decided after each round — and stop at the first definite
round or at the reporting target, whichever comes first; a body whose
sign is never definite at the target refuses exactly as today. State
the loop's shape and the order of face refinement so the bits at any
round are the reporting door's bits at that round (D9: the reporting
door's result is unchanged, bit for bit — §3 pins it).

**The reporting door.** `mass_properties` certifies to `Target` as
today. A `Sign`-level certificate can be **continued** to `Target`
(`refine_to_target(certificate) -> MassProperties<Target>`, or the
equivalent) reusing the rounds already taken, so a caller that gates
and then measures pays one quadrature — `validate_geometric_certificate`
hands back the `Sign` certificate and the continuation is the second
half. That is the "two entry points, the second reusing the first's
rounds" of Ev's item, and it is what the riding item's consumers (the
tour at `demos/tour/src/main.rs:402,423`, the Python
`validate_geometric()` + `mass_properties()` pair) will call.

**What must not change.** `plus_v_invariant`'s decision rule (margin
over the surface-area lever, under the band); `QUAD_TARGET_LEN_FACTOR`
and its rationale (the reporting use); the K-funnel telemetry
(`props_quad_*` predicates keep their names and their counts on the
reporting path — the sign path adds a named predicate for the
sign-definiteness decision, per the crate's rule that acceptance and
refinement go through named predicates); every closed-form lane.

## 2. The check's docs

Tier 3's module docs state the ninth check's use ("deciding its sign
is an act of certification rather than a measurement"); after this
unit they also say what it costs and what it cannot refuse: a valid
solid cannot fail check 7 for quadrature budget unless its sign is
indefinite at the reporting target. `quad.rs`'s module docs gain the
two levels. Present tense only.

## 3. The pin

- **Bit identity of the reporting door.** Every `mass_properties`
  result across the corpus, the tour's bodies and `crates/topo`'s and
  `crates/geom-brep`'s own fixtures is byte-identical to main's
  (a digest row committed before the change, as PERF-2 and PERF-3 did);
  the K-report data (`docs/k-report-data/`) predicate counts for the
  reporting path unchanged — the k-lint gate is the mechanical pin.
- **The sign door agrees with the reporting door** on every body both
  accept: same sign, and the `Sign` certificate's enclosure contains
  the `Target` certificate's value.
- **The false refusal is gone**: a row that builds the teapot canal's
  shape (or the smallest fitted rational body that reproduces
  `rounds: 1`, `width_len ≫ target_len` with a definite sign) at
  ε = 1e-12 and asserts tier 3 passes at `Sign` level while the
  reporting door still refuses `QuadratureBudget` for it — the row that
  says the two levels are different things.
- **A body whose sign is indefinite at round 0 and definite later**
  (a thin shell, a body with a near-cancelling flux) refines exactly
  as far as needed and no further; and one whose sign is never
  definite refuses as today.
- **Continuation reuses rounds**: a counter or the K telemetry shows
  `Sign` then `refine_to_target` runs the same number of piece
  evaluations as `Target` alone.

## 4. Measurement to report

Release, 4 vCPU under the build slot, medians of 3: tier 3
(`validate_geometric`) on `loft_prism` and the corpus's NURBS-walled
bodies, before and after; `mass_properties` unchanged; the tour's
per-stop tier-3 time (`perf/explore-kernel`'s stage spans; fetch, do
not merge). The riding item's consumer change is measured after it
lands, not here.

## 5. Out of fence

The per-face schedule and targets of the reporting use; the closed-
form lanes; `recertify_approx` (tier 3 check 1 —
`tier3-approx-regrid-per-face-cost` is its own box); the consumers
(the riding item). TOPO territory (`validate.rs`, `props.rs`) and
PROPS territory (`geom-brep/src/props/*`), announced in
`work/perf/log.md`; TOPO's dispatched unit is the census readback
door and does not touch these functions — merge `origin/main` before
opening the PR and re-check.

## 6. Report

≤150 lines: the level type and why that shape, the sign loop's order
and its bit-identity argument, the false-refusal row's fixture, the
pins, the measurements of §4, deviations, findings outside the fence.
