---
id: store-constructed-carriers
kind: unit
title: Store the carriers circle, Center and fillet arcs are built from; delete the hand copies of bulge→carrier and the bulge accessor
status: dispatched
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
branch: claude/clever-bardeen-4itqb3
---


Unit 5 of the #3218 lowering. Outputs move by ulps, and that is the point. The seven hand copies of the bulge→carrier formula go (`seg::arc_carrier`, `path.rs`, `lift.rs`, `skin::segment_curve`, `SketchSegment::eval`, `anchor::signed_area`, `viewer::flatten`), and so does `tube.rs`'s hand traversal. The bulge accessor is deleted with its last reader (Ev, #3218). Re-check whether `FilletArcFlattenedInStorage` and `profile-fillet-radius-off-at-eps-1e-6` dissolve.

**Unit 1 left bulge stored beside the canonical segment
(`ProfileLoop::bulges`, `ValidatedSegment::bulge`) to stay byte-identical.
This unit retires that storage**, together with every reader unit 1 left
on it except the geom-brep boundary, which is unit 2's:
- `seg::build_seg`'s straightness margin, sagitta and apex (K-stream
  margins, so re-baseline the K CSVs);
- `ValidatedSegment::lift` / `ProfileLoop::map_scalar`, which rebuild
  at U from (chord, b). The rebuild is re-derived from the stored
  carrier, which is the certified-lift design question unit 1 deferred;
- `lift.rs`'s `compare` ulps;
- the `anchor::derive_naming` bit-match and `signed_area`;
- the `stackup` digest over (x, y, b);
- `viewer::flatten`.

Unit 1's report on the PR lists each with its reason.

Two more readers of the kept bulge, found in review of #3224:
- `lift.rs::chain_form` writes `ArcData::Bulge { b: bulges[src] }`, and
  `lift_seamed` refuses a loop whose bulges are not finite. Once bulges
  retire, that writer needs a b derived from Δθ, and `tan(Δθ/4)` is
  lossy (b = 1.0 comes back 0.9999999999999999), so the lift owes a
  lossless route: a program step spelled on the carrier, or a bulge
  derivation shown exact.
- **The validate-time carrier check arrives with this unit.** D1 says a
  stored carrier is verified at validation. Unit 1 derives every
  carrier at the lowering, so it holds by construction and `build_seg`
  does not re-check it. Once this unit stores the carriers the
  constructions build, the carrier is no longer a function of the
  chord and b, and validation needs the predicate that verifies it
  against its vertices.

Found in the fixture-door migration (#3231): a loop built through the
canonical fixture door stores its carrier as given, but `map_scalar`,
`reversed` and the lift re-derive the carrier from the stored
`tan(Δθ/4)` bulge. So for such loops the result can differ from the
given carrier in the last bits. Retiring the stored bulge here removes
that.

From #3231's review:
- The magnitude is worse than last bits. A one-segment full circle
  built through the canonical door has a zero chord, so re-lowering it
  from the kept bulge gives a NaN centre and zero radius.
- `RawLoop::new` adds a carrier→bulge copy, `(sweep/4).tan()`, which
  joins `sugar.rs::bulge_from_center`'s tail and `path/verbs.rs`. All
  three go when the kept bulge retires.

**From `geom-brep-sketch-segment-full-turn` (2026-09-25; corrected in
its fix pass).** The net count of bulge→carrier copies is **six**, not
seven minus two.
- **Two are gone.** `SketchSegment::eval` reads the segment's stored
  centre and sweep. `skin::segment_curve` reads the stored centre,
  radius and sweep. Both are still `seg::arc_carrier`'s derivation,
  carried across the boundary.
- **One was added.** `geom-brep/tests/shared/arc.rs::lowered_arc`
  restates `seg::arc_carrier` plus `Δθ = 4·atan b` at `Interval`. The
  arc-evaluation anchor rows need a carrier derived from a wide chord,
  and `geom-brep` sits below `profile` in the layering, so it cannot
  call the lowering.
- **Beside that copy, not counted.** The same two suites
  (`arc_eval_anchor.rs::short_arc` and
  `review_arceval_r1_probes.rs::short_sub_arc`) re-spell the retired
  restriction's sub-arc bulge `tan(atan b·Δs)` to build a short arc
  whose centre is derived from its own chord.
- **Routed through the lowering, not copies.**
  `sweep::test_support::bulge_arc` forwards to `bulge_loop`. The tour's
  sweep path authors `arc_to(Bulge)` and reads the canonical segment back.

`ValidatedSegment::bulge` has no reader left in `geom-brep`, `topo` or
`sweep`. Its readers are the lift (profile), `anchor`, the `stackup`
digest and `viewer::flatten`.

**The sweep boundary is in this unit's check (from #3254's review).**
`geom_brep::SketchSegment::Arc` carries `a, b, centre, radius, sweep`.
`eval` reads `a`, `centre` and `sweep`; `sweep::skin::segment_curve`
(public through `sweep` and `pncad`) also trusts `radius`; nothing
checks their consistency at the type's door, so an inconsistent
`radius` converts through `segment_curve` while certification, which
reads `eval`, passes. The validate-time endpoint-on-carrier check owed
here must cover the segment as it crosses into `geom-brep`, not only the
profile's stored segment. Taking `segment_curve`'s radius from
`|a − centre|` was tried and moves the elbow's STEP golden, sidecars and
mesh digests, so it was left for this unit.

## Ruling (Ev, #3453, 2026-09-30)

The design is in D1's Profile-format clause as merged:
- **The program is the stored authored form; the loop is its cache.**
  Each arc mode's lowering is that mode's one conversion.
- **Validate's consistency checks, three per stored arc, as
  ε-decisions:** the start lies on the circle, the rotated start lands
  on the end, and 0 < |Δθ| ≤ 2π (a one-segment loop is a full turn).
- **Lifts:** pinned lifts and `reversed` copy the stored fields; the
  guided lift replays the program.
- **Foldable spelling.** Each lowering spells its output in the shape's
  own algebra: the radius as authored, and Δθ as one `4·atan(X)` with X
  algebraic, never `atan2`.
- **Registrations** only for what the algebra cannot close: the
  construction registers, and the sweep registers rigidity.
- **The writer** emits `Center`.
- **One shared `geom-core` arc type**, with `radius` kept.

Order: 5a, then the shared type, then 5b. The register-equal allowlist
gains the shared type's site; it was named on the PR and approved with
it.

## After 5a (#3527, merged 2026-10-02)

**Done by 5a; not this unit's any more:**
- the stored bulge (`ProfileLoop.bulges`, `ValidatedSegment.bulge`) and
  the bulge accessor are gone; the loop stores (vertex, `Segment`) pairs
  through `ProfileLoop::from_chain`;
- `map_scalar`, `reversed`, `ValidatedSegment::lift` and `lift_onto` copy
  the stored fields; `lift::chain_form` writes `ArcTo(Center)`;
- `build_seg`'s readers (margin, sagitta, apex), `anchor::derive_naming`,
  `signed_area`, the `stackup` digest and `viewer::flatten` read the
  stored carrier;
- the validate-time consistency checks (`arc_start_on_carrier`,
  `arc_landing`, `arc_sweep_range`, with the scene-resolution refusal),
  decided for tables and held by construction for a `ConstructedLoop`;
- `Arc2::register_endpoints` (called by `lower_arc`) and the sweep's
  rigidity-only registrations.

**What remains:**
1. The emission layer still lowers every arc through a bulge: each arc
   mode computes a bulge and `Core::finish` re-derives the carrier from
   the chord (`lower_chain`). Each construction stores the `Arc2` it
   builds instead, spelled per D1: the radius as authored, Δθ as one
   `4·atan(X)` with X algebraic, never `atan2`. A `Center` arc stores the
   authored centre. Each construction registers only the endpoint facts
   its algebra proves.
2. The bulge→carrier hand copies left after 5a: `path.rs::arc_carrier`
   (and `family.rs::bulge_carrier` through it), the tangent arc's
   `atan2` delta, `sugar::bulge_from_center` / `bulge_from_via` as
   lowering steps, `verbs.rs`'s `(signed/4).tan()`, and
   `geom-brep/tests/shared/arc.rs::lowered_arc`.
3. The `geom-brep` boundary: `sweep::skin::segment_curve` trusts
   `arc.radius` and reads the start angle by endpoint `atan2`.
4. Re-check `FilletArcFlattenedInStorage` and
   `profile-fillet-radius-off-at-eps-1e-6`, and whether 3 resolves
   `sketch-segment-eval-could-be-exact-at-both-ends`.
