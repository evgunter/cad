# CLEAVE: re-take of `topo-mints-indeterminates-outside-the-funnel`

Measurement only (no fix, no PR). Base: `origin/main` at `82b9ceb2`
(2026-10-03). Item: `work/cleave/topo-mints-indeterminates-outside-the-funnel.md`.

Shape swept: a `geom_core::Indeterminate` built by hand in
`crates/topo/src` (struct literal, `crate::invalid_margin::invalid`,
a local `invalid`/`label`/`margin`/`definite` closure or fn,
`MarginDiag::INVALID`) after a definite sign or decided quantity, so
the escalation or contradiction is on no frame's escalation log.
`#[cfg(test)]` code is excluded. Two things changed since the item
was written and run through every row below:

- **One crate-level helper now.** The four local `invalid(band, name)`
  definitions the item counted have gone: every spelling-2 site calls
  `crate::invalid_margin::invalid` (`crates/topo/src/invalid_margin.rs:26`),
  whose module doc names the three things a caller may mean by it
  (impossible sign / question not validly posed / two bounds
  straddling). `sectors.rs::invalid_escalation` (`:358`) is the one
  local wrapper left. Spelling-1 helpers that remain:
  `carrier_eq.rs::definite` (`:623`) and the `label`/`margin` closures
  in `boolean/mod.rs` (`:3844`, `:4116`, `:4338`).
- **The ladders moved.** `plane_eq.rs::declared_rung` no longer exists:
  the declared posture is `carrier_eq::declared_reading`
  (`carrier_eq.rs:665`, called from `plane_eq.rs:310`). The first-parent
  log for `fn declared_rung` lands on `71a5bbf6` (#3795, TANG, "lever the
  declared door's angular data") — attribution not checked further.
  Rung 4's decided zero is typed as `LadderRefusal::Coplanar`
  (`plane_eq.rs:250`, PR 3513) but is re-minted as `INVALID` by
  `LadderRefusal::untyped` (`plane_eq.rs:269`) for every public caller.

Classes: **(a)** a real gate that skipped the funnel (route through a
`k_stats` door; it belongs on the log). **(b)** a definite
CONTRADICTION or FINDING dressed as `Invalid` (wants a payload saying
what was measured; belongs on no escalation log). **(c)** other, named
per row; mostly *impossible sign* (a nonnegative quantity decided
`Negative`, so no door admits `{Zero, Positive}`) and *disagreement*
(two definite verdicts that cannot both hold). **(d)** not this shape.

## 1. The item's own sites, re-taken

| # | site on main | fn | predicate | preceding decision | lands in | status | class |
|---|---|---|---|---|---|---|---|
| I1 | `boolean/contact_verify.rs:165` | `rest_pair_verdict` | `contact_rest_senses_opposed` | ladder decided `SameOriented` (`rest::carrier_pair_verdict`, declared posture) | `ContactRefusal::Contradicted` | present | b |
| I2 | `contact_verify.rs:177` | `rest_pair_verdict` | `contact_rest_ladder_invariant` | ladder answered `Distinct` on a declared pair | `ContactRefusal::Escalated` | present | c |
| I3 | `contact_verify.rs:328` | `tangent_locus_relation` | `contact_tangent_on_1` / `_on_2` | `Ok(Positive)` of the padded residual | `Contradicted` (steer `FIT_DEFERRAL`) | present | b |
| I4 | `contact_verify.rs:362` | same | `contact_tangent_opposed` | `Ok(Positive)` of `n1·n2` | `Contradicted` | present | b |
| I5 | `contact_verify.rs:376` | same | `contact_tangent_independent` | `contact_tangent_opposed` `Ok(Zero)` | `Contradicted` | present | b |
| I6 | `contact_verify.rs:426` | same | `contact_tangent_parallel` | `Ok(Positive \| Negative)` | `Contradicted` | present | b |
| I7 | `contact_verify.rs:452` | same | `contact_tangent_second_order` | undeclared and `second_order_definite` is `Some(false)` (decided Zero/Negative) or `None` (in band; the funnel's real diag dropped at `:406`) | `ContactRefusal::Escalated` | present | a |
| I8 | `census.rs:2071` | (material-side pass) | `material_wedge_side` | `classify_dihedral` → `Transverse` after the edge screen decided agreement | `undecided` list | present | c |
| I9 | `census.rs:1260` | `gap_is_zero` | the caller's `name` | `Ok(Negative)` of a nonnegative gap | `ValidationError::CensusEscalated` | present | c |
| I10 | `boolean/contain.rs:937` | `point_on_circle` | `bool_contact_arc` | `Ok(Negative)` of `circle_miss` (a `sqrt`, ≥ 0) | `Err(Indeterminate)` → caller | present | c |
| I11 | `contain.rs:356` | `boundary_pre_pass` | `bool_contact_arc_end_vertex` | every `decide(END_VERTEX, …)` definite after the edge read `End` | `ContainError::Escalated` | present | c |
| I12 | ~~`sectors.rs` `bool_dir_same` `Ok(Zero)`~~ | `direction_sense` | `bool_dir_same` | — | — | **retired**: `validate::decide_nonzero_reported` at `sectors.rs:951` and `recl.rs:924` (first-parent `-G` lands on merge `1070dcd1`, #3809 — unconfirmed) | — |
| I13 | `boolean/sectors.rs:1040` (helper `:358`) | sector pairing | `bool_faces_parallel` | `Ok(Negative)` of a norm | `BooleanError::Escalated { SelfCheck::Normals }` | present (decision typed by 3513) | c |
| I14 | `splitting/rules.rs:179` | sector classification | `split_sector_extent` | `Ok(Zero \| Negative)` | `SplitReduceError::SliverSector` | present | a |
| I15 | `rules.rs:259` | same | `enters_material` | `EntersMaterial::Tangent` after the parallelism gate decided not-parallel | `SliverSector` | present | c |
| I16 | `boolean/plane_eq.rs:323` (helper `:407`) | `plane_ladder` | `bool_plane_parallel` | `Ok(Negative)` of a norm | `CarrierEqError::Escalated { rung: Norm }` | present (rung typed by 3513) | c |
| I17 | `plane_eq.rs:352` (helper `:114`) and `carrier_eq.rs:703` | `plane_ladder`, `declared_reading` | `bool_plane_orient` | `decide_reported` `Ok(Zero)` | `Escalated { rung: Orientation }` | **moved**: the margin is now the decided one (PR 3506); still built by hand, off the log | a |
| I18 | `plane_eq.rs:269` | `LadderRefusal::untyped` | `bool_plane_offset` | rung 4 `decide_reported` `Ok(Zero)` | `CarrierEqError::Undeclared { INVALID }` | **moved**: typed `Coplanar` inside, re-minted `INVALID` at the public boundary | b |
| I19 | `carrier_eq.rs:694` | `declared_reading` | `bool_plane_parallel` (named), decided by `bool_plane_reach_floor` | orientation not definite and `bool_plane_reach_floor` `Ok(Positive)` | `Contradicted { PlanesNotParallel }` | **moved** from `declared_rung`'s `Ok(Positive)` arm | b |
| I20 | ~~`declared_rung` `bool_plane_parallel` `Ok(Negative)` → `Escalated`~~ | — | — | — | — | **retired**: the declared posture no longer decides parallelism | — |
| I21 | ~~`declared_rung` `bool_plane_orient` `Ok(Zero)`~~ | — | — | — | — | folded into I17 (`carrier_eq.rs:703`) | — |
| I22 | `carrier_eq.rs:767` | `declared_reading` | the attributed datum (`bool_plane_offset` for planes) | upper `Ok(Positive)` and floor `Ok(Positive)` | `Contradicted { fact }` | **moved** from `declared_rung`'s offset arm | b |
| I23 | `boolean/mod.rs:1354` (called from `reduce.rs:3497`) | `one_vertex` | `bool_contact_vertex` | `Negative` of `norm3` | `BooleanError::Escalated { VertexOnVertex }` | **moved** out of `vertex_on_curved_face` | c |
| I24 | `boolean/rim_wedge.rs:1092` | `classify_shared_rim` | `material_cusp_side` | `Zero` after the same quantity's magnitude decided positive | `Err(Indeterminate)` | present | a |
| I25 | ~~`carrier_eq.rs` `data_rungs` `Ok(Pos \| Neg)` → `INVALID`~~ | — | — | — | — | **retired**: now `Ok(Distinct)` (`carrier_eq.rs:1058`). Its all-`Zero` fallback is N3 | — |
| I26 | `splitting/containment.rs:809`, `:845`, `:863` | `ConicArc::hit` | `rows.on` | `Negative` of a nonnegative miss or bound | `Err(Indeterminate)` | present | c |
| I27 | `containment.rs:885` | same | `rows.end` | `Negative` of `norm3` (`Zero` returns `End` at `:879` first) | same | present | c |
| I28 | `splitting/order.rs:112` | `in_plane_frame` | `split_join_frame_arm` | every member decided `Zero`/`Negative` | `Err(Indeterminate)` | present | a |

Per-row reasoning, one line each:

- I1: the relation is a definite carrier verdict, so a declared `Rest` with aligned senses is a measured contradiction. Two mints carry this predicate name: this one (census and `reduce` paths) and `boolean/mod.rs:3873` (`sense_contradiction`, the Boolean's door; see N7).
- I2: this is not an escalation. It is the ladder breaking its own contract ("a `Distinct` here would be the ladder breaking its own contract"), and it reads like `BooleanError::ClassificationInvariant`.
- I3–I6: a definite sign that refutes C4's table is a contradiction. The funnel logged the verdict; the minted `INVALID` drops the margin, which a `decide_reported` would have kept. I5 also names a predicate no `decide` call used.
- I7: on the undeclared path this is a gate. `Some(false)` is `decide_positive`'s exact shape. The declared path bridges instead, so a single door call needs a site decision (as in I28). `None` throws away the funnel's own diag and mints a worse one.
- I8: two definite verdicts disagree (the edge screen says agreement, the dihedral says Transverse). This is the helper's "not validly posed" meaning, and no door applies.
- I9, I10, I13, I16, I23, I26, I27: a nonnegative quantity decided `Negative` is unreachable for a finite margin and is a codomain guard. No `k_stats` door has the pass set `{Zero, Positive}` (`decide_positive`, `decide_negative` and `decide_nonzero` all differ). Unpinned and unreached as far as found.
- I11: the two passes disagree by construction ("the answer is always an escalation"), so this is not a gate.
- I15: the parallelism gate and `enters_material` disagree on the same normals.
- I17: `decide_nonzero_reported` is exactly this door: decided-`Zero` keeps its margin and is recorded on the frame.
- I18: an undeclared coincidence decided exactly zero is a definite finding. The typed `Coplanar` already exists, and only `untyped()` throws it away.
- I19, I22: a declared pair that was measured apart is a contradiction. I19 also names a predicate (`bool_plane_parallel`) that did not decide.
- I24: the same quantity was decided nonzero one decision earlier, so this is `decide_nonzero`'s shape (the item's note).
- I28: the #3686 correction stands; it wants one gate escalation after the loop.

## 2. Sites the item does not name

| # | site on main | fn | predicate | preceding decision | lands in | class |
|---|---|---|---|---|---|---|
| N1 | `boolean/vtxfac.rs:232` | on-face pierce | `bool_sector_coplanar` | tilt `Ok(Positive \| Negative)` on a DECLARED pair | `Contact/Seam/ContinuationContradicted { fact: PlanesNotParallel? }` | b. Introduced by `c99ca82c` on PR 3513's branch ("decided tilt contradicts a declared Rest") |
| N2 | `rim_wedge.rs:559` | `reach` (seam departures) | `seam_traversal_unread` (decided as `seam_rim_traversal` / `seam_line_traversal`) | `decide(…)? == Zero` of the traversal cosine | `Err(Indeterminate)` → `BooleanError::coincidence(site, spent, …)` | a (`decide_nonzero`); it also renames the predicate |
| N3 | `carrier_eq.rs:1075` (helper `:623`) | `data_rungs` | the kind's first datum | every datum `Ok(Zero)` | `Undeclared { INVALID }` | b. The curved twin of I18, filed `work/tang/decided-coincidence-carries-a-synthetic-invalid-margin.md` |
| N4 | `carrier_eq.rs:731` | `declared_reading` | `carrier_kind` | none (structural: `reading()` is `None`) | `Contradicted { KindsDiffer }` | b, structural: no sign precedes, but it is the same fabricated-`INVALID` payload |
| N5 | `carrier_eq.rs:759` | `declared_reading` | upper datum | upper `Ok(Negative)` (a sum of magnitudes) | `Unsettled` with the decided margin | c, impossible sign (honest margin, off-log) |
| N6 | `carrier_eq.rs:808` | `coincident_as_declared` | upper datum | the sum decided nonzero where every datum decided zero | `Undeclared` with the decided margin | c, disagreement (honest margin, off-log) |
| N7 | `boolean/mod.rs:3844–3873` | `sense_contradiction` | `continuation_senses_aligned`, `seam_senses_aligned`, `contact_rest_senses_opposed` | the structural sense bit on a carrier the ladder decided one | `Continuation/Seam/ContactContradicted` | b (comment: "the predicate labels the finding … never enters the K funnel") |
| N8 | `mod.rs:4116` closure, used at `:4135` (OneCarrier, structural), `:4197` (`TangentLocusError::NotTangent { predicate }`, decided in `geom_brep`), `:4286` (4 seam findings) | `verify_tangency_declaration` | `claim.conformal()`, the locus row, `seam_cusp` / `seam_sides_mixed` / `seam_locus_no_edge` / `seam_locus_untouched` | `rim_wedge::departures` / locus verdicts | `*Contradicted { fact }` | b |
| N9 | `mod.rs:4338` closure, 7 uses `:4356–4405` | `tangent_rim_refusal` | `contact_tangent_rim_{seam,nested,mixed,no_edge,untouched,transverse,lamina}` | `classify_shared_rim` / `departures` verdicts | `ContactContradicted` | b |
| N10 | `mod.rs:4235` | `verify_tangency_declaration` | relabels I4's `contact_tangent_opposed` → `seam_senses_aligned` | — | `SeamContradicted` | b (a relabel of I4's mint, not a new one) |
| N11 | `solid_contain.rs:1250`, `:2843` (`narrower_than_period` `:2782`) | `point_on_wall_in_face`, `chart_azimuth_margin` | `bool_wall_trim_period` | `decide(...) == Positive` is false (decided `Zero`/`Negative`) | `PointInSolidError::Escalated` | a (`decide_positive`, or `_reported` if the width is a size) |
| N12 | `solid_contain.rs:1890` | `cone_nappe` | `bool_cone_trim_side` | `Zero` | `PointInSolidError::Escalated` | a (`decide_nonzero`) |
| N13 | `solid_contain.rs:2669` | `point_on_chart_wall` | `bool_wall_trim` | `Zero` | same | a (`decide_nonzero`) |
| N14 | `solid_contain.rs:1885` | `cone_nappe` | `bool_cone_trim_nappe` | `Negative` (the window spans both nappes) | same | c: a definite out-of-lane fact dressed as an escalation (pass set `{Pos, Zero}`, no door) |
| N15 | `solid_contain.rs:2655` | `point_on_chart_wall` | `bool_wall_junction` | two pieces' sides decided and disagree | same | c, disagreement (`:2661` is a structural flag pair, d) |
| N16 | `splitting/rules.rs:429` | sided tangent sector | `enters_material` | `EntersMaterial::Tangent` (no parallelism gate in this function) | `SliverSector` | a, but the door would be a `geom_brep::enters` variant, not a `k_stats` call |
| N17 | `rules.rs:453`, `:459` | same | `wall_bend_order2` | material side and bend decided inconsistently, or sectors disagree | `SliverSector` | c, disagreement |
| N18 | `rim_wedge.rs:1035`, `:1115`; `validate.rs:5076` (twin) | `classify_shared_rim`; `material_arm_error` | `dihedral_wedge`; `material_cusp_side` / `material_wedge_side` (`Split`) | per-station verdicts disagree | `Err` / `ValidationError::SliverDihedral` | c (LINALG set these aside, and I agree) |
| N19 | `sectors.rs:378` | `bisector_zero_refusal` (`vtxfac.rs:832`, `recl.rs:1262`) | `bool_sector_bisector_side` | a `Zero` reading at K ≤ 2 | `BooleanError::Escalated { BisectorSide }`, enclosure `±zero` | c: an honest stated enclosure, but built by hand and off the log |
| N20 | `validate.rs:6114` | check-4 material side | `material_cusp_side` | `Ok(Zero)` | `SliverDihedral` | a; restfront's, filed |

Not this shape (d), and checked: `rim_wedge.rs:497` (`seam_cover_unread`,
lookup failure) and `:1007` (spline chart); `solid_contain.rs:3354`
(empty level list) and `:5310` (`bool_ray_torus_count`, two bounds);
`containment.rs:712`, `:870` (`rows.straddle`, the helper's documented
meaning) and `:1741` (two passes, test-only identity pass);
`flush.rs:286` (`EXTENT_UNREAD`, no decision);
`refusal_routes.rs:1132` (`NeighbourOffset::reported` rebuilds the
decided margin it was given). `sector_shape.rs`'s two are retired
(LINALG, `decide_positive`); every remaining hit there is under
`#[cfg(test)]`. `offer_rows.rs:1899`, `chart_region`, `euler`,
`merge_faces`, `pcurves`, `props`, `chord_join` and `circle_torus`
hits are all test code.

## 3. Counts (live sites only; retired rows I12, I20, I21, I25 not counted)

| class | item sites (I) | new (N) | total |
|---|---|---|---|
| a, gate skipped the funnel | I7, I14, I17, I24, I28 = 5 | N2, N11 (×2), N12, N13, N16, N20 = 7 | 12 |
| b, contradiction or finding dressed `Invalid` | I1, I3, I4, I5, I6, I18, I19, I22 = 8 | N1, N3, N4, N7 (×3), N8 (×6), N9 (×7), N10 = 20 | 28 |
| c, impossible sign | I9, I10, I13, I16, I23, I26 (×3), I27 = 9 | N5 = 1 | 10 |
| c, disagreement / not validly posed / out of lane | I8, I11, I15 = 3 | N6, N14, N15, N17 (×2), N18 (×3), N19 = 9 | 12 |
| c, broken internal contract | I2 = 1 | — | 1 |

Retired since the item was written: four (I12 `bool_dir_same`; I20;
I21, merged into I17; I25 `data_rungs`' nonzero arm). Moved: six (I17,
I18, I19, I22, I23, and `declared_rung` itself). PR 3513 retired
none of this row's mints. It typed the decisions around I13, I16 and
I18 (`SelfCheck::Normals`, `PlaneRung::Norm`, `LadderRefusal::Coplanar`)
and added N1.

The class-(b) population is much larger than the item's five, because
`boolean/mod.rs` mints its contradiction labels deliberately. Its
comments call them "Display-only labels … never enter the K funnel".
They therefore never claimed to be escalations; their only fault is
the fabricated `INVALID` in the `margin` field. Two other points cut
across classes:

- **Impossible sign (c):** none of these would be fixed by a door.
  They need either a `{Zero, Positive}` door or a decision that they
  are invariant guards.
- **Off-log with an honest margin:** I17, N5, N6 and N19 carry the
  decided margin, but nothing records them on the frame.

## 4. Reachability: what I ran and what I did not

**Ran:** two throwaway tests in `crates/editor-core/tests`, reverted
and not committed:

1. **`pr4` sliding union** (the `m4_pr4_diff::slide_union` document,
   flush planes declared) at `tx = 0.5, 0.99, 1.0`. At 0.5 and 0.99 the
   union now **succeeds**, which is the opposite of what the item says
   for those offsets. At 1.0 it refuses
   `NodeErrorKind::UndeclaredCoincidence { evidence: FlushEvidence {
   relation: SameOpposite, rung: DecidedCoincident }, diag: Indeterminate
   { margin: MarginDiag(Invalid), predicate: Some("bool_plane_offset") } }`,
   and the **user text still reads** *"…two operand faces are
   coincident and opposed (a resting contact), with no shared source or
   declared intent (margin is invalid (NaN or a refused enclosure)
   against the ambiguity band (1e-9, 1e-8)) — Recourse: …"*.
   - The mint is I18 (`plane_eq.rs:269`). It reaches the text through
     `flush::pair_finding` and then editor-core's
     `UndeclaredCoincidenceFinding::story`
     (`crates/editor-core/src/eval/mod.rs:2147`), which renders
     `diag.payload()` verbatim.
   - That makes it a **third text reader** that neither filed row
     names. `work/topo/plane-offset-rung-…` names `flush::pair_finding`
     and topo's `UndeclaredCoincidence` Display, and topo's Display
     (`boolean/mod.rs:2967`) already special-cases `is_invalid()`.
2. **`r2_p7`** (`a`'s x = 1 wall declared `Rest` against `far`'s x = 6
   wall; members `[a, big, far]`). It refuses `ContactContradicted {
   fact: Some(PlanesApart), margin: Indeterminate { margin:
   MarginDiag(Invalid), predicate: Some("bool_plane_offset") }, steer:
   Some(…Fit { gap }…) }`.
   - **The text no longer shows the margin.** It reads *"…the declared
     Rest contact between the operands' faces is contradicted: the
     declared planes are parallel but apart. Recourse: …"*. PR 3493
     made `ContactContradicted`'s Display read `fact` and never the
     margin (closed row `work/topo/declaration-contradicted-renders-…`).
   - The fabricated `INVALID` is still in the public payload field
     (`margin`) and in `Debug`. It is minted at I22
     (`carrier_eq.rs:767`).

**Ran:** `cargo nextest run -p sweep --test all` over
`reach_continuation::` and `pi_seam_and_kiss_through_the_boolean::`.
All 37 tests pass. These tests pin class-(b) payload predicates
through public `topo::union_with`:

- `contact_rest_senses_opposed` and `continuation_senses_aligned`
  (N7, `reach_continuation.rs:328`, `:340`);
- `contact_tangent_parallel` (I6, via the Boolean door,
  `pi_seam…:456`);
- `contact_tangent_rim_seam` (N9, `:396`);
- `seam_cusp` and the other seam findings (N8).

They pin the predicate name only, not the `Invalid` margin. Each is
a public `BooleanError` field.

**Cited, not run:**

- N11 `bool_wall_trim_period` is user-visible through
  `point_in_solid`. `work/contact/revolved-tube-wall-refuses-bool-wall-trim-period.md`
  measured 567 of 729 probes of a fully revolved tube refusing
  `Escalated { diag: Indeterminate { margin: Invalid, predicate:
  Some("bool_wall_trim_period") } }`. That is a class-(a) mint
  reaching users at volume.
- `pncad-py` tags the kind only (`tags.rs:1580`, `:2999`), not the
  margin.
- I1's census path reaches `ValidationError::ContactContradicted`,
  whose `margin` is "rendered by `Debug`, never by the message"
  (`validate.rs:1527`).

**Not run:**

- Any repro for I2, I3, I5, I7, I8–I11, I13–I17, I23, I24, I26–I28,
  N1–N6, N12–N20. A name search finds no test pinning
  `contact_tangent_independent`, `contact_tangent_on_1`,
  `contact_rest_ladder_invariant`, `contact_tangent_second_order`,
  `bool_sector_coplanar`, `seam_traversal_unread`,
  `bool_cone_trim_side`, `bool_cone_trim_nappe`,
  `contact_tangent_rim_transverse` or `contact_tangent_rim_lamina`.
- Whether any of these refusals also appears on a frame's escalation
  log through a different path. This was read from the code, not
  measured with a `Bracket`.

## 5. Owners and existing filings

`python3 scripts/work.py territory --files -` over the touched files:

- **cleave + hone:** `plane_eq`, `mod.rs`, `reduce`, `rim_wedge`,
  `vtxfac`.
- **cleave + hone + tang:** `carrier_eq`.
- **cleave + contact + hone:** `contact_verify`, `contain`,
  `solid_contain`.
- **cleave + germ + hone + reach:** `sectors`.
- **cleave + hone + reach:** `splitting/{containment,order,rules}`.
- **contact only:** `census.rs`.
- **restfront only:** `validate.rs`.
- **no program:** `flush.rs`.

Already filed elsewhere (open unless noted):

- I18 and its readers: `work/topo/plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin.md`
  (P2, still accurate; it misses editor-core's `story` reader, see
  section 4).
- I18 and N3: `work/tang/decided-coincidence-carries-a-synthetic-invalid-margin.md`
  (P3, design). Its line "`data_rungs`'s definite-nonzero
  `Contradicted`" is now **stale**: that arm returns `Distinct` (I25).
- I7: `work/contact/contact-verify-logs-a-second-order-escalation-its-outcome-overruled.md`.
  This covers the log side (an early escalation left on the log), not
  the mint.
- I10, I11, I23: `work/contact/contain-escalation-carries-no-decision.md`.
  This covers decision naming, not the mint.
- N11: `work/contact/revolved-tube-wall-refuses-bool-wall-trim-period.md`
  (P1, "not diagnosed": it does not say the escalation is a definite
  `Zero`/`Negative` minted by hand) and
  `work/cleave/point-in-solid-curved-arms-read-the-band-before-the-face.md`.
- N12, N14: `work/pred/cone-nappe-is-decided-in-five-places.md` (the
  duplication, not the mint).
- N20, and the `material_cusp_side` shape of I24:
  `work/restfront/validate-material-side-zero-mints-an-indeterminate.md`.
- `decide_positive`'s own decided-Zero-as-`INVALID`, relevant to how
  I14, N11 and I28 would route:
  `work/verdict/decide-positive-synthesizes-invalid-for-a-decided-zero.md`.
- **Dangling references in the item:**
  `work/props/indeterminate-error-arms-sweep.md` and
  `work/props/sector-shape-mints-indeterminates-through-an-invalid-helper.md`
  were deleted when PROPS closed (`5e9f6f53`, 2026-10-03, "twelve rows
  re-homed to FLUX"). No `work/flux/` file carries either id. The
  sweep's correction note survives only in history and in
  `docs/doc-ledger/props-leaves-the-tracker.md`, whose content I did
  not check.

## 6. Existing vocabulary for "a declaration contradicted by a measured value"

Inventory only, with no design proposed.

- `geom_core::Decided { sign, margin: MarginDiag }`, `predicate.rs:1306`:
  a definite outcome with the margin it was classified on. It is
  returned by `k_stats::decide_reported` (`k_stats.rs:524`) and
  `decide_flagged_reported` (`:584`).
- `geom_core::MarginDiag`, `predicate.rs:1020`: private `Reading::{Value,
  Enclosure, Invalid}`, with constructors `MarginDiag::value(f64)`
  (`:1182`), `::enclosure(lo, hi)` (`:1188`) and `::INVALID` (`:1178`).
  Its shape is read through `MarginKind` (`:1031`), and its numbers
  through `ErrorTextReading` (`:1076`).
- `geom_core::SizedPass`, `predicate.rs:1109`.
- `geom_brep::recourse` (`crates/geom-brep/src/recourse.rs`):
  - `Classified { margin, band }` (`:115`): "the reporting margin a band
    classified, where the variant reporting the verdict keeps it".
  - `Refused::{Zero(Classified), Negative { margin }}` (`:125`).
  - `RefusedArm::{Undecided(&Indeterminate), Zero(Classified),
    SignCertain}` (`:169`).
  - `StoredDefinite::{Contradiction, Lever}` (`:215`): "a contradiction
    between a stored description and its stored geometry".
  - `Unsized` (`:79`) and `Reading` (`:23`).
- `topo::Contradiction`, `boolean/refusal_routes.rs:62`: 18 typed facts
  with `fact()` prose. The fact is carried, but the measured value is
  not.
  - Plane: `PlanesNotParallel`, `PlanesApart`.
  - Kind: `KindsDiffer`.
  - Cylinder: `CylinderAxesNotParallel`, `CylinderAxesApart`,
    `CylinderRadiiDiffer`.
  - Sphere: `SphereCentresDiffer`, `SphereRadiiDiffer`.
  - Torus: `TorusAxesNotParallel`, `TorusCentresDiffer`,
    `TorusMajorRadiiDiffer`, `TorusTubeRadiiDiffer`.
  - Tangency: `OneCarrier`.
  - Seam: `SeamCusp`, `SeamSidesMixed`, `SeamFaceRunsOn`,
    `SeamUntouched`.
- `topo::CarrierEqError` (`PlaneEqError`), `boolean/carrier_eq.rs:89`:
  - `Contradicted { fact: Contradiction, diag }` (`:128`): its doc says
    "with an `INVALID` margin: the verdict is definite, and the rung
    keeps no measure".
  - `Undeclared { diag, relation }` (`:108`): "a decided-zero margin
    encodes as `MarginKind::Invalid`".
  - `Unsettled { diag }` (`:121`).
  - `Escalated { rung: PlaneRung, diag }` (`:93`).
- `topo::boolean::plane_eq::LadderRefusal::Coplanar { offset: Classified,
  relation }`, `plane_eq.rs:250` (crate-private): the decided zero,
  typed with its decided margin.
- `topo::boolean::refusal_routes::NeighbourOffset::{Zero(Classified),
  Undecided(Indeterminate)}`, `refusal_routes.rs:1112`, carried by
  `BooleanError::CoplanarNeighbours { offset }` (`boolean/mod.rs:1662`).
- `topo::ContactRefusal::Contradicted { diag, steer }`, `contact.rs:341`.
  Its Display reads `CONTRADICTION_REASON` (`contact.rs:292`), never
  the margin.
- `topo::BooleanError` (`boolean/mod.rs`), with Displays that read
  `fact`/`steer`, never the margin:
  - `ContactContradicted { declaration, fact: Option<Contradiction>,
    margin: Indeterminate, steer }` (`:1742`);
  - `ContinuationContradicted { a, b, fact, margin }` (`:1764`);
  - `SeamContradicted { a, b, fact, margin }` (`:1803`);
  - `UndeclaredCoincidence { diag, .. }` (`:1714`), whose Display
    special-cases `is_invalid()` as "exactly zero".
- `topo::ValidationError::ContactContradicted { declaration, witness:
  String, margin, steer }`, `validate.rs:1517`: the margin is
  "rendered by `Debug`, never by the message".
- `topo::flush::FlushRung::DecidedCoincident`, `flush.rs:154`: read
  off `diag.margin.is_invalid()` (`flush.rs:302`).
- `geom_brep::TangentLocusError::NotTangent { apart: bool, predicate }`,
  `locus.rs:45`: a definite verdict carrying its row name and a bool,
  with no margin.
- `topo::ContactVerdict::{Definite, Bridged}`: the positive half.

Readers that branch on `margin.is_invalid()` as a stand-in for
"decided exactly zero" (these are what make I18 and N3's `INVALID`
load-bearing):

- `carrier_eq.rs:315` (`pair_door_verdict`);
- `flush.rs:302`;
- `boolean/mod.rs:2967`;
- `refusal_routes.rs:1337`, `:1505`;
- `census.rs:7712`.

## Doubts recorded

- The commit attributions for I12, I23 and the `declared_rung` move
  come from `git log --first-parent -G` and land on big merges
  (`1070dcd1`, #3809). Treat them as unconfirmed.
- I15 against N16: I read `rules.rs:259` as following a parallelism
  gate (its comment says so), and `:429` as having no gate in its own
  function. I did not trace callers to check that `:429` is not also
  guarded upstream.
- The (c) "impossible sign" verdicts rest on the margin being a
  `norm`/`sqrt` at both scalars. I did not check every `Interval`
  path for a lower bound below `-escalate`.
