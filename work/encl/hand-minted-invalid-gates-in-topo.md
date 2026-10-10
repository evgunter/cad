---
id: hand-minted-invalid-gates-in-topo
kind: issue
title: topo: a few dozen hand-minted MarginDiag::INVALID escalations may be gate verdicts the funnel now carries tagged
status: open
opened: 2026-09-29
---

(ENCL implementer, residue of
`certify-collapsed-arm-gates-route-as-the-decision-they-guard`.)

## What

The second pass of ENCL's collapse-gate sweep (PR 3431) looked for
gates minted by hand rather than through the funnel: production
`MarginDiag::INVALID` in `topo`. As of 2026-10-08 that is a few dozen
sites, in `validate`, `boolean::{mod, rim_wedge, solid_contain,
carrier_eq, plane_eq, refusal_routes, vtxfac}`, `props`,
`chart_region`, `flush`, `merge_faces` and `invalid_margin`
(`contact_verify`'s seven are on CONTACT's
`contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`). None is
read yet. Each is either a real poison or a definite verdict reported
as one; the second kind should come from the funnel, which now carries
the decided margin tagged with the refused sign
(`MarginDiag::rejected_sign`).

`dihedral::classify_material_pairing_as`'s `decide_nonzero` gate is
the same class through the funnel: its definite zero reaches the
validator as `WedgeCheck::MaterialSide`, whose ending reads no margin.

## Shape

Read each site; route a definite verdict through the funnel's gate and
end it as its own decision.

## The door that exists

A gate's rejection carries the margin the classifier decided, tagged
with the sign it refused (`geom_core::MarginDiag::rejected_sign`;
`geom_brep::recourse::Refused::rejected` reads it as a verdict). A
dihedral or `enters_material` escalation is a `geom_brep::LeverEscalation`:
its `rung` names the arm or the reading, and
`LeverEscalation::collapsed_arm` hands back the arm's verdict where the
gate decided it (ENCL, `work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`,
PR 3431). Certification (`CertCheck::TransversalityArm`,
`CertifyError::ArmCollapsed`, `CertCheck::ParamSpanMeter`) and the
validator (`WedgeCheck::Arm`, `ValidationError::NoDihedralArm`) route
through them.

## Scope (ENCL scoping pass, 2026-10-09, against main `2f10bc9a4d`)

There are 35 production mint sites (38 arms). 22 spell `MarginDiag::INVALID`, and 13 go through `invalid_margin::invalid`. This row is now the umbrella. Its active units are:

- **`material-pairing-gate-definite-zero-ends-as-unreadable`** (ENCL, M): the material-pairing gate and `validate`'s cusp-side Zero.
- **`topo-poisoned-escalations-offer-unfollowable-endings`** (ENCL): the ending-only check over the poison/contradiction arms.
- **The `solid_contain` period and nappe verdicts** (`:1286`, `:2983`, `cone_nappe` `:1934`/`:1939`): these overlap PRED's `cone-nappe-is-decided-in-five-places` and `period-headroom-margin-has-no-shared-home`, so they ride those rows (a seam note is posted on PRED).
- **Splitting** `rules:197` and `classify:608`: these overlap CLEAVE's parked `topo-mints-indeterminates-outside-the-funnel` (`design: true`, blocked on intent stage 4), so CLEAVE is coordinated by seam note.

Held under D10 (declared-pair / declaration ground): every `rim_wedge` site; the declared-pair `Contradicted` labels (`carrier_eq:729`, `boolean/mod.rs` `:5195`/`:5522`/`:5751`, `vtxfac:550`); `merge_faces:2463`; and probably `flush:321`.

The site-by-site table follows.

# Scope: `hand-minted-invalid-gates-in-topo` (ENCL)

Read against origin/main `2f10bc9a4d` (2026-10-09). Read-only lane.

## Sweep and its blind spot

- **Pattern 1:** `INVALID` in `crates/topo/src/**/*.rs`, with `*_tests.rs`, `test_support*` and
  `contact_verify.rs` (CONTACT's seven) excluded. Each hit's position was then checked against the
  file's `#[cfg(test)]` items: inline test modules, and `#[cfg(test)]` fns and helpers.
- **Pattern 2:** callers of the one helper that wraps the mint, `crate::invalid_margin::invalid(band, name)`.
  The row lists `invalid_margin` as a single site, but the helper has **13 production callers**, and
  none of them spells `INVALID`.
- **Stale entries in the row's file list.**
  - `refusal_routes.rs`: its only literal is `:2419`, inside `mod tests` (`:1612`–`:3752`).
  - `props.rs`: its literals `:4669-4671` are inside `mod shell_role_refusal_tests`.
  - So neither file has a production mint.
- **Not counted:**
  - `euler_ring.rs` and `review_m1_pr2/release_corruption.rs` (doc text only);
  - `boolean/shell_witness.rs:508` (in `mod tally_rows`, test);
  - `validate.rs:2510` (a `#[cfg(test)] #[test]` fn);
  - `boolean/mod.rs:6493/6529` and `contain.rs:1238` (test).
- **Blind spot.** Neither pattern sees:
  - a mint through a `use … as` alias;
  - a mint through a crate-local helper with another name. `sector_shape.rs`'s and
    `sectors.rs`'s `invalid_escalation` are such helpers (PROPS / CLEAVE rows already cover them). I
    did not sweep those.
  - a definite verdict discarded without a mint (`Err(_) => claim.unsupported()`, or `.ok()`). Two
    of these turned up incidentally (Surprises 3 and 4).

## Dispositions

- **P** — real poison: no margin could be read.
  - **P(c)**: two decided readings contradict each other. This is the "impossible sign" or
    "question not validly posed" kind of `invalid_margin`'s doc. A contradiction is a kernel defect,
    so its ending should be the defect ending.
  - **P(s)**: a straddle or split. Several definite readings disagree across samples, and no single
    margin states the result.
  - **P(k)**: kind or structure. No margin exists at all.
- **D** — a definite verdict reported as poison.
- **U** — unclear.
- **"D10"** — the site stands on declared-pair or declaration ground, which D10 stage 4 retires
  (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`).

### Direct `MarginDiag::INVALID` mints (22 sites, 25 arms)

| # | site | fn | question / how reached | disp | ending check / note |
|---|---|---|---|---|---|
| 1 | `validate.rs:5591` | `MaterialStations::after_positive` | `material_cusp_side`. `decide` gives `Ok(Zero)` on the signed sagitta, although the same magnitude decided positive one step earlier | P(c), unreachable guard | `Stopped` → `SliverDihedral{MaterialSide}` → DEFECT, which is followable. The rim path also reaches it (PR 4433 moved it here). Already on RESTFRONT's row `validate-material-side-zero-mints-an-indeterminate`, whose fix is `decide_nonzero` so the Zero carries its tag |
| 2 | `validate.rs:5691` | `material_arm_error` | `MaterialArmOutcome::Split`: the stations disagree on pairing or end | P(s) | DEFECT. Whether the geometry can reach a split is on RESTFRONT's `ring-contact-and-sliver-split-endings-want-their-decisions` |
| 3 | `boolean/rim_wedge.rs:498` | `contains` (`seam_cover_unread`) | Missing face or surface, curved read `None`, **and every non-`Escalated` `ContainError`** (`Curved(PointInSolidError)`, `RayExhausted`, `Uncrossable`, …) lifted via `_ => unread` | U — throws away a typed refusal; it is not a sign verdict | D10 (seam / tangent-rim declaration verify only). Settled by carrying the `ContainError` |
| 4 | `rim_wedge.rs:562` | `reach` | `seam_traversal_unread`: `decide` gives `Zero` on a cos that `rides` already makes ±1 | P(c) | D10 |
| 5 | `rim_wedge.rs:1005` | `classify_shared_rim` | Spline chart on either side: the question cannot be posed | P(k) | D10. The caller swallows it (`Err(_) => claim.unsupported()`) |
| 6 | `rim_wedge.rs:1033` | `classify_shared_rim` | Transverse at one station, Smooth at another | P(s) | D10, swallowed the same way |
| 7 | `rim_wedge.rs:1068` | `classify_shared_rim` | Material fold `Split` | P(s) | D10, swallowed |
| 8 | `boolean/solid_contain.rs:1286` | `wall_outline` | `bool_wall_trim_period`: `narrower_than_period` decided the window **definitely ≥ one period** (Zero or Negative) | **D** | `PointInSolidError::Escalated` renders "too close to call at this tolerance … declare/move". Wrong for a decided full-period window |
| 9 | `solid_contain.rs:2983` | `chart_azimuth_margin` | Same verdict as #8, a second caller of `narrower_than_period` | **D** | Same |
| 10a | `solid_contain.rs:1934` (mint `:1919`) | `cone_nappe` | `bool_cone_trim_nappe` decided Negative: the window spans the apex | **D** | Same ending. PRED `cone-nappe-is-decided-in-five-places` overlaps |
| 10b | `solid_contain.rs:1939` | `cone_nappe` | `bool_cone_trim_side` decided Zero | **D** | Same |
| 11a | `solid_contain.rs:2756` (mint `:2696`) | `point_on_chart_wall` | Junction pieces decided on opposite sides | P(c) | Ending "too close … declare/move" is **not followable** for a contradiction |
| 11b | `solid_contain.rs:2762` | same fn | Both ends of a piece active | P(c) | Same |
| 11c | `solid_contain.rs:2770` | same fn | `bool_wall_trim` decided Zero on the azimuth margin (on the window edge) | D/U — is this a graze (`Ok(None)`) like the sibling Zero arms? | Same |
| 12 | `solid_contain.rs:3530` | `latitude_extremes` | No boundary levels: a structural empty | P(k) | Ending says "too close"; it is a corrupt or empty face |
| 13 | `boolean/plane_eq.rs:363` | `unreadable_norm` | A norm decided negative: operand poison | P | Ending is owned by TOPO's `boolean-unreadable-norm-ends-as-a-kernel-defect` (released from the D10 hold) |
| 14 | `boolean/carrier_eq.rs:190` | `CoincidenceMeasure::decide` | Datum not finite | P | `Unreadable` arm: fine |
| 15 | `carrier_eq.rs:729` | `definite` (3 callers: `:812`, `:849`, `:888`) | A declared pair **definitely** distinct → `Contradicted{fact, diag}`. The doc says "the rung keeps no measure" | **D** | **D10** (declared pairs) |
| 16 | `boolean/mod.rs:5195` | `sense_contradiction` | A structural sense bit contradicts the class; no `decide` ran. The `INVALID` is only a predicate label | D (label) | **D10** |
| 17 | `boolean/mod.rs:5522` | `verify_tangency_declaration` (`label`) | Labels on findings that `decide` calls under their own names decided definitely | D (label) | **D10** |
| 18 | `boolean/mod.rs:5751` | `tangent_rim_refusal` (`label`) | Same, for the rim routing (Transverse, Lamina, seam …) | D (label) | **D10** |
| 19 | `boolean/vtxfac.rs:550` | (sector coplanar read) | `bool_sector_coplanar` tilt decided ±, contradicting a declared class | **D** | **D10** |
| 20 | `chart_region.rs:1477` | cyl transfer | `chart_region_cyl_axis_sense` decided Zero, contradicting the tilt gate's Zero | P(c) | `ChartRegionError::Escalated` "too close to call: {diag}". Check that the diag's own ending reads poison |
| 21 | `flush.rs:321` | `pair_finding` | `PairUnread::Extent`: the face extent is unreadable | P(k) | Mis-armed: it goes to `PairUndecided::InBand` → `PairInBand` ("neither flush nor apart … separate the geometry" plus an unreadable-margin note). It belongs on `PairUnreadable` (defect). Likely D10, since flush feeds declarations |
| 22 | `merge_faces.rs:2463` | coplanar merge, declared branch | `merge_declared_extent`: `pair_extent` failed | P(k) | **D10** (declared pair) |
| — | `invalid_margin.rs:28` | `invalid` | The helper itself | — | See below |

### Through `invalid_margin::invalid` (13 production callers)

| # | site | fn / predicate | how | disp | note |
|---|---|---|---|---|---|
| 23 | `splitting/order.rs:126` | `split_join_frame_arm` | Every schedule ray decided non-positive. This cannot happen with non-parallel schedule rays | P(c) | Open PR 4224 edits this function |
| 24 | `splitting/containment.rs:311` | `ReadEscalation::straddle` | Two bounds straddle the band | P(s) | Carries `Escalation::Straddle`: correct |
| 25 | `splitting/containment.rs:1989` | `point_in_arc_loop_boundary_disagreement` | This walk's row and the caller's disagree | P(c) | `Escalation::Decided` |
| 26 | `splitting/rules.rs:197` | `split_sector_extent` | The face extent (a magnitude) decided **Zero**: a collapsed face | **D** | `SliverSector` ending "too close to call … move the split plane". CLEAVE's parked `topo-mints-indeterminates-outside-the-funnel` already lists it |
| 27 | `rules.rs:319` | `enters_material` | `Tangent` after the parallelism gate | P(c) | `SliverSector` ending is not a defect ending. On the same CLEAVE row |
| 28 | `rules.rs:488` | `wall_bend_order2` | `Exits` or `Tangent` where rule (a) read "enters" | P(c) | Same |
| 29 | `rules.rs:500` | `wall_bend_order2` | Sectors disagree | P(c) | Same |
| 30 | `splitting/classify.rs:608` | `split_conic_graze_side` | `decide` gives `Zero`. This is reachable at K ≤ 2 according to its own comment | **D** | → `BellyGraze`. `decide_nonzero` keeps the tag |
| 31 | `census.rs:2583` | `material_wedge_side` | `classify_dihedral` gives a definite **Transverse** after the edge screen | P(c) | `CensusEscalated` → `too_close` poison arm "check the inputs, then declare/move". The declare arm is not followable for a contradiction. On CLEAVE's parked row |
| 32 | `boolean/sphere_region.rs:384` | `bool_sphere_region_roots_count` | `CircleRoots::CountDisagrees` (D9 invariant) | P(c) | Check the ending of `RegionRefusal::Escalated` |
| 33 | `solid_contain.rs:5401` | `bool_ray_torus_count` | `TorusRoots::CountDisagrees` | P(c) | Ending is `PointInSolidError::Escalated` "too close … declare/move": **not followable** |
| 34 | `boolean/contain.rs:607` | `END_VERTEX` | Every end decided against every vertex, yet the point is still in the band of the end | P(c)/U | `Escalation::Decided` |
| 35 | `chord_join.rs:905` | `agreed_section` | Two section readings of different classes | P(s) | `SectionError::Escalated`. Open PRs 4399 and 4394 touch the file |

### The funnel-gate case the row names

- **The gate.** `geom_brep::dihedral::classify_material_pairing_as` (`dihedral.rs:1043`) uses
  `decide_nonzero` (`:1056`), so a definite Zero comes back as an `Indeterminate` carrying the tagged
  margin (`rejected_sign() == Some(Zero)`, `passes = NonZero`). Its doc says a caller that
  established smoothness "reads it as a defect".
- **Readers.**
  - **Validator** (`validate.rs` `MaterialStations::before_decision`, `:5552`): `Err(cause)` →
    `Break` → `SecondOrderWalk::Stopped` → `SliverDihedral{check: MaterialSide}` (`:6681`).
    - `WedgeCheck::MaterialSide::ending` is a flat DEFECT that reads no margin.
    - Its `lead` says "could not be read consistently along it", which is wrong for a decided Zero.
    - It also gives the same DEFECT to a genuinely **in-band** pairing.
    - After PR 4433 the rim (`classify_shared_rim`) reads through the same hook. There the error is
      swallowed by `tangent_rim_refusal`'s `Err(_) => claim.unsupported()` (D10).
  - **`census.rs:2593`** pushes the tagged cause into `CensusEscalated`. `too_close` then ends it
    "declare/move" (the cause is not poison), with no reading of the tag.
  - **`splitting/finish.rs:832`** → `SplitFinishError::DescribeEscalated`.
  - **`census.rs:4241`** discards it with `.ok()`.
- **Disposition: D.** Route `rejected_sign() == Some(Zero)` into a decision of its own: "smooth
  edge, normals perpendicular". That is a contradiction of the smooth verdict, so it ends DEFECT and
  quotes the margin. Keep `SliverDihedral{MaterialSide}` for an in-band pairing, and give that arm a
  sized ending (the pairing decision's lever and tighten offer). Cost M: one new `ValidationError`
  arm or field, the census arm, and the split-finish arm.

## Counts (arms)

| disposition | active | D10-held | total |
|---|---|---|---|
| D | 7 (#8, #9, #10a, #10b, #26, #30, plus #11c as D/U) + the dihedral gate | 5 (#15, #16, #17, #18, #19) | 12 + gate |
| P | 19 | 5 (#4, #5, #6, #7, #22) | 24 |
| P(k) mis-armed, likely D10 | — | 1 (#21) | 1 |
| U | — | 1 (#3) | 1 |

That is 38 arms over 35 production mint sites (22 direct, 13 through the helper).

## Proposed units

**D units, ordered by user impact:**

1. **The material-pairing gate's definite Zero** (validator, census, split finish). Cost M.
   - Every tier-3 at-rest validation of a smooth edge reaches it, so it is the most user-visible.
   - It fixes the wrong lead text and DEFECT-for-in-band.
   - Fold in #1 (`after_positive` → `decide_nonzero`) to close RESTFRONT's row too.
   - Ground: RESTFRONT (`validate.rs`), RESTREAD (`census.rs`), CLEAVE (`splitting/finish.rs`).
   - No open PR touches these lines (PR 4189 touches `census.rs` and `dihedral.rs`: check for
     conflicts).
2. **`solid_contain` period and nappe verdicts** (#8, #9, #10a, #10b, plus #11c once settled). Cost M.
   - Make `narrower_than_period` a gate (`decide_positive`). Then a decided full-period window ends
     as its own `PointInSolidError` arm, a kernel limit like `WallOutlineUnsupported`, and is no
     longer "too close at this tolerance".
   - Do the same for both `cone_nappe` arms.
   - Point-in-solid is on every curved boolean's containment path, so the impact is high.
   - Overlaps: PRED `cone-nappe-is-decided-in-five-places`, PRED
     `period-headroom-margin-has-no-shared-home`, and the reach frontier
     `full-period-wall-has-no-containment-verdict` (named in the closed GERM row, but no row file
     exists now).
   - Ground: CLEAVE, HONE, INSIDE.
3. **Splitting definite verdicts** (#26 `split_sector_extent` Zero → a collapsed-face verdict; #30
   `split_conic_graze_side` → `decide_nonzero`). Cost E.
   - Ground: CLEAVE / GAUGE / HONE.
   - #26 sits on CLEAVE's **parked** `topo-mints-indeterminates-outside-the-funnel` (blocked_on
     `intent-stage4-is-built`, `design: true`). Confirm with CLEAVE, or take that row's rules.rs
     items, before landing #26. #30 is not on that row.

**P sites that need only an ending check (one ENCL unit, cost E–M).** The fix is to make the carrying
error's Display read poison the way `validate::own_close` / `too_close` and PR 4453 do: a poisoned
or contradiction cause ends in DEFECT, and the declare/move menu is never offered for it.

- `PointInSolidError::Escalated`: #11a, #11b, #12, #33, and #8–#10 until unit 2 lands.
- `SplitReduceError::SliverSector`: #27, #28, #29.
- `ChartRegionError::Escalated`: #20.
- `RegionRefusal::Escalated`: #32.
- `SectionError::Escalated`: #35.
- `ContainError::Escalated{Decided}`: #34.
- `BellyGraze`: #30, if unit 3 slips.
- `CensusEscalated` poison arm (#31): "check inputs, then declare/move" offers declare for a
  contradiction.
- Already fine: #1, #2 (DEFECT), #14, #24, #25. #13 is owned by TOPO's open row.

**Blocked by the D10 hold** (declarations, contact records, declared pairs): #3–#7 (`rim_wedge`;
every caller is a seam or tangent-rim declaration verifier), #15–#19 (declared-pair `Contradicted`
labels), #21 (flush detector, which feeds declarations; confirm), #22 (merge, declared pair). Stage 4
retires declared pairs, so these should be re-read when they land rather than fixed now. Nothing here
stands on placement, Expr/parameters or Measure/Assertion ground.

## Open PRs touching these files

None of the open PRs edits a mint line itself; these are conflict risks only.

| PR | files |
|---|---|
| 4189 | `dihedral.rs`, `census.rs`, `classify.rs`, `merge_faces.rs`, `chord_join.rs`, `boolean/mod.rs` |
| 4224 | `order.rs` (around the frame arm), `containment.rs` |
| 4415 | `validate.rs`, `boolean/mod.rs` |
| 4413 | `vtxfac.rs`, `boolean/mod.rs` |
| 4418, 4398 | `boolean/mod.rs` |
| 4399, 4394 | `chord_join.rs` |
| 4467, 4441 | `containment.rs` |

## Owners (`work.py territory`)

| files | owner |
|---|---|
| `validate` | restfront |
| `census` | restread |
| `chart_region` | chart |
| `merge_faces` | fuse, topo |
| `boolean/*` | cleave + hone (`solid_contain`: + inside; `carrier_eq`, `chord_join`: + tang; `contain`: + inside, reachtail; `sphere_region`: + orbit) |
| `splitting/*` | cleave, gauge, hone |
| `flush`, `invalid_margin`, `dihedral` | not listed by territory |
