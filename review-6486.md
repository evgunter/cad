# Review of PR 6486, reviewer r2: the football with conical ends builds

**Frozen head reviewed:** `49d88cc0f11bc1cce1ef9ef4e8a479263af7fd92`. Probes: branch `spiricarm-rev6486-r2/probes` (`d35c463e`), all green on the head.

## Verdict: APPROVE-WITH-FIXES

Every claim held under execution. The sheet is classified, built, walked, measured and meshed correctly on every profile I could build, including ones the PR's oracle does not cover: concave walls, partial revolves, near-horn offsets and the wide apple. The fixes are stale prose that still names retired decisions (MINOR-1). Nothing is MAJOR.

## Probes (all on the probes branch)
- `geom-brep projected::r2_spindle_meters_probe`: fuzzes `Meters::Spindle::over` with 400k points and boxes seeded on this sheet, the other sheet, the apexes, the axis and at random, at scales 1e-9 to 1e-1. Each certified result is checked against the exact meridian-plane distance to *this* sheet's arc (apex endpoints included). 192k certified, none violated, worst dist/bound 0.50 (f inflated 2×). **Mutants:** dropping `positive(minor·ρ − d·|R|)` goes red immediately. Flipping σ stays green (see NOTE-4).
- `mesh cert::r2_cert_torus_is_sound_on_lemon_and_apple_sheets`: 20k random triangles inside `|v| ≤ v*`, signed `R ∈ (−0.98r, 0.98r)`, against the parametric deviation. Sound.
- `sweep r2_probe_6486` (6 rows):
  - concave lemon and apple walls (arc traversed clockwise, material outside the tube): build, tier 3, volume and area exact to 1e-16 relative, mesh closed;
  - an apple arc and a lemon arc dipping across the axis with both ends clear: `ArcCrossesAxis`;
  - near-horn ladder `rc − r ∈ {±1e-3 … ±1e-14, 0}`: `|d| ∈ [3e-9, 1e-8]` refuses `SliverAxisClearance{ArcReach}` on both sides. `|d| ≤ 1e-10` builds a horn-class `Spindle` (R up to r + 1e-10), passes tier 3 and measures exact. `d ≥ 1e-6` gives a torus;
  - partial revolves (1, 2 and 4 rad) of the lemon, the apple and the concave lemon (meridian edges on a sheet, plane caps): tier 3, props exact to the `θ/2π` share plus caps, mesh closed;
  - the wide apple `R = 0.8, |v| ≤ 2.4`: builds, tier 3, props exact (15.8791944…), and `tessellate` refuses `UnsupportedCurvedDomain{off_bbox: 54, first_uv: (3.03, 2.4)}`, exactly as filed;
  - the oracle's pads: `volume_pad = area_pad = 0` on the lemon and the apple, so the 1e-10 assertion is tight.
- **Tier 3 mutants** (in `/tmp`, reverted; recorded here): with the walk unwired, all four `door_roster::a_sheet_*` rows go red, and so does `a_conical_ended_football_revolves`. Making `Some(Ok(true))` pass turns `…inside_a_face_is_not_a_vertex` red. `g = r(Rv − r sin v)` turns `…shown_clear…passes` red. `g = r(|R|v + r sin v)` survives, but it is equivalent: on the sheet both slopes are ≥ 0 and the walk reads only the area's sign.

## Findings

**MINOR-1 — retired decisions still named in prose** (`sure`). Each is a premise the change invalidated (Q4), and in each case the doc rotted while the code is right:
- `crates/sweep/src/revolve/mod.rs:1074-1076` (`revolve`'s `# Errors`): "unsupported toroids", a variant that no longer exists;
- `mod.rs:554-556` (`RevolveError::ArcCrossesAxis`): "span beyond the half-period around an on-axis center, or an apex on the negative side". That describes the retired `ArcSpan`/`ArcApex` pair. The variant is now `ArcInterior`'s, and covers sheet arcs too;
- `mod.rs:1145-1150` (doc of `a_revolve_escalation_ends_by_its_decisions_kind`): "the arc span are angles… The arc's apex and clearance are distances". Those decisions are gone;
- `axis.rs:186-188` (`radial_extent`): "Membership is the chord half-plane test the arc classes use". The arc classes now use the turn-to-`m` reading; no arc class uses a chord test.

My sweep covered `rg 'toroid|ArcSpan|ArcApex|ArcClearance|arc_span|apex|clearance'` over `crates/sweep/src` plus a prose grep across crates and docs. It cannot match a paraphrase like "the sphere's span check". `docs/k-report-data` is disclosed in the PR.

**NOTE-2 — `classify_segment`: a sheet arc that ends on the axis *and* dips across it refuses `SheetApex`, not `ArcCrossesAxis`** (`likely`). `axis.rs:571` (the `va.pinned || vb.pinned` check) runs before `interior_clear()` (`axis.rs:580`). The arc rule reads the interior first. `SheetApex`'s recourse ("end the arc off the axis") then leads to a second refusal, `ArcCrossesAxis`. Both refuse typed, so this is not a soundness issue.

**NOTE-3 — the LATHE closure** (`unsure`). The closed row had two halves. The definite `UnsupportedToroid` arm is truly gone. The in-band half ("ends in `COINCIDENCE_RECOURSE`, not the convention's words") persists: `ArcReach` is still `rc − r` under the revolve's own name, ending sized. The ratified arc rule ("the near-horn band refuses as a sliver") arguably settled that half. The `## Closed` text should say so rather than claim the decision retired outright.

**NOTE-4 — `projected::Meters::over`, Spindle arm: soundness rests on the second check alone** (`likely`). The σ weighting only chooses *where* the lever certifies. Any positive lower bound on `|M² − r²|` is sound, and the flipped-σ mutant passes the fuzz. What keeps the bound to *this* sheet is `r·ρ_lo ≥ |R|·d` (projected.rs:1734), and the mutant that drops it fails at once. The doc (1683-1694) reads as if the factor choice does the sheet selection. The PR says "written, not fuzzed". The fuzz above is a ready guard, and nothing schedules one otherwise (Q6).

**NOTE-5 — the football's mesh row does not read sheet-face winding** (`likely`). `assert_outward` (`crates/sweep/tests/common/differential.rs:978-985`) reads only sphere and cylinder faces. A sheet face meshed inward is caught only through `check_volume`'s `δ·A` bound. That bound is correct as a chordal-deficit bound, and on these bodies a flipped band would miss by far more, but it is the weaker reading.

**NOTE-6 — `ShapeKind::Sheet`'s slope `r(|R| + r)` is loose on a lemon** (`likely`). `chart_singularity.rs:1213`: on the sheet `g′ = rρ ≤ r(R + r)`, so using `|R|` over-reads by `(|R|+r)/(R+r)`, which is 3× on `LEMON` and grows without bound toward `R → −r`. It is sound, but it widens the walk's error and so its straddle refusals.

## Style (questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8)
- **Q1** `axis.rs:137, 204, 519`: `e_r = Vec2::new(dir.y, −dir.x)` is now spelled three times in one file (`placed`, `radial_extent`, `interior_clear`). `radial_extent` already says "same construction as `AxisFrame::build`'s". The new copy adds a third (`sure`).
- **Q1** `axis.rs:182-237` vs `513-537`: two spellings of "does this arc hold the carrier point `c ± r·ê_r`" in one file. `radial_extent` uses a chord half-plane `copysign` fold, and `interior_clear` uses the turn to `m` less the span. They now disagree in method, and MINOR-1's stale sentence is the only text tying them together (`likely`).
- **Q7** `projected.rs:1138-1152` + `pcurve_cache.rs:3763-3768`: `meters_at` builds a whole `Meters` from the *surface* only so the caller can read `.nappe()` back. The real `Meters` is then rebuilt from the *twin* in `twin_span_hulls`. A `nappe_at` returning `Option<Nappe>` would say what is used (`likely`).
- **Q1** `projected.rs:207` `ProjectedChart::nappe` and `projected.rs:1659` `Meters::nappe`: twin accessors with the same body shape (`likely`).
- **Q7** `pcurve_cache.rs:8926` and `9009`: `Along::Latitude` now means "a sphere's latitude, and a spindle sheet's tube angle". One window name for two angles is what the filed wide-apple row is about (`likely`).
- **Q2** `crates/mesh/src/curved.rs:1140-1141`: "Tier 3 refuses a sheet face reaching an apex, so no pole column is read". `tessellate` takes an unvalidated `Body` (the football test meshes before it validates), so the comment states an invariant this door does not enforce. Today only a forged body reaches it (`unsure`).
- **Q4** `crates/mesh/src/walk.rs:637`: "`revolve` refuses near-horn toroids at a certified bar". That is still true of the band, but a horn at the band's zero now builds (probe: `R = r + 1e-10` builds) (`unsure`).
- **Q5** `ArcCrossesAxis`'s Display ("dips across") now also answers a horn circle *tangent* at `m` (`one_segment_loop`'s "tangent away from its vertex"), which touches without crossing (`unsure`).
- **Q2** `pcurve_cache.rs:3698`: the `Meters::Spindle { major, minor, .. }` pattern has a `..` with no other fields. Its comment "the sign of `major` names the sheet" restates that the bits are keyed (`unsure`, nit).
- **Q6** "the lever's soundness argument is written, not fuzzed" and "no row puts an apex inside a sheet face's loops" are disclosed in the PR body only. The second is stage 2's. The first has no work row (`likely`).
- **Q3**: the oracle can fail. Its pads are zero, the `R`-sign row is a direct datum check, and with the walk unwired it goes red. No assertion is monotone in the wrong direction (`sure`).
- **Q8**: I read `crates/sweep/src/revolve/axis.rs` (705 lines) end to end, which found the stale `radial_extent` sentence. `validate.rs` (18k lines) and `pcurve_cache.rs` (11k) were read only around the diff, not whole.

## Claims
1. Classification: **upheld** (`sure`). The sphere's accept set is unchanged: the turn to `m` has no branch cut on a reachable start. The ring path is unchanged. Band and tangency probes are above.
2. `Meters::Spindle` sound from THIS sheet: **upheld** by fuzz (`likely`; fuzz, not proof). See NOTE-4.
3. Six sign fixes: **upheld** (`likely`). Each checked by hand: the meridian `alpha`/`rad`, `b*` as the +90° turn in `(ê_r(α), axis)`, `|R|` spine, `about_core` (now 0 on a sheet), `props_meridian_fit`, `torus_meridian_orient`. Sweep of the shape: `pcurve_cache.rs:5283` (`R+r` = widest ρ), `5997` (`|R|`), `9372` (`|R|+r`), `11127` (`chart_distance`, signed form is `m`); `props/curved.rs:4957` `torus_arms` (azimuth arm read only for the cone, 3184); `4981` and `5370` (signed, correct); `mesh` grid steps and `cert_torus`. No misses found.
4. Tier 3's walk: **upheld** (`sure`, by mutants above).
5. Props and mesh on a sheet: **upheld** (`sure` for these bodies). The closed forms were re-derived independently: `π r[R²s + Rr(v+sc) + r²(s − s³/3)]` and `2πr(Rv + r sin v)`. The certificate is fuzzed above.
6. The oracle: **upheld** (`sure`). The closed forms are independent of the code, the bands are as claimed (apple `v₁ = 1.6 > π/2`, near-apex `0.97·acos(0.5)`), the wrong-sheet row is a datum check, and `δ·A` is a correct chordal-deficit bound.
7. Re-pinned rows: **upheld** (`likely`, by inspection). Each moved only where the decision moved, and none was loosened. The removed "deliberately wrong stored cone scalars" check is now held by the signature.
8. `Option<Nappe>` signature: **upheld** (`sure`). `meters_at` takes the same `pcurve_projected_nappe` decision with the same error mapping, and the `UnsupportedChart` refusal is unchanged for spline kinds.
9. Residue: **upheld** with NOTE-3. The tour-stop row is real and accurate as filed. The wide-apple evidence was reproduced exactly. No sheet built through the revolve was mishandled silently: every one I built either measured exact or refused typed.

## Disclosures
- I did not read PR 6486's comments or reviews, or any `spiricarm-rev6486-*` branch other than my own. No accidental glimpses.
- I posted nothing to GitHub.
- The topo mutants were applied and reverted in the working tree; they are recorded above, not pushed.
- I did not run the full suites, only the targeted rows above.
