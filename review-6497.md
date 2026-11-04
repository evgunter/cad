# Review R2 — PR 6497 (`spiricarm/spindle-apex`), Surface::Spindle stage 2

**Frozen head reviewed: `c7142ee5772809d2720dfcfbb84d15293cbd6d04`.** Brief sha256 `ed4748b2…f753f6`, verified.
Probes: branch `spiricarm-rev6497-r2/probes` (`1856683b`): `crates/sweep/tests/review_6497_r2_probes.rs`, `crates/mesh/tests/review_6497_r2_cert_probe.rs`.
Mutants were run through an `R2_MUT` env switch in a scratch tree; they were reverted and not committed.

## Verdict: **APPROVE-WITH-FIXES** (0 MAJOR / 3 MINOR / 6 NOTE)

Nothing that builds is mis-measured or mis-meshed silently in anything I executed. Every off-suite shape I built either measured exactly or refused loudly.

## Findings

**MINOR 1 — `sweep::revolve::axis::classify_segment`: the horn snap moves an apex by `√(2r|d|)` for a profile drawn apex to apex** (sure, executed).
The PR's reason for `R = r` is symmetric, and the snap only moves the failure to the other drawing. I drew an apple near the horn from apex to apex, as two half arcs (one arc is refused `NearFullArc`): `R = 1 − 1e-10` and `1 − 1e-11`, with `r = 1`.
- The profile validates, `ArcReach` decides zero, and the revolve stores `R = r` exactly.
- The two apex vertices sit at `y = ±1.414e-5` and `±4.47e-6`, against a band of 1e-9. The cusp is at the origin.
- Tier 3 refuses `SheetApexReached` ×4, and `tessellate` refuses `UnsupportedCurvedDomain`.

So the result is loud and refused; on main this profile refused too (`SheetApex`). But the `SheetApexReached` recourse ("end the face at the apex, as a vertex") tells the user to do exactly what they did. At `1 − 5e-10` the profile itself refuses `path_junction_turn` in band, so the reachable window is roughly `d ∈ (1e-12, 5e-10)`.
- Claim 1 ¶1: the sphere arm does store its centre on the axis (`surfaces.rs:50`, `foot3`), but there the poles move by `O(d)`. A sheet's apexes move by `O(√d)`, so this is not the same practice.
- ¶2 upheld: the datum moves by at most `|d| < z`.
- ¶3 upheld: nothing is certified against the snapped geometry silently.

**MINOR 2 — `geom-brep::props::curved::sheet_material_sign`: a new guard with no red row, and one arm swallows an undecided reading** (sure / likely).
The unit rule says every new guard ships with a row that goes red when its check is removed. Two of my mutants survive the whole PR suite (the football, needle and ladder files, the `door_roster` and `cert_spindle` rows):
- (a) `(Ok(left), Ok(_)) => Ok(left)` in place of `Unencoded`;
- (b) one closing reading answering `Unencoded` instead of its side.

So no row shows that the needle's side is encoded or that the football's is not. Separately, `(Ok(side), Err(_)) | (Err(_), Ok(side)) => Ok(side)` takes the closing reading's side whatever the other reading's error was, including an in-band escalation, where the truth may be "both close", i.e. no side. Against [loud-failure] I would want the `Err` read by kind (likely).

**MINOR 3 — `work/spiricarm/a-football-revolved-part-way-refuses-at-its-cap-meridians-certificate.md`: P3 is under-banded** (likely).
The row is accurate, and `a_football_revolved_part_way_refuses_typed` is green (2.4 s). But a user-drawable `Partial(θ)` lemon or apple refuses `RevolveError::Op` with a message that reads as a kernel defect. `work/README.md` puts "verb breadth beyond the everyday shapes" at P1, and P0 includes "something the GUI cannot author at all". P3 ("library usability…") does not fit; I'd band it P1, or P2 at the least. The horn's `one_segment_loop` row (claim 7) is the same refusal, and it is filed in that row's "same refusal class" paragraph.

**NOTE 1 — PR body / `validate.rs` check 6: the football's faces are not read through a seam region** (sure, executed).
`topo::props::read_seam_regions` reads only `Surface::Sphere` faces (`props.rs:4789`). A sheet face whose material sign is `Unencoded` finds no region and is exempt (`validate.rs:8113-8117`). The body's "Check 6 then goes to the seam region's reading, as for a sphere's lune" is false for sheets. The bit is still guarded elsewhere:
- one face's bit flipped: `LaminaWedge` ×2;
- both bits flipped: `NegativeVolume`;
- `revert()`: `NegativeVolume` (lemon and apple, probes).

So "no side" is a documented residual exemption, not a refusal or a silent default, and the volume stays right.

**NOTE 2 — PR body, `chart_iso_family`: "Every other caller passes `own_interval`" is inaccurate** (sure).
The only non-test callers are `edge_join.rs:553` and `:563`, and both now pass `c.params()`. So the span change applies to every surface kind's joinable check, not only to sheets. `Placement::own` is `over(own_interval)` (`pcurve_cache.rs:8922`), so the rest is a real behaviour change for every partial edge. CI is green; I did not probe other kinds (unsure whether any reading moved).

**NOTE 3 — `the_football_revolves::a_football_restored_on_the_other_sheet_is_refused` cannot go red through any apex reading** (sure, executed).
It is refused by edge certification: `EdgeCertification`/`Pcurve` `CarrierOffChart` ("the carrier's points are not on the spindle sheet's revolved circle"), margin 1.0 on the lemon and 0.8 on the apple. It asserts only `is_err()`. It meets the spec's letter, but it is not the wrong-apex witness the body sells.

**NOTE 4 — further surviving mutants** (sure, executed). Each turns no row red:
- (c) the apex `Joint`'s `flat: (pb − pa).norm()` → 0;
- (g) `fold_torus_pieces` allowed to join across an apex (drop `a.apex_after.is_none()`);
- (h) the apex `EdgeTurn::exact` lever → `minor`.

(g) is unreachable while a sheet face's two meridians are different circles. (c) and (h) are closure tolerances with no witness row.

**NOTE 5 — the near-horn ladder at `d = ±z/2`** (likely). Props reports the horn's values with pad 0, while the user's `R` differs by `|d|`, about 1e-8 absolute in volume. That is within the band-scaled contract and matches the sphere snap. The test's `2|d|·max(A/V, 1)` allowance is an argued fudge with no closed-form derivation at the claim site.

**NOTE 6 — `editor-core::eval::measure` (`measure.rs:414-415`)** (sure). A spindle sheet face (the football) falls to `(Some(_), _) => "a free-form face"`. This predates the PR, but the face is newly buildable.

## Claims
1. **Partly falsified.** ¶1: the snap is the sphere's practice in form, but not in what it moves (MINOR 1). ¶2 and ¶3 upheld.
2. **Upheld.**
   - Mutant (i), the old at-vertex `SheetApexReached` arm restored, reds the football/apple, the needle and `a_sheet_apex_at_a_face_vertex_passes`.
   - Mutant (j), `singular_at` without the sheet, reds the football/apple and the needle.
   - `chart_iso_family` reads the edge's span (with NOTE 2).
   - A non-vertex apex refuses: my near-horn apple gets `SheetApexReached`, and the roster's interior and inside-edge rows stand.
3. **Upheld.**
   - The volume is certified correctly on the lemon (×2), the apple (×2) and the needle. Apples at `R = 0.9`, `0.999` and `1 − 1e-7` measure exactly: `|ΔV| ≤ 4.3e-14`.
   - "No side" is an exemption (NOTE 1). Wrong sides, injected by flipping bits, are refused.
   - No lune narrower than π is buildable, so the case where the side rules' regions differ is unreachable.
4. **Upheld.**
   - The sheet arm takes `pole_columns`.
   - My `cert_spindle` fuzz: 200k triangles with the apex corner in a random slot; 20% horn (`R = r`), 10% `r(1 − 1e-9)`, 10% needle lemons `−r(1 − 1e-6)`, the rest `±0.999r`; `Δu ≤ 1.5`, `Δv ≤ min(v*, 1.5)`. 0 violations; worst distance/bound ratio 0.99999 (tight but sound).
   - Mutant (k), `Chart::poles` without the dedup, reds the needle and the roster horn row.
   - Mutant (m), the apex's `v` dropped, reds the cert fuzz.
   - admesh on the exported `lemon.stl` (1152 facets): 0 disconnected, degenerate, reversed or fixed; one part.
5. **Upheld.**
   - All six listed mutants are red, as the body's table says: (e) the side flip, (j), (k), (l) snap removal (the needle and the ladder), (i) and (m).
   - Seventh mutants that survive: (a), (b), (c), (g) and (h) (MINOR 2, NOTE 4).
   - The oracles are independent: Pappus and `π∫ρ²dz`, re-derived by hand. The wrong-sheet row is refused at tier 3, but by edge certification (NOTE 3).
6. **No hit mis-handles an apex on the football's or the apple's path.** My sweep, differently shaped from the PR's: every non-test line naming `{Surface,SurfaceKind,ChartKind,ShapeKind,ChartArm,Kind}::Torus` with no `Spindle` within ±40 lines, `#[cfg(test)]` tails cut. That gave 135 hits. On the path:
   - `face_normal.rs:346` (an explicit `Spindle → Ok(None)` 66 lines on);
   - `sector_face.rs:387` (the catch-all `Unsupported`, loud, on the splitting door);
   - `tangent.rs:213` (`tangent_certificate_lane` omits sheets, so a sheet tangency is neither demanded nor certified; conservative, unsure);
   - `measure.rs:414` (NOTE 6).

   The rest are boolean, offset, blend and STEP-import doors, which a sheet is refused at upstream. The sweep is blind to a numeric `ρ = 0` floor and to kind tests behind helper predicates.
7. **Upheld.**
   - `revolve_errors`: one row removed (the profile builds).
   - `one_segment_loop`'s horn row `SheetApex → Op`: kernel-defect-shaped, and filed (MINOR 3).
   - The ladder's allowance (NOTE 5).
   - The roster and `REVOLVE_ERROR` rows, and the pncad tag, moved only with the variant.
8. The filed row is accurate, but the band is not (MINOR 3). The wide-apple evidence is accurate: the lower needle `−π ≤ v ≤ −π/2` passes tier 3 and measures exactly (`ΔV = 1.6e-15`), and `tessellate` refuses `UnsupportedCurvedDomain { off_bbox: 31 }`. The upper needle meshes. No silent mis-measure was found.

## Style
Questions exercised: Q1, Q2 (prose sweep over the added lines: only "as the sphere's / as a pole" analogies), Q3, Q4, Q5, Q6 and Q7. **Q8 only partly**: I read `sheet_material_sign`, `sheet_apex_jumps`, `torus_chart_loop`, `fold_torus_pieces`, check 6's singularity arm and `cert_spindle` whole, not the whole of `validate.rs` or `curved.rs`.
- `topo/src/chart.rs:209`: `dedup_by(|b, a| (a.1 - b.1).norm() == 0.0)`.
  - It is an exact-float structural rule, it runs for every chart kind, not only horns, and it keeps `+π` only because `singular_points` lists the up apex first. That order is unstated and unguarded. **likely.**
  - A horn read from another door at `R = r(1 − ε)` lists two poles at `±(π − √(2ε))`. **unsure** whether that matters.
- `props/curved.rs`: the `middle` closure (`carrier.eval((t0 + t1)·½)`) is written twice, in `sheet_apex_jumps` and in `torus_chart_loop`, and the latter rediscovers "is this end an apex" by index arithmetic on the previous piece. That is a parallel role, with one home to drift from. **sure.**
- `sheet_material_sign`'s doc: "Where neither does, the face's own reading refuses, as the left side's". This silently picks the left side's error to report. **likely.**
- `sweep/tests/one_segment_loop.rs:903`: the `apex` closure now accepts any `E::Op { .. }`, so a different operator refusal on that row stays green. The closure's name also no longer fits (the row is a pinch). **likely.**
- `the_football_revolves.rs`, the part-way row: it pins a kernel-defect refusal as expected behaviour. That is fine with the filed row, but it will go red when the row is fixed, which is the intent. **NOTE-grade, sure.**
- `sweep/src/revolve/mod.rs:1057` and `geom-brep/src/pcurve_cache.rs:9476`: doc lines left unwrapped after an edit. **sure** (trivial).
- `geom-brep/src/tangent.rs:213`: `round` names `Torus | Cone` and not `Spindle`. A class omission in a shared predicate, of the kind the unit's census rule targets. **unsure** whether it is intended.
- The class to sweep (class-not-instance rule): sheet-blind `Sphere`-only gates in the curved sense machinery. `read_seam_regions` is one (NOTE 1); look next at `sphere_region_material_sign`'s callers and `CurvedSenseInverted`'s exempt list. **likely.**

## Disclosures
- No glimpse of any other `spiricarm-rev6497-*` branch, or of the PR's comments or reviews. I read only the PR body, via `pull_request_read get`.
- Nothing was posted to GitHub.
- Builds and admesh ran `setsid`-detached and were polled in the foreground; none is left running.
- Hosted CI was not re-run. My runs are targeted nextest filters at the witness tolerance only, not the 1e-6 or 1e-12 rows.
