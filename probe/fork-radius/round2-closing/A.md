## For Ev

**Recommendation (likely).** An arc's radius is the document's quantity carrying the sign its construction decided, as a literal ±1 in the spelling: `r` for the circle and the fillet, `σ·ρ` for the tangent, via and bulge arcs. No `abs`, nothing told to the symbolic tier, the tier unchanged. Readers that compare a measured distance with a stored radius, or take an angle of a radial vector, use the stored radius in a form that needs no sign. Every registration this question produced retires. Both designers now recommend this. It is your route B ruling (#3186: `4·atan(σ·b)`, not `4·atan|b|`) applied to the radius.

Terms. *Symbolic tier*: the certifier's algebra that proves a residual zero as an expression in the document's variables, at any box width. *Box*: the range a study gives a variable. *Gate*: a decision whose other answers refuse. *Radial*: the unit direction centre → vertex the profile records. *Registration*: a site telling the tier two expressions are equal, an axiom resting on that site's proof (E12). *Strut*: the straight edge an extruded vertex sweeps.

**What was measured** (the tour's plate over its full study box, one leaf):
- With `r` stored and the doors reading the radial, the arc's carrier start, carrier end and its description against its carrier all certify with no door registration. This was the recommendation's central claim. (confirmed)
- The first refusal is then outside the profile and the doors: the pcurve cache's test that the strut lies on the hole's wall, which reads the exact distance `‖p − axis‖ − R`, here `√(r²) − r`.
- With `|r| ≡ r` registered instead (the other route), the refusal is the same cache's angle for the strut, `atan2(0, r)`: the same sign in a form no registration reaches. This confirms rejecting the registration.
- Every verb that builds an arc stores `|·|` of a signed quantity whose sign it had already decided.

**Two corrections to what I said before.** I predicted the verb's own read-back would refuse first; it did not. I said the kernel's on-surface residuals are the sign-free squared form; that is true of one function (`implicit_residual`), and the exact-distance form `‖w‖ − R` is written at twenty-odd other places across `geom-brep`, `topo` and `sweep`.

**Neither measured reader needs the sign.** Each holds the surface's radius:
- *Distance.* `‖w‖ − R` is `(‖w‖² − R²)/(‖w‖ + R)`. The numerator is a polynomial, zero whenever the point was built on the surface. The kernel already has this spelling, written for this reason (`norm_gap`: "so … `√(p²) = p` is never asked of the arithmetic"); the cache's incidence test does not use it.
- *Angle.* The angle of a radial vector `w` is the angle of `w/R`. For the strut that is `(r/r, 0) = (1, 0)`, a constant, angle 0. The value is unchanged because `R` is positive; with `|r|` stored it would be `r/|r|` and not fold.

Both are re-pointings of a reader to something it already holds. No new stated datum is needed, so my earlier suggestion that the door state the edge's image is withdrawn. (likely; by hand algebra, not run)

**The design, as final state.**
- One rule for constructions: a length whose sign a decision holds is spelled with that sign as a literal. `arc_span` and `literal_sign` already do this.
- One rule for readers: the stored radius is the length. A measured norm is compared with it only through the one sign-free gap, and a direction is made unit by dividing by it. One function for each, used everywhere ([one-way]); two open items already ask for the first (`point-to-carrier-distance-is-spelled-seven-times`, `a-points-radial-offset-from-an-axis-is-spelled-inline`).
- `centre + r·x` is the vertex at every value of `r`, so the arc's facts are identities, and no registered fact is conditional on a gate. In the measurement the surviving rim registrations were recorded as `√(r²) ≡ r`, which is false for negative `r` and which the witness cannot catch; they go.
- Lanes: at `f64`, `Interval` and `Dual` nothing changes by a bit where the gate decided positive. At `Sym` the expressions change. Eager and deferring sessions agree because nothing is registered or read.

**What it leaves possible.** A reader written later as `w.norm() − radius` on an identity will refuse over a wide box until it uses the gap function. That is loud, in the safe direction, and a source gate can forbid the raw spelling outside the one function.

**The alternative kept out: a record of decided signs in the tier.** Sound (a definite verdict is a proof about the whole leaf), and it would make every reader fold as written. Not recommended now: no measured reader needs it; what it proves depends on which decisions came first, so eager and deferring sessions can differ in reach; it adds tier work to decisions the numeric check already settles; it changes E12 and `[intent.structural]`; and you declined its nearest relative on #3283. I would not build it behind an off switch either: file it as an item, to open if a reader appears that holds nothing sign-free to read. (unsure, less so than before)

**Ratified text.** No decision changes. E12's Decision paragraph says a symbolic Zero holds "for every parameter value" and a registration records "one function of the parameters"; DESIGN.md's structural-coincidence clause says "for every value at which the construction takes the decisions it took here (E12)". Both designers ask that E12 say which it means. Under this recommendation the arc satisfies the strict one. E12's description of the plate's chart phase is re-worded with the code.

**Confidence.** The spelling: likely (its core is measured). The two re-pointed readers fold: likely. Nothing further stands behind them on the cut plate: unsure, not traced past the first refusal. Every magnitude site has a literal sign in hand: likely, with the bulge caveat sent to the orchestrator.

## For the orchestrator

**1. Measurements.** (b) confirms. (a) confirms rejecting C. Overturned: my read-back prediction, and my "squared form" generalisation.

Is the azimuth the reader that needs the sign? No: `Azimuth::of(w_r·cv / R, w_r·u_ref / R)` is sign-free once `R` is stored signed. Check when cutting: that rule E reduces `r/r` to 1 before the `atan2` fold sees it (A1 folds zero over a form non-negative by syntax; a positive constant qualifies), and that `place.phase` adds nothing symbolic to a constant angle. If the tier does not fold `atan2(0, 1)`, that is a one-line tier gap, not a sign.

Root readers: `chart_distance` (pcurve_cache.rs, cylinder `(w.norm() − radius).abs()`, sphere, torus) is the measured one. A crude grep on `doors-read-arcs` for `norm() − radius` / `rho − radius` finds, non-test:
- `pcurve_cache.rs` ≈12599, 12608, 12802
- `curve_pair/pose/window.rs` ≈890; `curve_pair/meet_rows.rs` ≈84; `props/curved.rs` ≈1839
- `geom/surfaces.rs` ≈1968, 1972
- `sweep/blend/reach.rs` ≈504, 3664; `blend/surgery.rs` ≈2700
- `topo`: `boolean/carrier_touch.rs` ≈187, 320; `contain.rs` ≈960, 1046; `torus_region.rs` ≈511/514; `section_cert/ruling.rs` ≈409; `bound_side.rs` ≈2222; `injective.rs` ≈4690; `offset_axial.rs` ≈408, 2962; `validate.rs` ≈10490

I did not classify which are identity readers on a certified path and which are real margins (a real margin needs no change: it is not zero). The already sign-free spellings to copy: `norm_gap` (pcurve_cache.rs ≈6600), `(rho − r)·(rho + r)` (`projected.rs` ≈2461, `circle_lane_rows.rs` ≈545).

Angle readers: the `Azimuth::of(x·cv, x·u_ref)` family in `pcurve_cache.rs` (≈11092, 11138, 11281, 11403, 11516, 11615, 11763, 12736), `edge_nurbs.rs` ≈1322, `curve_face/circle_torus.rs` ≈587, `curve_pair/pose.rs` ≈1541. Only those whose vector is `r`-scaled and on the chart matter; the measured one is the cylinder ruling arm (≈11092).

**2. Sites and what each stores.**
- `circle_loop`: `r` (gate `PathCircleRadius`).
- `fillet_arc`: the authored fillet radius, no `abs` (the constructors' gates).
- `tangent_arc` and the via/onto arm: `σ·ρ`, σ the literal of the decided side.
- `sugar.rs` offset carriers: `sgn·side·ρ` with those literals; intermediate only.
- `from_chord_radials` / `from_chord` / `lowered`: `σ·L(1+x²)/(4x)`, σ the bulge's sign. Caveat: σ comes from `literal_sign` (`choose`, F43), which returns `None` at a tie, and `from_chord` passes `T::one()`. The lowering should take the sign as a required argument from the caller that decided the turn (the path door's bulge decision; `swept.rs` `arc_from` holds `turn`), and the tie arm should refuse as that decision does: an arc with an undecided turn cannot validate anyway. I did not confirm every `bulge_leg` caller sits behind a decided bulge; if one does not, that caller is missing a decision. Test-support lowering at `f64` uses `signum`.
- `center_arc`: radius is the measured `‖a − c‖`; keep.

**3. Units, in dependency order.**
- **U1 (one unit): the readers' two functions.** A sign-free radial gap (promote `norm_gap`'s spelling to one shared function) used by `chart_distance` and the profile's own `rim − radius` reads; the chart azimuth of a radial vector divided by the chart's radius. Lands first and is behaviour-neutral at `f64`/`Interval` under today's `|r|` (the gap is the same real; `r/|r|` simply does not fold yet).
- **U2 (one unit, needs U1): the verbs store the signed spelling** (all sites in 2), `from_chord` takes its sign, `Arc2.radius`'s doc. In the same unit retire `register_radials`, the rim pair of `register_endpoints`, `register_rigidity`, `register_placed_carrier_ends` and their allowlist rows, since each is false or redundant the moment `r` is stored. `carrier_end` and `mint_radial` divide by the radius or go. Acceptance: the item's four tests; then trace the cut plate past the hole's extrude and file what stands next.
- **U3 (separate, small):** the source gate forbidding raw `norm() − radius` outside U1's function, and a sweep of the grep list above for identity readers.
- **U4 (text, with U2, descriptive):** E12's plate passage. The E12 "which wording" question goes to Ev on the `[ev]` PR.
- **Filed, not built:** the decided-sign record.

**On B's correction.** I did not concede it: on `doors-read` at `fc11523f`, `register_radials` returns `[(&'static str, SymRegistration); 8]` (four points, per component), so B's "four" holds only if the branch changed it since. Either count is immaterial: it retires.

**Not checked.** Nothing run. The re-pointed forms are hand algebra. The grep list is unclassified. Whether the boolean re-derives the strut's pcurve through the same cache arm (so U1 covers the Subtract too) I did not trace.
