# BLEND-8 — the must-carry rule's in-band policy has one home (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 8). **Item:**
`work/blend/smooth-arm-siblings-disagree-on-the-in-band-case.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one wrapper and two callers, but the wrapper changes what one verb
  does on in-band geometry and touches a file no program's `paths` reach
  (announced to PROPS at dispatch, `work/props/log.md`); the rows that make
  the change visible have to construct near-osculating joins on purpose.
- **STRUCTURAL** — the policy is already ratified; the unit gives it one
  spelling. The one numeric question (how many stations) is answered by an
  argument stated in this spec, not by a measurement the lane makes.

## The decision, taken by the orchestrator (2026-09-07, logged)

`geom_brep::tangent_second_order` (`crates/geom-brep/src/dihedral.rs:252`) is
the must-carry rule's one metered spelling and its doc states the contract:
**Positive → jet-determinate, the intrinsic `TangentIntersection`; Zero or
Negative → under-determined, the conventional description BY THIS PREDICATE;
Err → in-band, escalated TYPED at the caller (D4 ¶3).** The two callers keep
their own policy and disagree:

- `crates/sweep/src/extrude.rs` (~`:954`, the strut arm) follows the contract:
  in-band escalates `ExtrudeError::SliverJoin`; it reads ONE point (the
  midpoint) and applies no lane gate.
- `crates/sweep/src/revolve/upgrade.rs::jet_determinate` (~`:180`) folds `Err`
  into `false` and KEEPS the conventional description — a silent in-band
  build; it gates on `tangent_certificate_lane` and reads the certification
  schedule's interior stations (`CERT_SAMPLES`).

**The rule's policy is the predicate's documented one** — in-band escalates
typed. Revolve's fold is code that drifted from a contract meant to hold
(`docs/prompts/reviewer-style-lane.md` Q4's second case), and D4 ¶3 says an
in-band verdict is never silently either side. This is a behaviour change for
revolve: a join whose second-order margin lands in `(ε, K·ε)` refuses
`RevolveError::SliverJoin` where it built with a conventional description
before. That is the fail-loud answer and it is not a design fork: DUAL/D4 and
the predicate's own doc already say it.

**The stations are the certification schedule's** — revolve's choice. The
constructor stores what tier 3 will demand, and tier 3 re-asks the question
at `CERT_SAMPLES` stations, so reading the same stations is what keeps "the
demanded set and the stored set one set" (the doc's own argument for one
home). The symmetry arguments (κ_rel constant along a ruling; constant along a
latitude circle of a coaxial pair) are true and are exactly the kind of
argument-not-spelling the item names; the wrapper may state them as the reason
the extra samples never disagree, it may not use them to read fewer. The
K-stream cost for a smooth strut rises from one sample to `CERT_SAMPLES − 2`;
the PR states the count and, if the k-lint gate fires on the distribution,
re-derives per the K-REPORT runbook (discipline §3) — never by reading fewer.

**The lane gate is part of the rule.** `tangent_certificate_lane`
(`crates/geom-brep/src/tangent.rs:164`) says whether the certificate can
certify this carrier over this pair at all; a pair outside the lane cannot
store `TangentIntersection` whatever the jet says, so the wrapper gates first
and answers "conventional" for an out-of-lane pair without metering — the
extrude strut inherits the gate (today it has only the argument that its
pairs are in the lane; the gate makes it a check).

## The change

1. **One wrapper beside `tangent_second_order` in `dihedral.rs`** — the
   must-carry rule over an EDGE, not a point: takes the two surfaces, the
   carrier and its window, the extent and the band; gates on the lane;
   reads the schedule's interior stations through `sample_param`; returns a
   three-way typed answer (jet-determinate / under-determined /
   in-band-with-the-first-escalation) with the first station's `SecondOrder`
   beside it, since both callers need the numbers again. Name it for what
   it decides; doc states the contract above and the two symmetry facts as
   why the stations agree, and cites this as the one home the
   `folded_lever_arm` doc calls aspirational.
2. **Both callers call it and nothing else** decides a smooth join's
   description: extrude's strut arm maps in-band to `SliverJoin` as today;
   revolve's `upgrade_intersection` maps in-band to `RevolveError::SliverJoin`
   through the `sliver` closure it already takes, and `jet_determinate` goes.
3. **Rows**, in `crates/sweep/tests/blend8_must_carry.rs` (aggregated;
   `test_support` fixtures): a revolve with a line–arc profile join whose
   relative curvature is chosen so the sagitta margin `|κ_rel|·arm²/2` lands
   in the band — derive the arc radius from the band in the row's doc — and
   assert `SliverJoin` typed; the same join with the margin definite on
   each side (Positive → `TangentIntersection` stored; Zero → conventional
   stored) as the trio; the extrude twin of the trio on a strut; an
   out-of-lane pair on both verbs storing the conventional description with
   no K sample emitted (count the samples through the `Probe` scalar or the
   k-stats bracket). Every existing revolve and extrude fixture's stored
   descriptions unchanged (below).
4. **The mutant**, in the PR body: fold in-band to conventional in the
   wrapper and show exactly the two in-band rows red.
5. **The doc trail**: `folded_lever_arm`'s doc (`dihedral.rs` ~`:200`) still
   names issue 1439's siblings — restate the present count (the tier-3
   validator's and the boolean rebuild's remain; say so and no more);
   `extrude.rs`'s module doc sentence on the must-carry (`:35`) and
   `upgrade.rs`'s (`:87`) point at the wrapper; `docs/FILLET-H6-SPEC.md` is
   gone (ledger) so nothing to fix there.

## Constraints, binding

- **Every stored description in the tree is unchanged**: dump every edge's
  description kind for every revolve and extrude fixture in the sweep suites
  at the merge base and the head (a small `#[ignore]`d dumper in the new
  suite, or `bitdump.rs` if it carries descriptions) and diff. A moved
  description is a finding — either the fixture was in-band and now refuses
  (then the row says so and the change is the unit's point) or the wrapper
  is wrong.
- **No new predicate name; `tangent_second_order` stays the one metered
  spelling.** The wrapper composes it; it does not re-meter.
- **`dihedral.rs` gains the wrapper and nothing else changes there**; the
  file is PROPS-adjacent and the seam is announced.
- **Comments state the invariant**; the drift is the PR body's story.

## Acceptance

- The description differential clean on every fixture, both SHAs stated, or
  each moved description explained as an in-band join now refusing.
- The trio rows on both verbs, the out-of-lane rows, the mutant table.
- The K-stream count per smooth strut before and after, stated; k-lint
  green or re-derived per the runbook with the reason.
- Hosted CI green (full matrix).

## Out of scope

The two remaining hand-rolled siblings (`topo::validate`'s and
`topo::boolean::ops`'s — issue 1439's work, other programs' ground); any
change to `tangent_certificate_lane`'s admitted set; `classify_dihedral`.

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** No stored description in any existing revolve or extrude fixture
  moved (re-run the differential).
- **C2** An in-band smooth join refuses typed on BOTH verbs, and a definite
  one on either side stores what the contract says (construct your own
  near-osculating join from the band; the row's radius derivation is the
  implementer's, not the oracle).
- **C3** The wrapper reads exactly the certification schedule's interior
  stations and nothing decides a smooth join's description outside it (grep
  `tangent_second_order` callers; the count of K samples per strut matches
  the PR's claim).
- **C4** The lane gate is applied before any metering on both verbs, and an
  out-of-lane pair emits no sample.
- **C5** The symmetry claims in the wrapper's doc are true for every pair the
  lane admits (find a pair in the lane where κ_rel varies along the carrier).
