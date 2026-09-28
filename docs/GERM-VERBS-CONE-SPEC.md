# GERM VERBS-CONE — the cone as a boolean operand (unit spec)

This spec admits `Surface::Cone` as a boolean operand
(`work/germ/VERBS-CONE.md`). It lists every kind dispatch the admission
reaches (§1), derives each arm the cone needs (§2), and cuts the units in
dependency order (§3). The roster flip is the last unit. Until it lands,
the cone keeps refusing typed at the operand gate. The spec is deleted
when its last unit merges.

It builds on PR 3372 (the section certificate, merged) and PR 3375 (the
circle × torus root lane, in review). §3 names the units that need 3375's
`wall_crossing` restructure.

Read on `e6f3eaaf9`. Line numbers ride beside names and may rot.

## 0. Survey: what was measured, and what moved

**The preview is stale. Since PR 3372, every preview fixture refuses
typed; none answers wrong.** The item's preview was measured on
`378f66744`, before the section certificate merged. I re-ran it on
`e6f3eaaf9` with `Cone` added to both `boolean_arm_exists` and
`revert_arm_exists` in a scratch tree (a throwaway probe, reverted). The
cone is the preview's: the triangle `(0,0) (1,0) (0,1)` revolved fully
about `y`, then `merge_coplanar_faces`. All four ops (∪, ∩, A∖B, B∖A)
gave the same class on every fixture:

| # | B operand | what came back (all four ops) |
|---|---|---|
| P1 | the preview's 300° bite (arc about `(y,z) = (0.45, 0.8)`, `r = 0.3`, `x ∈ [−2, 2]`), no crossings | `FallbackExtentUnsupported`, R-reach (cone × oblique cylinder) |
| P2 | the bite plus the preview's pin through the base disc | `CurvedPairUnsupported { site: InteriorLoopGuard, kind: Cone, other_kind: Cylinder }` |
| P2a | the pin alone | `InteriorLoopGuard`, cone × plane: R-reach. The preview answered this one right; it now refuses. |
| P3 | apex pin `x ∈ [−.02, .02]`, `y ∈ [.5, 1.5]`, `z ∈ [.02, .06]` | `FallbackExtentUnsupported`, R-reach. **The op reached the no-crossings path: see below.** |
| P4 | brick `y ∈ [.2, .3]`, `z ∈ [.9, 1.2]`, clear of the cone, boxes overlapping | `CurvedPierceUnsupported` (the `f2` door) |
| P5 | axis-normal slab `y ∈ [.3, .6]` | `CurvedBooleanUnsupported { kind: Cone }` (the join) |
| P6 | a small box strictly inside the cone | `FallbackExtentUnsupported`, R-reach |
| P7 | a 3π/2 partial cone against the P5 slab | `CurvedBooleanUnsupported { kind: Cone }` (the join, before any containment) |
| P8 | a quarter cone against a brick clear of it inside its box | `CurvedPierceUnsupported` |
| P9 | the cone nested in a big box | correct: ∪ `OperandB`, ∩ `OperandA` (`π/3`), A∖B `Empty`, B∖A `Voided` (`216 − π/3`) |

So the admission is **safe today and useless**: R-reach, the crossing
layer's doors and the join refuse every cone pair whose boxes overlap.
The preview's two wrong answers were the no-crossings path with no cone
gate, and the crossings path with no interior-loop guard. Both were
closed by PR 3372's per-pair pass, which scopes cone faces on both paths
(`ops.rs` `SectionPath::scope`, :765) and answers R-reach for every cone
pair (`section_cert::classify`'s `_ => Intractable`, :461).

**A measured premise-S break, masked by R-reach.** P3's four long edges
cross the cone's lateral face near the apex, at `y = 1 − ρ ∈ [0.937,
0.972]`, inside the face. Yet the op reached `FallbackExtentUnsupported`,
which runs only when the reduction recorded **no** crossing. Each of those
edges has both ends inside the double cone, one in each nappe.
`curved_face_arm`'s `(Negative, Negative) => Ok(CurvedEvent::None)`
(`reduce.rs` :1523) cleared them on convexity, which the double cone does
not have (§2.3). Today R-reach refuses the pair. Premise S (sweep
completeness) is what makes the certificate's W1 and W2 clearances sound,
so **the day a cone × plane row lands without §2.1, P3 becomes a valid,
wrong body.** That fixes the order: the root lanes (U1, U2) land before
the certificate rows (U4). The flip comes after both.

**Also moved:**

- **The extent gates.** `cylinder_extent_gate` and `torus_extent_gate`
  no longer exist. The no-crossings path now runs `sphere_extent_scan`,
  then `section_extent_pass`, which is the certificate on
  `SectionPath::Fallback` (`ops.rs` :979). The "cone arm in the extent
  gates" is therefore the certificate's cone rows, plus one change to the
  sphere scan (§2.6).
- **"A partial revolve refuses every op at `PartialConeFace`"** holds
  only where containment is reached first. P7 met the join first. The
  refusal is real: `cone_trimmed_window` (`solid_contain.rs` :1498)
  refuses any sector wider than π (§2.4).

**No live defect on main.** The operand gate (`first_unsupported_pair`,
`reduce.rs` :284) refuses every cone face whose box meets the partner,
before any of the sites below. It is pinned by `verbs_germarms.rs`
(:256), `verbs_gate_r1_probes.rs` (:386) and `review_m3_pr4.rs` (:590).
A declaration cannot cover a cone pair, because cones are outside the
C8 inventory (`validate_declarations`, `mod.rs` :2596).
