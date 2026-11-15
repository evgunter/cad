## For Ev

**Recommendation (likely).** Store an arc's radius *signed*: the document's quantity times the ±1 its construction already decided, never a magnitude taken with `abs`. Two readers in the boundary-representation crate (`geom-brep`) currently reach the radius through a square root or an angle. Rewrite them so they never take the root of an `r`-scaled quantity. Leave the symbolic tier unchanged, and register nothing about a sign.

**What was measured.** Setting: the tour's plate over its full study box, one leaf, with the sweep reading the profile's recorded directions.
- **(a) `|r| ≡ r` registered in the circle verb.** The hole's extrude refuses later than before, at the pcurve check. The pcurve's phase is the angle of the vector `(r, 0)`, which is 0 only where `r > 0`. The registration cannot reach it: there is no `abs` in that expression.
- **(b) The circle verb stores `r`, with no registration anywhere.**
  - The carrier start, carrier end and edge-description checks all pass.
  - The first refusal moves to a line-on-cylinder incidence test that reads `‖p − axis‖ − R`, which here is `√(r²) − r`.
  - The uncut plate stops at the same place.
- **(c) Every verb that builds an arc** stores the magnitude of a signed quantity whose sign a gate already decided.

So storing the sign closes everything inside the profile and the sweep's doors. What is left is two readers further down that recover the sign through a root.

**The two remaining readers.** Both are rewritten inside the reader. Neither needs a new stored value, so nobody has to state or check one.
1. **Distance from a point to a cylinder (and to a sphere)**, in the incidence test (`chart_distance`). Today it is `‖w‖ − R`; it becomes `(w·w − R²)/(‖w‖ + R)`.
   - It is the same real number wherever the denominator is non-zero: an exact rewrite, not an approximation.
   - Its numerator has no root in it. For the strut it is `r² − r² = 0` as a polynomial, so the zero is a theorem at any box width.
   - It is also better conditioned. `‖w‖ − R` loses its leading digits exactly when the point is near the surface, which is when incidence is being decided. The rewritten form does not.
   - The denominator is positive wherever the radius gate passed.
   - The kernel already has `(d² − R²)/(2R)` (`implicit.rs`), but that is not a distance. The incidence test needs the exact form.
2. **The phase of a cylinder ruling**, in the pcurve cache. Today it is the angle of the line's radial offset `w_r`, which is `atan2(0, r)`; it becomes the angle of `w_r / R`.
   - Scaling a vector by a positive number does not change its angle, so the result is the same in every lane.
   - The scaled vector is `r·x / r = x`, so the angle is a literal 0, not an opaque term the symbolic tier cannot simplify.

So the angle reader does have a structural alternative. My earlier fallback, a per-session record of decided signs, has no case today, and I don't recommend building it.

**What each site stores** (the list is sure; each spelling is likely):

| site | stores | sign comes from |
|---|---|---|
| circle, split circle | `r` | the radius gate |
| fillet arc (`fillet_arc`) | authored `r` | the fillet's positivity gates |
| bulge arc (`from_chord_radials`) | `σ·L(1+x²)/(4x)` | σ = the bulge's sign, already used for the radials (`literal_sign`) |
| tangent arc, and the through-point arm | `σ·ρ` | σ = the decided side |
| arc-and-arc fillet offset circles (`sugar.rs`) | `sgn·side·ρ` | the external/internal offset gates |

- **Bulge.** `L > 0` and `1 + x² > 0`, so the radius's sign is the bulge's; no decision is missing. A bulge whose sign ties is a straight segment within the band and should not be built as an arc.
- **Left as they are:** an arc length (`radius·|turn|`) and a margin (`|ρ| − |h|`), which are not radii. An arc whose radius really is a measured distance (`‖a − c‖`) also keeps its norm.

**Facts that retire.**
- `Arc2::register_radials`. The circle's facts become the same expression on both sides, and the bulge's and tangent arc's become polynomial identities because `σ² = 1`.
- The rim half of `register_endpoints`, and `register_rigidity`'s rim fact, for arcs that record their directions.
- The registrations the sweep's doors were adding.

Measurement (b) shows these must go, not just become unnecessary. With `r` stored, those rim facts are still accepted at `Interval`, but they then say `√(r²) ≡ r`. That holds only where `r > 0`, yet it is recorded as if it held everywhere.

**The final state in every lane.**
- `f64`, `Interval` and `Dual`: no bit changes wherever the gate passed, except that the rewritten distance rounds better.
- `Sym`, in both eager and deferring sessions: the expressions become identities in the parameters for every value of `r`, so nothing depends on which decisions were taken first.
- A box that straddles the band: the gate refuses and the box is split, as today.

**Ratified text.** No decision changes. E12's Decision paragraph says a zero holds "for every parameter value", while the structural-coincidence clause and the registration door's doc say "wherever the construction takes the decisions it took". E12 should say which it means.
- Under this recommendation the arcs need only the strict reading.
- A rim registration written under that wording was in fact conditional, and nobody could tell.
- E12's sentence about the plate's chart phase, `atan2(0, r²/sqrt(r²))`, is re-worded along with the code. (likely)

**What stays possible.** A future reader could again take the root or the angle of an `r`-scaled quantity. The rule is short: read the stored radius or the stored direction, or rewrite the root out as above. The torus arm of the same distance function is the next instance of this pattern; this change does not touch it.

**Confidence.**
- The recommendation: likely.
- Both rewrites fold for the plate: likely; not run.
- The fallback is not needed: likely for today's documents.

## For the orchestrator

1. **The measurements against my round-1 position.**
   - They confirm it: (b) shows start, end and mapped source fold once `r` is stored.
   - They contradict A's prediction that the verb's own read-back (`read_back_arc`, `rim(a) − radius`) would refuse first. It did not, and I did not trace why. Re-point it anyway, to the radial or the rewritten form, so it doesn't depend on whatever let it pass.
   - The angle reader is in `crates/geom-brep/src/pcurve_cache.rs`, the cylinder-meridian arm (`Azimuth::of(w_r.dot(cv), w_r.dot(u_ref))`). Divide `w_r` by the radius there. The cone-ruling arm's `r_dir` is already a decided ±unit vector, so it is unaffected.
   - `chart_distance` is in the same file. Its readers are `farthest_at` → `incidence_at`, plus two in `line_incidence_rows.rs`. The incidence comment's lower-bound argument still holds, because the rewritten form is the exact distance.
2. **Units to cut, in dependency order.**
   - **U1, one unit (`geom-brep`):**
     - `chart_distance`'s cylinder and sphere arms rewritten;
     - the meridian angle divided by the radius;
     - an audit of `geom-brep` and `topo` for any other root or angle of a radial offset (the torus arm, `implicit.rs` callers, the curve-face arms that use `Azimuth::of`). File what is out of scope rather than fixing it.

     It is safe on main as it stands, and should land first or together with U2.
   - **U2, one unit (`profile` + `geom-core`):**
     - `Arc2.radius` stored signed at all five sites, with its doc saying so;
     - `from_chord` takes the turn, or is removed;
     - the read-back re-pointed;
     - `register_radials` and the rim facts retired, with their allowlist rows.

     Its tests are the item's own: the uncut plate keeps certifying, and an arc's `u_ref` and rims are exact for each verb.
   - **U3, the existing doors row (`the-sweep-doors-read-the-profiles-recorded-directions`):** the doors read the recorded radials, and `register_placed_carrier_ends`' start fact and `register_rigidity`'s rim fact go. It depends on U2. The plate with its holes subtracted is the closing test for U1, U2 and U3 together.
   - **U4, an `[ev]` text change:** E12's wording. Independent of the others.
   - **Not cut:** the session sign record. File it only if a later reader turns up with nothing sign-free to read.
3. **A's round 1.** I agree with A's withdrawal and with its points against the sign record (results depend on decision order, cost, PR 6993's switch-off list, #3283). Its corrected count, about ten production sites that read the rim at a vertex, fits this plan: U2 re-points those in `profile` and U1 covers `geom-brep`. I have not seen A's full text.
4. **Unchecked.**
   - Whether the tier folds `(√X)²` to `X`. Spelling the numerator as `w.norm_squared()` avoids needing it.
   - Whether `Azimuth::of` at `Sym<Interval>` returns a literal 0 for the vector `(0, 1)`. `manifest.rs` says rule D folds `atan2(0, N)` when `N` is a positive constant, so I expect it does.
