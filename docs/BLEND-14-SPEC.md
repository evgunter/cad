# BLEND-14 — a blend's contact edge takes its description from the must-carry rule (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 14). **Item:**
`work/blend/blend-contact-edges-mint-the-intrinsic-description-without-the-rule.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**NUMERIC**.

- **M** — one arm of one function routes through a wrapper that exists, but
  the row that makes the change visible has to construct a near-osculating
  blend on purpose, and the differential has to prove the corpus did not
  move.
- **NUMERIC** — the question is where `|1/r_band ∓ κ_support|·r_band²/2`
  lands against the band, and which support geometry puts it there.

**Territory note.** `crates/sweep/src/blend/surgery.rs` and
`crates/sweep/tests/**` are BLEND's; `geom_brep::must_carry_over_edge` is
called, not changed (its file is PROPS-adjacent and stays untouched).

## The claim

**`attach_contact` mints `EdgeDescriptionSpec::TangentIntersection` for a
blend's contact edge on a structural flag, without the must-carry rule.**
`crates/sweep/src/blend/surgery.rs::attach_contact` picks the description
from `is_seam` / `transverse` and otherwise stores the intrinsic tangency
with no lane gate, no station walk and no in-band escalation — the shape
BLEND-9 (PR #2491) removed from revolve one level over. Measured (BLEND-9's
fix pass and its delta): every contact edge on the corpus reads `Positive`
at every interior station, minima `4e-2 … 7.5e-2`, seven orders above
`K·ε`; the closed form `|1/r_band ∓ κ_support|·r_band²/2` collapses when the
band osculates a support of its OWN convexity (the difference branch,
`κ_support → 1/r_band`), which on a curved support is ordinary geometry at
ordinary scale. A `Nurbs`-supported band is outside
`tangent_certificate_lane` and cannot store an intrinsic tangency at all;
today nothing gates it.

**The decision, taken by the orchestrator (2026-09-13).** The arm routes
through `geom_brep::must_carry_over_edge` with the contact carrier, its
window, `edge_extent` and the band: `JetDeterminate` → `TangentIntersection`
as today; `UnderDetermined` → the conventional description (a lane-refused
pair stores what the certificate can certify — say which
`EdgeDescriptionSpec` that is for a contact edge, or refuse typed if no
conventional description exists for a band–support pair); `InBand` → a
typed refusal at the blend door (a new `BlendError` arm or the existing
`Escalated` with the predicate `tangent_second_order` and a recourse naming
the radius as the lever — decide by what `BlendError`'s vocabulary already
carries and say why). No new predicate name; `tangent_second_order` stays
the one metered spelling; the K cost per contact edge (`CERT_SAMPLES − 2`
samples where it was 0) is stated.

**Ratified and not re-litigated:** the must-carry rule's contract (D4 ¶3,
`must_carry_over_edge`'s doc); the seam and transverse arms; the
certificate's stations.

## Phase 1 — measure before touching anything

Over every blend fixture in `crates/sweep/tests/**` that carves (the bit-dump
corpus is the floor), every contact edge storing `TangentIntersection`: the
lane verdict, the seven station readings, the minimum margin, and the closed
form's prediction from `(r_band, κ_support, convexity)` — one table. Then the
near-osculating family: a convex band on a convex cylinder support with
`R → r_band` (the rod's branch) at `R/r_band ∈ {2, 1.5, 1.1, 1.01, 1 + 10⁻⁴,
1 + 10⁻⁶}`, and a concave band in a concave support the same way; for each,
whether the blend arm's own gates refuse first (which predicate), and where
the second-order margin lands at the three ε rows. **The stop clause:** if
no blend the door admits reaches an in-band or under-determined verdict at
any ε row, stop at the report — the change would then have no observable
row, and the orchestrator decides whether a lane-gate-only change is worth
landing.

## Phase 2 — the change

1. `attach_contact`'s intrinsic arm calls `must_carry_over_edge`; the three
   verdicts map as decided above; the comment block at the site shrinks to
   the invariant (the rule decides, the site does not argue).
2. **Rows**, in `crates/sweep/tests/contact_edge_must_carry.rs` (aggregated;
   `test_support` fixtures): the near-osculating blend that reaches `InBand`
   refusing typed with the predicate and margin read off the error, at the
   ε row(s) where it lands in band; the same family definite on each side
   (`JetDeterminate` storing the intrinsic description; a lane-refused pair
   storing the conventional one or refusing typed); the corpus census as a
   row (every contact edge's verdict `JetDeterminate`, the count pinned);
   the K count per contact edge through the `Probe` scalar.
3. **The mutant**, in the PR body: restore the structural mint and show
   exactly the in-band and lane rows red.

## Constraints, binding

- **Every stored description in the tree is unchanged**: the bit-dump
  differential at both SHAs (the corpus reads `Positive` everywhere, so
  nothing moves; a moved description is a finding).
- **No new predicate name; `dihedral.rs` untouched.**
- **Comments state the invariant**; the measurement is the PR body's story.

## Acceptance

- The Phase 1 table and the near-osculating family with the stop clause
  answered; the rows; the mutant table; the differential clean; the K count
  stated; hosted CI green.

## Out of scope

The seam and transverse arms; the corner ball's own edges (if they take a
different path, say so and file); `tangent_certificate_lane`'s admitted set.

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** No stored description in the tree moved (re-run the differential).
- **C2** The near-osculating blend really reaches the in-band verdict
  through the public door (construct your own from the closed form; the
  row's `R/r_band` is the implementer's, not your oracle) and the typed
  refusal names the right predicate and lever.
- **C3** The three verdicts map to the right descriptions: a lane-refused
  band–support pair (construct one) stores what the certificate can certify
  or refuses typed, never the intrinsic tangency.
- **C4** The rule is called once per contact edge with the right extent and
  window (count K samples through the `Probe` scalar; compare with the PR).
- **C5** Nothing else in `sweep::blend` mints `TangentIntersection` outside
  the rule (sweep the pattern BLEND-9's fix pass added, and the chord-join
  site it named on another program's ground).
