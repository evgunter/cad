REVIEW — PR 7686 `boxfinish/signed-radius` at frozen head ddbd1deced1 (this unit's 35 commits from ff1e1293ef0), base f2815a232a

**Verdict: NOT-MERGEABLE-AS-IS.** Two reasons. (1) A row that is green on the base is red on the head at ε 1e-12: an `f64` validation decision moved that the body does not list (MAJOR-1). (2) Nothing has gated this head. Its one check run, `merge ref`, failed because the PR conflicts with main (`mergeable_state: dirty`), so CI created no run. The design holds and the headline is real: the cut plate evaluates whole, and it is this unit that does it. The fix for (1) is inside the unit's fence.

## MAJOR
**MAJOR-1** `crates/profile/src/seg.rs` `check_carrier` (the table arm's `ArcStartOnCarrier`, now `arc.off_carrier(a)`). DEMONSTRATED, sure.
- Base `f2815a232a`: profile passes 768/768 at ε 1e-6, 1e-9 and 1e-12.
- Head at ε 1e-12: `a_far_arcs_lens_reads_its_own_width::a_far_arcs_lens_past_ten_eps_wide_validates` is red. Payload: `ArcBelowSceneResolution { check: OnCarrier, margin 6.1035e-11, headroom −9.9e5 }`.
- Reverting only that one line to `arc.rim(a) − arc.radius` makes the row green again on head.
- Cause: at a radius of 1e6, `‖w‖² − r²` keeps the squares' rounding: one ulp of `‖w‖²` (1.2e-4 at 1e12) over `‖w‖ + r` = 2e6 is 6.1e-11, the margin read. The root form `rim − r` rounded to an exact 0 there. So the rationalized gap reads a resolution residue as a miss on a far-centred arc, at the band where the scene cannot resolve it.
- This is the `Consistency::Decide` path, which `Profile::validate` takes for every `ProfileLoop` (a lattice loop's `into_loop()` included). It is not test-only.
- Not in the body's `f64` list. The body's profile differential ran at 1e-9 only. The per-PR gate runs the 1e-12 row of a touched eps-sensitive crate, so a hosted run would have shown it.
- The same rounding moves into every reader re-posed on the gap: the read-back's start and through point, `circle_side`, `end_clear_of_circle`, and `carrier_line_circle_margin`'s inline `(r² − h²)/(r + |h|)`. Each wants the same re-check at far scales (BY-INSPECTION, likely).

**MAJOR-2 (gate)**: no hosted run exists for ddbd1dece. Check run 138266480756 `merge ref` = failure ("conflicts with its base … CI created NO run"); looked at the start and at the end, unchanged. I ran the lint job's steps locally (below). Every verdict below rests on those local runs, not on a gate.

## MINOR
1. **Receipts the body leaves out.** `crates/editor-core/tests/m10_bulge_interval.rs` `D_TAB_AT_THE_NOMINAL`: `arc_span` [6,4]→[4,6] (commit 85b899d64) is two more theorems lost, besides `circle_side`'s two. The bulge split's `pcurve_*_chart_incident` rows move [8,4]→[12,0] and [10,4]→[14,0]. The body says the D-tab loses two. It loses four and gains eight. The `circle_side` comment cites no row for "outruns the coefficient ring". BY-INSPECTION, sure.
2. **The body's verb table, base column, is wrong in three cells on `f2815a232a`.** At ±1e-6, door shut, base stops at `ArcStartOnCarrier` for bulge, tangent and via, not `SegmentDirection`. Circle `SegmentDirection`, onto `SegmentDirection` and fillet `CarrierLineCircle` match. Head matches every cell I took (twelve). No verb stops earlier on head. DEMONSTRATED, sure.
3. **`sugar.rs` `offset_line_circle` and `offset_circles`: "the enclosing-class gate decided it non-negative" is false.** Gate 2b refuses only a definite Negative. A Zero (ρ in band, possibly negative in value) passes, and `fillet_offset_lever` reads only `r2`. The same function still says "both ρ straddle zero in general". Dropping `abs` therefore moves `FilletOffsetLineCircle`, `FilletOffsetCircles*` and `near()` for an in-band ρ < 0 by up to 2|ρ|. That is an `f64` move the body does not list, though my 80-case fillet sweep moved no decision. BY-INSPECTION, likely.
4. **The `arc_to(Bulge)` tie arm (`BuiltArc::lowered`, `turn: None`) is untested, and is a deviation that owes a schedule.** The spec said a tie refuses. My probe at `Interval` (bulge [−0.05, 0.05], [−1e-12, 1e-12], [−0.05, 0.4]): the door escalates `PathJunctionTurn` on an Invalid margin before validation sees the arc. So the doc's "for validation to decide" describes a path I could not reach, and nothing pins what the arm stores. The evidence was added to DECIDE's exact-zero row, which is not a row for this arm. DEMONSTRATED (the escalation), sure (no test).
5. **Readers the sweep missed.** `seg.rs` carrier-circles chord `depth = (r₁ − rim(foot) + r₂ − rim(foot))/2` (radius first, so a `rim(..) − radius` grep misses it). `topo/src/boolean/reduce.rs` `junction_end_held`, `(w − axis·h).norm() − rim.radius`. Neither is in the hit list or in `a-measured-norm-less-a-stored-radius-…`'s sites (file the topo one on its owner). BY-INSPECTION, sure (missed); unsure (the effect).
6. **The two flagged-and-kept fillet sites have no row.** `sugar::tangent_point`'s `copysign(rho)` and `line_line_fillet_trims`' setback `radius·half_tan.abs()`. The second is the design's `σ·…` case left as an `abs` atom. The "fillet row" (`fillet-side-and-centre-arcs-…`) names neither. BY-INSPECTION, sure.
7. **`register_placed_carrier_ends` (swept.rs) says its facts "hold for every carrier, a copied or table one included".** That holds only while `spun_onto`'s measured mint keeps the frame node for node.
   - At an inexact-witness symbolic scalar (`Sym<f64>`, a lane editor-core uses; whether a sweep runs there I did not trace), a radial off unit past its rounding is squared. The registered "carrier start ≡ place(c + r·radial)" is then false by the squaring.
   - At `Interval`, a table radial off by 1e-15 builds a wall whose stored `u_ref.x` is the point [1.000000000000001], an enclosure holding no unit (DEMONSTRATED). Stretches of 0.1·ε and 0.4·ε poison the frame and refuse typed (DEMONSTRATED).
   - Reachable only through the test-only `new_directed` today. That is a premise for `UnitVec2`, not a fact the doc may state. BY-INSPECTION for the `Sym<f64>` half, likely.
8. **A negative stored radius (table door) moved from `InconsistentArc { OnCarrier }` to `Escalated { ArcStartOnCarrier, Invalid }`**, the 0/0 mirror. Nothing reads "on", which is the claim. But a typed defect became an escalation, and the body does not list it. Test door only. DEMONSTRATED, sure.

## NOTE
- The tie arm stores `rim(start)` under `Facts::Registered`. Its carrier-end facts are identities (/radius = /rim), so this is sound.
- `sym11` past the ceiling: the link's replay box moved with its ceiling (3.03e2→4.395e2·ε). "Refuses there" is therefore not a like-for-like regression, but its doc still says "The link replays whole too".
- The read-back's overflow escalation (`a_read_that_overflows_is_never_a_defect`): an `f64` escalation offers a refinement that cannot help, where the old answer named the scene. unsure.

## Claims
1 held: 170 boxed loops over 12 verbs × 5 widths × 3 offsets at `Interval` and `Sym<Interval>`, every radius `lo > 0`. A via point or tangent target straddling its line escalates at the door (DEMONSTRATED).
2 broken in part: MINOR-5. geom-brep and topo's other readers are filed (open row).
3 broken: MAJOR-1. Overflow is not the only class; resolution residue at far scales is another. The mirror wall reads nothing "on" (DEMONSTRATED).
4 broken as a schedule: MINOR-4.
5 held: both kept registrations are root-free and hold at either σ (σ² = 1), a non-unit radial and any sweep (a removable 0/0 at radius 0, as before). E12's M10-9 text admits literal-sign identities and the #3453 ruling admits "what the algebra does not close". The design's reason for retiring them (√(r²) ≡ r) does not apply to them, so keeping them is an improvement, disclosed.
6 broken for the table door: MINOR-7. Production sources (literal, sin_cos, the chord and tangent formulas, the lift) are units to rounding.
7 held: no consumer compares a minted radial with `(a − c)/radius` (`carrier_end` is read only by `register_endpoints` on constructed arcs). The test copy drifted (Style).
8 held: the lift is test-support only. Its sentence covers enclosure scalars only (Style).
9 held: the headline row is green on head. Red under mutant A (circle `radius.abs()`: both hole extrudes refuse `carrier_matches_mapped_source`). Red under mutant B, the run the body says was not taken (door arm on the normalized offset: both refuse `carrier_endpoint_start`). The boxed body holds all 9 members' 16 vertices at `Interval` (widest enclosure 1.6e-4). Volume was not checked.
10 held (head): MINOR-2 for the body's base column.
11 partly: MINOR-1. The replacing numeric readings are at the nominal, so sound. Pin re-derived from components: m10_10 algebra-off `carrier_on_surface` +16 = 2 holes × 2 rims × 2 arcs × 2 endpoint samples whose gap numerator is the zero form. I re-derived only one.
12 held: all three restated rows pass at 1e-6, 1e-9 and 1e-12. m10_10 at 1e3·ε no longer distinguishes the algebra (it certifies off). The algebra-needed property moved to 1e4·ε and is still pinned. "Needs no door" was green under |r| on base too, so it never pinned the sign.
13 held: A and B red as stated, B newly run. (d): no row on head can see a curved straddle's wrong side, as the PR says. The disc builds but does not finish over a box, so the spec's conditional row is not yet owed. That obligation survives only in INSIDE's open row once this row closes.
14 held: the three clauses match the code paths (m10_10 door-shut split = shipped). No normative sentence moved. The unchanged "folds as a fact about atan2 on a syntactic class" now describes `atan2(0, 1)`, a constant (unsure).
15 broken: MAJOR-1, MINOR-8. My differential: 960 verb cases (12 verbs × 80 members), validate then extrude at `f64`, head vs base. Validated forms are identical in all 960 and no decision moved. Bodies' bits moved in 487: bulge 127, via 132, tangent 80, onto 64, split circle 80, fillet 2. That is the body's frame class, wider than its corpus count (boss_union only).
16:
- profile: MAJOR-1.
- sweep slow set at default ε: head 3820/3825, 5 red (bisector near node, tilted rod other spins, section end at the band edge, notched rod, germarms ring lane). Each is red on the base with the same payload, held by open rows: `green/main-red-the-near-node-poses-at-the-witness-band-again`, `green/the-tilted-rods-other-spins-row-…` and `main-red-two-slow-rows-no-row-holds-at-da61db71a`, `green/main-red-eleven-editor-core-and-sweep-rows-…`, `onehome/a-body-moved-by-a-boxed-turn-…`, `sectcone/a-carried-sites-foot-escalates-…`. Base was taken on those 5 rows only (a full base run was too slow here).
- editor-core re-cut pin files and restated rows at 1e-6, 1e-9 and 1e-12: 62/72 at each, the same ten `revalue_corpus` reds, red on the base at 1e-6 and 1e-12 with the same `drifted` payloads (only the `moved` receipts differ), held by `green/main-red-revalue-corpus-carrier-and-witness-on-surface-drift-since-7411`, `onecopy/main-red-revalue-boss-union-…` and INLANE's apart-move rows.
- topo's slow set not run: topo depends on neither profile nor sweep and calls none of the changed `Arc2` readers, so it cannot observe this unit's code.
- Tour: the headline row only, not the 108-row suite.
17 partly: dispositions spot-checked (circle_loop, tangent_arc, fillet_arc, `carrier_line_circle_margin`, pair.rs `diff.abs()`) are right in kind. The two flagged sites are MINOR-6. Docs made false: see Style.
18 partly: `ValidatedSegment::lift` has no sentence for a point `U` (at `f64` or `Dual64` the normalize re-rounds, so the lift is no longer bit-for-bit). `from_chord`'s `sign` is ±1 by convention only (a doc "the caller's to have right"). The others are pure arithmetic or delegate to `radius_gap`'s per-scalar sentences.
19: after this PR a recorded radial can still be:
   - a table's written vector, unit only to the band, stored as written;
   - absent (tied bulge, fillets, Decided arcs), with validation minting `w/‖w‖`;
   - a lifted radial normalized at `U`;
   - at `Interval`, an enclosure (sin_cos, the chord formula) that holds a unit but is not one; a point within the charge but off unit can be stored (DEMONSTRATED).
   The frame door does not depend on `UnitVec2`. It depends on main's measured mint, which squares or poisons, and the registration's identity rests on the mint keeping the bits (MINOR-7).

## End-to-end
The change solves the right problem: the plate's `|r| − r` is gone with no registration, and a boxed cut plate is the right body at its members. What it misses in practice is far-scale `f64` validation (MAJOR-1): the rationalized gap trades a root's absorbing rounding for a square's exposed one. There is also no pin on what an arc stores when nothing decided its sign.

## Style
Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (read swept.rs's item list whole and geom-core arc.rs end to end).
- Q1 `swept.rs` tests `arc_from`: "a second spelling of profile's private mint", now dividing by the stored radius while `seg::mint_radial` divides by the rim. The copy disagrees with what it copies. sure.
- Q1 `seg.rs` `carrier_line_circle_margin` hand-spells the rationalized gap instead of going through `radius_gap`, a second home of the shape the unit gave one home. sure.
- Q1 frames: a line wall is turned onto its normalized chord (`frame_toward`), an arc onto its un-normalized radial (`frame_onto`). Two frame policies one module apart. likely.
- Q1 `Arc2::carrier_end`, `on_turned_radial` and `landing`: three spellings of "the end from a start", two now a one-line wrapper apart. unsure.
- Q7 `swept.rs` `sketch_vector`'s Arc arm: kept with a doc saying "which no door reads". A dead arm. sure.
- Q2/Q4 `sugar.rs`: "decided … non-negative" beside "both ρ straddle zero in general" in one function (MINOR-3). sure.
- Q4 stale citations of facts this change moved:
  - `validate.rs` `loop_orientation`: "`Seg::radius` is `signed_radius.abs()`" (sure);
  - `sweep/tests/copied_carriers_at_interval.rs` header names `register_rigidity` and `register_placed_carrier_end`, both gone (sure);
  - `sym11_exact_channel_rows.rs` `PAST_THE_CEILING` doc: "The link replays whole too … rule H proves … `carrier_matches_mapped_source`", against its own pin `Some("mapped-source residual")` (sure);
  - `geom_core::radius_gap`'s (sibling) "A caller that holds a signed radius passes its magnitude" now instructs a caller holding `σ·ρ` to take the `abs` this unit removed (likely);
  - `lift_onto`'s "every stored scalar … through from_f64 … everything else carried", beside the radials it now re-derives (sure).
- Q5 `Arc2::radius`: "positive in value" is a promise no door checks; `RawLoop` and `map_scalar` carry any sign (likely).
- Q3 the tie arm and `lift`'s point-scalar branch have no row that can fail. The stretched-radial row is `f64`-only, so its title's claim is untested at enclosure scalars. sure.
- Q6 MINOR-4 and MINOR-6 are disclosed deviations with no schedule. `mint_radial` keeping the rim ("the mint is not this unit's") names no row. sure.
- Q7 `ValidatedSegment::lift`'s normalization survives for one test door after its production reason went. Taste: drop it or move it into the fixture. likely.
- Q8 the size of arc.rs's and swept.rs's doc prose: the registrations' proof is restated in four places (arc.rs, swept.rs, sym.rs's module doc and the allowlist header). likely.

## Rubric
- Idiom and structure: 4. One gap function, signed literals at the verbs, small diffs. Docked for the inline duplicate and the dead arm.
- Test quality: 3. The mutants are real and the restated rows can fail. But the profile 1e-12 row that catches MAJOR-1 was never run, and neither the tie arm nor the enclosure scalar of the stretched radial is pinned.
- Doc and comment honesty: 3. The disclosures are careful, but five stale citations, one false invariant (sugar) and an undercounted receipt table.

## Spec deviations
Reported:
- `register_radials` and `register_placed_carrier_ends` kept: an improvement.
- The tie arm stores a measured radius rather than refusing: owes a row.
- `mint_radial` keeps the rim: owes a row.
- `lift` normalization.
- copysign and setback flagged but not scheduled.
SILENT:
- the far-scale `f64` move (MAJOR-1);
- the in-band ρ move (MINOR-3);
- the D-tab `arc_span` loss (MINOR-1);
- the negative table radius' refusal kind (MINOR-8).

## What I executed
Branch `boxfinish/review-signed-radius-own` (from ddbd1dece), commits 96d6d2463..557b1ad03, pushed:
- `profile/tests/review_signed_radius_own.rs`: the radius search, the straddles;
- `profile/tests/review_signed_radius_own_cells.rs`: claim 10, also run on base;
- `sweep/review-probes/own_f64_diff.rs`: its own `[[test]]`, the 960-case differential and the stretched radial at `Interval`;
- `demos/tour/tests/review_signed_radius_own.rs`: claim 9 members.
Mutants run and reverted: A, B, and the `check_carrier` revert.
Gates on head:
- `cargo fmt --check` (workspace and benches) clean;
- `python3 scripts/work.py lint` ok;
- `scripts/run-gates.sh` (GATE_BASE=f2815a232a) rc 0;
- `payload-rung-sweep --check` rc 0;
- clippy `--workspace --exclude viewer --all-targets --all-features -- -D warnings --force-warn clippy::disallowed_methods` rc 0, conic-centre census OK (203 reads);
- doc gate `scripts/doc-gate.sh --workspace-pass --skip-viewer-toolkit` OK.
Read only: the sibling's code, E12's other clauses, the editor-core pins I did not re-derive.
