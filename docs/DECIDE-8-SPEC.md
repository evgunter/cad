# DECIDE-8 — the apothem's sign: stated where it is decided, or read where it is not (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/the-apothems-sign-is-a-value-read.md` (P2, cost H), filed
on Ev's ruling on `[ev]` #3186 (Decision 2). **Unit:**
`work/decide/DECIDE-8.md`. **Branch:** `decide/8-apothem-sign`, cut from
`props/sign-hull` at DECIDE-7's merge. The PR targets `props/sign-hull`,
and no `main` is merged into it. **Implementer:** Opus. **Review tier:**
set by what Phase 1 finds (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item whole, and the section "What stands on DECIDE-3's and SYM-9's
  tree (DECIDE-4)" of `work/decide/rule-d-reaches-the-unit-bulge-only.md`;
- `[ev]` #3186's body and Ev's ruling: route B for `b`'s sign, and this
  item on its own;
- `crates/sweep/src/swept.rs`: `turn_negates`, `turn_axis`,
  `centre_on_material_side`, `turned_span`, `placed_segment_spec`, and
  the closed forms at lines 410–420 (`apothem = len·(1 − b²)/(4b)`);
- `crates/profile/src/path/family.rs`'s `bulge_carrier`: what the
  profile decides about an arc (`path_arc_bulge`, `SegmentKind::Arc
  { turn }`), and whether any decision there already separates `|b| < 1`
  from `|b| > 1`;
- `crates/profile/src/seg.rs:162` and `sugar.rs:1370` (the major-arc
  branch, which `sugar.rs` calls defensive for fillets);
- `crates/geom-core/src/sym/manifest.rs` and `sym/signed.rs`, for what
  rule C and the decision read are, and the dials on `SymRules`;
- `crates/editor-core/tests/m10_bulge_interval.rs` and
  `m10_bulge_renders.txt` (the six early forms, uncut).

## The claim

On the `0.5` parameter control of R2's D-tab
(`r2_d_tab_parameter_dyadic`) six arc-family decisions are not frozen
and not theorems:
- `line_span` ×2, `arc_span` ×2 and `contact_at_shared_vertex` ×2;
- plus `line_span` ×2 on the `0.4` parameter D-tab.

Each early form compares the magnitude `sqrt(r² − (L/2)²)`, spelled
`|4(b² − 1)|`-over-`|2b|`, against the signed offset `L(1 − b²)/(4b)`.
It is zero exactly where the apothem is positive. The turn `σ` (route B)
relates `|b|` to `b`, but the apothem's sign is `sign(1 − b²)·σ`, and
nothing hands the tier `sign(1 − b²)`.

**Ratified and not re-litigated:**
- Ev's Decision 1 on #3186: the span from the decided turn, shipped as
  DECIDE-5;
- Decision 2: this item, not a widening of DECIDE-4;
- rule C stays shipped OFF as the dial stands (route A was rejected on
  #3186 for the re-labels it costs on every document);
- rule G, the exact quotient, and SYM-9's ladder.

## Phase 1 — before building anything

1. **The structural look (the item's "owed a look").**
   - Does anything upstream of the sweep already decide which side of
     the chord the centre lies on, as a certified `Sign` over the box?
     That is, `sign(1 − b²)`, or `sign(apothem)` itself: minor arc
     against major.
   - Look at the profile's classification (`bulge_carrier`,
     `path_arc_bulge`), the carrier's construction in `placed_segment_spec`,
     and any major-arc test the profile or sugar makes.
   - Say which, with `file:line`. If one exists, name the decision and
     where it is dropped before the sweep.
2. **The narrowed read, measured behind a dial shipped OFF.** Build the
   item's candidate:
   - a fold of `abs`/`sqrt` atoms whose argument has a CERTIFIED sign
     over the leaf's box;
   - ordered behind the door and every value-free fold, as the decision
     read is;
   - its zeros `sign_gated`.

   On the six instrument documents plus the bulge controls (both
   parameter D-tabs, both `0.5` controls, the boss), count:
   - what it takes: the six, and anything else;
   - what it re-labels: theorem → `sign_gated`, door → `sign_gated`;
   - what it loses, against rule C on as the dial stands (#3186's route
     A numbers).

   Leaf cost, best of 3, release, with the dial on against off.
3. **One table, and the route.**

**Stop rules.**
- **If item 1 finds the sign decided upstream,** Phase 2 states it the
  way route B states the turn. The sweep spells the apothem from the
  decided side, value channel bit-identical. That is route B's pattern
  under Ev's Decision 1, and it does not stop. The dial from item 2 is
  then removed, not shipped: its table goes in the item as the
  comparison.
- **If only the narrowed read reaches the six,** stop after Phase 1
  and report. The read is a new answer the tier gives on the box, and
  whether it ships is Ev's call. The orchestrator has the fork weighed
  by two designers (`docs/prompts/designer.md`, the procedure in
  `memories/orchestration-model.md`), then takes it to Ev as an `[ev]`
  PR carrying the table. The dial may land OFF as the Phase 1 record.
- **If neither reaches them,** the unit closes at Phase 1 and the item
  records why.

## Phase 2 — the structural route, if item 1 finds it

- **The spelling.** Spell the apothem from the decided side through ONE
  helper beside `turn_negates`, with the `Zero` convention argued as
  DECIDE-5 argued its own. No second spelling of the convention.
- **Bit-identity.** Show that no value moves: the value-channel pins,
  the sweep crate's tests, and the digests that hash values.
- **Re-take every split and ceiling the change moves.** Re-baseline and
  say each; the six are the expected movers. Ev's standing words:
  "never skip out on a change that would make the code better because
  it would require rebaselining".
- **A row that fails** if the apothem is spelled through `abs` again,
  at a parameter bulge on each side of `|b| = 1` that the documents
  reach.
- **Leaf cost**, best of 3, release, before and after.

## Scope

- **Files:**
  - `crates/sweep/src/swept.rs`;
  - `crates/profile/src/path/family.rs`, only to carry a decision it
    already makes;
  - `crates/geom-core/src/sym/` for the narrowed read behind its dial;
  - tests; the unit and item files.
- **Territory:** `crates/sweep` is CARVE's and BAND's, and `crates/profile`
  is PROFILE's. Run `work.py territory --base origin/props/sign-hull`
  and announce the seams in the PR body.
- **Not in scope:**
  - a new decision in the profile program (carrying one it makes is in
    scope, adding one is not);
  - rule C's shipped default;
  - any other dial's default.

## Review

**Set by Phase 1, recorded in `work/decide/log.md` at spec time.**
- The structural route: a single FULL review, as DECIDE-5.
- A Phase 1 stop: the designers weigh the fork, the `[ev]` PR carries
  the table, and the dial's code gets a single FULL review before it
  lands off.

Claims to falsify:
1. the structural look's answer (a decision that exists, or not);
2. the narrowed read's soundness: a certified sign over the box, never
   a sampled one;
3. bit-identity, if the structural route is taken;
4. the re-baselines;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes the item or re-states it
  with what is left.
