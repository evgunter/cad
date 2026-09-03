# VERBS-1031B — the arc-bounded winding arm (merge_faces)

RATIFIED 2026-09-03 (orchestrator; from the opening measurement in
`work/verbs/VERBS-1031B.md` and Evan's option-(1) ruling, in-chat
2026-09-03: "my former decision rule still makes sense and you should
do (1)" — the repair op, NOT the producer change; option 2 (full
`revolve` minting off-axis planar walls as one face) is REJECTED by
that ruling and fenced below). Branch `verbs/1031b`. Difficulty
pre-logged: **M-**.

Anchors verified by reading on `mngr/kernel-verbs` at ratification
(main `f955ddc75`); the implementer re-measures every anchor at
dispatch — the corpus-first law binds.

## The measured defect (the unit's whole content)

`merge_coplanar_faces` on the teapot cup refuses
`MergedFaceRoleAmbiguous { face: FaceKey(4v1) }` — raised at
`crates/topo/src/merge_faces.rs:1556`, the `_ =>` arm of
`merged_outline_ring`'s positives match. The instrumented mechanism
(the 2026-09-03 measurement, one level deeper than issue #1031's own
record): **the kef/kemr surgery COMPLETES** — a ring is minted — and
the role pass then sees `outer winding None, ring winding None,
positives: []`, because `loop_winding` (`merge_faces.rs:1457`) bails
`Ok(None)` at the `all_lines` guard (`:1479-1488`) for any cycle with
a non-Line carrier, and the merged annulus's outline and ring are both
**circles**. The gap is exactly one arm: winding for arc-bounded
cycles.

The pair itself, for the record the fixture rows must carry: the
cup's shoulder is a LATITUDE annulus (plane normal (0,1,0)) — the
issue's "meridian plane" gloss was wrong — whose seam is two disjoint
collinear Line segments with all four endpoints at valence 4 and no
shared vertex, so `redundant_subdivision_vertex`
(`merge_faces.rs:1125`, half-A's valence-2 machinery) correctly never
fires on it. Three such full-valence pairs per side, plus two genuine
pole-split base caps (valence 2) that half-A's machinery already
serves once the structural-planar refusal stops refusing the whole
call.

## Opening measurement (before any code, in the PR body)

Run `merge_coplanar_faces` on the teapot cup fixture
(`verbs_teapot.rs`'s `teapot_pot` meridian, full revolve, then the
shell_open cup — replicate the fixture the 2026-09-03 measurement
used) at the unit's head. Expected:
`MergedFaceRoleAmbiguous { face: FaceKey(4v1) }` verbatim. Quote the
payload and confirm the raising site is still `merged_outline_ring`'s
`_ =>` arm. If the door moved, STOP and report — the spec re-cuts on
the measured door.

## The one item — the arc arm in `loop_winding`

**The machinery already exists in this tree and MUST be ported, not
re-derived from scratch**: `boolean::join::ring_run_ccw`
(`crates/topo/src/boolean/join.rs:1167`, the `run_term` closure at
`:1219-1249`) already computes arc-bounded winding for the ring lane —
chord Newell term for every half-edge, PLUS a per-conic **bulge**
term, PLUS arc-length perimeter metering — and decides on the SAME
named predicate `bool_ring_run_winding` with the same
`Margin::over_lever(normal·newell, perimeter)` shape.
`loop_winding`'s own doc (`merge_faces.rs:1433-1436`) already says the
predicate "must state it identically at all three of its sites" and
points at `ring_run_ccw` for the derivation. This unit makes
`loop_winding` the fourth site stating it identically.

1. **The decomposition, derived (verify, don't invent).** For a
   closed planar cycle whose edges are chords and circular arcs, the
   enclosed vector area decomposes exactly per-edge:

       2·A⃗ = Σ_edges (p_prev − p₀) × (p − p₀)   [the chord Newell sum]
            + Σ_conic-edges  axis · sa·sb · (Δ − sin Δ)   [the bulge]

   For a circular arc of radius R spanning signed angle Δ (signed by
   traversal — positive when the half-edge runs with increasing carrier
   parameter, i.e. `he == edge.he_plus` gives `Δ = t1 − t0`, else
   `t0 − t1`; see `join.rs:1245`), the area between the arc and its
   chord is `R²(Δ − sin Δ)/2` — so twice that, matching the cross
   sum's 2A convention, is `R²(Δ − sin Δ)`, an odd function of Δ
   (traversal-signed, as required). The ellipse generalizes with
   `sa·sb = major·minor` (the affine image of the circle scales areas
   by `major·minor/R²`). The chord term is unchanged for every edge —
   the bulge is a CORRECTION on top of the chord polygon, so the mixed
   Line+Circle cycle is **handled, exactly**, with no case split
   beyond the per-edge carrier match. State this derivation at the
   site (the reviewer must be able to see why the substitution is
   exact rather than plausible).

2. **The carrier match, verbatim from `run_term`**: `Circle → (axis,
   R, R)`, `Ellipse → (axis, major, minor)`, `Line → no bulge, chord
   only`, `Nurbs → the cycle stays undecidable`. On Nurbs the arm
   does NOT approximate: the `all_lines` guard narrows to
   "all carriers are Line, Circle, or Ellipse"; a cycle carrying a
   Nurbs edge still returns `Ok(None)` with the guard's comment
   updated to say what is now true (chord winding says nothing about a
   fitted carrier's region, and no closed form exists — the honest
   remainder, refused by the caller when roles hinge on it).

3. **The perimeter lever moves with the area** (the F4 metering
   story, identical at all sites): a conic edge contributes `|Δ|·sa`
   — the circle's exact arc length and the ellipse's upper bound
   (over-large P escalates, never decides — `join.rs:1246-1248`'s own
   sentence); a Line edge contributes its chord. The zero-perimeter
   poison behavior (`merge_faces.rs:1438-1448`) is unchanged.

4. **The Line-only path is bit-identical.** The existing chord loop's
   arithmetic and accumulation order must not move for all-Line
   cycles — the bulge term is structurally absent (the match returns
   the zero vector), not numerically zero-added in a reordered sum,
   OR the sum order is preserved exactly; whichever spelling is
   chosen, the byte-level claim is: every existing Line-only
   winding decision is bit-identical. A MUTATION must demonstrate the
   arm never fires on Line-only cycles: break the bulge term (e.g.
   `Δ − sin Δ → Δ + sin Δ`) and every pre-existing Line-only fixture
   stays green while the new arc rows go red (red-then-green both
   directions, reported in the PR).

5. **One predicate, one metering statement, four sites.** The decide
   stays `bool_ring_run_winding` with `Margin::over_lever(2A, P)`;
   the doc note at `merge_faces.rs:1433-1436` updates its site count
   and keeps pointing at `ring_run_ccw` as the derivation home. No
   new predicate name — the question ("is this cycle positively
   wound, metered by mean width") is unchanged; only the carrier
   coverage grows. If the implementer finds the sites' statements
   have drifted from each other, that is a finding to report, not to
   silently fix.

## Acceptance

- **The cup merges.** `merge_coplanar_faces` on the teapot cup
  SUCCEEDS: all three full-valence coplanar pairs per side merge
  (shoulders + cavity shoulders), the two genuine pole-split base
  caps merge (half-A's machinery, now reachable because the
  structural refusal no longer aborts the whole call), and the cup
  emerges with its shoulder annuli whole. Pinned with the census
  deltas (faces/vertices/edges before → after, the KERNEL-VERBS
  precedent shape), tier-3 validation green, and the **re-posed twin
  rule** (the same cup under a `transform_rigid` off every axis plane
  behaves identically).
- **The boolean after the merge is MEASURED and recorded, whatever it
  is.** The 2026-09-03 record notes this outcome is unmeasured
  (measuring it required the repair to exist). Run
  `subtract(merged_cup, box)` and record the payload verbatim — a
  refusal at a further door is a FINDING, not a failure; if it
  refuses, retype nothing, cite the door, and the row pins it as the
  honest boundary.
- **Differential**: `subtract(unmerged_cup, box)` still refuses
  `NonMaximalFaces` at `gate_maximal_faces`'s same-surface-key planar
  branch (`crates/topo/src/boolean/reduce.rs:584`) — the unmerged
  body's row does not move.
- **The winding rows themselves**: a direct unit row on a hand-built
  or fixture-derived arc-bounded annulus (outline circle + ring
  circle) asserting outer → Positive, ring → Negative about the
  outward normal; the mixed Line+Circle cycle asserted through the
  cup's own merged shoulder; a Nurbs-carrying cycle still `None`
  (the guard's honest remainder pinned).

## Register sync (rides in this unit)

`docs/KERNEL-VERBS.md`'s curved-boolean-breadth row (currently line
~67) carries two sentences this measurement settles; both sync here,
citing the 2026-09-03 measurement record:

- "the gate-admission question stays DEFERRED pending a re-measurement
  of the widened-gate sequence on the REPAIRED lantern" — the
  re-measurement was taken: on the repaired lantern the widened-gate
  sequence runs gate → F7 passes → `CurvedPierceUnsupported`
  (`reduce.rs:1099`, the shared curved-pierce substrate). The
  deferral CLOSES by citation; the lily class is the substrate's, not
  the coplanar pair's.
- "the cup seam's straightness was never measured (unverified —
  measurement pending)" — it is measured now: two disjoint collinear
  Line segments, all four endpoints valence 4, no shared vertex; NOT
  the pole shape, and the winding arm (this unit) is the whole gap.

## Fences

- **No revolve/producer changes** — option 2 was rejected by Evan's
  2026-09-03 ruling; the two-π-band convention and every
  half-wall-counting fixture stay untouched.
- **No operand-gate widening** (`boolean_arm_exists` and
  `gate_maximal_faces` do not move).
- **`merge_faces.rs` and its tests only**, plus the register sync and
  the acceptance fixtures' own files.
- The join lane's `ring_run_ccw` is READ, not touched — if porting
  reveals a defect there, that is a finding for adjudication, not a
  drive-by fix.
- `redundant_subdivision_vertex` (half-A) does not move.

## STOP (pre-registered)

**If the role decision needs more than the winding arm** — the
positives come back and `normalize_merged_roles` (or anything after
it) still cannot complete the cup's merge, i.e. a second gap sits
behind the first — **STOP and report** with the payload and raising
site. The refusal-text lesson binds: this spec's premise is the
instrumented mechanism (windings None → positives empty), and if the
measurement at the unit's head shows a different binding constraint,
the spec re-cuts rather than the implementer improvising.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). Suites via hosted CI at STEP level — no local
full-suite runs; targeted suites, probes, and the mutation
demonstrations only. Opening measurement before code, in the PR body,
payloads verbatim. Deviations declared with schedules. The tracker
item is `work/verbs/VERBS-1031B.md` — status moves with the unit
(`scripts/work.py lint` before any push touching `work/`).
