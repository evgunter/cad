# CURVED-SPIRIC PR-1 — the exact spiric rim carrier (spec)

**Program:** CURVED (`work/curved/`). **Ratified input:** `docs/CURVED-SPIRIC-DESIGN.md`
§0a (Ev, #1858): Q1 = (b), Q2 = (b1) `Curve3::Spiric` as a per-configuration special
case; Q3(i) a data-free exact `Pcurve` variant; Q4(i) an export-only approximating
spline; Q5(ii) the carrier ships first and the elbow stops at the lune's
`VolumeUncomputable` door; Q6 the pinch is independent. The rung-3 fitted carrier
stays the general route; the C9 ring `sqrt` is a separate `[ev]` conversation.
**Track:** kernel change, one PR, dual review. **Survey:** every cite below was
re-derived by symbol on main `a7532e677` (2026-09-13); line numbers ride along and
may rot, names do not.

**Pre-logged difficulty: H. Task class: STRUCTURAL.** The design doc said M–H; the
survey moves it to H on breadth alone — 72 non-test code arms outside `curves.rs`
name `Curve3::Nurbs` across 37 files (§2), 99 more in 39 test files, a new `Pcurve` variant with 46
internal and 13 external consumer sites (§3), and six rows plus a demo wall that
flip (§6). The numerics are three closed forms with stated bounds (§1) — none is
a solve, none is a fit — so the class is structural: the risk is an arm that
silently assumes, not a number that is wrong.

## 0. Survey — what held, what moved, what the design doc missed

- **Held: the elbow's door.** `topo::shell`/`shell_open` on the klein elbow refuse
  `ReplaceFaceError::TogetherAxialEdge { what: "a circular edge between two charts
  whose centre is off the axis" }`, raised in `offset_axial.rs:mint_carrier`'s
  distinct-charts `Curve3::Circle` arm through `latitude_posture` → `centre_on_axis`,
  predicate **`offset_axial_centre`**. Pinned by
  `verbs_shell.rs:the_klein_wall_pair_waits_on_the_partial_revolve_rim` (open AND
  sealed, same face/edge/predicate), `torax_axial.rs:torax_the_klein_elbow_rim_refuses_at_the_carrier_mint`
  (`R = 1.2`, `r = 0.275`, `t = 0.05`), `torax_interval.rs:interval_the_klein_elbow_rim_refuses_at_the_carrier_mint`,
  `shell7_seam_corner.rs:a_partial_two_arc_torus_refuses_at_its_spiric_rim`
  (`R = 2`, `r = 0.5`) and the tour's `torusvessel.rs` wall 1 (the sectioned
  vessel). **The brief's predicate name `offset_axial_latitude` no longer exists**:
  `docs/predicate-dimension-audit.md` records it folded into `offset_axial_centre`
  at SHELL-7 (one name, one distribution); `offset_axial_latitude_tilt` is the
  sibling tilt predicate. §6's planted red is written against the surviving name.
- **Held: `Curve3` is `Line | Circle | Ellipse | Nurbs`** (`geom/src/curves.rs`);
  `param_near` answers `None` for `Ellipse | Nurbs`; `Pcurve` is
  `Harmonic | Fitted | General | IsoLine | IsoArc` (`pcurve_cache.rs`).
- **Held: the rim is `Intersection`-described with `Derived` authority.** A partial
  revolve's wedge-cap meridians upgrade to `Intersection { cap, wall, witness }`
  (`sweep/src/revolve/mod.rs` module doc) and `description.rs:authority_of` answers
  `Derived` for every `Intersection` spec, so `offset_axial.rs:restate` computes no
  `reauthor` for the rim — the kind-changing mint does NOT meet the "sketch arc has
  no moved circle" refusal. (Measured from the source; the opening measurement
  confirms it.)
- **Moved: the census.** 102 raw non-test `Curve3::Nurbs` hits in 39 files (72 code
  arms outside `curves.rs` in 37 files, 11 doc, 1 in-src test, the rest the enum's
  own home), 82 `Ellipse` hits in 36 files — the design doc's ~60/72.
  §2 is the current table.
- **Moved: the STEP-import claim.** The design doc predicted an adopted spline under
  `Intersection{torus, plane}` "owes the C2 hull at rest, which the torus poison
  denies". Not at this head: `certify.rs:run_checks` admits a `Nurbs` carrier under
  `Intersection` (the 9-sample gate); tier 3 check 2 re-runs THAT certification only
  (`validate.rs`, "certify a body at rest; it says nothing about the C9 ring"); and
  `pcurves.rs:mint_faces` answers a face's `UnsupportedCarrier` by `clear_face_caches`
  and continues — an uncached face is legal. The first typed refusal on re-import is
  tier 3 check 7: `VolumeUncomputable { Face { PropsError::Unimplemented } }` from
  `props/curved.rs:torus_boundary`'s `Nurbs` arm. §5 is written to that measurement.
- **New: the pcurve variant is not on the elbow's critical path.** By the same
  `mint_faces` rule, a body whose torus wall carries spiric edges with NO pcurve
  arm reaches `validate_geometric` uncached and stops at check 7. Q3(i) is ratified
  and ships here, ordered last (§9); the orchestrator may split it (§10).
- **New: the trimmed tessellation lane has no torus or plane arm**
  (`mesh/src/trimmed.rs:tessellate_trimmed` — `Cylinder`, `Nurbs`/`Approx`, else
  `trim_frontier`), so a spiric-bounded torus wall cannot be meshed after this unit.
  A typed frontier, filed on MESH (§7), not a STOP.

## 1. The variant and its closed forms

**Frame.** Torus `T(R, r)` with `center c`, unit `axis a`, `R > r > 0`
(`surfaces.rs:Surface::Torus`, `S(u, v) = c + e(u)·(R + r cos v) + a·r sin v`,
`v = 0` on the outer equator, `v = π/2` toward `+a`). Cap plane
`Π = { x : (x − c)·n = d }` with unit `n ⊥ a` and SIGNED `d`; `m = a × n`.

```rust
/// The spiric of Perseus — the section of a ring torus by a plane parallel to
/// its axis — restricted to ONE of its two ovals. `P(v) = center + u_ref·offset
/// + (axis × u_ref)·√((R + r cos v)² − offset²) + axis·(r sin v)`, v ∈ ℝ, 2π-periodic.
Spiric { center: Point3<T>, axis: Vec3<T>, u_ref: Vec3<T>,
         major_radius: T, minor_radius: T, offset: T }
```

Conventions (D2: carried as data, unchecked by the evaluators, decided at the mint):
`axis`, `u_ref` unit and orthogonal; the carried oval is the one on the `+m` side of
the plane's trace, `m = axis × u_ref`; the two-oval regime `|offset| < R − r` (the
reach guard below); `v = 0` is the seam at the outer-equator point `ρ = R + r`;
increasing `v` winds counterclockwise viewed from the tip of `u_ref` (checked:
`dP/dv(0) = a·r`, `dP/dv(π/2) ∝ −m`, and `m × a = n`). The two spellings
`(axis, u_ref, offset)` and `(−axis, −u_ref, −offset)` describe the same oval in
opposite senses, exactly as a circle's `axis` sign does — the sense is frame data,
there is no `bool`. Under `u_ref ↦ −u_ref, offset ↦ −offset` alone the OTHER oval
is named. Six fields, all `T`; `scalar_lift::map_scalar` maps each; the memo tags
it `u8(4)`.

**Evaluators** (`curves.rs`, generic over `SpanLocate`/`Real`, fixed order, D9; one
`sin_cos`, one `sqrt`, no branch): with `(s, c) = v.sin_cos()`, `ρ = R + r·c`,
`f = (ρ² − d²).sqrt()`, `f′ = −r·ρ·s / f`,
`f″ = −r·((ρ·c − r·s²)/f + r·ρ²·s²/f³)`:
`eval = center + u_ref·d + m·f + axis·(r·s)`; `deriv = m·f′ + axis·(r·c)`;
`deriv2 = m·f″ − axis·(r·s)`. Off-regime input (`f` of a negative) is poison by
`Real::sqrt`'s totality policy, never a panic.

**`param_near(p, near)`** — the circle arm's anchored-difference form in the
meridian half-plane: `h = (p − c)·a`, `ρ = |(p − c) − a·h|`, `w = (ρ − R, h)`,
`r̂ = (cos near, sin near)`, `τ̂ = (−sin near, cos near)`,
`v = near + atan2(w·τ̂, w·r̂)`. Both `atan2` arguments are lengths (m), the
quotient is scale-free; the branch is the one within a half-turn of `near`, so the
`|δ| = π` tie and the midpoint-anchor precondition are the circle arm's verbatim
(`param_near`'s docs). Answers for the point's minor angle on EITHER oval — the
side is the mint's decision (§4), not this arithmetic's.

**Speed.** `|dP/dv|² = r²cos²v + r²ρ²sin²v/(ρ² − d²) ∈ [r², r²(R−r)²/((R−r)²−d²)]`
(the second factor is `≥ 1`, largest at `ρ = R − r`). The floor `r` is the
parameter meter everywhere a kind supplies one: `certify.rs:run_checks`'s span arm
(`Margin::levered(span, minor_radius)` forward, `levered(τ − span, minor_radius)`
winding — the ellipse arm's shape, `WindingExceeded` on a definite overshoot),
`pcurve_cache.rs:param_rate`, `split.rs:split_edge`'s interiority scale. Dimension:
parameter × (m/parameter) = m.

**`edge_extent`** (`certify.rs`): `max(chord, r·(1 − cos(Δv/2)))`, the circle's
formula at the minor radius. Proof of the lower bound: in the `(m, a)` plane the
spiric is the image of the minor circle `(r cos v, r sin v)` under
`(x, y) ↦ (g(x), y)` with `g(x) = √((R + x)² − d²)`, `g′ = ρ/f ≥ 1`, so no
distance shrinks and the spiric's point-set diameter dominates the circle's, which
dominates the circle formula.

**Chord step** (`mesh/src/chords.rs`): `sup|C″| ≤ r + (r² + rρ_max)/f_min +
r²ρ_max²/f_min³` with `ρ_max = R + r`, `f_min = √((R−r)² − d²)` (from `f″` above,
`|f″| ≤ r((r + ρ_max)/f_min + rρ_max²/f_min³)`, plus the `a` channel's `r`); step
`cap_angular(curvature_step(δ, sup|C″|))`, the ellipse's `curvature_step` pattern.
Plain `f64` like `ellipse_step` (a sizing quantity; conservative by the bound's
slack, not by rounding). No torus tightening (`torus_boundary_step` is the circle
arm's; a spiric is neither rim nor meridian traversal).

**Box** (`geom/src/curves/boxes.rs:spiric_arc_aabb`, dispatched from
`conic_arc_aabb`): per axis `e`, through `Brk`:
`(c + n·d)·e + (m·e)·[f_min, f_max] + (a·e)·r·[−1, 1]`, `f_min/f_max =
Brk::sqrt_nonneg(ρ_min/max² − d²)` with `ρ_min/max = R ∓ r` as brackets, seeded with
the endpoint hull exactly as `ellipse_arc_aabb`. Whole-period, outward-rounded, a
C10 superset; exact extremes need a transcendental root and are not owed. Its
boolean reader is `boolean/boxes.rs:edge_box_rule`, which gains
`EdgeBoxRule::Spiric { .. }` reading this door — reachable only from its own rows
today, because the operand gate refuses the kind (§2).

**Named predicates** (all `decide`d, all registered in
`docs/predicate-dimension-audit.md`'s topo/geom-brep tables with these dims):

| name | site | comparand | dim | verdict |
|---|---|---|---|---|
| `offset_axial_rim_torus_reach` | `mint_carrier` spiric arm | `(R − r′) − |d|`: inner-equator radius minus the moved cap's stand-off, decided BEFORE the root `√((R−r′)²−d²)` (the `offset_axial_rim_reach` idiom); `(R−r′)² − d² > 0 ⟺ R − r′ > |d|` given `R > r′` | m | Positive ⇒ mint; else refuse "a meridian cap standing tangent to or beyond the torus's inner equator, whose section is not two ovals" |
| `offset_axial_rim_side` | same | `m·(q_mid − c)` for the OLD rim's midpoint `q_mid`; a projection of a metre vector, and since `q_mid` lies in the meridian plane it equals `±ρ(q_mid) ∈ ±[R − r, R + r]` — no lever | m | Positive ⇒ `+m` oval; Negative ⇒ flip `u_ref, d`; Zero unreachable on a ring torus, refused typed |
| `offset_axial_rim_sense` | same | `old.axis · n`, a cosine of unit vectors, levered at `frame.extent` | m | Positive ⇒ `(a, n, d)`; Negative ⇒ `(−a, −n, −d)`; Zero refused (the old plane is not the cap's) |
| `offset_axial_rim_meridian` | same | old circle centre's distance from the tube-centre circle in `(ρ, h)`: the `offset_axial_seam_meridian` comparand on a distinct-charts edge | m | Zero required |
| `offset_axial_rim_tube` | same | old circle radius minus the OLD minor radius | m | Zero required |
| `offset_axial_rim_plane` | reused, sphere arm's | `sin(old.axis, cap normal) × extent` | m | Zero required |
| `pcurve_spiric_chart_axis` | `chart_pcurve` torus arm | `chart.axis · carrier.axis`, levered at `major_radius` | m | sign ⇒ `sense`; Zero refused |

## 2. Dispatch census — every non-test site, and the spiric arm

Grep: `Curve3::Nurbs` over `crates demos tools --include=*.rs`, `/tests/` excluded,
each hit attributed to its enclosing `fn` by script; `Ellipse` hits that name no
further site are folded in. **Blind spot, stated:** a `match` on a carrier whose
kinds are exhausted by a wildcard (`_ =>`, `other =>`; 636 such arms in `src`) is
invisible to this grep and to the compiler — the two found by reading are
`offset_axial.rs:corner_arms` (`other ⇒` endpoint chord: for a spiric a smaller lever
arm, the escalating direction, acceptable) and `props/curved.rs:torus_boundary`
(`_ ⇒ NotIsoRectangle`, the door §4 predicts). The implementer sweeps by SHAPE
(`match .*carrier`) and puts the hit list in the PR. The compile break is the exact
set of exhaustive matches; the 99 test-file hits in 39 files and the 28 test files
matching `Pcurve::` are the test-side break, each an arm or a `let … else` that
already refuses. Arms: **CF** closed form; **D** delegation (mechanical field map);
**R** typed refusal naming the frontier; **U** unreachable arm written for
exhaustiveness with the reason at the site.

| site | arm |
|---|---|
| `geom/src/curves.rs:Curve3::{eval,deriv,deriv2,param_near}` | CF §1 |
| `geom/src/curves/boxes.rs:conic_arc_aabb` | CF → `spiric_arc_aabb` |
| `geom/src/scalar_lift.rs:map_scalar` | D per field |
| `geom-brep/src/certify.rs:edge_extent` | CF §1 |
| `certify.rs:carrier_kind` | `"spiric"` |
| `certify.rs:run_checks` span arm | CF: levered at `minor_radius`, forward + winding |
| `certify.rs:run_checks` `Nurbs`-under-non-`Intersection` gate, `PlaneNurbs` let-else | no change (the spiric is admitted under `Intersection`; residuals via `implicit_residual` are kind-blind) |
| `geom-brep/src/pcurve_cache.rs:carrier_harmonic` | `None` |
| `pcurve_cache.rs:param_rate` / `param_rate_gate` | `minor_radius`, extent 1 |
| `pcurve_cache.rs:operand` (fitted-lane spline) | R `UnsupportedCarrier` (with `Line | Ellipse`) |
| `pcurve_cache.rs:run_fitted_checks` lane gate | R (not in the fitted lane) |
| `pcurve_cache.rs:chart_pcurve` plane arm | CF §3 (before `carrier_harmonic`) |
| `chart_pcurve` cylinder / cone / sphere arms | R `UnsupportedCarrier`: a spiric lies on no such chart |
| `chart_pcurve` torus arm | CF §3 |
| `geom-brep/src/props/curved.rs:{cylinder,cone,sphere}_boundary` | R `Unimplemented` beside `Nurbs` (a spiric lies on none) |
| `props/curved.rs:torus_boundary` | R `NotIsoRectangle { "torus boundary edge is not a circle" }` — already the wildcard; make it a named arm |
| `props/loop_area.rs:loop_vector_area` | R: the cap's oval area is an elliptic integral (design §4.7) — `PropsError::Unimplemented` naming PR-2 |
| `geom-brep/src/ssi.rs:branch_tubes`, `finish_r3/r4`; `sweep/src/loft.rs:assemble` | no change (let-else / constructors) |
| `mesh/src/chords.rs:compute_chords` | CF §1 |
| `mesh/src/trimmed.rs:{has_trim_carrier,trim_frontier}` | `true` (routes the face to the trimmed lane, whose torus/plane roster is the MESH frontier, §7) |
| `mesh/src/memo.rs:curve3` | tag `u8(4)`, six fields |
| `step-export/src/writer.rs:carrier_kind` / `edge_curve` | `"spiric"` / §5 |
| `step-import/src/geometry.rs:endpoint_params` | U: no STEP entity mints a spiric |
| `step-import/src/adopt.rs:mapped_self_description` | `None` (with `Ellipse | Nurbs`) |
| `step-import/src/entities.rs:nurbs` | no change |
| `editor-core/src/eval/measure.rs:curve_reach` | CF `from(center) + R + r` |
| `topo/src/transform.rs:map_carrier` | D: `center, axis, u_ref` mapped, radii and `offset` verbatim |
| `topo/src/readback.rs:edge_pose` | CF pose `{ origin: center, axis, u_ref: Some(u_ref) }` — the frame the model holds |
| `topo/src/query.rs:CurveKind::{of, ALL}` | `Spiric`, `ALL` becomes 5; if `pncad-py` mirrors `CurveKind`, the tag lands in the same PR (sweep by symbol) |
| `topo/src/chart_iso.rs:classify_kind` | `None` (neither rim nor meridian traversal) |
| `topo/src/split.rs:split_edge` | `minor_radius` |
| `topo/src/offset_axial.rs:mint_carrier` / `param_on` | §4 |
| `topo/src/replace_face.rs:transport_curve` (cylinder, cone arms) | `None` beside `Ellipse` (a spiric lies on neither) |
| `replace_face.rs:translate_curve` | D: `center + delta` |
| `replace_face.rs:homothety` | `None` (not on a sphere) |
| `replace_face.rs:cone_v_range` | U |
| `replace_face.rs:plan_edge` | no change (constructs `Nurbs`) |
| `topo/src/pcurves.rs:nurbs_iso_derive`, `derive_general_image` | no change (let-else on `Nurbs`) |
| `topo/src/loop_winding.rs:planar_loop_winding` ×2 | `None` — the honest remainder, as `Nurbs` |
| `topo/src/chord_join.rs:between_edge_in_plane` | R `SectionInvariant` (the join lanes are fenced; never assume "on") |
| `topo/src/boolean/reduce.rs:gate_operand_edges` | R `CurvedEdgeUnsupported` — the boolean fence (§7) |
| `boolean/boxes.rs:edge_box_rule` | `EdgeBoxRule::Spiric` → `spiric_arc_aabb` |
| `boolean/contain.rs:loop_shape` | as `Ellipse`: `bears_arc = true, one_circle = false` |
| `boolean/join.rs:ring_run_ccw` | as `Nurbs` (chord only); unreachable behind the gate |
| `boolean/sectors.rs:corrupt` (tangent) | CF: the conic arm's `deriv` |
| `topo/src/splitting/classify.rs` gate / `conic_plane_crossing_roots` | R `CurvedEdgeUnsupported` / `Err(())` |
| `splitting/finish.rs:describe_section_boundary` | `None` |
| `splitting/join.rs:certify_section_area` | as `Nurbs` |
| `splitting/neighborhood.rs:chord` | CF: the conic arm's jet (`deriv`, `deriv2`) |
| `topo/src/props.rs:face_flux` `is_trimmed` | `true` → the quadrature lane; `trig_at_start` R `QuadratureUnsupported` naming PR-2 |
| `topo/src/props.rs:nurbs_face` | let-else, no change |
| `topo/src/cert_m3r1_probes.rs`, `merge_faces.rs` in-src rows, `transform.rs:described_carrier` | constructors, no change |
| `sweep/examples/r2_p2_consumer.rs:main` | `"Spiric"` (compiled by `--all-targets`) |
| `demos/tour/src/curvedcut.rs` | let-else on `Ellipse`, no change |

**Load-bearing paragraphs.** *`certify.rs:edge_extent`* feeds transversality's
lever (`classify_dihedral`) and tier 3's dihedral pass — the stretch proof in §1 is
what licenses the circle formula; a wrong (larger) arm would let a sliver classify
transverse, so the row in §6 pins the bound below the sampled diameter.
*`mesh/chords.rs`* — the step is the hull-free sibling of `nurbs_chord_count`; a
dropped term in `sup|C″|` under-samples, which the §6 sag row sees.
*`transform.rs:map_carrier`* — a rigid map preserves `R, r, d` and orthonormality;
no re-decision (the ellipse's argument). *`step-export`* — §5. *`chart_iso.rs`* —
`None` is correct AND it makes `torus_boundary_step` refuse if ever handed a
spiric; the spiric chord arm does not call it. *`readback.rs`* — the pose is the
stored frame; `sense` is `true` like every other kind.

## 3. The exact `Pcurve` variant

```rust
/// The exact chart image of a `Curve3::Spiric` on the two charts it lives on.
Spiric { major: T, minor: T, offset: T, image: SpiricImage<T> }
enum SpiricImage<T> {
    /// Plane cap: `P(t) = p0 + pm·f(t) + pa·sin t`, `f(t) = √((R + r cos t)² − d²)`.
    Cap { p0: Point2<T>, pm: Vec2<T>, pa: Vec2<T> },
    /// Torus wall: `u(t) = u0 + sense·atan2(f(t), d)`, `v(t) = v0 + sense·t`.
    Wall { u0: T, v0: T, sense: T },
}
```

Data-free: no fitted net; `major, minor, offset` are the CARRIER's own fields,
copied so `eval` is closed (a `Pcurve` evaluates from `t` alone). `f > 0`
everywhere in the two-oval regime, so `atan2(f, d) ∈ (0, π)` is smooth in `t` and the
azimuth channel never crosses a cut nor winds; the `v` channel is the parameter
itself. **Derivation of `Wall`**: the chart azimuth of `n·d + m·f` is
`φ_n + atan2(f, d)` when `chart.axis = a` (the +90° direction from `n` is `a × n = m`)
and `φ_n − atan2(f, d)` when `chart.axis = −a`, where also `h = a·r sin t = −a′·r sin t`
gives `v = −t`; hence one sign `sense = ±1` on both channels, decided by
`pcurve_spiric_chart_axis` (§1), `u0 = atan2(n·v_ref_T, n·u_ref_T)` by one `atan2`,
`v0 = 0` at mint and `k·τ` after the loop walk's branch shift (`shift_polar_branch`
and the azimuth shift both add `k·τ` in `T`, the `Harmonic` `p0` precedent).
**Derivation of `Cap`**: `p0 = chart(c + n·d − origin)`, `pm = chart(m)`,
`pa = chart(a·r)` through the plane's affine chart, coefficient by coefficient.

**Minting** (`topo/src/pcurves.rs`): `chart_mints` already answers `true` for the
torus, `false` for the plane (derive on demand); `pcurve_of`/`mint_face` reach
`chart_pcurve`, whose plane and torus arms gain the two constructors above (the
spiric arm precedes `carrier_harmonic`, which stays `None`). `walk_loop`'s joint
continuity and closure checks read `eval` and need nothing new; `chart_edge`'s `_`
arm already yields an `Envelope` from `eval` over the span hull (poison at a point
scalar, sound at `Interval`), so `chart_boundary` needs no arm.

**Certification** (`pcurve_cache.rs`, a new arm of `PcurveCache::certify` beside the
harmonic lane, same five checks in the same order): check 1 admits `Spiric` on
`Plane`/`Torus` charts only; check 2 meters the span at `minor` (forward, winding
`≤ τ`); the chart-winding gates on the torus are the `v` channel at `minor_radius`
(`pcurve_azimuth_period`) and the `u` channel's extent, which is `≤ π` by
structure (`atan2`'s range) and is metered anyway at `azimuth_lever`; check 3 is
the shared schedule; **check 4, the meter**: the residual `|S(P(tᵢ)) − C(tᵢ)|` at
`CERT_SAMPLES`, folded as every lane folds it. The between-samples statement, one
per image: *Cap* — `S(P(t)) − C(t) = k₀ + k₁·f(t) + k₂·sin t` with constant vector
coefficients (`k₀ = origin + u_ref·p0.x + v_ref·p0.y − c − n·d`, etc.), so
`sup ≤ |k₀| + |k₁|·f_max + |k₂|` in closed form — `EnvelopeStatement::MapResidualClosedForm`,
whose doc gains the sentence "or `span{1, f, sin}` for the spiric cap"; *Wall* — the
stored `major, minor, offset, sense` are compared to the carrier's fields as
`f64` STRUCTURE (bit-equal brackets, the C6 read `chart_region.rs:exact_zero`
uses; a mismatch refuses `UnsupportedCarrier`), the map is then the algebraic
identity `e(φ_n + σ·atan2(f, d))·ρ = n·d + m·f` in which the ONLY derived numbers
are `u0` and `v0`; an error `δ` in `u0` is a residual `≥ (R − r)·2|sin(δ/2)|` at
EVERY sample and an error in `v0` that is not a multiple of `τ` is visible at every
sample too, so the schedule certifies them and nothing hides between samples —
recorded as `EnvelopeStatement::SpiricIdentity` with `envelope = 0`. Check 5 is
trim containment through `chart_box`: *Cap* `p0 + pm·[f_min, f_max] + pa·[−1, 1]`,
*Wall* the hull of `u0 + sense·atan2({f_min, f_max}, d)` and `v0 + sense·[t0, t1]`.
`PcurveCertificate.statement` records the new statement; `max_residual` the
schedule's.

**Consumers, after the variant** (the 13 external sites): `pcurves.rs:chart_edge`
(no arm, above); `chart_region.rs:pcurve_entry` R "Spiric image is not a straight
segment" (the planar trim inventory); `boolean/boxes.rs:harmonic_extent`,
`harmonic_travel`, `solid_contain.rs:torus_chart_windows`, `chord_join.rs:chart_azimuth_range`
`None` (the torus window walk refuses typed — the operand gate refuses the kind
first); `mesh/trimmed.rs:trim_polygon` admits the closed form (unreachable until the
lane's roster grows); `mesh/memo.rs:pcurve` tag `u8(5)`; `mesh/chords.rs:nurbs_tighten`
skips it as it skips `Harmonic`; `replace_face.rs:shift_chart_v` R;
`props.rs:chan` R `QuadratureUnsupported`; `description.rs:iso` `None`;
`demos/tour/src/uvdump.rs` and `sweep/examples/r2_p2_consumer.rs` print it.

## 4. `mint_carrier`'s kind-changing arm, and where the elbow goes

The arm sits beside the sphere-wall rim arm (`ball_meridian`), gated on the chart
pair `(Constraint::Torus, Constraint::Meridian { m, c })` in either order and on
`old` being a `Curve3::Circle`; anything else refuses "an edge between a torus wall
and a meridian cap whose carrier is not a circle". It reads `wall.new` = the moved
torus (`R`, `r′ = r + distance`, `center`, `axis`) and `wall.old`'s `r`; verifies
the OLD rim in full — `offset_axial_rim_meridian`, `offset_axial_rim_tube`,
`offset_axial_rim_plane` — so the carrier really is the wall × cap meridian circle
at rest, not only the binding relation (the ordinal-111 guard lesson); then the
moved cap's signed stand-off `d = c − m̂·(center − 0)` along `m̂` (the sphere arm's
`t`), the reach guard `offset_axial_rim_torus_reach`, the side
`offset_axial_rim_side` on the OLD rim's midpoint `q_mid = old.eval((t0+t1)/2)`,
the sense `offset_axial_rim_sense`, and mints the struct literal
`Curve3::Spiric { center, axis: ±a, u_ref: ±m̂, major_radius: R, minor_radius: r′,
offset: ±d }`. **Inline arithmetic only**: no `plane_torus_section` call (that arm
refuses this pose by design and stays so), no marching, no SSI — the module's law
at its module doc, "the carrier keeps its KIND", gains its one stated exception:
*a meridian rim on a torus wall changes kind, because its moved locus is not a
circle; every verification below still applies*.

`param_on` gains the `(Spiric, Circle)` arm: the anchor is the OLD point's minor
angle on the OLD torus, `v_q = atan2(h_q − h_c, ρ_q − R)`, and
`t = carrier.param_near(p, v_q)` — the window is re-derived in the new parameter
(the old window's turn is not carried across a kind change, its SENSE is, via
`offset_axial_rim_sense`); the endpoint meter `offset_axial_edge_agreement` and the
caller's midpoint meter `offset_axial_edge_on_surface` (against `surface_residual`'s
torus and plane arms) run unchanged and are the net for every mutant in §6.

**The door the elbow moves to** (prediction, measured at dispatch and quoted in the
PR body): mint → `param_on` ×2 → midpoint meter → `restate` (`Intersection`, witness
= midpoint; no `reauthor`) → attach `run_checks` (span at `r′`, residuals zero to
rounding against both implicit forms, transversality through `edge_extent`) →
`shell`'s insertion → `mint_pcurves` (the torus wall's spiric half-edges through §3,
or uncached if §3 is split out) → `validate_geometric`: checks 1–6 pass (check 4's
dihedral uses the same `edge_extent`); **check 7 refuses**
`ShellError::NotValid { errors: [ValidationError::VolumeUncomputable { source:
MassPropsError::Face { source: PropsError::NotIsoRectangle { what: "torus boundary
edge is not a circle" }, .. } }] }` from the cavity's torus wall, which precedes its
caps in arena order; if a cap is visited first the payload is `loop_vector_area`'s
`Unimplemented` — the row quotes whichever the run shows and names the other. The
lune's row reads `NotIsoRectangle { what: "props_meridian_great" }` on the
CAVITY's lens face — the operand's own wall measures since the rim-free wedge
arm (`torax_the_sphere_lune_next_door_is_the_props_inventory`): same door,
different premise — the acceptance says so rather than claiming identity.

## 5. STEP export (Q4(i)) and what import does

`writer.rs:edge_curve`'s `Spiric` arm mints an EXPORT-ONLY cubic non-rational
`B_SPLINE_CURVE_WITH_KNOTS` through `geom::curves::fit::interpolate_with_params`
with the interpolation parameters = the `v` samples themselves (so the spline's
parameter IS `v` and the difference `D(v) = spline(v) − P(v)` vanishes at every
node). Certificate, curve side only: on each node interval of width `h`,
`sup|D| ≤ h²·(M_s + M_P)/8`, `M_P = sup|C″|` from §1 and `M_s = sup|spline″|` from
the derivative hull (`geom_core::spline::hull::SplineCoeffs::derivative_coeffs`
twice, `sup_norm_bound`) — the same hull machinery the rung-3 fit reads, with no
surface operand, so the torus poison does not enter. The node count is the least
power of two whose bound is `≤ ε/4` (the factor keeps re-import's 9-sample
`carrier_on_surface_*` gate off the margin), capped at 1024; past the cap the arm
refuses `StepExportError::UnsupportedCurve { kind: "spiric: export tolerance not
met" }`. The tolerance is STATED IN THE FILE: `FILE_DESCRIPTION`'s description
string carries `spiric edges approximated to <bound> m` (one sentence per file,
worst bound over its spiric edges). Import (§0): the spline adopts under
`Intersection{torus, plane}`, passes the 9-sample gate, its torus face mints no
cache, and `validate_geometric` refuses check 7 with `PropsError::Unimplemented` —
the same door as the native hollow, one payload over. The row pins that.

## 6. Rows and mutants (each red-first; the mutant it kills named)

Suites: `crates/geom/src/curves.rs` in-src rows for the kind; a new
`crates/sweep/tests/spiric_rim.rs` aggregated per `tests/all.rs`; `torax_axial`,
`verbs_shell`, `torax_interval`, `shell7_seam_corner` re-pinned in place.

1. **Residuals against BOTH implicit forms** (`geom` rows; `Tol` at the three CI
   ε cells — the matrix runs all three, no narrowing): `implicit_residual(torus, P(v))`
   and `(P(v) − c)·n − d` at 10⁴ samples on the `torax` elbow numbers and the
   `shell7` two-arc numbers, `≤ 4e-16` m at f64; `deriv`/`deriv2` against central
   differences. Kills: the `P±` sign (the other oval passes THIS row — it is killed
   by row 4's endpoint meter, said so in-row), `d` vs `−d` (plane residual `2|d|`).
2. **Speed bounds and `edge_extent`**: sampled `|dP/dv|` within `[r, r(R−r)/f_min]`;
   the extent below the sampled point-set diameter on every sub-span. Kills: a
   dropped `ρ²/(ρ²−d²)` factor.
3. **Box**: 10⁴ sampled points inside `spiric_arc_aabb` on every axis, at f64 and
   `Interval`. Kills: *a box axis dropped* (the `a·e` term), a `Brk` rounding turned
   inward.
4. **`verbs_shell` and `torax_axial` flip**: both elbow rows assert the §4 payload
   WITH the old door recorded in the row's doc (`TogetherAxialEdge`, "…whose centre is
   off the axis", `offset_axial_centre`); `torax` keeps its half-width/half-height
   check as the reason the carrier is not a circle. `shell7_seam_corner`'s two-arc
   row flips the same way and re-measures its claim that no door-built operand
   reaches `reauthor`'s out-of-plane decide. `torusvessel.rs` wall 1 MOVES to the
   new door (its "retire this probe" sentence waits for PR-2 — the third panel needs
   a volume). Kills: the wrong oval (`offset_axial_edge_agreement`, gap
   `≈ 2·half-width`), the wrong sense (`IntervalNotForward`), the reach guard's
   sign.
5. **The planted red**: `offset_axial_centre`'s off-axis refusal stays reachable —
   on a genuine LATITUDE fixture if one exists through a public door (a circle
   between two distinct non-torus, non-sphere charts whose centre is off the axis:
   the survey did not find one), else mutation-demonstrated red-then-green with the
   fixture question answered in-row (the RIMCAP shape). State which.
6. **Bit-identity**: wedge (`sf2b_axial`), lune, barrel, teapot belly, every TORAX
   row (`torax_axial`'s 14 others), `bitdump.rs`, and `shell7_dump`'s print minus its
   two klein entries (whose new lines the PR quotes) at merge base vs head —
   identical; any moved bit is a finding.
7. **Rigid re-pose parity**: the hollowed-to-check-7 elbow's minted spiric carriers
   re-posed by `transform` equal the carriers of the re-posed operand's own hollow,
   field by field within 1 ulp (the `torax_the_lune_cavity_survives_a_rigid_re_pose`
   shape, on the cavity BEFORE tier 3 — whose only refusal at
   `validate_geometric_structural` is check 7's closed-form `VolumeUncomputable`).
8. **`Interval` lane**: `torax_interval`'s klein row flips to the §4 door at
   `T = Interval`, and a sibling encloses the minted rim: `eval` at bracketed `v`
   contains the f64 point, residual enclosures straddle zero. Every new `decide`
   site executes there (the file's law).
9. **Pcurves**: the cavity torus wall's spiric half-edges certify (`statement =
   SpiricIdentity`, `max_residual ≤ ε`); the plane cap's derived image certifies
   `MapResidualClosedForm`; `validate_pcurves` clean. Kills: `sense` flipped, `u0`
   off by `1e-3` (residual `≥ (R−r)·1e-3` at every sample).
10. **STEP**: export the spiric-carrying body → one `B_SPLINE_CURVE_WITH_KNOTS` per
    spiric edge, bound `≤ ε/4`, the sentence in `FILE_DESCRIPTION`; re-import lands
    at the §5 door, payload quoted. Kills: a node-count cap ignored.
11. **Census refusals**: one row per typed arm reachable through a public door
    (`gate_operand_edges`, `pcurve_entry`, `loop_vector_area`, `torus_boundary`),
    payload asserted; unreachable arms documented at the site, not rowed.

## 7. Fences

- No boolean wall moves (klein 3/4, teapot 2/3, lily); `gate_operand_edges`
  refuses the kind; no join/section/pierce arm.
- No `route` change — not the rung, not the flag, not the note.
  `plane_torus_section` keeps refusing the tilted pose.
- No ring work (C9 `sqrt` is its own `[ev]`); no `Approx`, no fit anywhere but the
  export-only spline; no props lane — `loop_vector_area`/`face_flux` refuse typed
  naming PR-2.
- `docs/DESIGN.md` D3, the sentence "Same design for curves (line / circle /
  ellipse / NURBS)." becomes "(line / circle / ellipse / spiric / NURBS — the
  spiric is the axis-parallel plane×torus section, one oval, in the torus's own
  minor angle)"; `crates/geom-brep/README.md` C1's rung-2 sentence names
  `Curve3::Spiric` beside `Ellipse` and C4's lane list gains `Spiric`;
  `docs/KERNEL-VERBS.md` shell row's elbow clause moves to the props door;
  `offset_axial.rs`'s kind law gains its exception; the audit gains §1's rows.
  All ratified by #1858 §0a and ride this PR.
- Filed, not built: the trimmed lane's torus/plane roster (MESH, at dispatch);
  the klein demo's disc-revolve re-authoring and `torusvessel`'s third panel wait
  for PR-2 (recorded on `c5-plane-torus-cone-cylinder-arms` rows 3/4/8).

## 8. STOP conditions (pre-registered)

1. The opening measurement (both elbow rows, the two-arc row, the sectioned
   vessel, at the head) lands on a door other than §4's chain — in particular a
   `reauthor` refusal (the authority is measured `Derived`; if not, STOP) or a
   check before 7 — and the payload is not one a typed arm in §2 names.
2. A consumer in §2 cannot refuse typed without a public-type widening.
3. The wall pcurve's certification needs a bound the schedule cannot state — e.g.
   the structural field comparison fails at `Interval` because the door minted the
   carrier and the chart from different brackets.
4. Any bit moves in row 6.

## 9. PR shape, order, obligations

One PR, commits in this order so every intermediate head measures: (1) the variant
and its `geom` rows; (2) the census arms (the compile break is the checklist);
(3) `mint_carrier` + `param_on` + the four suite flips + the opening/closing
measurement in the PR body; (4) STEP; (5) the `Pcurve` variant and its rows; (6)
docs and audit rows. `docs/prompts/implementer-discipline.md` binds: hosted CI is
the verification of record (twelve test jobs, five k-lint rows), no `CI-Config`
trailer, own `CARGO_TARGET_DIR` outside the tree, foreground polling, the sweep's
blind spot stated, findings filed on their programs' slates in the same PR.
No Co-Authored-By (blinded implementer). Do not merge; the orchestrator does.

## 10. Open questions for the orchestrator

- Split? §0's finding: the elbow reaches check 7 without §3. Recommendation: keep
  one PR (Ev ruled Q3(i) into PR-1), but if the dual's load argues otherwise, §3 +
  §5 cut cleanly as PR-1b with the intermediate door already measured.
- `readback::edge_pose` on a spiric: the pose above, or `NoCanonicalFrame`? The
  spec chooses the pose; a reviewer may prefer the refusal.

## 11. What this lane could not verify

The §4 door chain is read from the source, not executed (no cargo run in this
lane); the cap-vs-wall arena order at check 7; whether a genuine latitude fixture
for row 5 exists; the Python mirror of `CurveKind`.

---

## 12. Rulings at ratification (CURVED orchestrator, 2026-09-13)

Ratified as written, with these answers to §10 and one re-cut of §9:

1. **Two PRs, not one.** The unit is H on breadth alone (~72 source
   arms, ~99 test-side hits, four suite flips, a new pcurve lane with
   thirteen external consumers), and §0 measured that the elbow reaches
   check 7 WITHOUT the pcurve variant. So **PR-1a** = §1 (the variant
   and its `geom` rows), §2 (the census arms), §4 (`mint_carrier` +
   `param_on` + the suite flips + the opening/closing measurement),
   §7's doc edits and audit rows, rows 1–8 and 11; **PR-1b** = §3 (the
   `Pcurve::Spiric` variant and its certification/mint) + §5 (STEP)
   with rows 9–10, opening after 1a merges. Ev's Q3(i) ruling puts the
   pcurve variant in this UNIT; it is its second PR. Each PR is its own
   v6 dual and its own A/B slot (1a = block CURVED-B2 slot 1, FABLE;
   1b draws next). Branches `curved/spiric-1a`, `curved/spiric-1b`.
2. **`readback::edge_pose` on a spiric answers the stored frame as a
   pose** (the six fields ARE a canonical frame; `NoCanonicalFrame` is
   for kinds without one). A reviewer who prefers the refusal argues it
   in the dual.
3. **Row 5**: mutation-demonstrated red-then-green is accepted, with
   the fixture question answered in-row (the RIMCAP shape); if the
   implementer finds a public-door latitude fixture, it is preferred.
4. **The MESH finding** (the trimmed tessellation lane has no torus or
   plane arm, so a spiric-bounded wall cannot mesh after this unit) is
   filed by the orchestrator at ratification as
   `work/issues/trimmed-tessellation-lacks-torus-and-plane-arms.md`
   (S-MESH's crate); PR-1a's typed frontier row names it.
5. **Pre-log stands: PR-1a H / STRUCTURAL; PR-1b M / STRUCTURAL**
   (the pcurve certification is a structural identity check plus one
   closed-form residual; the STEP spline is fitted for export only).

**Opening act (PR-1a, before any code)**: the §8 STOP 1 measurement —
both elbow rows, the two-arc row and the sectioned vessel at the head,
payloads quoted; the authority of the rim measured `Derived`.
