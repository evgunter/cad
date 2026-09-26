---
id: the-apothems-sign-is-a-value-read
kind: issue
title: the apothem's sign: six arc-family decisions on a parameter bulge are zero exactly where the apothem L(1-b^2)/(4b) is positive, which only a value read reaches
status: dispatched
opened: 2026-09-25
priority: P2
cost: H
refs: [rule-d-reaches-the-unit-bulge-only, 3186]
parent: DECIDE-4
---

**Filed from DECIDE-4's fork, on Ev's ruling on `[ev]` #3186 (2026-09-25):**
this becomes its own item, not a widening of DECIDE-4.

## What stands

DECIDE-4's Phase 1 counted these on the `0.5` parameter control of R2's
D-tab (`r2_d_tab_parameter_dyadic`):

- `line_span`, `arc_span` and `contact_at_shared_vertex`, two each;
- `line_span`'s two on the `0.4` parameter D-tab.

None of them is frozen. The early form is
`u·(|2b|·2(1 − b²) − |4(b² − 1)|·b) / (|2b|·2b)`: the magnitude
`sqrt(r² − (L/2)²)` of the centre's offset against the signed offset
`L(1 − b²)/(4b)`. Evaluated at points in every sign region, it is zero
exactly on `0 < b < 1` and on `b < −1`, which is where the apothem is
positive. So these decisions stand on the apothem's sign, not on `b`'s.

The uncut forms are in `crates/editor-core/tests/m10_bulge_renders.txt`
(on `props/sign-hull` from DECIDE-4's merge), and the attribution is the
section "What stands on DECIDE-3's and SYM-9's tree (DECIDE-4)" of
`rule-d-reaches-the-unit-bulge-only`.

## Why neither sign route reaches it

- **The turn `σ` (route B, ruled for `b`'s sign)** relates `|b|` to `b`.
  The apothem's sign is `sign(1 − b²)·σ`, which the profile program does
  not decide.
- **Rule C on as the dial stands (route A)** does take all six. But it
  also re-labels theorems as `sign_gated` on every document, loses the
  door on `pcurve_map_residual`, and raises `numeric` everywhere. It was
  rejected on #3186.

## The candidate

A NARROWED rule C: a read that folds only `abs`/`sqrt` atoms whose
argument has a certified sign over the leaf's box, ordered behind the
door and behind every value-free fold, as the decision read is. Its
zeros are `sign_gated`, which is the honest label for a fold that holds
on the box and not identically. It is unmeasured. The first step is to
build it behind a dial and count what it takes and what it re-labels,
on the six documents and the bulge controls.

A structural alternative, owed a look before the read is built: whether
the sweep's arc construction already decides which side of the chord
the centre lies on. If it does, that decided sign could be stated the
way route B states the turn.

## What Phase 1 found (DECIDE-8)

Measured on `decide/8-apothem-sign` (the code at `f1e04e124`, the
instrument's leaf row widened to the two parameter D-tabs in the next
commit), ε = 1e-9, the drive's default ladder (`CAD_M10_10_RETRY=default`).

### 1. The structural look: the apothem's sign is not decided upstream

Nothing upstream of the sweep decides `sign(1 − b²)`, or the apothem's
sign, as a certified `Sign` over the box. Every decision the profile
makes about a bulge arc, read in full:

- **`path_arc_bulge`** (`profile/src/path/family.rs`, `bulge_carrier`,
  line 955) decides `sign(b)`: the winding. It is route B's `σ`.
- **`segment_straightness`** (`profile/src/seg.rs`, `build_seg`, line
  221) decides `sign(L·b/2)`: line against arc, and the turn it hands
  `SegmentKind::Arc { turn }` (`validate.rs`, line 1104). `sign(b)`
  again.
- **`arc_diameter_clearance`** (`seg.rs`, `build_seg`, line 233)
  decides `2r − |a − apex| > 0`, the near-full-circle gate. The
  half-span chord `|a − apex| = 2r·sin(θ/4)` with `|θ/4| < π/2`, so the
  margin is positive on every valid arc, minor or major. It does not
  separate `|b| < 1` from `|b| > 1`.
- **`path_arc_sweep`** (`profile/src/path/verbs.rs`, `tangent_arc_leg`,
  line 384) decides `angle > 0` for the endpoint-free modes only.
- **The apothem itself is SIGNED and read by no predicate**:
  `seg.rs`'s `arc_carrier` (line 173) and `path.rs`'s `arc_carrier`
  (line 2602) spell `L·(1 − b²)/(4b)` and put the centre at
  `mid + n̂·apothem`. The radius is `|L·(1 + b²)/(4b)|` (`seg.rs`, line
  177), and that `abs` is where the magnitude enters.
- **`sugar.rs`'s major-arc branch** (`fillet_bulge`, line 1394) is a
  `copysign` on a value (`apothem.copysign(sgn·cross)`, line 1402), not
  a decision. It is reached only by fillets, and the gates keep a
  fillet under half a turn.
- **The sweep receives the profile's carrier and turn**
  (`sweep/src/swept.rs`, `swept_segments`, line 256, into
  `placed_segment_spec`, line 548) and decides nothing about the side
  of the chord.

**One decision equals `sign(1 − b²)` on the D-tab, by the D-tab's
geometry.** `path_junction_turn` (`path.rs`, `junction_check`, line
2637) decides `sin` of the turn from the incoming leg to the arc's
start tangent. On the D-tab the incoming line is perpendicular to the
chord, so that turn is `π/2 − 2·atan b`, whose sine is
`(1 − b²)/(1 + b²)`. That is a property of the joint, not of the arc: on
any other incoming direction it is another quantity. `junction_check`
drops the `Sign` once it is not `Zero`. Carrying it would be a new
decision in the profile program, not one it already makes, so it is
out of this unit's scope.

**Where the six are asked.** They are asked in the PROFILE's
validation pass, not in the sweep. A grep of every non-test source
under `crates/` for the three predicate names finds three decide
sites, all in `profile`: `seg.rs` lines 282 (`line_span`) and 306
(`arc_span`), and `validate.rs` line 1840 (`contact_at_shared_vertex`).
`validate.rs` lines 674–694 name them in a table and decide nothing.
`seg.rs`'s `line_arc` (line 681)
puts the line/circle candidates at `foot ± sqrt(r² − h²)` (line 706).
On the D-tab the line through the arc's endpoint is perpendicular to
the chord, so `h = L/2` and `sqrt(r² − (L/2)²)` is the apothem's
MAGNITUDE. The shared vertex sits at the SIGNED apothem.
`line_span` and `arc_span` are then asked at that candidate (line
709), and `contact_at_shared_vertex` compares it with the vertex
(`validate.rs`, line 1840). A spelling in the sweep would therefore not
reach them: they are decided before the sweep runs. **The structural
route is not open.**

### 2. The narrowed read, behind `SymRules::signed_root_last` (shipped off)

**What it is.** Rule C's fold (`signed::fold`: `sqrt(X) → ±R` where
`X = R²` as forms, and `abs(R) → ±R`, the sign CERTIFIED by
`RingInterval`'s outward-rounded enclosure of `R` over the leaf's
parameter brackets, strictly one-signed, never sampled). It runs at
each `sqrt`/`abs` node and at rule G's magnitude step (`root.rs`,
`magnitude_of_root`). Rule G's side-condition read (source 4) stays
behind `signed_root`.

**Where it is asked.** In a walk of its own (`WalkKind::SignRead`, its
own memo), as a LAST rung (`Rung::SignRead`). It is asked after the
plain, early, top and door rungs and after every retry attempt, on the
decision path only, and only in a session with a parameter bracket. So
it can take a decision out of `numeric` and out of nothing else. Unit
rows: `sym::tests::the_last_read_takes_a_signed_root_the_ladder_left_numeric`,
`the_last_read_relabels_no_theorem_that_rule_c_relabels` (with rule C
on, `sqrt(r²) − |r|` is counted `sign_gated`; asked last, it stays a
theorem) and `the_last_read_never_folds_without_a_certified_sign`.

**The splits** (`m10_10_splits_at_the_nominal_under_a_rule_set`, dev,
`CAD_M10_10_RULES=shipped | last_read | all`; `all` is route A, rule C
in the early walk). The totals are summed over every predicate
(theorem/gated/registered/numeric):

| document | shipped | the read (`last_read`) | route A (`all`) |
| --- | --- | --- | --- |
| boss, `bulge = 2` | 375/2/96/233 | the same | 168/287/0/251 |
| D-tab, literal `0.4` | 571/0/84/422 | the same | 487/136/14/440 |
| D-tab, parameter `0.4` | 571/0/80/426 | the same | 467/156/4/450 |
| control, literal `0.5` | 643/0/70/364 | the same | 559/136/0/382 |
| control, parameter `0.5` | 607/0/90/380 | **607/6/90/374** | 397/284/0/396 |
| bracket (class sample) | 1104/7/150/760 | **1104/35/150/732** | 486/757/0/778 |
| plate | 811/0/140/462 | the same | 643/272/0/498 |
| annulus | 328/0/140/209 | the same | 156/276/0/245 |
| link | 549/0/120/433 | the same | 345/256/24/477 |

- **What it takes: the six, and 28 on the bracket.**
  - On the `0.5` parameter control, `arc_span`, `line_span` and
    `contact_at_shared_vertex` each go 4/0/0/4 → 4/2/0/2. Those are the
    six asked decisions the item names. The two left in each are the
    DEFINITE ones, never asked.
  - On the bracket (`fillet_r` a parameter): `tangent_on_surface_1`
    and `_2` 9/0/0/9 → 9/9/0/0 each, `tangent_hull_sup` 0/0/0/2 →
    0/2/0/0, `carrier_on_surface_2` 225/0/0/18 → 225/4/0/14,
    `witness_on_surface_1` 28/0/0/1 → 28/1/0/0, `witness_on_surface_2`
    26/0/0/3 → 26/1/0/2, `line_span` 6/5/0/1 → 6/6/0/0 and
    `contact_at_shared_vertex` 19/0/0/5 → 19/1/0/4.
- **What it re-labels: nothing.** The theorem and registered columns
  are identical to the shipped set's on all nine documents, per
  predicate. Theorem → `sign_gated`: 0. Door → `sign_gated`: 0.
- **What it loses against route A: nothing measured.** Route A's
  `numeric` is never below the read's on any predicate of any of the
  nine. The read takes every decision route A takes out of `numeric`
  there. Re-taken at this head, route A still re-labels theorems on
  every document (the boss 375 → 168, the plate 811 → 643). It loses
  the door on `pcurve_map_residual` everywhere (the plate 0/0/36/0 →
  0/0/0/36). It raises `numeric` on every document (the control
  380 → 396, the plate 462 → 498). #3186's route-A numbers were taken
  before DECIDE-5; the control's shipped `numeric` was 392 then and is
  380 now.
- **What neither reaches at the shipped ring: the `0.4` parameter
  D-tab's `line_span` pair.** Route A does not take them either.
  Route A with one retry at a 512-bit ring (`CAD_M10_10_RETRY=ring_512`)
  takes them, with `arc_span`'s and `contact_at_shared_vertex`'s pairs
  (each 4/0/0/4 → 4/2/0/2). The read with the same retry does not:
  it is asked at the first attempt's ring, and `fl(0.4)`'s mantissa is
  in front (DECIDE-4's `Coefficient` causes).

**Leaf cost** (`m10_10_leaf_cost_with_and_without_the_algebra`,
release, one whole-box leaf, the `ON + the ladder` column, best of 3,
`CAD_M10_10_TAKES=3`). The pad is taken on this leaf instrument only.
Receipts are theorem/gated/registered/numeric. Every `certifies_whole`
verdict is unmoved. Route A's column is without the pad:

| document | off | on | receipt, off → on | route A |
| --- | --- | --- | --- | --- |
| plate `1e2·ε` | 0.227 s | 0.218 s | unmoved | 0.100 s |
| plate, real study | 0.226 s | 0.224 s | unmoved | 0.042 s |
| bracket `1e1·ε` | 2.055 s | 2.852 s | 1104/7/150/760 → 1104/35/150/732 | 3.634 s |
| annulus `1e1·ε` | 0.229 s | 0.222 s | unmoved | 0.078 s |
| pad `1e2·ε` | 38.998 s | 41.858 s | unmoved, 890/6/150/907 | — |
| link `1e1·ε` | 5.968 s | 6.706 s | unmoved | 6.285 s |
| boss `1e2·ε` | 0.162 s | 0.174 s | unmoved | 0.077 s |
| D-tab, parameter `0.4`, `1e2·ε` | 0.273 s | 0.306 s | unmoved | 0.216 s |
| control, parameter `0.5`, `1e2·ε` | 3.529 s | 3.896 s | 607/0/90/380 → 607/6/90/374 | 2.948 s |

That is +39% on the bracket, +12% on the link and the `0.4` D-tab,
+10% on the control, +7% on the pad and the boss. The plate and the
annulus are unmoved to the noise.

### 3. The route

**Only the read reaches the six** (the spec's second stop rule). The
structural look found no decision of the apothem's sign upstream. The
six are asked in the profile's validation, where a sweep-side spelling
cannot reach them. The read takes them, re-labels nothing and loses no
door, at the cost above. Whether the tier answers there on the box
(`sign_gated`) is Ev's call. The unit stops after Phase 1 with the
dial shipped OFF.
