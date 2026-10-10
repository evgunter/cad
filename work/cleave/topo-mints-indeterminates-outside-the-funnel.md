---
id: topo-mints-indeterminates-outside-the-funnel
kind: issue
title: topo mints Indeterminates outside the funnel after a definite sign, in two spellings, at eleven shipped sites
status: open
opened: 2026-09-20
priority: P0
cost: M
design: true
---



## What

`topo` builds `Indeterminate`s of its own after a DEFINITE sign, at
sites the funnel never sees — so the escalation (or contradiction) the
caller receives is on no frame's escalation log, and a consumer reaches
it only through the op's error arms. Found by the PROPS
escalation-channel unit (PR 2928), which retired exactly this shape at
the eight `geom-brep` sites it owned and then swept for the SHAPE rather
than the spelling. These are `curved`'s ground, so they are filed.

**Spelling 1 — the struct literal**, `crates/topo/src/boolean/contact_verify.rs`,
**seven** shipped mints (an eighth `Indeterminate {` in that file is a
fixture inside `#[cfg(test)]`):

- `contact_rest_senses_opposed` — the `CarrierRelation::SameOriented`
  arm → `ContactRefusal::Contradicted`
- `contact_rest_ladder_invariant` — the `CarrierRelation::Distinct` arm
  → `ContactRefusal::Escalated`
- the residual arm, under the decided predicate's own name
  (`contact_tangent_on_1` / `_on_2`), after `Ok(Sign::Positive)` →
  `Contradicted`
- `contact_tangent_opposed`, after `Ok(Sign::Positive)` → `Contradicted`
- `contact_tangent_independent` → `Contradicted`
- `contact_tangent_parallel` → `Contradicted`
- `contact_tangent_second_order` → `Escalated`

**Five of the seven are `Contradicted`, not `Escalated`** — see the note
below, which is the harder half.

**Spelling 2 — a local `invalid(band, name)` helper** returning an
`Indeterminate` (or a fault wrapping one). `topo` holds four separate
definitions of this helper, and the census PR 2928 added over
`geom-brep` — a scan for the `Indeterminate {` literal — would not see a
single one of them even if it were pointed at this crate. On curved's
ground:

- `crates/topo/src/census.rs` — `undecided.push(invalid(band,
  "material_wedge_side"))` after a definite `DihedralClass::Transverse`,
  and a second at the `cause: invalid(band, name)` site above it.
- `crates/topo/src/boolean/contain.rs` — `Ok(Sign::Negative) =>
  Err(invalid(band, "bool_contact_arc"))`, plus the
  `ContainError::Escalated(invalid(band, "bool_contact_edge"))` site.
- `crates/topo/src/boolean/sectors.rs` — `invalid_escalation(band,
  "bool_dir_same")` after `Ok(Sign::Zero)`, and `"bool_faces_parallel"`.
- `crates/topo/src/splitting/rules.rs` — two: a hand-built
  `split_sector_extent` refusal after a definite sign, and an
  `enters_material` refusal minted after a definite
  `EntersMaterial::Tangent`.

(`crates/topo/src/sector_shape.rs` holds two more of spelling 2, on
PROPS' ground:
`work/props/sector-shape-mints-indeterminates-through-an-invalid-helper.md`.)

## Note on the payload, which is the harder half

Five of the seven `contact_verify` mints are not escalations at all. A
DEFINITE `Positive` residual is a CONTRADICTION of a declared contact,
and dressing it as `MarginDiag::Invalid` says "the margin was poison"
about a margin that was measured.
`crates/geom-brep/src/props/quad.rs` names this class in prose — *"turns
into `Indeterminate{margin: Invalid}`: a MIS-TYPED refusal"*.

**This is also a correction to PR 2928's own sweep, and it is the
sharper finding.** That sweep keeps a group of variants on the reason
that they "carry the classifier's diagnostic as evidence"
(`ContactRefusal::Contradicted` among them). For these five the
diagnostic is FABRICATED — an `Invalid` nobody classified — so for them
that reason is hollow, and what actually keeps the variant is the
declaration and the steer it carries beside the diagnostic, not the
diagnostic. `work/props/indeterminate-error-arms-sweep.md` now says so.

So the fix is not only to route the mint through a funnel door; it is to
decide, per arm, whether the fact being reported is an indeterminacy
(route it, and it belongs on the log) or a definite contradiction (give
it a payload that says what was measured, and it belongs on no
escalation log at all).

`geom_core::k_stats` gained `decide_positive`, `decide_nonzero` and
`gate_measured` in PR 2928; those are the doors for the arms that really
are gates.

## A third file, found by EMIT (2026-09-23)

`crates/topo/src/boolean/plane_eq.rs` holds seven more `Indeterminate {
margin: MarginDiag::Invalid, .. }` struct literals minted AFTER a definite
sign — spelling 1, in a file the list above does not name (no open
program's territory claims it):

- `oriented_plane_eq_verdict`: `bool_plane_parallel` `Ok(Negative)`
  → `Escalated`; `bool_plane_orient` `Ok(Zero)` → `Escalated`;
  `bool_plane_offset` `Ok(Zero)` → `PlaneEqError::Undeclared` (rung 4).
- `declared_rung`: `bool_plane_parallel` `Ok(Positive)` → `Contradicted`,
  `Ok(Negative)` → `Escalated`; `bool_plane_orient` `Ok(Zero)` →
  `Escalated`; `bool_plane_offset` `Ok(Positive | Negative)` →
  `Contradicted`.

The rung-4 site's payload is user-visible and the same shape as the note
above. Measured on `tests/fixture/pr4.rs`'s sliding union, with B's
transform moved to x = 1.0 so B's −x wall rests on A's +x wall: the union
refuses, as the contact contract requires (the declaration covers only
the flush planes), and the refusal reads *"… coincident with opposed
orientations (resting contact) … predicate 'bool_plane_offset'
indeterminate: margin is invalid (NaN or a poisoned enclosure) …"*
about a margin that DECIDED exactly Zero. The same sentence appears on
the undeclared flush case at x = 0.5 and 0.99. The refusal is correct;
its diagnostic states a poison that did not occur.
`work/stack/certified-lane-non-real-contract-audit.md` records the same
payload from the M10-DI review as a member of its class; this is its
minting site.

## Also reached from a union's declaration channel (GATHER, 2026-09-24)

`declared_rung`'s `Contradicted` arm (`bool_plane_offset`
`Ok(Positive | Negative)`) is user-visible through a union too. R2's
`r2_p7` declares `a`'s x = 1 wall Rest against `far`'s x = 6 wall.
The union refuses `ContactContradicted` in every member order (the
pairwise pre-pass judges a declared pair whatever the order), and the
refusal reads
*"… contradicted by predicate 'bool_plane_offset' indeterminate: margin
is invalid (NaN or a poisoned enclosure) …"* about two planes 5 units
apart whose offset DECIDED nonzero. Pinned (by kind, not text) in
`docm8_flat_merged::a_member_face_contained_whole_satisfies_its_pair_and_a_contradicted_one_refuses`.

## More sites, found by LINALG's sweep (2026-10-01)

The `linalg/decided-not-minted` branch retired `sector_shape`'s two
(`decide_positive` at both gates) and then swept `MarginDiag::INVALID`
literals and `invalid`-style helpers across `crates/*/src`. On this
row's ground, minted after a DEFINITE sign and named by neither list
above:

- `crates/topo/src/boolean/reduce.rs` `vertex_on_curved_face`:
  `Ok(Sign::Negative)` of a distance → a struct-literal
  `BooleanError::Escalated { decision: VertexOnVertex, .. }`.
- `crates/topo/src/boolean/carrier_eq.rs` `data_rungs`:
  `Ok(Sign::Positive | Sign::Negative)` → an `INVALID` diagnostic
  under the decided datum's own name — the "contradiction dressed as
  poison" half of the note above.
- `crates/topo/src/splitting/containment.rs`, through the crate-level
  helper `crate::invalid_margin::invalid` (a fifth spelling of
  spelling 2): `hit`'s `rows.on` arms after a definite
  `Sign::Negative` of a nonnegative miss (three sites), and the
  `rows.end` arm after a definite sign. Its `rows.straddle` sites are
  the helper's documented two-bounds-straddle meaning, not a gate.

Not this class, and left: `rim_wedge.rs`'s spline-chart and
corner/tangency mints and `carrier_eq.rs`'s `KindsDiffer` /
`Undeclared` fallbacks (no decision precedes them),
`solid_contain.rs`'s `bool_ray_torus_count` (two bounds that
disagree), and `splitting/order.rs`'s `in_plane_frame` fallback (every
schedule member decided and none positive — no escalation to return).

**Correction to the paragraph above (review of PR #3686).**
`splitting/order.rs`'s `in_plane_frame` fallback IS this row's shape:
every schedule member's `split_join_frame_arm` decision answered a
definite `Ok(Sign::Zero | Sign::Negative)`, and the function then
mints `Indeterminate { margin: INVALID, predicate:
Some("split_join_frame_arm") }` by hand, on no frame's log. The fix is
not a one-line `decide_positive` — gating each member would log an
escalation for every member a later one rescues — so it wants a
decision at the site (log one gate escalation after the loop, through
`k_stats`, rather than per member).

## Re-taken against main (CLEAVE measurement lane, 2026-10-03, `82b9ceb2`)

The full table is on branch `analysis/cleave/mints-retake`,
`analysis/cleave-mints-retake.md`.

Summary: of the item's 28 sites, 24 are still live on main. A sweep for the
same shape found about 35 more.

| class | live sites |
|---|---|
| (a) real indeterminacy off the funnel | 12 |
| (b) definite contradiction dressed as `INVALID` | 28 (20 new; 17 of them are display-only contradiction labels in `boolean/mod.rs`) |
| (c) impossible sign: no door fits, nothing admits `{Zero, Positive}` | 10 |
| (c) disagreement or out of lane | 12 |
| (c) broken invariant | 1 |

What changed since this row was filed:
- **Retired:** `bool_dir_same` and `data_rungs`' nonzero arm. `declared_rung` moved to `carrier_eq::declared_reading`, but `plane_eq.rs:269`'s `untyped()` turns its typed zero back into `INVALID` for every public caller.
- **The four local `invalid` helpers** now share one crate helper, `invalid_margin::invalid`.
- **PR 3513 retired none of this row's mints and added one:** `vtxfac.rs:232` `bool_sector_coplanar`, class b.

User-visible, measured:
- **pr4 sliding union.** At x = 0.5 and 0.99 it now succeeds. At x = 1.0 its text still says "margin is invalid (NaN …)", rendered by editor-core's `UndeclaredCoincidenceFinding::story` (`eval/mod.rs:2147`).
- **`r2_p7`.** The margin no longer reaches the text since PR 3493, but `INVALID` survives in the public `margin` field.

Cited, not run: `bool_wall_trim_period` refuses 567 of 729 `point_in_solid`
probes on a revolved tube (`work/inside/revolved-tube-wall-refuses-bool-wall-trim-period.md`).

Six readers treat `is_invalid()` as "decided exactly zero":
`carrier_eq.rs:315`, `flush.rs:302`, `boolean/mod.rs:2967`,
`refusal_routes.rs:1337`, `refusal_routes.rs:1505` and `census.rs:7712`.

Dangling links: `work/props/...` was deleted when PROPS closed.

Next: a designer pair, on three questions:
- what payload class (b) carries;
- which door, if any, class (c)'s impossible signs need;
- how the six `is_invalid()` readers change.

## Designed (2026-10-03; designer pair converged, no ratified text changes)

Labels and the rounds are on `analysis/design-fork/topo-mints` (byte 123).

**The premise is one level too low.** The mints are forced by a field typed
wrong. Every contradiction, finding and self-check error in `topo` carries
"which predicate, what band, what it saw" in a slot typed `Indeterminate`.
A site that holds a decided answer therefore has to forge an escalation.
The funnel's own gate doors do the same: `decide_positive` and
`decide_nonzero` reject a decided sign as `INVALID`. Routing classes (b) and
(c) through the funnel would be wrong, because `drive::log_read` answers
`Bisect` for a logged non-sliver. A definite verdict on an enclosure never
flips under refinement, so the driver would split until its budget ran out.

The final state:

1. **Only the funnel builds an `Indeterminate`.** It becomes
   `#[non_exhaustive]` (fields readable); `MarginDiag::INVALID` becomes
   crate-private; fixtures use a `test-support` constructor. This deletes
   `invalid_margin.rs` and `sectors.rs::invalid_escalation`, and makes the
   `reporting-margin-door.sh` literal pin a compile error. Last step, once
   every site has its home.
2. **Gate rejections keep the decided margin.** The `_reported` doors fold
   into the plain ones, and `MarginKind::Invalid` means poison and nothing
   else. This closes
   `work/verdict/decide-positive-synthesizes-invalid-for-a-decided-zero.md`.
3. **Class (b) definite contradictions become typed facts, on no log.**
   A new `geom_core::Definite { predicate, sign, margin, band }` is the
   evidence type. Contradiction errors become
   `Contradicted { fact: Contradiction, evidence: Option<Definite> }`, where
   `None` means a structural finding (a sense bit, two kinds). The
   predicate-name labels in `boolean/mod.rs` (`contact_tangent_rim_*`,
   `seam_senses_aligned`, …) become `Contradiction` variants, and the tests
   that pin those names re-pin by fact.
4. **Coincidence is typed.** `Coincidence::{Decided(Definite),
   InBand(Indeterminate)}` is the payload of `CarrierEqError::Undeclared`,
   `BooleanError::UndeclaredCoincidence` and editor-core's twin. Three
   existing types collapse onto it: `LadderRefusal::Coplanar`,
   `NeighbourOffset` and geom-brep's `RefusedArm`. `untyped()` and
   `reported()` go. The six `is_invalid()`-as-zero readers match the
   variant, and `flush.rs`'s poison-read-as-coincident case becomes
   `InBand`. The pr4 union at x = 1.0 then reads "decided zero", not "NaN".
5. **Impossible signs go through a magnitude door.**
   `k_stats::decide_magnitude(name, margin, band) -> Result<Magnitude,
   Indeterminate>`, with `Magnitude::{Zero, Positive}`. A decided `Negative`
   is `unreachable!` inside the door, with the predicate, band and margin in
   the message (D9 row 4, Ev's `a0781edfa`). `unreachable!` is deliberately
   outside the workspace's `clippy::panic` family (`49168e708`).
   `decide_invariant`'s "never a panic" covers integral backstops, not
   impossible branches. The door accepts only quantities nonnegative by
   construction (a norm, a sqrt, a sum of those). A difference that is
   nonnegative only mathematically can round negative, so it is reachable
   and does not belong here.
6. **Class (a) real indeterminacies** go through the gate doors. Two sound
   bounds on ONE quantity that straddle the band are a real indeterminacy:
   their margin is the enclosure `[lo, hi]`, and they go on the log
   (`rows.straddle`, `bool_ray_torus_count`).
7. **Two definite verdicts that disagree are never an escalation.**
   - One fact decided twice (I8, the edge screen against the dihedral; N17):
     decide once, so the second question takes the first verdict as typed
     input.
   - Independent honest measurements (N15; N18, a rim whose class changes
     along its length): a typed finding carrying both `Definite`s, told with
     its decision's story (D4 ¶1 (iv)), or a typed `Unsupported*` where the
     state is a lane limit. Decide which per site, after reading it.
   - Unreachable combinations (I2; N6 at K = 10): item 5's shape, a panic.
   - I11 (a body certified at a coarser band) is a D4 ¶1 (iv) finding.

Build order, one PR each:
1. the geom-core doors (items 2, 5 and `Definite`);
2. `Coincidence` and the six readers (item 4);
3. contradiction typing (item 3);
4. the remaining class (a)/(c) sites (items 6 and 7, per site);
5. the seal (item 1).

Also: `sweep/src/blend/battery.rs::short_arm` is the same shape, so it goes
with step 4. `refusal_routes::NeighbourOffset::reported` goes with step 2.

## Landed in PR 3974 (TOPO)

The payload of `CarrierEqError::Undeclared` is now
`topo::CoincidenceMeasure` (`boolean/carrier_eq.rs`), with three arms that
every reader matches:

- `Zero { predicate, decided: Classified }`: the datum the band decided
  zero, with the margin it decided (the plane's `bool_plane_offset`; a
  curved kind's first datum);
- `Undecided(Indeterminate)`: a datum in band, or the pair door's
  declared reading standing past the band;
- `Unreadable(Indeterminate)`: a datum that is not finite (NaN or ±∞),
  decided in one place (`CoincidenceMeasure::decide`, the ladders' datum
  read).

`LadderRefusal`, `untyped()` and `plane_eq_typed` are gone. The readers
are `flush::pair_finding` (`Zero` is the finding), `pair_door_verdict`,
the maximal-faces gate (`Zero` and `Undecided` become `NeighbourOffset`),
the Boolean's raise sites and the conformal screen.
`BooleanError::UndeclaredCoincidence` keeps `diag: Indeterminate`, built
by `CoincidenceMeasure::reported()`, so editor-core's twin is unchanged.
The `wire` row `refusal-menu-stamps-decided-coincident-on-an-in-band-coincidence`
is the cost of that.

**The one divergence from step 4.** Poison is not `InBand`. It is its own
arm, `Unreadable`, and at the Boolean it ends as an operand defect:
`BooleanError::PoisonedCarrierDatum`, `KERNEL_OR_FILE_DEFECT_ENDING`. Every
Boolean door reaches it through `boolean::readable_coincidence`. At the
flush detector it is `PairUndecided::Unreadable` /
`FlushRefusal::PairUnreadable`. The reason is D4 ¶1 (i): the recourse
follows from the decision, and an in-band arm's recourse offers the
declaration and the move. A non-finite datum is not a coincidence the user
can declare or move out of; it is a stored face that describes no shape,
as tier 3's `PoisonedSurfaceDatum` says of the same datum.

This is CLEAVE's to adopt or reshape when it builds step 2.
## Step 1 landed (PR 3979, branch `cleave/mints-doors`)

The geom-core doors. Gate rejections keep the decided margin. The
`_reported` twins are folded into the plain doors, and
`MarginKind::Invalid` now means poison only. The escalation text says
"lies past the ambiguity band — a decided sign this decision cannot
use" and offers no tolerance on a sign-certain reading.
`k_stats::decide_magnitude` is the magnitude door.

- **Migrated to the magnitude door:** I9, I10, I13, I23, I26 (×3) and
  I27, plus four `chart_region` sites that the retake had filed as test
  code (`carrier_tilt`, and `norm_gate`'s `cyl_tilt`, `cyl_offset` and
  `cyl_transfer`).
- **`sweep` `short_arm`:** now goes through `decide_positive`
  (`blend::classify_positive`).
- **`Definite`:** deferred to step 3, which has its first consumer.

Not migrated, for step 4:

- **I16** (`bool_plane_parallel`). The arm is
  `ExtentBall::radius()`, and the public `oriented_plane_eq` /
  `ConsumedExtent::unwitnessed(ExtentBall::new(c, r))` takes any
  radius, so the margin can decide Negative from input.
- **N5.** The Positive arm quotes the decided margin, and
  `decide_magnitude` returns no margin.

New step-4 sites, of the same gate shape, off the log:

- `chart_region::definite_diag` ×3 (`cyl_band_area`, `cross_order`,
  `area`). Each echoes `Value(lo)`, which collapses an enclosure.
- `chart_region`'s `cyl_axis_sense` decided Zero (a disagreement with
  the tilt gate).
- `merge_faces`' LoopWinding decided zero.

Filed on flux:
`work/flux/a-gate-rejection-of-a-decided-enclosure-bisects-to-budget.md`.

## Step 2 WIP, held by #3990 (2026-10-03)

The step-2 lane was dispatched after the hold notice, then withdrawn. Its
work is on branch `cleave/mints-coincidence` at `e138240de`, pushed as
unreviewed WIP with no PR.

**Done.** `topo::Coincidence::{Decided { predicate, margin }, InBand(Indeterminate)}`
is the payload of `CarrierEqError::Undeclared`,
`BooleanError::UndeclaredCoincidence`/`CoplanarNeighbours`, and
editor-core's `UndeclaredCoincidence`/`UndeclarableContact`. Removed:
- `LadderRefusal`/`untyped()`;
- `NeighbourOffset`/`reported()`;
- the dead `ContactRefusal::Undeclared`.

**Not folded in.** `RefusedArm` is not folded, because it has a
`SignCertain` arm; `Coincidence::arm()` converts to it.

**Readers.** The `is_invalid()` readers in `carrier_eq`, `flush` and
`boolean/mod.rs` now match on the variant. Poison reads as `InBand` and is
never coincident. The two `refusal_routes` readers and the one `census`
reader now see only poison, so they are left as they are.

**Built only.** `cargo check` passes for the workspace and the demos. Not
yet done:
- tests, clippy, gates, and the ε rows;
- the pins for pr4 at x = 1.0 and for `r2_p7`;
- the sweep write-up.

**Open doubts:**
- `Coincidence::quoted()` rebuilds an `Indeterminate` from the decided margin for three arms: a step-3 residue and two unreachable arms.
- `coincident_as_declared` still reads a past-band bound as `InBand`; that site belongs to step 4.
- editor-core labels every coincidence `DecidedCoincident`.

Weigh this against the #3990 ruling before reusing any of it.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: held step 2's payload sites (CarrierEqError::Undeclared, UndeclaredCoincidence) and contact_verify's declared-contact contradictions are what stage 4 retires; steps 4–5 could be split off as workable. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E deletes step 2's user-visible payload sites: `BooleanError::UndeclaredCoincidence`, editor-core's `UndeclaredCoincidence` / `UndeclarableContact`, and `NeighbourOffset`. `CarrierEqError`'s Zero arm is no longer a refusal, because rung 4 glues. `CoincidenceMeasure` stays the ladders' typed payload (`crates/topo/src/boolean/carrier_eq.rs:137`). The pr4 x = 1.0 union's "margin is invalid" sentence has no site left to render it.

Steps 3–5 are untouched. `contact_verify`'s minted `MarginDiag::INVALID` contradictions are all live (`crates/topo/src/boolean/contact_verify.rs:166`, `:180`, `:332`, `:366`, `:380`, `:430`, `:455`), and so are `plane_eq.rs:304` and the class (a)/(c) sites. E adds one shape of class (b): a pair decided one carrier at an arm the glue door did not glue escalates through `unglued_coincidence` (`crates/topo/src/boolean/mod.rs:1007`). That escalation carries the decided margin, or `INVALID` where none was read (`recl.rs:151`, `vtxfac.rs:789`), inside an `Indeterminate`. The row resumes at step 3.

## Re-scoped (2026-10-10)

Read against `origin/main` at `98a3817a1d`. Scope: production code in
`crates/topo/src` and `crates/sweep/src`.

**Patterns.**
- Pass 1 looked for `MarginDiag::INVALID`, `invalid_margin::invalid`,
  `invalid_escalation`, `Indeterminate {` and `invalid(`.
- Pass 2 looked for every `terminal_sliver:` field. Every struct literal
  has to set that field, so pass 2 also finds literals that carry an
  honest margin or have a helper of another name.
- Test code was dropped from both passes: hits inside a `#[cfg(test)]`
  item, hits in `*test*` files, and `refusal_routes`' `#[path]`-mounted
  `offer_rows`.

**Blind spots.**
- A `use … as` alias for `Indeterminate`, `MarginDiag` or `INVALID`.
  None exists in either crate.
- A `..diag` struct update that renames a funnel diagnostic. The only
  one is `boolean/mod.rs:5765`.
- A decided verdict thrown away without a mint (`Err(_) =>
  claim.unsupported()`). That shape is ENCL's, and it is not swept here.

**Outside these crates** there is no production hand mint in `geom-brep`,
`editor-core`, `pncad-py`, `step-export` or `viewer`. Every hit there is
a test. The workspace holds 136 test literals and 85 `INVALID` spellings,
which are what the seal has to migrate.

### Census: 69 arms, by class and owner

Class (d) is a fourth class, not this row's shape. It covers a site that
read no margin: honest poison, or a structural fault. Those sites still
matter to the seal, because they hand-build an `Indeterminate`.

| owner | (a) | (b) | (c) | (d) | total |
|---|---|---|---|---|---|
| CLEAVE (this row) | 7 | 1 | 13 | 3 | 24 |
| D10-held (declared-pair / declaration ground) | 3 | 23 | 4 | 5 | 35 |
| PRED | 3 | — | 1 | — | 4 |
| CHART | 3 | — | 1 | — | 4 |
| TOPO | — | — | 1 | — | 1 |
| RESTFRONT | — | — | 1 | — | 1 |
| **total** | 16 | 24 | 21 | 8 | 69 |

ENCL holds no live unit. Its umbrella `hand-minted-invalid-gates-in-topo`
is still open, but both of its units have closed: PR 4474, the
material-pairing gate, and PR 4497, the poisoned endings. Its remaining
sites are the PRED, CLEAVE and D10 rows in this table.

**CLEAVE**: 23 topo sites, plus one in sweep.
`invalid_margin.rs:26` is the helper itself and is not counted.

| # | site | predicate / shape | class |
|---|---|---|---|
| C1 | `splitting/rules.rs:197` `apply_rule_a` | `split_sector_extent`: a face extent (a magnitude) decided Zero | a |
| C2 | `rules.rs:319` `apply_rule_a` | `enters_material` reads `Tangent` after the parallelism gate | c, one fact decided twice |
| C3 | `rules.rs:488` `wall_graze` | `wall_bend_order2`: `Exits`/`Tangent` after rule (a) read "enters" | c, decided twice |
| C4 | `rules.rs:500` `wall_graze` | `wall_bend_order2`: the sectors disagree | c, disagreement or straddle (ledger F11) |
| C5 | `splitting/classify.rs:608` | `split_conic_graze_side` decided Zero (reachable at K ≤ 2) | a |
| C6 | `splitting/containment.rs:311` `ReadEscalation::straddle` | two bounds straddle the band | a (item 6) |
| C7 | `containment.rs:1993` | `point_in_arc_loop_boundary_disagreement` | c, disagreement |
| C8 | `splitting/order.rs:126` `in_plane_frame` | `split_join_frame_arm`: every member decided non-positive | a (the #3686 correction) |
| C9 | `chord_join.rs:907` `agreed_section` | `pc_parallel_gap_disagreement`, `pc_axis_plane_parallel_disagreement` | c, two independent readings |
| C10 | `boolean/contain.rs:626` | `bool_contact_arc_end_vertex` (I11) | c, disagreement |
| C11 | `boolean/sectors.rs:418` `bisector_zero_refusal` | `bool_sector_bisector_side`: an honest `±zero` enclosure, off the log (N19) | c |
| C12 | `boolean/plane_eq.rs:102` `orientation_zero` | `bool_plane_orient` decided Zero, decided margin, off the log (I17). Caller `:254` is undeclared; caller `carrier_eq.rs:726` is held | a |
| C13 | `boolean/solid_contain.rs:2760` | `bool_wall_junction`: the pieces' sides disagree | c |
| C14 | `solid_contain.rs:2766` | `bool_wall_junction`: both ends of a piece are active | c |
| C15 | `solid_contain.rs:2774` | `bool_wall_trim` decided Zero. It may be a graze (`Ok(None)`), like the sibling Zero arms | a |
| C16 | `solid_contain.rs:3534` `latitude_extremes` | `bool_sphere_trim_latitude`: no levels | d, structural |
| C17 | `solid_contain.rs:5405` | `bool_ray_torus_count`, `CountDisagrees` | c, invariant |
| C18 | `boolean/sphere_region.rs:384` | `bool_sphere_region_roots_count`, `CountDisagrees` | c, invariant |
| C19 | `census.rs:2583` (RESTREAD ground) | `material_wedge_side`: `Transverse` after the edge screen (I8) | c, decided twice |
| C20 | `boolean/carrier_eq.rs:168` `CoincidenceMeasure::decide` | an unreadable datum that is not finite | d, honest poison |
| C21 | `carrier_eq.rs:836` `coincident_as_declared` | the sum decided nonzero where every datum decided zero (N6), honest margin | c, disagreement |
| C22 | `boolean/mod.rs:1238` `unglued_coincidence` | `carrier_unglued_coincidence`: a decided Zero dressed as an escalation (added by E) | b |
| C23 | `merge_faces.rs:2901` (FUSE/TOPO ground) | `LoopWinding` decided zero, decided margin, off the log (step 1's list) | a |
| C24 | `sweep/src/blend/battery.rs:128` `measured` (BAND/CARVE ground) | a NaN reading → `INVALID` | d, honest poison |

C22 no longer reaches `INVALID`. Its two callers (`recl.rs:150` and
`vtxfac.rs:788`) now raise `ClassificationInvariant` when no margin was
read.

**D10-held: 35 arms.** Every caller is a declaration verifier, a declared
seat, or a declared-pair reading. D10 stage 4 deletes these rather than
types them: `declared-pairs-retire` covers `BooleanCoincidence` (Contact,
Continuation and Seam), and `mates-declare-no-contact` covers the census
`ContactClass` path.

- `boolean/contact_verify.rs`:
  - `:171`, `:337`, `:371`, `:385`, `:435`: class (b), I1 and I3–I6.
  - `:185`: class (c), I2, an invariant.
  - `:460`: class (a), I7, on the undeclared detector posture.
  - These seven are filed on SECTOR's
    `contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`, which
    moved there from CONTACT.
- `carrier_eq.rs` `definite()` (`:646`), used at `:722`
  (PlanesNotParallel), `:759` (KindsDiffer) and `:798` (the attributed
  fact): class (b) ×3.
- `carrier_eq.rs:768` `unsettled`: one arm is class (c) (an impossible
  Negative) and one is class (a) (a straddle).
- `boolean/mod.rs`:
  - `:5297` `sense_contradiction`: class (b) ×3.
  - `:5643` `verify_tangency_declaration` label: class (b) ×3.
  - `:5765`, a relabel: class (b).
  - `:5871` `tangent_rim_refusal` label: class (b) ×7.
- `vtxfac.rs:549` `bool_sector_coplanar`: class (b).
- `rim_wedge.rs`:
  - `:509`: class (d).
  - `:573`: class (a).
  - `:1017`: class (d).
  - `:1045`, `:1079`: class (c).
- `merge_faces.rs:2344` and `:2381` `merge_pair_extent`: class (d).
- `flush.rs:315` `EXTENT_UNREAD`: class (d).

**Other programs' rows.**
- **PRED.** `solid_contain.rs:1289` and `:2987` (`bool_wall_trim_period`,
  class (a), also INSIDE's `revolved-tube-wall-refuses-bool-wall-trim-period`)
  sit on `period-headroom-margin-has-no-shared-home`. `:1938` (nappe
  Negative, class (c)) and `:1943` (side Zero, class (a)) sit on
  `cone-nappe-is-decided-in-five-places`. ENCL's seam note on PRED's
  log is dated 2026-10-09.
- **CHART.** `chart_region.rs:1752`, `:3230` and `:3466` (`definite_diag`,
  class (a)) are on `chart-definite-diag-labels-an-interval-lower-end-as-an-f64-value`.
  `:1512` (`chart_region_cyl_axis_sense` Zero, class (c)) is on
  `chart-region-mints-indeterminates-after-a-definite-sign`.
- **TOPO.** `plane_eq.rs:303` `unreadable_norm` (a norm decided
  Negative from input) is on `boolean-unreadable-norm-ends-as-a-kernel-defect`.
- **RESTFRONT.** `validate.rs:5747` `material_arm_error` `Split` is on
  `ring-contact-and-sliver-split-endings-want-their-decisions`.

**Retired since the 2026-10-03 retake.**
- **Step 1 (PR 3979):** I9, I10, I13, I23, I26 ×3, I27,
  `chart_region`'s tilt and `norm_gate`, and `short_arm`.
- **PR 3974 and stage 4 E:** I18 and N3. `CoincidenceMeasure` now has
  two arms, `Undecided` and `Unreadable`.
- **PR 4433 and PR 4474:** I24's validator twin and N20.
- **Rule (b)'s rework:** N16. `rules.rs` now returns `Ok(None)` on
  `Tangent`.

### Does step 3 still fit? No. Step 3 is superseded, and main already has the doors it wanted

- **Every class-(b) arm but one is on D10-held ground.** Of the 24
  class-(b) arms, 23 are D10-held, and stage 4 PR F deletes the variants
  that carry them: `ContactContradicted`, `ContinuationContradicted`,
  `SeamContradicted`, `ContactRefusal::Contradicted`, and
  `CarrierEqError::Contradicted` through `declared_reading`. Typing their
  evidence now would type variants that are about to go. The one
  non-held class-(b) arm is C22, a decided Zero rather than a
  contradiction, and design item 7 covers it. After stage 4, what
  survives is re-censused (unit 5 below).
- **`Definite { predicate, sign, margin, band }` would be a third name
  for a decided reading.** Main already has two:
  - `geom_core::Decided { sign, margin }`, from `decide_reported` and
    `decide_magnitude_reported`;
  - `geom_brep::recourse::Classified { margin, band }`.

  On the ending side, ENCL landed `RefusedArm::SignCertain(Option<MarginDiag>)`,
  `lever_recourse` and `MarginDiag::unreadable_note`. On the gate side,
  `MarginDiag::rejected_sign` is item 2's "a gate rejection keeps the
  decided margin", and `decide_magnitude` is item 5. If a contradiction
  survives stage 4, its evidence is `(predicate, Decided)` beside the
  band its error already carries. No new type is needed.
- **Main also has the door for "how a reading stands".**
  `splitting::containment::Escalation::{Margin, Straddle, Decided}`
  carries it, and `RefusedArm::Straddle` ends it. CLEAVE's
  `split-escalations-end-a-poisoned-margin-in-the-plane-lever` asks for
  that door to be carried to `SliverSector`, `SplitJoinError` and
  `SectionError`, which is what units 1 and 2 need.
- **Main has no door for item 6's straddle enclosure on the log.**
  There is no `k_stats` door for a straddle, so unit 2 adds one. That is
  SCALAR's ground, in `geom-core`.
- **The conflict with ENCL's endings is real, and it constrains every
  unit.**
  - Some readers give a contradiction its defect ending because the mint
    says `INVALID`: `chart_region.rs:433` (`ChartRegionError::Escalated`),
    `validate.rs:2509` (`own_close`/`too_close`, which C19 reaches), and
    `geom_brep` `RefusedArm::unreadable`.
  - A unit that swaps `INVALID` for an honest decided margin on a class-(c)
    arm therefore flips that arm's ending, silently, to the lever and
    tighten menu, unless the arm becomes a typed contradiction or a typed
    `Escalation::Decided`. Each unit pins each moved arm's ending, before
    and after.
- **A cost of routing class (a) through the gate doors.** SCALAR's open
  `a-gate-rejection-of-a-decided-enclosure-bisects-to-budget` says the
  driver bisects a decided gate rejection until its budget runs out. That
  is not a blocker, because every existing gate rejection has the same
  cost, but units 1 and 4 add rejections to it.

### CLEAVE's remaining units

Each unit is one PR, filed as its own row with `parent:` set to this row.

1. `split-sector-rules-mint-no-invalid` (M): C1–C5.
2. `split-straddles-and-disagreements-carry-their-reading` (M): C6–C9,
   plus the `k_stats` straddle door.
3. `point-in-solid-mints-outside-its-period-and-nappe-rows` (M): C13–C18.
   PRED's four sites stay on PRED.
4. `boolean-gate-mints-route-through-the-funnel` (M): C10, C11, C12, C19
   and C23. It touches RESTREAD's and FUSE/TOPO's ground; the PR
   announces the seam.
5. `glue-disagreements-and-held-contradictions-after-stage-4` (M,
   `design: true`): C20–C22 and C24's reading, plus the re-census of
   the 35 D10-held arms after stage 4. Parked on `intent-stage4-is-built`.
6. `the-indeterminate-seal` (M): step 5. Parked on units 1–5 and on the
   other programs' rows. The seal cannot land early, because
   `#[non_exhaustive]` and a crate-private `INVALID` are crate-wide in
   `geom-core`: they break every remaining hand mint in every crate at
   once. It therefore waits on PRED, CHART, TOPO, RESTFRONT and SECTOR's
   rows as well as on stage 4.
7. `hand-minted-indeterminate-ratchet-gate` (E, P3): a `scripts/gates/`
   per-file count of production hand mints that can only go down.
   - This holds the census until the seal can land.
   - New mints have appeared since the census was taken: N1 in PR 3513,
     C22 in E, and `definite_diag` at step 1.

Units 1–6 are P1, not P0: no remaining site is a wrong answer on normal
geometry, and the user-visible text that drove P0 went with stage 4 E.
The row's header is left as the orchestrator set it.
