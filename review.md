# Review of PR #3843, frozen head e284e7f11f

Lane `reach-dual3843-r1`. Wall clock 13:58 to 15:28 UTC, 2026-10-02. No glimpse: I read only the PR body (`get`) and the head's check runs. I read no comments, reviews or other `analysis/reach-dual/*` branch.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 4 · NOTE 4. The gate is sound everywhere I could reach it, and I found no wrong body. The weakness is the evidence: two of the three fences have no row that goes red when they are removed, and the load-bearing sphere tightening can be made unsound with every PR row still green.

## Exercise (DEMONSTRATED BY EXECUTION; sources in `probes/`)
- **Suites.** `topo` and `sweep` pass 3900/3900 at ε 1e-9, 1e-6 and 1e-12. CI on e284e7f1 is green: test, lint, mesh and corrupt-input. The brief's known 1e-12 red, `rigid_map_near_eps_plane_nurbs`, does not exist in this tree (NOTE N1).
- **Soundness sweep** (`r1_gate_soundness_sweep`). Fixtures:
  - capped cylinders at R = 5/4, the tangent hemisphere R = 1, a flat cap R = 3, and a bulging cap larger than a hemisphere (R = 5/4, centre at 1.75);
  - roundings with t = ¼, 0.4, 0.1;
  - the truncated ball.

  Each ran at scale ×1e-3, ×1 and ×1e3, posed and unposed, against 136 planes per case. The planes are 14 random tilts at the face's extent ±{1e-3, 1e-7, 5e-2}, plus a random one, plus 10 tilts through the cylinder only.

  The oracle never uses the kernel. It is analytic face samples (the face must not straddle any admitted plane), a midpoint integral of the column field clipped by the plane, the halves summing to the body within their pads, tiers 1–3, and `point_in_solid` against column membership.

  Results: at 1e-9, **189 admitted, 0 straddling, 0 wrong volumes, 0 PiS disagreements**. At 1e-6, 166 admitted, all clean. At 1e-12, every shape is clean (two runs: the first three caps, then 121 admitted across the rest).
- **Azimuth-windowed faces** (`r1_gate_soundness_wedges`). Partial revolutions by 1.0, 2.5 and 4.5 rad of the caps, the rounding and the truncated ball give lunes and partial tori. They are clean at all three ε.
- **Halves reused as operands** (`r1_resplit_halves`). The cap-carrying half was re-split at a tilted plane. The result is π·0.35 within 5e-10 of the closed form, and a plane through the cap still refuses.
- **Mutants** (`probes/mut.py`, run against the PR rows plus my probes):

  | mutant | what turns red |
  |---|---|
  | M1 gate always passes | flank row, my sweep, pad row |
  | M2 box = whole ball | the clear-cut row |
  | M3 pad = 0 | **nothing** |
  | M4 edge's face-box clearance dropped | loft row |
  | M5 edge gate always passes | **nothing** |
  | M6 zone collapsed to the rim level (unsound) | **only my sweep** |
  | M7 zone end at a pole wrong | clear-cut row (incidentally, from an inverted span) |
  | M8 spline same-side `continue` removed | loft row |
  | M9 any one-sided corner clears | flank row, my sweep, pad row |

## MINOR
1. **The pad has no row that can fail** (`topo/tests/split_gate_per_face.rs:110-132`; `classify.rs:180-184`). Demonstrated by execution: M3 leaves every row green. At 0.9−ε the corner margin is already in band, so the band refuses with or without the pad. The PR body calls this "a pad row"; it cannot see the pad.
2. **The edge gate has no row that can fail** (`classify.rs:78-86`). Demonstrated by execution: M5 leaves every row green.
   - Every spline edge in the corpus bounds an unarmed face that refuses first. Any straddling edge is refused anyway by the new `insert_crossings` arm.
   - The case only the gate covers is a spline or spiric edge between armed faces whose ends sit on one side while its belly crosses. With the gate gone, the `continue` at `classify.rs:669-675` would pass it uncut, silently. Nothing exercises that case.
3. **No PR row sees an unsound sphere zone** (`classify.rs:131-134`). Demonstrated by execution: M6 (the zone collapsed onto the rim latitude, which admits level planes through the cap) is red only in my sweep.
   - The cap refusal row is tilted (φ = 0.3), so it also meets the collapsed slab.
   - The flank rows use the truncated ball, whose zone runs pole to pole, so they cannot see it either.

   Claim 1's tightening is the load-bearing part of this PR, and its only red row is a reviewer's.
4. **The recourse text overpromises** (`splitting/mod.rs:364-372, 384-386`). Demonstrated by execution in `r1_rotated_fixture_liveness`.
   - "Recourse: move the split plane clear of that face" is not sufficient. The gate clears an axis-aligned box of a tilted slab, not the face.
   - The PR's own fixture, cut at its own mid-height plane with body and plane turned together about z, splits for θ ≤ 0.2. It refuses for θ = 0.3, 0.5 and 1.0, with the plane about 0.5 m from the cap. It splits again at π/2.
   - The posed half of my sweep admitted 0 of 2176 planes, including the cylinder-only cuts.
   - Separately, by inspection: the `Nurbs` arm's recourse is false for the corrupt "face with no surface" case (`classify.rs:46-50`). That case is refused before the plane is read, so no placement helps.

## NOTE
- N1 (execution). `rigid_map_near_eps_plane_nurbs` is absent from the tree, and `topo` is green at 1e-12. The brief's premise is stale.
- N2 (execution, pre-existing). A wedge of a **plain** cylinder (θ = 1, no sphere) split at φ = 0.1 escalates `props_quad_converged` at ε 1e-9, and tier 3 reports `VolumeUncomputable`. This is not this PR's. It is newly reachable with a cap attached.
- N3 (execution, unreproduced). Once at 1e-12, a 1 mm cap fixture's `revolve` escalated (pcurve `Certify` `Envelope`). The rerun built the same fixture. Possible nondeterminism in `revolve`; not this PR's.
- N4 (execution). Under M1, the topo row `an_unarmed_face_refuses_only_where_the_plane_may_meet_it` stays green. A later stage also raises `CurvedBooleanUnsupported` for the swapped-top cube, so that row does not pin the gate. The sweep flank row does (claim 4 holds there).

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q8)
- Q1 (`classify.rs:106-159`) — sure. `gate_face_reach` is a second spelling of a sphere box rule that `boxes::FaceBoxRule` owns. It is disclosed and scheduled (`work/boxes/sphere-operand-box-is-the-whole-ball.md`, open, P1). It derives the zone from `solid_contain`'s latitude fold, while the scheduled row proposes the `Harmonic`-image chart window, so the two may not agree when folded.
- Q1 (`classify.rs:142-155`) — likely. The `SpanBox` is rebuilt from a point pair by hand, the inverse of `census::span_pts`.
- Q2 (`classify.rs:665-668`) — likely. The comment carries a cross-module invariant ("the gate admits … only behind a box") that nothing enforces locally, and M5 shows nothing notices if the gate stops holding it.
- Q3 — sure. MINORs 1–3 and N4 are rows that cannot go red.
- Q4 — likely clean. I grepped `editor-core`, `pncad`, `mesh` and the READMEs for the old sentence; only the deleted `curved_face_refuses` asserted it. Its replacement covers the face, the kind and the "may meet"/recourse text, but drops the old torus-on-a-tilted-axis carrier.
- Q5 (`classify.rs:600-620`) — likely. The `insert_crossings` header still names two lanes, Line and Conic. The new spline/spiric arm is undocumented there.
- Q5 (`classify.rs:36-40`) — unsure. Calling an unresolved surface key a face that "may meet the plane" frames corruption as reach.
- Q6 — likely. The pose sensitivity (MINOR 4) is neither disclosed nor scheduled.
- Q6 — unsure. The new predicates `split_gate_box_side` (m) and `split_gate_sphere_axis` are absent from `docs/predicate-dimension-audit.md`.
- Q6 — unsure. Approx is cleared by the fit's control net (`boxes.rs:1311-1316`), while the gate comment says the fit is not the described surface. The two comments argue opposite ways.
- Q8 — partly exercised. In `classify.rs` (938 lines) I read the gate, the new helpers and `insert_crossings` in full, and only skimmed `classify_vertices`, the conic lane and the tests by symbol. Nothing found in the skimmed part; I do not claim a full read.

## Probe sources
`probes/r1_reach3843_probes.rs`: run it as a module of `crates/sweep/tests/all.rs`. Set `R1_FROM=n` to start at shape n.

`probes/mut.py`: run it from the repo root. It edits `classify.rs` and restores it afterwards.
