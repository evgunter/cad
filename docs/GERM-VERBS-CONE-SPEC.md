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

## 1. The inventory: every kind dispatch the admission reaches

**How it was swept.** I grepped `crates/topo/src/boolean/`,
`face_normal.rs`, `sector_face.rs`, `chord_join.rs`, `pcurves.rs` and
`geom-brep/src/implicit.rs` for `Surface::Cone`, `SurfaceKind::`,
`S::`/`Sf::` kind patterns and `Surface::Cylinder` (a kind match that
names the cylinder but not the cone is a candidate `_ =>` arm). Every hit
in production code was read in context. Hits in `#[cfg(test)]` modules
were dropped (`boxes.rs` :2015 on, `contact_verify.rs` :467 on,
`join.rs` :2040 on, `mod.rs` :2633 on, `rim_wedge.rs` :551,
`sectors.rs` :816).

**Blind spot.** A dispatch that reads a surface through a helper, and not
through a pattern, is invisible to that grep: `face_plane` returning
`None`, `face_carrier` returning `None`, a chart function returning
`Err`. So there was a second pass over the callers of the helpers that
return `None` or `Err` by kind: `face_plane`, `rest::face_carrier`,
`face_outward_normal_at`, `sector_face`, `circle_residual_extremes`,
`chart_boundary` and `cone_trimmed_window`. The probe (§0) is the third
pass: every fixture went through the whole pipeline, and each refusal
names its raising site.

**Legend.** **H**: handles the cone. **R**: refuses the cone typed.
**S**: would mishandle the cone silently once the roster admits it.

### 1.1 Gates and rosters

| site | status | what it does with a cone |
|---|---|---|
| `reduce.rs` `boolean_arm_exists` (:175), read by `first_unsupported_pair` (:284) | R | `CurvedPairUnsupported { site: OperandGate }`: the gate the flip opens (U7) |
| `reduce.rs` `revert_arm_exists` (:196), read at the ∖/∩ front door (`ops.rs` :442) | R | `site: RevertRoster` (Q2) |
| `reduce.rs` `gate_operand_edges` (:423) | H | edge-carrier gate: lines, circles and ellipses pass, whatever their face's kind |
| `mod.rs` `validate_declarations` `inventory_face` (:2596) | R | `InvalidDeclaration`: C8, cone is not declarable. Stays. |

### 1.2 The sweep and the crossing layer (`reduce.rs`)

| site | status | what it does with a cone |
|---|---|---|
| `sweep_direction` plane lane (:683–950): a cone operand's rims and generators against a plane face | H | `conic_plane_crossing_roots` reads the edge's carrier, not its face's kind |
| `curved_face_arm` with a cone operand's edge against a cylinder, sphere or torus face | H | dispatches on the partner's kind; a cone body's edges are lines and circles |
| `curved_face_arm` NURBS/Approx guard (:1161–1172) | H | the cone is listed and passes to the arms below |
| circle rung: `circle_clearance` against a cone face (:1237) | R | `circle_residual_extremes` has no cone form (`implicit.rs` :653), so it answers `None`, and `.ok_or_else(frontier)?` raises `CurvedPierceUnsupported`. Every circle edge whose box meets a cone face refuses, coaxial rims included (U2). |
| carrier neither line nor circle (:1332) | R | `CurvedPierceUnsupported`, kind-generic |
| `(Zero, ·)` and straddle arms: `wall_crossing` roots (:1753), cone `_ => Unsettled` (:1825) | R | the door (U1, U2) |
| same-side torus root arm (:1510–1519) | R | guarded on `Torus`; a cone falls to the two arms below |
| **`(Negative, Negative) => Ok(CurvedEvent::None)` (:1523)** | **S** | **convexity of the residual along a line: false on the double cone (§2.3). Measured: P3's four crossings are cleared silently. R-reach masks it today.** |
| `(Positive, Positive)` dip bound, `f2` match `_ => return Err(frontier())` (:1579) | R | `CurvedPierceUnsupported` (P4, P8). The cone's `f″` is unbounded near the axis, so the dip bound does not port (§2.3). |
| `vertex_on_curved_face` (:1994) → `curved_face_containment` | H | cone arm, `cone_face_containment` (`contain.rs` :916); a partial sector gives `Trim(None)`, the door (§2.4) |

### 1.3 Containment

| site | status | what it does with a cone |
|---|---|---|
| `contain.rs` `curved_face_placement` cone arm (:628) | H | double-cone carrier first, then the face's own trim. The mirror nappe is `Trim(Out)`, never `OffCarrier`. |
| `solid_contain.rs` `face_geo` (:778), `cast_ray` cone quadratic (:4323) | H | the ray × cone quadratic, nappe and slant window |
| `solid_contain.rs` `cone_trimmed_window` (:1498) | R | `PartialConeFace` for an apex-closed sector wider than π: the walk crosses the apex by nearest branch and reports a full period (§2.4). Through `cone_face_containment` it is `Trim(None)`, which is also a door. |
| `solid_contain.rs` `face_plane_datum` (:543) | R | `KindUnsupported`, plane-only by contract; never asked of a cone by the lanes above |

### 1.4 The section certificate, both paths

| site | status | what it does with a cone |
|---|---|---|
| `ops.rs` `SectionPath::scope` (:765) | H | cone faces are in scope on both paths |
| `section_cert.rs` `classify` `_ => Intractable` (:461) | R | R-reach. Crossings path: `InteriorLoopGuard` (P2, P2a). No-crossings path: `FallbackExtentUnsupported` (P1, P3, P6). |
| `section_cert.rs` `torus_pair` partner `_ => Intractable` (:545) | R | torus × cone: R-reach, coaxial included (§2.5.4) |
| `ops.rs` `ChartCache::describes` → `pcurves::chart_boundary` | R (conservative) | an apex-closed cone face refuses `SingularChartJoint`, so W2 is never available on the preview cone's face (§2.4) |
| `ops.rs` `place_witness` (:854) | H | through `curved_face_placement` |
| `ops.rs` `sphere_extent_scan` cone arm (:2274) | R | `CurvedBooleanUnsupported { kind: Cone }` whenever a closed sphere group's ball box meets a cone face. `SectionPath::Fallback` excludes every sphere pair, so the certificate never sees sphere × cone on that path (§2.6). |

### 1.5 Pierce, sectors, join

| site | status | what it does with a cone |
|---|---|---|
| `vtxfac.rs` pierce normal: `face_outward_normal_at` → `Ok(None)` (`face_normal.rs` :217), refused at `vtxfac.rs` :136 | R | `CurvedBooleanUnsupported`, the pierced face a cone |
| `sector_face.rs` `sector_face` (:209) | R | `SectorFaceError::Unsupported` |
| `vtxfac.rs` :225 and :244, `recl.rs` `carrier_of` (:51), both through `rest::face_carrier` (:557) → `None` | R | `CurvedBooleanUnsupported` |
| `sectors.rs` `tangent_lump` (:437), through `rest::tangent_locus` (:759) → `Unsupported` | R | `CurvedBooleanUnsupported` |
| `join.rs` germ-pair dispatch `(a_s, b_s) =>` (:456) | R | `CurvedBooleanUnsupported { kind: Cone }` (P5, P7). Every pair with a cone face, the plane × cone pair included. |
| `join.rs` `pair_section_frame` `_ => NoArm` (:943) | R | `GermFrameUnsupported` |
| `chord_join.rs` `bool_planar_chord_spec` wall `_ =>` (:1447) | R | `SectionInvariant`; unreachable behind the join |

**Summary.** 9 sites handle the cone. 20 refuse it typed. **One is
silent: the `(Negative, Negative)` convexity arm.** A second shape shows
up only in the admission's own order: every certificate row that clears on
W1 or W2 rests on premise S, which that arm breaks. The preview's silent
wrong answers (no cone extent gate; the crossings path unguarded) were
closed by PR 3372.

What admission leaves refused **by design**: every pair that reaches the
join. A cone face with an event on it has no join arm (plane × cone
included), and neither the pierce normal nor the sector normal has a cone
arm. Admission in this spec is the torus's shape (PR 3265): it answers
where the cone faces' pairs are certified with no event on them, and it
refuses typed at the join otherwise. The join arms are §3's U8 and Q1.
