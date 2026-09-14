# SYM-6 — the door's witness moves with the run's ε (spec)

**Program:** SYM (`work/sym/plan.md`, the door lane). **Items:**
`work/sym/the-witness-slack-is-eps-independent.md` (D1) and
`work/sym/the-span-identity-is-not-a-theorem-of-the-floats.md` (D2).
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review). Block SYM-B1 slot 2. **Pre-draw fields,
logged before the draw:** difficulty **M**, task-class **STRUCTURAL**.

- **M** — one parameter threaded through the sweep pipeline to a
  trait method with four implementations, a one-line slack formula,
  and two adversarial probes that must keep refusing correctly.
- **STRUCTURAL** — no certification decision moves: the lane that
  decides (`Interval`) witnesses by an exact meet and ignores the
  parameter; what changes is which lies the cheap `f64` check catches.

**Ev picked route (1) of D1 on `[ev]` PR #2552 (2026-09-14)**, which
this spec implements; D2 was settled the same day (Phase 3, Amendment A2).

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
two items whole (the measured failure of the absolute-ε spelling — CI
run 34048088597 — and the `witness-not-ambient` gate, both of which
this spec is shaped by); `crates/geom-core/src/real.rs`
(`Real::register_equal`'s contract, `WITNESS_REL`'s argument, the
`f64` impl); `interval.rs`'s impl (the exact meet); `sym.rs`'s
`Sym::register_equal` and the `Real for Sym` forward; `Probe`'s;
`crates/sweep/src/swept.rs` (`register_rim_identity`,
`register_span_identity`, `placed_segment_spec` and its callers),
`crates/sweep/src/extrude.rs` (the arc wall's registration),
`crates/sweep/src/revolve/{full,surfaces}.rs`;
`scripts/gates/register-equal-allowlist.sh` and
`scripts/gates/witness-not-ambient.sh` (read both headers — the first
greps `\.register_equal\(` and does not care about arity, the second
is why the tolerance must ARRIVE rather than be minted).

## The claim

The `f64` witness compares at `WITNESS_REL = 1e-9` relative to the
larger magnitude, floored at one, whatever the run's ε. At ε = 1e-12
the door accepts two sides a thousand band-widths apart — loosest
exactly where the numeric-first shield is tightest. **The slack
becomes the run's ε, RELATIVE and floored the same way**:
`|a − b| ≤ tol.eps() · max(|a|, |b|, 1)`. Relative because the
absolute-ε spelling is measured wrong far from the origin (rounding
at coordinates of 10⁹ is ~10⁻⁷ absolute, which refuses a true
identity); floored at one so a near-zero pair is compared absolutely
at ε; threaded because kernel library code may not mint a tolerance
witness — the run's ε is an entry-point commitment handed down as
`tol: Tol`.

**Ratified and not re-litigated:** the door's contract (an axiom
whose soundness rests on the registrant's proof; the witness refuses
only a lie visible at the point); the allowlist by site; the
`witness-not-ambient` gate; `Interval`'s exact-meet witness.

## Phase 1 — before touching anything

The call graph: every `register_equal` call site (the allowlist's
roster), every caller of `register_rim_identity`, `register_span_identity`
and `placed_segment_spec`, and for each whether a `Tol` is already in
scope at the caller (the revolve builders hold one; state the extrude
and loft sites). One table in the PR body: site, caller, `Tol` in
scope yes/no, and the distance (in call frames) to the nearest holder.
That table is the unit's shape; if a site is more than two frames
from a holder, say so before threading.

## Phase 2 — the change

1. `Real::register_equal(self, other: Self, tol: Tol) -> SymRegistration`;
   the default arm unchanged; `f64`'s slack `tol.eps() · scale` with
   `scale = max(|a|, |b|, 1)`; `Probe` delegates; `Interval` ignores
   `tol` (say so at the site: the meet is exact); `Sym` forwards it to
   its value channel and records as today. `WITNESS_REL` is deleted
   (its argument moves to the `f64` impl's doc, present tense: why
   relative, why floored, why the run's ε).
2. `swept::register_rim_identity(rim, radius, tol)`,
   `register_span_identity(…, tol)`, `placed_segment_spec(…, tol)`,
   the revolve and extrude registrants — `tol` passed from the nearest
   holder, never minted. Every call site converted; `cargo build` is
   the sweep.
3. The gates: `register-equal-allowlist.sh` green unchanged (the
   roster is by site); `witness-not-ambient.sh` green (nothing minted).
4. **The two adversarial probes re-run at all three ε rows** and their
   refusal counts stated:
   `sweep::verbs_tubewall_r2_probes::r2_stored_inner_and_outer_radii_are_always_distinct`
   (the torus at minor radius 10¹⁸, walls below one ULP) and
   `mesh::r1_probes_issue1362::r1_the_ball_tessellates_honestly_at_every_placement_the_doors_admit`.
   Both must stay green — a refusal is counted, never asserted — and
   the count of refusals per row is a measurement this unit records
   (before: the `1e-9` witness; after: ε at each row).
5. The M10-9 pins (`registrations_contradicted == 0` on the fixtures)
   green at every ε row; `m10_9_witness_limits_interval` re-read and
   its numbers re-taken where the slack enters them.

## Phase 3 — D2: the refusal arm split by witness kind (Amendment A2)

**Amendment A1 (2026-09-14, at dispatch)** held this phase on Ev's D2
pick. **Amendment A2 (2026-09-14, ~17:00Z): Ev took the refined
proposal on #2552** ("that refinement sounds good"), so Phase 3 is in
scope for this unit, after Phase 2, as follows. The door cannot tell
"the claim is false" from "the arithmetic could not tell at this
scale" by the GAP (at the torus the sides are many ULPs apart, so a
scale-keyed arm would be a second tolerance that misfiles its own
case); it can by the KIND of witness.

1. `SymRegistration` gains an arm **`Disputed`**: REFUSED by an
   INEXACT witness — the lane scalar compares points at a slack
   (`f64`, `Probe`) and the two values were apart by more than it. The
   claim may be true and the arithmetic could not tell at this scale.
   Nothing is recorded; it is counted in `registrations_refused` as
   `Contradicted` is today; **never asserted on**.
2. **`Contradicted` is reserved for an EXACT witness**: `Interval`'s
   certified brackets are disjoint — a proof that the two reals differ
   at a point of the box, or an upstream enclosure that does not
   contain its real; a defect either way. Its doc says so in the
   present tense; `f64` and `Probe` never answer it.
3. `Sym::register_equal` forwards its value channel's arm unchanged
   (a `Disputed` value channel refuses exactly as a `Contradicted` one:
   nothing reaches the registry). Every `match` on the enum is updated
   — the compiler is the sweep. The two sweep registrants bind
   `Disputed` without asserting (today's handling, the comment
   re-pointed) and **`debug_assert!` on `Contradicted`** — the
   assertion M10-9's fix pass removed returns, in the lane where a
   refusal is a proof. The two adversarial probes stay green (they run
   at `f64`, where the answer is now `Disputed`); their refusal counts
   are stated per arm.
4. **The fixture-scale row**: at `Sym<Interval>` on the five M10-10
   evidence documents at all three ε rows, the receipt's
   `registrations_refused == 0` (in that lane the count is
   `Contradicted` + `Cyclic`, each a proof of a defect), so a
   registrant that starts stating a lie on a real document reds a
   row. `the-span-identity-is-not-a-theorem-of-the-floats` closes on
   that row; its note about the rim assertion is recorded as settled
   by item 3.
5. No new tolerance anywhere. No new receipt column unless the row
   needs one to say `Disputed` apart from `Contradicted` at `f64`
   (state which, and why, if taken).

## Scope

- Files: `crates/geom-core/src/real.rs` (PROPS' file — the seam is
  announced; SCALAR's subject is the trait, and a parameter on one
  method is not a change to what `Real` is), `interval.rs`, `sym.rs`,
  `probe.rs` or wherever `Probe`'s impl lives; `crates/sweep/src/*`
  (S-BOOL's and BLEND's territory — announced; the change is the
  parameter and nothing else); tests under `crates/geom-core/tests/m10_9_*`
  and `crates/editor-core/tests/m10_9_*`.
- No change to what any registrant states, to the registry, or to
  `Interval`'s witness; `SymRegistration`'s arms change only as Phase 3
  says (one arm added, one arm's meaning narrowed).
- No new tolerance anywhere: `tol` arrives or the code does not compile.

## Acceptance

- Full hosted matrix green; both gates green; the two adversarial
  probes green at every ε row with their refusal counts stated
  before/after; the M10-9 pins unmoved.
- `WITNESS_REL` gone; the `f64` impl's doc carries the argument.
- The call-site table; the refusal-count table per arm; `Disputed`
  never answered by `Interval` and `Contradicted` never by `f64`/`Probe`
  (each impl's doc); the registrants' `Contradicted` assertion restored;
  the fixture-scale zero-refusals row green at every ε row.

## Review

The full v6 dual (`docs/MODEL-AB-LOG.md` protocol v6; ordinal from
SYM's band at dispatch). Claims to falsify: (1) no certification
decision moves — the M10-8/9/10 pins, and the reviewers' own document
driven at `Sym<Interval>` before/after; (2) the slack is relative and
floored, and a true identity at coordinates of 10⁹ is witnessed at
every ε row (the reviewers build one); (3) a lie of `k · ε · scale`
for `k > 1` is refused at every ε row (a planted registrant); (4) the
refusal counts on the adversarial probes are what the PR says; (5)
`Interval` never answers `Disputed` and `f64`/`Probe` never answer
`Contradicted` — a planted swap in either impl reds a row; (6) the
fixture-scale row reds under a planted lying registrant, and the
restored assertion fires only in the `Interval` lane; plus
`docs/prompts/reviewer-style-lane.md` in full. Union fix pass on the
implementer's lane; delta by R1; the row lands at merge.

## Landing

Status `review` on `work/sym/SYM-6.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the
witness-slack row closes with it, the span-identity row on Phase 3.
