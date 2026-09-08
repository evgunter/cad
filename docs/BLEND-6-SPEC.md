# BLEND-6 — ring clearance takes the containment form, and the hostless annulus gets the same closed-form clearance (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 6). **Items, one unit:**
`work/blend/ring-clearance-refuses-a-nested-trim-circle.md` and
`work/blend/hostless-rim-on-a-ringed-host-refuses.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**NUMERIC**.

- **M** — two gates move, one arm of one closed-form pass gains a second form
  and a second caller, and the fixtures already exist in the tree; what the
  unit is paid for is the ARGUMENT that says which form applies where, and
  the rows that would go red if the wrong form were chosen.
- **NUMERIC** — the deliverable is a signed clearance margin in meters, metered
  under `fillet3_ring_clearance`, and two closed-form volumes as the oracles.

## The claim

**A trim circle clears a circular boundary when the two circles do not cross,
and which side of "not crossing" is admissible is fixed by what the trim circle
REPLACES.** `ring_clearance_pass` (`crates/sweep/src/blend/surgery.rs`, ~`:1947`)
meters every ring of every touched support face against every blend trimline
in closed form, before any mutation. Its circle arms all spell one form, the
external separation `‖cj − ci‖ − si − aj`, and that form is right for exactly
one relation: a ring the excised strip must not reach. It is wrong for two:

1. **The LADDER rim's host OUTER boundary when that boundary is a circle**
   (arm (b), the `Curve3::Circle` branch of the outer-cycle walk, ~`:2113`).
   The host face lies INSIDE its outer boundary, so a widened trim circle that
   is nested inside a circular boundary with room to spare is the healthy
   case, and the external form reads `−(si + aj)` on it — the witness is the
   repaired boss's dome rim (the first item, refusing at `−1.59`).
2. **The hostless ANNULUS rim's host RINGS** (`resolve_rim`'s `HostSide::Struts`
   arm, "Frontier arm 1", ~`:1198`–`:1212`). Nothing meters them today; the
   arm refuses any ring at all. The band's host trim becomes the face's new
   outer boundary, so a ring is admissible exactly when it lies INSIDE the
   trim circle with margin — the containment form `si − (‖cj − ci‖ + aj)` —
   and a ring outside it sits in the strip the carve excises, which the
   external form would wrongly admit.

So there are two closed-form margins over the same pair of circles, and the
rule for choosing is not "take the better one":

- **Ring vs a trimline whose strip lies OUTSIDE the ring** (every existing
  arm): external separation, unchanged.
- **Trim circle vs a circular OUTER boundary of its own host** (case 1): the
  boundary must contain the trim circle — margin `aj − (‖cj − ci‖ + si)` —
  OR, for a circle edge that is only a convex arc of a mixed outer cycle
  (a rounded-rectangle corner: the arc's full circle does not enclose the
  face), external separation. The admissible margin is the MAX of the two;
  a boundary circle nested inside the trim circle (`‖cj − ci‖ + aj < si`) is
  the carve consuming its own host and is refused by both terms being
  negative, which is the answer wanted.
- **Host ring vs the hostless annulus's host trim** (case 2): containment of
  the ring in the trim circle, `si − (‖cj − ci‖ + aj)`, and nothing else.

One function spells the pair — the two margins from two circles, named for
what they mean — and the three callers say which relation they meter. The
predicate name stays `fillet3_ring_clearance` (a K-corpus family; V3's
fence), the band is the run's linear band, and every margin is in meters.

**Ratified and not re-litigated:** `crates/sweep/README.md` A3-2 (the hostless
annulus, what `resolve_rim` routes on, the two host conditions it states — the
ring-free one is what this unit REPLACES with a metered clearance, and the
sentence follows the code), BLEND-VOCAB V1–V4; predicate 2's sampled screen
and its exactness step-down for the annulus rim (`surgery.rs` ~`:2050`–`:2068`)
stay as they are — this unit adds an exact backstop where the item found one
missing, it does not touch the screen.

## Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md`: measure, then change.

Fixtures, all in the tree: `review_fillet_h5_r1_probes.rs::boss` and
`fillet_h5_hostless_rim.rs::boss(up)` (the same revolve, `(0,0) (1,0) (1,1)
(0.5,1)[bulge tan(π/8)] (0,1.5)` about y, `merge_coplanar_faces` after; the
dimple twin has bulge `−tan(π/8)`). Home ONE spelling of the boss and its
dimple in `crates/sweep/src/test_support.rs` (the crate's own S52 lesson; the
copies in the two probe files then call it — those files are S-TCOST's glob
and the edit is announced by this spec).

Locally and uncommitted, with the two gates OFF (the external-form circle arm
short-circuited to pass; the `rings.is_empty()` refusal removed), run through
`fillet_edges`, radius 0.1, and record per fixture: whether the carve builds;
the `validate_geometric` verdict; the census before/after; the volume against
the closed form with `volume_pad`; every refusal met on the way with its site.

- **boss, dome rim** `rim_arcs_at(&body, 0.5, 1.0)` — LADDER (the rim is a
  ring of the flat top), convex: the plane–sphere cut, `test_support::plane_sphere_cut`.
- **boss, top outer rim** `rim_arcs_at(&body, 1.0, 1.0)` — hostless ANNULUS on
  a host that carries the dome rim as a ring, convex: the plane–cylinder cut,
  by Pappus over the fill triangle-minus-quadrant (`test_support::pappus`).
- **dimple, both rims** — the same two doors on the concave side (the flat
  top's ring is now the dimple rim), `V₁ > V₀`.
- **boss, BOTH rims in one call** — the ladder's trim widens the ring the
  annulus must clear (`effective`), and the two margins are then metered
  against each other's widened circles: record what each meter reads.

**This table is the unit's first deliverable and decides its shape.**

**Stop clause.** If a carve with the gates off produces a body that is
tier-3 INVALID for a reason other than the clearance being genuinely
violated (a ring left dangling on the wrong face, a strut through a ring, a
host face losing its rings across the annulus surgery), the defect is in
the surgery walk, not in the gate: stop at the report, file it, and the
orchestrator re-scopes. A refusal from a gate this unit does not own
(the half-band gate, `wall_seam`, predicate 2) on one of the four rows is
a finding to report, not a gate to widen.

## Phase 2 — the change

1. **One home for the two margins**: a function beside `ring_clearance` in
   `surgery.rs` taking two circles `(ci, si)`, `(cj, aj)` and returning the
   external and the containment margins, each named; doc states the rule
   above (which relation each caller meters and why). No new predicate.
2. **Arm (b)'s outer-boundary circle branch** meters `max(external,
   containment-of-trim-in-boundary)`. The line branch is untouched. The doc
   block above it (`:2075`–`:2095`) shrinks to the present rule: class (1)
   is no longer a false-refusal class; class (2) — a distant line edge whose
   EXTENSION passes near the trim circle — stays stated as unmeasured.
3. **`resolve_rim`'s hostless arm** replaces the `rings.is_empty()` refusal
   with a metered pass: every ring of the host against the host trim circle
   under the containment form, `fillet3_ring_clearance`, refusing
   `RingClearance` typed with the margin as the ladder does. Put it where the
   ladder's meter runs — the pre-mutation pass — not inline in routing: the
   routing arm asserts, the pass meters (the module's own split). The
   "Frontier arm 1" paragraph goes.
4. **Rows**, in one new suite `crates/sweep/tests/blend6_ring_clearance.rs`
   (follow `tests/all.rs`'s aggregation; fixtures from `test_support`, no
   copies): the four carves above, each tier-3 valid with its closed-form
   volume and `volume_pad` asserted; the both-rims-in-one-call row; a
   REFUSING row per form — a ladder rim whose widened trim circle CROSSES a
   circular outer boundary (a boss whose flat top is narrow enough that
   `0.5 + setback > 1 − (something)`; derive the radius that lands the
   margin definitely negative and state it), and a hostless annulus whose
   host ring lies in the strip (the boss with a dome radius close enough to
   1 that `si < ‖cj − ci‖ + aj`) — each asserting `RingClearance` with a
   negative reading; the two-tolerance trio for the new caller
   (definite / exactly zero / in-band) in `m5_pr12_refusals.rs`'s family, so
   the in-band case escalates typed with the ring recourse.
5. **The pins that asserted the refusals flip**, each keeping its claim by
   name: `review_fillet_h5_r1_probes.rs::r1_a_hostless_rim_on_a_ringed_host_refuses_under_a_recourse_that_promises_it`
   becomes the carve row (or is retired into the new suite — say which);
   any row in `fillet_h5_hostless_rim.rs` or `fillet_h5_r2_probes.rs` that
   pins the ring-free condition likewise.
6. **The sentences follow the code**, present tense: `FILLET3_ASSEMBLY_RECOURSE`
   (`blend/mod.rs:710`–`:716`, "one ring-free face carrying every arc" → a
   face carrying every arc as its whole outer cycle, its rings clearing the
   band), `FILLET3_SEAM_VERTEX_RECOURSE` if it restates the condition,
   `HostSide`'s doc (`surgery.rs` ~`:380`–`:395`), `resolve_rim`'s doc
   (~`:1438`), README A3-2's two-conditions sentence. Sweep by SENTENCE:
   `rg -n -i 'ring-free|rings of its own|neither occurs' crates docs demos`;
   hit list and disposition in the PR body, blind spot stated.
7. **Rider — the stale `work/fillet/` pointers in `crates/sweep/src`**
   (that directory was deleted at DOC-LEDGER sweep 7): `surgery.rs:387,
   :392, :1204, :1438, :2088` → `work/blend/`; `surgery.rs:3792` →
   `work/props/tangent-parallel-certifier-passes-a-transverse-arc.md`;
   `build.rs:203` → `work/props/blend-size-gate-unmetered-under-epsilon.md`.
   Pointer edits only; the `crates/profile` ones are units 9–11's.

## Constraints, binding

- **Every carve that builds today is bit-identical to the merge base.** Run
  `crates/sweep/tests/bitdump.rs` at the merge base and at the head
  (`BITDUMP_DIR` armed) and diff; extend the dump with the boss's base rim
  (`fillet_h5_hostless_rim.rs`'s carve) if it lacks it. A moved bit on any
  fixture that carved before is a finding, not a re-baseline.
- **No new metered predicate; no sampled decision.** The two margins are
  closed forms over stored circles; `fillet3_ring_clearance` meters both.
- **The annulus's exactness step-down is not widened or narrowed**: the
  OUTER-boundary walk stays a LADDER-only question (the `let RimShape::Ladder`
  guard stays), because an annulus's trim circle replaces part of that
  boundary and predicate 2 meters the rest. What this unit adds for the
  annulus is the RING meter, which is a different question.
- **Comments state the invariant** (discipline §4); the history of the two
  gates is the PR body.

## Acceptance

- The Phase 1 table from the merge base, four carves with the gates off.
- The four carves green with closed-form volumes and `volume_pad` stated;
  the both-rims row; the two refusing rows with their derived radii; the trio.
- The bit-dump differential clean on every previously-carving fixture, with
  the two SHAs.
- Every sentence in §6 swept, hit list in the PR body; the rider's eight
  pointers moved.
- The boss and dimple homed once in `test_support`, the probe copies deleted.
- Hosted CI green (full matrix — twelve `test` jobs, five `k-lint`); if the
  k-lint gate fires on the new margins' distribution, re-derive per the
  K-REPORT runbook and say so — never silence it by changing geometry.

## Out of scope

`ladder-rim-phase-may-retire-a-new-split-key` (unit 8, the same file's rim
phase); the hostless arm's "Frontier arm 2" (an outer cycle wider than the
request — stays refused, its row stays); class (2) of the outer walk; any
change to predicate 2's screen; the curved single-host rim (closed as a
record).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every fixture that carved at the merge base carves bit-identically at
  the head (re-run the dump differential).
- **C2** The dome rim (ladder, nested trim circle) and the top outer rim
  (hostless annulus on a ringed host) carve to tier-3-valid solids at their
  closed-form volumes with `volume_pad == 0.0` — re-derive both closed forms
  independently; the rows' derivations are the implementer's, not the oracle.
- **C3** The form choice is right at every caller: construct a body for which
  `max(external, containment)` would admit a crossing if the wrong relation
  were metered (a boundary circle nested inside the trim circle; a ring in
  the strip) and show the head refuses it.
- **C4** The two refusing rows go red under a mutant that swaps the forms
  (containment where external is due, or vice versa), and the carving rows
  stay green under it only where the argument says they should.
- **C5** No sentence anywhere in `crates/`, `docs/`, `demos/` still states
  the ring-free condition or "neither occurs today"; the sweep's blind spot
  is stated; every `work/fillet/` pointer in `crates/sweep/src` is gone.
- **C6** The in-band case of the new caller escalates typed with the ring
  recourse (the trio), and the K-stream cost of the new meter is stated in
  the PR (samples per carve, before and after).
