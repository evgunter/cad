# FILLET-H4 — the material-adding closed-rim band (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`concave-closed-rim-has-no-band` (`work/fillet/concave-closed-rim-has-no-band.md`).
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **L**, task-class
**NUMERIC**.

- **L** — the plan's own grade. The code change may be small (§Phase 1 decides
  how small), but the unit is paid for by what it ESTABLISHES: that a walk
  written and pinned only on material-removing rims is side-blind, shown by
  rows that can see a fold and by a differential that shows nothing convex
  moved.
- **NUMERIC** — a closed-form volume on the material-adding side is the
  acceptance oracle, and any fold Phase 1 finds hardcoded convex is a sign
  decision. Mixed at worst; the ambiguous case records `numeric`.

## The claim

**A concave closed rim has a band, and it is the same band.** The blend of a
closed rim is a torus about the rim's own spine on either material side; on a
convex rim the surgery replaces the two support strips between the rim and the
trimlines with the band and REMOVES material, on a concave rim the identical
replacement ADDS it. The two rim carves — the quad LADDER and the ANNULUS,
`crates/sweep/src/blend/surgery.rs` (`rim_phase` at `:2520`,
`rim_phase_annulus` at `:3026`) — are walks over the body's topology: struts
or seam splits to the feet, a trimline `mef` per support, a `kef` per rim arc,
the crossing merges. Nothing in that walk is a material-side fact, and the one
sense the band carries is already a fold of the stored verdict:
`set_face_sense(fk, rim.chain.first().convexity.blend_sense())` (`:602`;
`Convexity::blend_sense`, `battery.rs:81`). What keeps the concave rim out is
one gate, `resolve_rim` (`:855`–`:860`):

> a concave chain adds material, which no closed-rim carve builds — not
> implemented

with its premise restated at `:833`–`:834` and in the recourse sentences.
The unit removes the gate and shows the premise was a gate and not a fact.

The open-chain half of this is DONE and is the precedent: BLEND-3 (concave
chamfer) and BLEND-4 (convexity-parametric fillet corner) made the open band
and the corner patch fold the stored verdict, and BLEND-3's central finding
binds here — *a band sense that folds convexity is INVISIBLE to every convex
fixture and red only on concave carves*. Every row this unit adds is designed
against that finding.

**Ratified and not re-litigated:** `crates/sweep/README.md` BLEND-VOCAB V1–V4
and ARMS3 A3-1…A3-3; the arm table (`arms.rs:192` `is_coaxial_torus`); the
`SeamVertex` incidence rule (A3-2); the rolling-ball sense convention
(`arms.rs` module docs: the ball rolls in the void on a concave chain,
material outside the tube, sense `false`, read off the stored verdict —
S10/S11).

## Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md`: a gate's sentence is not evidence
about what lies behind it.

Locally and uncommitted, delete the convexity arm of `resolve_rim`'s loop and
run the concave fixtures through `fillet_edges`:

- **the waist** — `revolved_about_y` (`test_support.rs:107`) of
  `(0,0)→(1,0)→(0.5,0.5)→(1,1)→(0,1)`, full revolution; the concave waist rim
  at `rim_arcs_at(&body, 0.5, 0.5)` (`:211`), two arcs (seam-split → the
  ANNULUS door). It is the fixture of
  `blend_seam_split_rim.rs::a_concave_seam_split_rim_still_refuses` (`:539`).
- **the lily lantern's mouth rim** (~0.253) —
  `demos/tour/tests/blend1_r1_wall6_probes.rs:142`.
- **a ladder twin** — the die's pips are `slab ∖ ball` (`m5_pr12_die.rs`) and
  route to the LADDER (a plane face with the rim as a RING, two half-caps);
  the concave twin is `slab ∪ ball`, a boss. Build it through the public
  boolean door and record which rim door it reaches.

Record in the PR body, per fixture: whether the carve builds; the
`validate_geometric` verdict; the census before/after; the volume against
the closed form (below) with `volume_pad`; and every refusal met on the way
with its site. **This table is the unit's first deliverable and decides its
shape.**

**Stop clause.** If a concave pair's `EdgeBlend` is itself wrong — a trimline
circle not on its support, a torus not tangent to both — the defect is in the
arms (`arms.rs` `sheet_center` `:549`, `Meridian::trace` `:676`/`:808`),
outside this item's statement: stop at the report, file it
(`CLAUDE.md` §*Filing an issue*), and the orchestrator re-scopes. A failure
in the SURGERY (a fold hardcoded convex, a wrong strip chosen, a tier-3 red on
the result) is this unit's substance, not a stop.

## Phase 2 — the change

1. **The gate goes**, in `resolve_rim` (`:855`–`:860`), and its premise with
   it (`:833`–`:834`). Both doors, ladder and annulus, take either material
   side. No new door, no new variant.
2. **Every side-dependent read in the two rim phases folds a STORED fact** —
   the chain's `Convexity` verdict, a face's stored sense bit, the half-edge
   traversal — never a sampled normal (S10/S11). Phase 1 names the ones that
   do not; fix each at its site, one fold, cross-cited where the same sign is
   spelled twice (the `corner_plan` precedent, `:718`–`:722`).
3. **Rows, in `crates/sweep/tests/` (a new suite; follow `tests/all.rs`'s
   aggregation), reusing `test_support` fixtures — no fixture copies
   (`work/tcost/blend-suite-fixture-and-oracle-copies.md`):**
   - **The waist carves**: one annulus band, `validate_geometric` clean, the
     census delta stated, and **the volume equals the source's plus the fill,
     by Pappus**: the meridian fill region is the curvilinear triangle between
     the two generators from the feet to the waist vertex and the fillet arc;
     with the 90° void wedge its area is `r²(1 − π/4)`, and
     `ΔV = 2π · ∫_fill x dA` — derive the centroid in the row's doc, assert
     against the derivation, and assert `volume_pad == 0.0` exactly (the
     revolve's iso-rectangle argument, `verbs_arms1_annulus.rs:359`). The sign
     is the point: `V₁ > V₀`.
   - **The convex twin of the same body** (the base or top rim) carves with
     `V₁ < V₀` by its cut, in the same row family, so the two signs sit side
     by side.
   - **The ladder twin** (`slab ∪ ball`): the concave ladder carve, tier-3
     valid, its spherical-cap fill closed form — OR, if Phase 1 shows the
     union body does not reach the ladder door, a row pinning the door it
     does reach with the reason, and the ladder's concave arm recorded as
     reachable-by-shape with no constructor, as an issue.
   - **The rows can see the fold**: name ONE mutant (the band sense read as
     `true`, or the fold Phase 1 fixed put back) and show in the PR body that
     it reds exactly the concave rows and none of the convex ones. A row that
     stays green under the mutant is not evidence.
   - **Naming totality on a concave band**: every output entity a recorded
     mint or a survivor, every retirement a source key
     (`blend_seam_split_rim.rs::a_seam_split_band_records_every_birth_and_every_death`'s
     shape, on the waist).
   - **Interval**: the waist row runs at `Interval` too (a
     `#![cfg(feature = "interval")]` twin in the aggregated binary, the
     `bool2_cone_doors_interval.rs` pattern); ask for the lane with
     `CI-Config: lane=interval` on one head and say so in the PR.
4. **The pins that asserted the gate flip**, each at its own site, keeping its
   claim by name (`memories/output-stability-as-justification.md`):
   `blend_seam_split_rim.rs:529`–`:562` (the gate row becomes the carve row
   or is retired into the new suite — say which),
   `review_blend1_r2_probes.rs:586`–`:660` (the composed seam-vertex pin's
   concave arm now asserts the CARVE: one annulus, tier-3 valid, `V₁ > V₀`),
   `demos/tour/tests/blend1_r1_wall6_probes.rs:9`, `:142`–`:170` (the mouth
   rim carves; the lily's fourth transverse rim is a frame change — decide
   whether the tour SHOULD fillet it, re-baseline with the reason if so,
   `docs/prompts/implementer-discipline.md` §3; never adjust the scene to
   keep the frame).
5. **The sentences become unconditional**, present tense only:
   `FILLET3_SEAM_VERTEX_RECOURSE` (`blend/mod.rs:435`–`:450`: the carve half
   loses its hedge and the doc paragraph shrinks to the rule it keeps — a
   recourse is true at every site its tag fires), `FILLET3_ASSEMBLY_RECOURSE`
   (`:464`: the concave-band clause goes), `resolve_rim`'s doc, README A3-1
   (`:99`, the sentence and its stale `work/issues/` pointer go) and A3-2's
   "on a concave rim … refusal" clause, `docs/KERNEL-VERBS.md:59` clause (i).
   Sweep by SENTENCE, not by the issue number: `rg -n 'concave' crates docs
   demos --type rust --type md` over blend prose, plus `1244`; hit list and
   disposition in the PR body, blind spot stated (§5 of the discipline).
6. **Rider `D322`** (`work/code-quality/D322.md`): `pub fn ring_clearance`
   (`surgery.rs:1759`) goes behind the crate's `test-support` feature via
   `test_support.rs`, its one outside caller `tests/m6_surgery.rs:447`
   follows; a visibility change and a re-export, nothing else. Close the
   row's header on this branch (`status: closed`, `pr`, `closed`).

## Constraints, binding

- **Every convex carve is bit-identical to the merge base.** Run
  `crates/sweep/tests/bitdump.rs` at the merge base and at the head
  (`BITDUMP_DIR` armed) and diff; extend the dump with the dome
  (`test_support::dome`), the `sphere_zone` rim pair and the lantern's three
  convex rims if it lacks them. Any moved bit on a convex fixture is a
  finding, not a re-baseline.
- **No new metered predicate**, unless Phase 1 shows one is needed; then it is
  `fillet3_*`-named, band-metered and trio-pinned like its siblings
  (two-tolerance, D4 ¶1).
- **No sampled normal decides anything** — S10/S11. `outward(..)` at a sample
  point may DESCRIBE (a refusal payload's margin); it may not choose a side.
- **Nothing about the open-chain band, the corner patch, the chamfer or the
  battery's predicates changes.**
- **Comments state the invariant** (discipline §4): the surgery's docs say what
  a closed-rim carve IS on either side; the history of the gate is this PR's
  body.

## Acceptance

- The Phase 1 table, from the merge base.
- The waist row and its convex twin green, with the Pappus derivation in the
  row; the ladder twin or its measured stop; the naming row; the interval
  twin, hosted at the requested lane and said so.
- The mutant table in the PR body: concave rows red, convex rows green.
- The bit-dump differential clean over every convex fixture, said so with the
  two SHAs.
- Every gate pin flipped or retired by name; every sentence in §5 swept, hit
  list in the PR body.
- `D322` closed on the branch.
- Hosted CI green at the drawn point plus the asked-for interval lane; the PR
  says which and whether drawn or asked.

## Out of scope

`repaired-pole-rim-serves-no-closed-door` (one host face carrying several
arcs of one circle — H5, next); the ruled-spine arms (H7); mid-curve run-outs
(A3-3); the rim selector (`no-public-rim-arc-selector`, on Ev); `D325`
(the corner fusion's `first_arc`, not opened here); any change to how the
arms compute a concave pair's torus (Phase 1's stop clause routes that out).

## Review

v6 dual on the frozen head, claims to falsify (the reviewers get these
verbatim plus `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every convex closed-rim carve is bit-identical to the merge base
  (the dump differential, re-run by the reviewer).
- **C2** The waist carves to a tier-3-valid solid whose volume is `V₀ + ΔV`
  with `ΔV` the Pappus fill, `volume_pad == 0.0` (re-derive the closed form
  independently; the row's derivation is the implementer's, not the oracle).
- **C3** Every side-dependent read in both rim phases is a fold of a stored
  verdict or sense bit; the PR's audit table of those reads is complete
  (grep the phases for anything that samples).
- **C4** The named mutant reds exactly the concave rows and nothing convex.
- **C5** No sentence anywhere in `crates/`, `docs/`, `demos/` still states
  the concave closed band as unbuilt; the sweep's blind spot is stated.
- **C6** The lantern's mouth rim carves through the tour's own door, and if a
  frame moved the PR says what moved and why.

## Re-scope at Phase 1 (2026-09-04, orchestrator)

**Phase 1's finding.** With the gate deleted locally, every concave closed
rim — the waist (ANNULUS), the lily mouth (ANNULUS) and a `cube ∪ ball`
boss (LADDER, which the boolean door does build) — reaches its rim door and
refuses at `seam_split_param` (`surgery.rs:2516`), the surgery correctly
declining to cut a seam at a foot that lies beyond the rim. The cause is
in the ARMS, measured at the arm: the curved arms fold each support's
stored sense bit and never the chain's convexity verdict
(`battery.rs:959`/`:974` into `Meridian::trace`/`Ruling::trace`;
`plane_sphere_blend` at `arms.rs:381`–`:394` from `battery.rs:869`/`:878`),
so on a concave chain each returns the convex rest MIRRORED through the
rim — the waist's spine `0.5 − r√2` against the void-side `0.5 + r√2` to
the last digit. Only `plane_plane_blend` (`:287`) and `corner_ball`
(`:920`) fold `convex` — BLEND-4's work, which is why the open concave band
carves. With the fold applied locally and the surgery UNCHANGED, all three
carve tier-3 clean with `volume_pad == 0.0` and `ΔV` positive and equal to
its closed form (the waist: Pappus `1.7387214704551e-3` against measured
`…556e-3`). Full record:
`work/fillet/concave-rim-arms-rest-ball-on-material-side.md`.

**The stop clause is discharged and the arm fold JOINS this unit.** The
arms are `crates/sweep/src/blend/*`, this program's ground; the fold is the
convention `arms.rs`'s module docs already state (the ball rolls in the void
on a concave chain, material outside the tube, sense read off the stored
verdict — S10/S11); and the plane–plane arm is its precedent. Nothing here
is a design fork. Difficulty and task-class stand (L / NUMERIC), re-logged
at the redirect in the block record.

**Phase 2, item 0 (before item 1):** the arm fold. `curved_arm` hands
`Meridian::trace` and `Ruling::trace` the side `sense == convex` (the
identity on a convex chain); `plane_sphere_blend` takes `convex` and folds
it the way `plane_plane_blend` folds `signed` — spine at `depth + signed`,
the offset sphere `R − r` where the sphere's sense agrees with the chain's
convexity and `R + r` where it does not, the plane trim at
`spine_center + n·signed`. ONE fold, cross-cited with `plane_plane_blend`'s
`signed` and `corner_ball`'s, the `corner_plan` precedent (`:718`–`:722`).
The `Ruling` site is folded for parity and its unreachability today stated
at the site (ruled pairs refuse at the open-chain door), not rowed. The
gate in `resolve_rim` retires in the SAME change — deleting it alone turns a
typed refusal into the span refusal.

**Acceptance, amended:** C1's convex differential is now load-bearing on
code that MOVED — it is measured with the dump, never argued by
construction, and the dump covers every convex arm family a fixture
reaches: plane–sphere (the dome, the pip), cone–cone and sphere–cone (the
waist's convex rims, the lantern's three), plus the open-chain die. The
boss is the ladder twin (item 3's "or" arm is not taken: the union builds
and routes to the LADDER). The mutant is the fold itself put back to
`sense`. The issue file above closes with this unit (`status: closed` on
the branch at landing, `pr` set). PR 1752 is the unit's PR; it takes the
brief's title when the unit lands.
