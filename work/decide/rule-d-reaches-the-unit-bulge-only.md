---
id: rule-d-reaches-the-unit-bulge-only
kind: issue
title: rule D's reach is the unit bulge: a parameter bulge is outside the mechanism and a literal bulge other than 1 leaves residue — the next ceiling class after M10-10
status: open
opened: 2026-09-12
refs: [2100, plate-ceiling-is-now-the-scaffold-pushforward, symbolic-tier-census]
parent: SYM-3
priority: P1
cost: D
---

**Filed by M10-10's fix pass from both reviews' end-to-end probes**
(PR #2100; R1's `m10_10_r1_probes_interval`, R2's
`m10_10_r2_probes_interval`, adopted in-tree as evidence rows). The
unit's headline — "the plate's four identity residuals all discharge
and its real study certifies" — is a property of the UNIT BULGE: every
measured document (the plate, R1's annulus, the split-bore disc)
authors its arcs through `LoopProgram::Circle`/`CircleSplit`, whose
kernel bulge is the literal `1`, so `θ = 4·atan 1`, `abs(1)` folds
under A0 and the two spellings of the arc meet at every sample. Two
documents that author a bulge any other way measure the limit.

## The two measurements

- **A LITERAL bulge other than 1** (R1's circular-segment boss: one
  chord closed by a major arc at `bulge = 2`, a bore inside, the web
  between the arc wall and the bore wall the measure;
  `r1_the_segment_bosss_per_predicate_split_at_the_nominal`): under
  the shipped set `carrier_matches_mapped_source` is 6 of 54 still
  numeric and `carrier_on_surface_2` 27 of 90, and the document's
  whole-certifying ceiling is UNMOVED by the algebra. Rule D folds the
  trig — `sin`/`cos` of `q·atan 2` are closed forms in `sqrt 5` — but
  what the carrier's frame carries at a non-unit bulge is `atan|b|`
  against the pushforward's `atan b` and the sagitta forms in `b`
  that the unit bulge collapses; the ring closes only part of it.
  (SYM-3's render corrects this guess: at a literal bulge `abs(2)`
  folds and the two atans are one; what stands is the carrier's
  `abs(signed_radius)` over the coefficient ring — "What stands
  (SYM-3)" below.)
- **A PARAMETER bulge** (R2's D-tab: a rectangle whose right side is
  an `ArcTo(Bulge)` arc, the bulge a document parameter, a hole
  inside; `r2_evidence_the_d_tab_end_to_end`): ceiling `3.52e2·ε`
  with the algebra on and off ALIKE. Entirely outside the mechanism —
  which is the honest limit rule D's own pins state
  (`rule_d_meets_the_carrier_and_the_pushforward_at_every_sample`: a
  parameter bulge stays numeric, `atan|b|` against `atan b` related by
  the turn SIGN, which no value-free rule reads).

## What this is

The next ceiling CLASS after M10-10, distinct from the two the census
already names (the carrier frame's squared components freezing at the
term budget on the link and the bracket; the identity-shaped
`line_span` on the pad): the arc family's algebra at a bulge that is
not `1`. It is a form-level question (what `atan|b|` is as a form when
`b` is not a literal; whether the sagitta closed forms in `b` meet the
carrier's frame), not a door's, and it is the reason the tour's stop 1
says "the unit bulge" beside its numbers rather than "arcs".

## What is owed

- The mechanism's reach stated wherever the unit's reach is claimed
  (done in the fix pass: the census, the tour's caption, E12's
  paragraph, the PR body).
- A measurement of WHAT stands at `bulge = 2` (the rendered residual,
  as M10-10 rendered the plate's four) before any rule is proposed;
  R1's boss is the fixture for it and its row prints the split.
- Whether `abs(b)` for a parameter `b` is a sign rule C's kind (a box
  that does not straddle zero decides it, `sign_gated`) or a structural
  fact of the authoring door (a bulge's sign is the arc's turn, which
  the profile program knows) — the same shape as the chart phase was,
  and the same two routes.

## Re-homed at M10's exit sweep (2026-09-13)

Here because rule D is the tier's own mechanism and this is the next ceiling class
after M10-10.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## What stands (SYM-3)

The two measurements rendered (`m10_10_evidence_interval`: the boss
and both D-tabs are in `documents()` as `r1_segment_boss`,
`r2_d_tab_literal`, `r2_d_tab_parameter`, plus two dyadic controls
`r2_d_tab_{literal,parameter}_dyadic` at `bulge = 0.5`; the nominal row
and `m10_10_what_stands_rendered` take them by `CAD_M10_10_DOC`), at
the nominal and at ceiling + δ, shipped and `without_the_algebra`, at
ε = 1e-6, 1e-9 and 1e-12. The trimmed renders are
`crates/editor-core/tests/m10_bulge_renders.txt`; the nominal splits
are pinned in `m10_bulge_interval`. The atoms do not differ by ε row:
the three rows print the same split and the same atom census on every
document, and the brackets scale with ε where they are ε-relative.

### The brackets

| document | rules | certifies .. refuses | over the band at ceiling + δ |
| --- | --- | --- | --- |
| boss (`bulge = 2`) | shipped AND off | `8.2611e2 .. 8.2643e2 · ε` (all three rows) | `carrier_matches_mapped_source` `[0, 1.0004·ε]` 1/125, sample 7 |
| D-tab, literal `0.4` | shipped | 0.4308 (1e-6) / 0.5611 (1e-9, 1e-12) of the REAL study | `dihedral_wedge` `[1.0e-5, 2.5e-2]` at 1e-6; `arc_diameter_clearance` `[−5.2e-8, 5.6e-4]` at the finer rows |
| D-tab, literal `0.4` | off | `7.8109e2 .. 7.8139e2 · ε` | `carrier_matches_mapped_source` 1/117 |
| D-tab, parameter (nominal `0.4`) | shipped AND off | `3.5218e2 .. 3.5232e2 · ε` | `carrier_matches_mapped_source` `[0, 1.0004·ε]` 1/89, sample 7 |
| control: D-tab, parameter (nominal `0.5`) | shipped AND off | `4.3375e2 .. 4.3392e2 · ε` | `carrier_matches_mapped_source` 1/90, sample 8 |

The literal D-tab's ceiling is not the bulge residue at all: the algebra
takes it into the real-margin class (`real-margin-dependency-widening`,
third site). The boss's and the parameter D-tab's are the residue, and
the algebra moves neither.

### The boss: which atoms stand, and which of the four causes

At the nominal (shipped): `carrier_matches_mapped_source` 72/0/48/6,
`carrier_on_surface_2` 63/0/0/27, `witness_on_surface_2` 7/0/0/3,
`carrier_on_surface_1` 81/0/0/9, `witness_on_surface_1` 9/0/0/1;
`pcurve_map_residual` 0/0/18/0 (the door's, as on the plate). Two
different residues:

**(a) `carrier_matches_mapped_source`, 6 of 54 — a FREEZE at the
coefficient ring (cause 3).** The six are samples `i = 3, 5, 7` of both
rims (`RRRnRnRnR`): `q = 3/2, 5/2, 7/2`, the odd half-multiples. The
early form carries NO trig atom — `EARLY atoms {"sqrt": 265}`: one
`sqrt(5)` and one half-angle atom `sqrt((1 + sqrt(5))/(2·sqrt(5)))`,
so rule D folded every `sin`/`cos` and the two spellings minted the
same atoms (not cause 2, not cause 4: `k ≤ 7`, `m = 1`, inside the
bounds). What froze is the subtraction of one rim component,

```
Sqrt #cb82e1e6 (1,1)/(1,0)
  Add (63,8)/(9,6)
    Powi^2 → Sub #21f7b5bf: FROZEN
      Add #91dbb4d4 (8,4)/(4,3)   coefficients ~2^112 (…·2^-26 mantissas of 5^k·L)
      Add #68e3f6e6 (8,8)/(10,6)  coefficients ~2^206 (2492302492096717365457573652641875·2^-101 · 44171176619459611917571735596706337801018798399365527325632025·2^-232 …)
    Powi^2 (54,8)/(9,6) ← Sub (12,4)/(4,3)
```

— at most 112 terms of degree ≤ 11 against a budget of 4096/128 and
`EARLY_AB_TERMS` 512, so the term caps are not it; the cross-multiplied
numerator needs ~318 bits against `COEFF_BITS = 256`. Confirmed by
measurement (a local patch, reverted): at 512 bits the predicate is
72/0/54/0 — all six through the door — and the boss's ceiling moves
`8.2611e2 → 9.3559e2 · ε` (1.13×), the over-band set becoming
`line_span` `[−1.0003·ε, 1.0003·ε]` 1/2, the identity-shaped
`line_span` the census names on the pad.

**(b) `carrier_on_surface_2`, 27 of 90 (and `witness_on_surface_2` 3,
`carrier_on_surface_1` 9, `witness_on_surface_1` 1) — an `abs` A0 does
not reach (cause 1), and the ring behind it.** The early form is
complete (no frozen node below it), `EARLY atoms {"abs": 6, "sqrt": 6}`:

```
( 44171176619459611917571735596706337801018798399365527325632025·2^-232
  + 38312388521647223851569068091606350623530185575·2^-171·chord_half
  − 1152921504606847·2^-50·chord_half·abs((5·sqrt(L²))/8)²
  + 99692099683868694618302946105675·2^-113·chord_half²
  − 256·chord_half²·abs((5·sqrt(L²))/8)²
  + 28823037615171175·2^-53·chord_half³ + 400·chord_half⁴
  − 1329227995784915928244039281409·2^-110·abs((5·sqrt(L²))/8)² )
/ ( … · abs((5·sqrt(L²))/8) )
        with L² = 1329227995784915928244039281409·2^-116 + 1152921504606847·2^-56·chord_half + 4·chord_half²
                = (4e-3 + 2·chord_half)²
```

i.e. `64·L²·((5L/8)² − abs(5L/8)²) / (2·abs(5L/8)·64L²)`: the sample
point's radius squared spelled as a polynomial in the chord against
the surface's radius squared spelled through `abs`. The atom is the
profile's `seg.rs` `radius: signed_radius.abs()` with `signed_radius =
L·(1 + b²)/(4b)`; `abs(2)` itself DID fold (A0), so the item's guess
"`atan|b|` against `atan b`" is not the boss's residue — at a literal
bulge the two atans are one. What A0 does not fold is an `abs` over a
NON-constant argument (`L` carries `chord_half`), and rule A squares
only `sqrt`. The plate carries the same atom and closes because there
it appears on both sides of its residuals. Measured (two local patches,
reverted, each at 256 bits): `abs(X) = X` for an `X` non-negative by
syntax (`manifest::nonneg`, the atan2 fold's own test, moved there by
SYM-8 — `(5/8)·
sqrt(L²)` qualifies) and `abs(X)² = X²` in rule A's walk give the SAME
numbers: `carrier_on_surface_2` 27 → 18, `carrier_on_surface_1` 9 → 0,
`witness_on_surface_2` 3 → 2, `witness_on_surface_1` 1 → 0 (20 of 40),
the remaining 18 + 2 then FROZEN on the products the opened atom joins
(`sqrt(rim·rim)²` at `Powi^2 #11817592: FROZEN`) — and
`carrier_matches_mapped_source` 48 → 38 registered, ten door decisions
lost that the opaque atom had matched on both sides
(`coefficient-ring-width-is-not-monotone-in-reach`). With the fold AND
512 bits: `carrier_on_surface_2` 72/0/2/16, `carrier_matches` 72/0/50/4.
The ceiling moves under none of the three. So the proposal is not one
line: the sound theorem is `abs(X) = X` on a syntactically non-negative
`X` (no sign read; 20 decisions at the nominal), and it is worth
nothing on the boss's ceiling until the ring can hold what it opens.

### The parameter bulge: the residual in `b`, and the two routes

At the nominal the parameter D-tab's split is the literal's number for
number (126/0/36/18, 117/0/0/27, 13/0/0/3, 135/0/0/9, 15/0/0/1, and
`carrier_endpoint_start` 24/0/8/4 — the rim identity `‖q − c‖ = r`,
the plate's own former ceiling residual, standing on the same two
residues: its early form carries `abs(R(b))` with the `fl(0.4)`
denominator and two frozen nodes; 24/0/12/0 on the `0.5` control,
12/0/12/0 on the boss, still 24/0/8/4 with the ring at 512 bits alone
and 24/0/12/0 once an `abs` fold rides with it — rendered in
`m10_bulge_renders.txt` and pinned), and
every blocked form at `0.4` carries frozen nodes with `fl(0.4)`'s odd
52-bit mantissa (`3602879701896397·2^-53`) in every denominator: the
radius `L(1+b²)/(4b)` has a 156-bit numerator over it and its square
is past the ring. So at `0.4` the sign never gets asked; the ring is
in front of it. The dyadic control (`0.5`) takes the mantissa out and
shows the sign question in the open. For one sample — `i = 0` of the
arc wall's rim, `carrier_on_surface_2`, fully rendered, no freeze,
`EARLY atoms {"abs": 6}`, with `δ` the bulge's box coordinate about
`0.5`, `b = ½ + δ`, `L = 4e-3` the chord:

```
( L⁴·(25/16 + 5/2·δ + 7/2·δ² + 2δ³ + δ⁴)
  − 16·L²·(¼ + δ + δ²)·abs(R(b))² )  /  ( 2·abs(R(b))·16·L²·(¼ + δ + δ²) ),
        R(b) = (5764607523034235·2^-60 + 1152921504606847·2^-58·δ + 1152921504606847·2^-58·δ²)/(2 + 4·δ)
             = L·(1 + b²)/(4b)
```

(read off the committed render: the constant `44171…·2^-236 / L⁴ =
25/16`, the `abs` group's coefficients `1329…·2^-112 = 16·L²` and
`1329…·2^-114 = 4·L²`; `¼ + δ + δ² = b²` and `25/16 + 5/2·δ + 7/2·δ² +
2δ³ + δ⁴ = (1 + b²)²`), i.e. `L⁴(1 + b²)² − 16L²b²·abs(R)²` over
`32·L²b²·abs(R)` — identically zero for EVERY `b ≠ 0` given `abs(R)² =
R²` — no
sign enters at this sample at all; the same `abs(signed_radius)` as
the boss's. Where the sign DOES enter is `carrier_matches_mapped_source`
(16 numeric at `0.5`): under the frozen `Add #f550918f` the carrier
side reads `sqrt(1 + abs(½ + δ)²)·abs(½ + δ)²·…·abs(R(b))` and the
pushforward side `sqrt(5·2^-2 + δ + δ²)` and
`sqrt((1 + sqrt(5·2^-2 + δ + δ²))/(2·sqrt(5·2^-2 + δ + δ²)))` — `sqrt(1
+ b²)` keyed on the polynomial `b²`, against `sqrt(1 + abs(b)²)` keyed
on the abs atom's square. The carrier's span is `sweep::swept::arc_span
= 4·atan|b|`; the pushforward's is `SketchSegment::eval`'s `4·atan b`.
The item's claim holds there exactly: two atoms for one quantity,
related through the sign of `b`, and the turn's sign also sits in
`turn_axis` (a decided `Sign`, not a form) and in the apothem's `1/b`.
At ceiling + δ the bounding decision (`carrier_matches_mapped_source`,
sample 7 at `0.4`, sample 8 at `0.5`) is `sqrt(?# + ?#)` — both
components frozen squares — so it carries the freeze AND the two atoms.

**The two routes, counted on the dyadic control at the nominal**
(`CAD_M10_10_NEEDLES`, per predicate: decisions carrying the needle in
the plain form / in the early form / numeric):

| predicate | `atan(1·abs(` (the carrier's turn) | any `abs(` |
| --- | --- | --- |
| `carrier_matches_mapped_source` | 8 / 0 / 16 | 8 / 0 / 16 |
| `carrier_on_surface_2` | 0 / 0 / 27 | 27 / 27 / 27 |
| `witness_on_surface_2` | 0 / 0 / 3 | 3 / 3 / 3 |
| `carrier_on_surface_1` | 0 / 0 / 9 | 9 / 9 / 9 |
| `witness_on_surface_1` | 0 / 0 / 1 | 1 / 1 / 1 |
| `line_span` | 0 / 0 / 6 | 2 / 2 / 6 |

(the plain count is a floor: a plain form can freeze too, and at `0.4`
it hides all of them — `carrier_matches_mapped_source` reads 0 / 2 / 18
there.)

- **Route A — rule C's clause 3 over `abs(·)`** (`sign_gated`, the
  machinery built and dial-off): folds every abs atom whose argument
  has a certified sign over the box — `abs(b)` (the box `0.4 ± 0.05·s`
  never straddles zero) AND `abs(R(b))` (`R`'s sign is `b`'s). Touches
  8 + 27 + 3 + 9 + 1 + 2 = **50 decisions** on the control at the
  nominal, of which the 40 on-surface ones need no sign at all and
  would fall to the sign-free fold above; the 8 turn-carrying ones are
  the ones only a sign can take. Counted as `sign_gated`, a
  conditional theorem.
- **Route B — the structural fact from the authoring door.** The
  profile program already DECIDES the turn: `path/family.rs`'s
  `bulge_carrier` calls `decide("path_arc_bulge", Margin::of(b), band)`
  and hands the sweep `SegmentKind::Arc { turn }`; the sweep uses it in
  `turn_axis` and then re-inspects the bulge through `abs` in
  `arc_span`. What the door would hand the tier is that decided sign
  `σ ∈ {+1, −1}` beside the bulge form — either by spelling the span
  `4·atan(σ·b)` (a constructor change: `atan(−b)` folds to the same
  `sqrt(1 + b²)` atom since `(−b)² = b²` as forms, and the closed
  forms' odd powers carry the turn), or as a registration
  `Sym::register_equal(abs(b), σ·b)` stated where `path_arc_bulge` was
  decided (the door's provenance, verified at the leaf's witness, no
  constructor spelling moved). Not a new atom kind. Touches the **8**
  turn-carrying `carrier_matches_mapped_source` decisions (plus what
  the freezes hide at ceiling + δ) and none of the 42 `abs(R)` ones.
  Under the shipped ring both routes stop at the same freezes on the
  bounding decision, so neither alone moves `3.52e2·ε`.

No choice is made here. The structural route changes what a
constructor states (or registers); that is Ev's call with the
orchestrator.

### The measurement patches, as applied (each built, measured, reverted)

**Patch B — the ring at 512 bits** (`crates/geom-core/src/sym/rational.rs`):

```diff
-pub(super) const COEFF_BITS: u64 = 256;
+pub(super) const COEFF_BITS: u64 = 512;
```

Its cost, dev build, one whole-box probe of the bisection: boss 0.81 →
3.17 s (3.9×), parameter D-tab 1.45 → 13.2 s (9×), literal D-tab 0.64 →
0.66 s (its forms freeze on constants either way); the `0.5` parameter
control 7.9 → 26 s under a fold-plus-512 pair.

**Patch A — `abs(X)² = X²` in rule A's per-node walk**
(`crates/geom-core/src/sym/algebra.rs`; `find_square` gains the
budget):

```diff
-fn find_square(f: &Form, rules: SymRules, atoms: &IndetMap<AtomInfo>) -> Option<Square> {
+fn find_square(
+    f: &Form,
+    rules: SymRules,
+    atoms: &IndetMap<AtomInfo>,
+    budget: SymBudget,
+) -> Option<Square> {
@@ match info.op {
                     SymOp::Sqrt if rules.sqrt_square => {
                         return Some(Square { id, x: (**arg).clone() });
                     }
+                    SymOp::Abs if rules.sqrt_square => {
+                        return Some(Square { id, x: (**arg).mul(arg, budget)? });
+                    }
@@ reduce_steps
-        let Some(sq) = find_square(&cur, rules, atoms) else {
+        let Some(sq) = find_square(&cur, rules, atoms, budget) else {
```

**Patch C — `abs(X) = X` for an `X` non-negative by syntax** (the
`atan2` fold's own test; `crates/geom-core/src/sym.rs`, the `Sqrt |
Abs` arm of the early walk, after A0's constant fold declines — so it
runs wherever A0 runs, which under the shipped set is the early walk
only: `a0 = const_fold && (early || !rules.early)`):

```diff
             match folded {
                 Some(k) => Some(gate(Form::poly(Poly::constant(k)))),
+                None if a0 && node.op == SymOp::Abs && trig::manifestly_nonneg(a, sess) => {
+                    Some(gate(a.clone()))
+                }
                 None if c => match signed::fold(node.op, a, &sess.params, budget) {
```

That patch is quoted as it was written and both of its symbols have
since moved: the predicate is `manifest::nonneg` (SYM-8 moved it out of
`trig`) and the arm it patches is where SYM-8's rule F now folds an
`abs` — on strict POSITIVITY rather than the non-negativity this patch
read, which is the narrowing that keeps the boss's
`abs((5/8)·sqrt(L²))` opaque and this row's ten decisions intact.

Measured: A, B and C alone; A + B; C + B. The parameter D-tab's
ceiling under each pair is on the ring issue
(`coefficient-ring-width-is-not-monotone-in-reach`), with the pair the
`2.82e2·ε` bracket belongs to named there.

### Instance or class: the corpus sweep

Pattern swept, and its unit: the tour is counted in registered `Stop`
names (`Stop { name: … }` and `stop(name, …)` constructors — `lily` is
one stop whose ten `lily_*` names are its pieces); the test corpus in
FILES; the pattern is `.arc_to(`, `.fillet(`, `tangent_arc_to`,
`CircleSplit {` and `ArcData::Bulge {`, with `circle(`/`LoopProgram::Circle`
read as the unit bulge. Re-taken at the fix pass on `e833a1417`.

- **The tour**: 13 registered stops author an arc at a non-unit bulge —
  `bodies.rs`'s `bracket` (`.fillet(0.5)`: `tan(π/8)`), `vase` and
  `sheave` (`arc_to(Via)`, revolved) and `budrim` (`bud_rim`'s
  `arc_to(Via)`, registered by `probe`), `budfillet` (`arc_to`),
  `fivewall` (two `arc_to`), `klein` (four `.fillet`, four
  `tangent_arc_to`), `lily` (six `arc_to`), `rocker` (`.fillet(R_KNEE)`),
  `s_duct` (`chain`'s `arc_to(Via)`), `teapot` (`CircleSplit(4)`:
  `tan(π/8)`, and centre-spelled arcs), `torusvessel` and
  `torusvesselcup` (`arc_to`). Unit only: `diefillet`'s three stops
  (`b: 1.0`), `plate`, `ring`, `bossplate`, `curvedcut`, `mcplate`
  (`Circle`). Ten of the tour's stop modules match the pattern.
- **The wild corpus**: eight STEP-imported cells, NO authored bulge —
  their arcs arrive as `Curve3::Circle` carriers with no sketch
  pushforward, so the mapped-source identity is never asked of them.
- **The test corpus**: 31 files under `crates/*/tests` match
  `.arc_to(|.fillet(|tangent_arc_to` (every one a non-unit bulge by
  the rule below: 22 in `profile/tests`, 5 in `sweep/tests`, and
  `editor-core`'s `bool12r2_ec_probe`, `mesh`'s `m5_s11_concave_sense`,
  `pncad`'s `all`, `step-export`'s `common`); 24 files (50 occurrences) across `crates/` with `ArcData::Bulge {`
  added, whose literal `b:` sites are non-unit in nine files (the boss
  `2`; the D-tab `0.4`/a parameter; `m4_pr6_eps_diff` `2e-6`;
  `m4_pr6_golden` `0.25`; `switch_program_vocabulary` `0.3`;
  `switch_slots` `0.2`, `0.45`; `m9_d1_r1_probes` a variable;
  profile's `path_program` `0.5`, `cert4r1_e2e`/`generic_replay` a
  variable) and `1.0` in eight.

**The pattern**: every fillet is `tan(θ/4)` of its corner, every `Via`
or `tangent_arc_to` arc is whatever its geometry makes it, and
`CircleSplit(n)` is `tan(π/(2n))` — `1` only at `n = 2`. The unit
bulge is the CIRCLE's, and only the circle's: a document that authors
an arc any other way is in this class. So the boss buys a family, not
a document: a third of the tour.

**The blind spot**: the pattern reads what a document AUTHORS. It
cannot see (i) bulges the kernel mints itself — `SketchSegment::restrict`
stores `tan(atan(b)·(s1 − s0))` for every sub-arc, so every boolean or
meridian split of a unit-bulge circle arc leaves NON-unit sub-arcs
behind, on the plate as much as anywhere (`mapped.rs`'s own docs name
the route); (ii) a `CircleSplit(n)`'s `n` (the word matched, the value
not read); (iii) the variable bulges the tests pass through `scl(b)`;
(iv) STEP-imported arcs, which carry no bulge at all. The sweep is as
of the merge base, not the merge.

### What this does NOT say

No rule, dial or budget moved. Rule C stays dial-off; `COEFF_BITS`
stays 256; the trig bounds stay. The measurement patches were local
and reverted, and their numbers are above so the next unit's claim has
a before AND an after to be read against.

## What SYM-5's rule E took (2026-09-14)

Both residues above are largely gone, and the ceiling with them.
SYM-5's **rule E** (the quotient's common factor,
`SymRules::common_factor`) cancels the shared monomial in the early
walk's quotients, which keeps the cross-multiplied numerators inside
[`rational::COEFF_BITS`] — so it does at 256 bits what this row
measured a 512-bit ring doing, and at 1.6× the probe cost rather than
at a wider ring.

Re-measured on R1's segment boss at `bulge = 2`, shipped set, rule E
off → on:

| predicate | off | on |
| --- | --- | --- |
| `carrier_matches_mapped_source` | 72/0/48/6 | **72/0/54/0** |
| `carrier_on_surface_2` | 63/0/0/27 | **81/0/0/9** |
| `witness_on_surface_2` | 7/0/0/3 | **9/0/0/1** |
| `carrier_on_surface_1` | 81/0/0/9 | 81/0/0/9 |
| `witness_on_surface_1` | 9/0/0/1 | 9/0/0/1 |

and the whole-certifying CEILING `8.2611e2 · ε → 9.3559e2 · ε` (1.13×),
which is exactly the move this row's local 512-bit experiment
predicted, over-band set and all. Residue (a) is closed. Residue (b) —
the `abs` A0 does not reach — is two thirds closed: 18 of its 27 are
theorems now, 9 stand, and `carrier_on_surface_1`'s 9 and
`witness_on_surface_1`'s 1 are untouched, so the `abs(signed_radius)`
against its own square spelled without the `abs` is still what stands
there.

The D-tab is unchanged on every row but two: `carrier_endpoint_start`
24/0/8/4 → 24/0/12/0 on both spellings, and
`carrier_matches_mapped_source` 126/0/36/18 → 126/0/42/12 on the
LITERAL against 126/0/38/16 on the PARAMETER — the first row where the
two spellings part at the nominal, so this row's "the same table as
the literal's" is now true of every row but that one
(`m10_bulge_interval` pins both).

## What stands on DECIDE-3's and SYM-9's tree (DECIDE-4)

Re-taken on `props/sign-hull` at `7f3c0cc3f` (SYM-9's merge), the shipped
set with the drive's default ladder (`DEFAULT_SYM_RETRY`), ε = 1e-9, dev
build for the splits and renders, release for the ceilings and the leaf
costs. The instrument is `m10_10_evidence_interval` with two knobs this
unit added: `CAD_M10_10_RETRY=default` (the rows ran `SymRetry::none()`
before) and, on the render row, `CAD_M10_10_PROFILE` (the freezes by
cause, and per asked decision its `DecisionRecord::causes`) and
`CAD_M10_10_DUMP` (every asked-numeric decision as a JSON line). The
ladder retried nothing on any of the six documents (`retried 0`), so
every split below is also the no-ladder split. The trimmed renders are
`crates/editor-core/tests/m10_bulge_renders.txt`, re-taken whole.

### How a decision was attributed

The columns split `numeric` into DEFINITE (the numeric channel
certified a non-zero sign; the tier was never asked, so nothing
stands) and the decisions the tier was asked and declined. Each asked
one is:

- **(i)** when its early form carries a frozen node (`?#id`). The
  causes are its own walks' (`DecisionRecord::causes`, summed per
  predicate); a node is charged to the decision that first built it,
  so a later decision on an already-frozen node reads none.
- otherwise its early form was EVALUATED (a Python evaluator over the
  rendered form, 120-digit decimals; `sqrt`/`abs` as functions) at
  points spanning every sign region of the parameters:
  - zero at every real point: **(ii)** if `abs(X)² = X²` or a
    manifest-sign fact is the whole of it, **(iv) value-free**
    otherwise;
  - zero for `b > 0` only: **(iii)**;
  - zero on some other sign region: **(iv)**, with the quantity named.

### The tables


**R1's segment boss, `bulge = 2`** (`r1_segment_boss`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 72/0/54/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_1` | 90/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_2` | 90/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `witness_on_surface_1` | 10/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `witness_on_surface_2` | 10/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_start` | 12/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 12/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 5/0/0/1 | 0 | 0 | 0 | 0 | 1 value-free |
| `line_span` | 0/2/0/0 | 0 | 0 | 0 | 0 | 0 |
| `contact_at_shared_vertex` | 6/0/0/3 | 3 | 0 | 0 | 0 | 0 |

**The D-tab, a literal `0.4`** (`r2_d_tab_literal`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 126/0/42/12 | 0 | 12 (Coefficient 391) | 0 | 0 | 0 |
| `carrier_on_surface_1` | 135/0/0/9 | 0 | 9 (Coefficient 81) | 0 | 0 | 0 |
| `carrier_on_surface_2` | 117/0/0/27 | 0 | 27 (Coefficient 289) | 0 | 0 | 0 |
| `witness_on_surface_1` | 15/0/0/1 | 0 | 1 (Coefficient 6) | 0 | 0 | 0 |
| `witness_on_surface_2` | 13/0/0/3 | 0 | 3 (Coefficient 6) | 0 | 0 | 0 |
| `carrier_endpoint_start` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 4/0/0/4 | 2 | 2 (Coefficient 8) | 0 | 0 | 0 |
| `line_span` | 4/0/0/4 | 2 | 2 (Coefficient 5) | 0 | 0 | 0 |
| `contact_at_shared_vertex` | 8/0/0/4 | 2 | 2 (Coefficient 7) | 0 | 0 | 0 |

**The D-tab, a parameter at `0.4`** (`r2_d_tab_parameter`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 126/0/38/16 | 0 | 16 (Coefficient 351, Terms 62) | 0 | 0 | 0 |
| `carrier_on_surface_1` | 135/0/0/9 | 0 | 9 (Coefficient 81) | 0 | 0 | 0 |
| `carrier_on_surface_2` | 117/0/0/27 | 0 | 27 (Coefficient 280, Terms 44) | 0 | 0 | 0 |
| `witness_on_surface_1` | 15/0/0/1 | 0 | 1 (Coefficient 6) | 0 | 0 | 0 |
| `witness_on_surface_2` | 13/0/0/3 | 0 | 3 (Coefficient 6) | 0 | 0 | 0 |
| `carrier_endpoint_start` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 4/0/0/4 | 2 | 2 (Coefficient 8) | 0 | 0 | 0 |
| `line_span` | 4/0/0/4 | 2 | 0 | 0 | 0 | 2 apothem sign |
| `contact_at_shared_vertex` | 8/0/0/4 | 2 | 2 (Coefficient 7) | 0 | 0 | 0 |

**Control: the D-tab, a literal `0.5`** (`r2_d_tab_literal_dyadic`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 144/0/36/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_1` | 144/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_2` | 144/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `witness_on_surface_1` | 16/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `witness_on_surface_2` | 16/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_start` | 28/0/8/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 28/0/8/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 6/0/0/2 | 2 | 0 | 0 | 0 | 0 |
| `line_span` | 6/0/0/2 | 2 | 0 | 0 | 0 | 0 |
| `contact_at_shared_vertex` | 10/0/0/2 | 2 | 0 | 0 | 0 | 0 |

**Control: the D-tab, a parameter at `0.5`** (`r2_d_tab_parameter_dyadic`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 126/0/38/16 | 0 | 12 (Degree 18, Terms 127) | 0 | 4 | 0 |
| `carrier_on_surface_1` | 144/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_2` | 138/0/0/6 | 0 | 6 (Degree 18, Terms 58) | 0 | 0 | 0 |
| `witness_on_surface_1` | 16/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `witness_on_surface_2` | 16/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_start` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 24/0/12/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 4/0/0/4 | 2 | 0 | 0 | 0 | 2 apothem sign |
| `line_span` | 4/0/0/4 | 2 | 0 | 0 | 0 | 2 apothem sign |
| `contact_at_shared_vertex` | 8/0/0/4 | 2 | 0 | 0 | 0 | 2 apothem sign |

**The class sample: R2's filleted bracket, its fillet arcs** (`r2_filleted_bracket`; every asked-numeric arc-family decision on it is on the two fillet rims — evaluation blocks 27–35 and 153–161 of `carrier_matches_mapped_source`, and every early form that is not wholly frozen carries `fillet_r`)

| predicate | split (T/G/R/n) | definite | (i) freeze | (ii) | (iii) | (iv) |
| --- | --- | --- | --- | --- | --- | --- |
| `carrier_matches_mapped_source` | 243/0/74/16 | 0 | 16 (Coefficient 294, Degree 72, Terms 111) | 0 | 0 | 0 |
| `carrier_on_surface_1` | 243/0/0/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_on_surface_2` | 225/0/0/18 | 0 | 18 (Coefficient 157, Degree 51, Terms 78) | 0 | 0 | 0 |
| `witness_on_surface_1` | 28/0/0/1 | 0 | 1 (Coefficient 10, Degree 2) | 0 | 0 | 0 |
| `witness_on_surface_2` | 26/0/0/3 | 0 | 3 (Coefficient 10, Degree 2) | 0 | 0 | 0 |
| `carrier_endpoint_start` | 46/0/20/0 | 0 | 0 | 0 | 0 | 0 |
| `carrier_endpoint_end` | 46/0/20/0 | 0 | 0 | 0 | 0 | 0 |
| `arc_span` | 8/0/0/2 | 0 | 2 (Coefficient 13, Degree 1, Terms 1) | 0 | 0 | 0 |
| `line_span` | 6/5/0/1 | 0 | 1 (Coefficient 5) | 0 | 0 | 0 |
| `contact_at_shared_vertex` | 19/0/0/5 | 4 | 1 | 0 | 0 | 0 |

### What each cause is, rendered

- **(i) on the literal and parameter D-tab at `0.4`**: every asked
  decision but two, and every one of them `Coefficient` (the ring):
  `fl(0.4)`'s mantissa `3602879701896397·2^-53` in every denominator,
  as SYM-3 found. The two exceptions are the parameter's `line_span`
  pair (below). The ring also hides an `abs` square here without a
  `?#`: the parameter's `line_span` carries
  `sqrt(-1·2^108 + abs(R)^2)`, whose `abs(R)² → R²` the per-node walk
  refused on the ring and fell back from.
- **(i) on the `0.5` parameter control**: `carrier_matches_mapped_source`
  12 (`sqrt(?# + ?#)`, both components frozen, `Terms` and `Degree`)
  and `carrier_on_surface_2` 6. Eight of them freeze on the bulk of the
  sign's second atom: route B (below) takes six of the twelve
  (samples 10, 11, 15 of both rims) and two of the six (57, 66).
- **(iii) on the `0.5` parameter control, 4 decisions**:
  `carrier_matches_mapped_source` samples 13 and 17 of both rims.
  Their early forms carry `abs(1 + 2·bulge)` (`|2b|`, `bulge` the box
  coordinate `δ`, `b = ½ + δ`), `abs((5 + 4δ + 4δ²)/(1 + 2δ))` (the
  radius `|R(b)|`) and the atom pair `sqrt(4 + abs(1 + 2δ)²)` /
  `sqrt(5 + 4δ + 4δ²)` — the carrier's `sqrt(1 + |b|²)` against the
  pushforward's `sqrt(1 + b²)`. Evaluated: zero at `b` = 0.5, 0.7, 0.2,
  1.4, 2.5 (every `b > 0`); `0.004` at `b = −0.3` and `b = −2`.
- **(iv) the sign of the apothem, 6 decisions on the `0.5` parameter
  control and 2 on the `0.4` one**: `line_span`, `arc_span` and
  `contact_at_shared_vertex`, two each, never frozen. `line_span`'s
  early form, read off the render with `u = 1152921504606847·2^-60`:
  `u·(|2b|·2(1 − b²) − |4(b² − 1)|·b) / (|2b|·2b)` — two magnitudes
  (`abs(1 + 2δ)`, `abs(−3 + 4δ + 4δ²)`) against their signed
  spellings. Evaluated: zero at `0 < b < 1` and at `b < −1`, non-zero
  at `b > 1` and at `−1 < b < 0`, i.e. zero exactly where the apothem
  `L(1 − b²)/(4b)` is positive. It is the magnitude `sqrt(r² − (L/2)²)`
  of the centre's offset against the signed offset. Not `b`'s sign
  alone: route B does not reach it and route A does.
- **(iv) value-free, 1 decision: the boss's `arc_span` sample 1**, and
  it is what bounds the boss's ceiling (below). Its early form is
  `2^-59·sqrt(5)·abs(c + 2^59·ch) − sqrt(P/Q)` (`c = 1152921504606847`,
  `ch` = `chord_half`), with `P` of degree 6 and `Q` of degree 4. Exact
  polynomial division over the rendered rationals: `Q | P`, quotient
  `5(c·2^-59 + ch)²` — so `sqrt(P/Q)` is `sqrt(5)·|c·2^-59 + ch|`, the
  other term. `Q = (1 + ch/a)⁴`, `a = c·2^-59`: the halves share the
  chord's polynomial factor, which rule E (a MONOMIAL common factor)
  does not divide out, and rule G's content split then declines on the
  ring (`Coefficient 2` in its walks). Evaluated: zero at `chord_half`
  = 0, ±1e-3, −5e-3, 0.3, −0.7. A 512- and a 1024-bit retry move
  nothing (`sym_9_what_each_retry_recovers`, `CAD_SYM_9_SHAPES=ring_512,ring_1024`
  on `r1_segment_boss`: totals `[374, 2, 96, 234]` on all three), so
  the ring is not what blocks: the common factor is.
- **(ii): none.** On today's tree no asked decision on any of the six
  documents is zero for every real value given `abs(X)² = X²` or a
  manifest-sign fact alone. SYM-3's 40 sign-free on-surface decisions
  on the `0.5` control went with DECIDE-3's `abs_square` (`carrier_on_surface_1`
  0 of 144, `witness_on_surface_{1,2}` 0 of 16), and the boss's 20
  with its canonical root. The pair `sqrt(4 + abs(1+2δ)²)` /
  `sqrt(5 + 4δ + 4δ²)` IS a sign-free pairing — rule D's hand-built
  root (`trig::sqrt_atom`) is minted over `1 + X·X` without the per-node
  walk's `abs(X)² = X²` — but it stands only inside (iii) and (i)
  decisions. Measured (a local patch, reverted: `reduce_steps` over
  the argument in `sqrt_atom` under `early_ab`): the atom goes
  (`grep -c` of it in the dump 2 → 0) and no split on the five bulge
  documents moves.

### The whole-certifying ceilings (release, `m10_10_ceilings_and_the_over_band_set`)

| document | shipped | over the band at ceiling + δ | `without_the_algebra` |
| --- | --- | --- | --- |
| boss, `bulge = 2` | `1.0309e3 .. 1.0313e3 · ε` (it was `9.3559e2`) | `arc_span` `[−1.0002e-9, 1.0002e-9]` 1/6 | `8.2611e2 .. 8.2643e2 · ε`, `carrier_matches_mapped_source` 1/125 |
| D-tab, literal `0.4` | 0.5611 of its real study | `arc_diameter_clearance` `[−5.16e-8, 5.63e-4]` 1/3 | `7.8109e2 .. 7.8139e2 · ε`, `carrier_matches_mapped_source` 1/117 |
| D-tab, parameter `0.4` | `3.5218e2 .. 3.5232e2 · ε` | `carrier_matches_mapped_source` `[0, 1.0004e-9]` 1/89 | the same |
| control, literal `0.5` | 0.5611 of its real study | `arc_diameter_clearance` 1/3 | `7.8109e2 .. 7.8139e2 · ε` |
| control, parameter `0.5` | `4.3375e2 .. 4.3392e2 · ε` | `carrier_matches_mapped_source` `[0, 1.0e-9]` 1/90 | the same |

The boss's ceiling is bounded by the one value-free (iv) decision.

### The needle table and the two routes

`CAD_M10_10_NEEDLES` on the `0.5` parameter control (plain / early /
asked-numeric):

| predicate | `atan(1·abs(` | any `abs(` |
| --- | --- | --- |
| `carrier_matches_mapped_source` | 8 / 0 / 16 | 8 / 6 / 16 |
| `carrier_on_surface_2` | 0 / 0 / 6 | 6 / 6 / 6 |
| `line_span` | 0 / 0 / 2 | 2 / 2 / 2 |
| `arc_span` | 0 / 0 / 2 | 0 / 2 / 2 |
| `contact_at_shared_vertex` | 0 / 0 / 2 | 0 / 2 / 2 |

(`carrier_on_surface_1`, `witness_on_surface_{1,2}` have no asked
decision left; SYM-3's table had 9, 3 and 1.)

**Route A — `signed_root` on** (the shipped set with that one dial,
which is `SymRules::all()`; `CAD_M10_10_RULES=all`, no patch). Per
predicate on the `0.5` parameter control, numeric moved out:
`carrier_matches_mapped_source` 16 → 8, `arc_span` 4 → 2,
`contact_at_shared_vertex` 4 → 2, `line_span` 4 → 2 — 14 decisions,
the apothem-sign six among them. But the dial does more than fold the
sign, and every document pays: THEOREMS are re-labelled `sign_gated`
(`carrier_on_surface_1` 144/0/0/0 → 27/117/0/0,
`carrier_on_surface_2` 138/0/0/6 → 81/57/0/6, and the like) and the
door is LOST on `pcurve_map_residual` (0/0/18/0 → 0/0/0/18) — so
`numeric` RISES on every measured document: the control 392 → 396,
the plate 462 → 498, the annulus 209 → 245, the link 453 → 477, the
bracket 760 → 782, the boss 234 → 251, the D-tabs 422 → 440 and
426 → 450. The plate's whole-box leaf reads `[643, 272, 0, 498]`
against `[811, 0, 140, 462]`.

**Route B — the span from the decided turn** (a local patch, reverted):

```diff
@@ -551,7 +551,13 @@ pub(crate) fn placed_segment_spec<T: Real, S: SweptChord<T>>(
                 radius,
                 u_ref: rim.normalize(),
             };
-            let param_end = arc_span(seg.bulge());
+            // Route B (measurement patch): the span spelled from the
+            // decided turn σ, `4·atan(σ·b)`, not `4·atan|b|`.
+            let param_end = T::from_f64(4.0)
+                * match turn {
+                    Sign::Negative => (T::zero() - seg.bulge()).atan(),
+                    Sign::Positive | Sign::Zero => seg.bulge().atan(),
+                };
             // The SPAN identity, at the same guarantee
             // (`register_span_identity` carries the proof). The
             // carrier and the span are bound out first so the
```

Numeric moved out, per predicate, on every document measured: the
`0.5` parameter control `carrier_matches_mapped_source` 126/0/38/16 →
126/0/48/6 and `carrier_on_surface_2` 138/0/0/6 → 140/0/0/4 (the four
(iii) decisions and eight (i) ones whose freeze stood on the second
atom); R2's link `carrier_matches_mapped_source` 108/0/40/32 →
108/0/62/10, and `carrier_on_surface_2` 92/0/6/10 → 88/0/8/12 — FOUR
THEOREMS LOST, two to the door and two to `numeric`. Nothing else
moves on the boss, both `0.4` D-tabs, the literal control, the bracket,
the plate or the annulus. It reaches none of the apothem-sign (iv).
The parameter control's ceiling moves `4.3375e2 → 5.1078e2 · ε`
(1.18×, still `carrier_matches_mapped_source` 1/89); the `0.4`
parameter D-tab's does not (the ring is in front).

**Costs**, the leaf instrument (release, one whole-box leaf, the
`ON + the ladder` column, which is the drive's default, best of three
takes, `CAD_M10_10_TAKES=3`, taken at the fix pass's head: rule G's
exact quotient shipped under all three columns). The pad's route
columns were not taken (a pad take is about 145 s); its single takes
at the Phase 1 head read 150.9 / 147.8 / 91.1 s.

| document | shipped | route A | route B |
| --- | --- | --- | --- |
| plate `1e2·ε` | 0.349 s | 0.121 s | 0.357 s |
| plate, real study | 0.346 s | 0.043 s | 0.361 s |
| bracket | 3.823 s | 5.651 s | 2.857 s |
| annulus | 0.364 s | 0.100 s | 0.371 s |
| link | 19.332 s | 19.016 s | 10.660 s |
| boss `1e2·ε` | 0.250 s | 0.092 s | 0.250 s |
| pad | 145.677 s | — | — |

Route A is cheaper where it abandons the door. Route B's link leaf
reads `[549, 0, 120, 433]` retried 16 against `[553, 0, 96, 453]`
retried 12, and the pad's (Phase 1 head) is unmoved at
`[890, 6, 150, 907]`.

The fork is on `[ev]` #3186, not on this branch: it recommends route B,
and the apothem's sign is its own item there.

### The class: `restrict`'s sub-arcs on the plate

**Zero.** A local probe (an `eprintln!` in `SketchSegment::restrict`'s
`Arc` arm, compiled into the test binary and reverted) fired zero
times over the plate's nominal replay and zero over the bracket's: no
arc-family decision on either is asked of a sub-arc `restrict`
minted. That closes the sweep's blind spot (i) for those two
documents and only for them: a boolean or a meridian split elsewhere
still mints `tan(atan(b)·(s1 − s0))`.

### The verdict, per stop rule

- **(ii) is empty** on all six documents, and **(iv) value-free is
  one decision**, the boss's `arc_span` — the decision that bounds the
  boss's ceiling. So Phase 2a is NOT empty: it is that one decision,
  and the narrowest rewrite that takes it is rule E's quotient at the
  root's door (a root argument whose denominator divides its
  numerator EXACTLY is the polynomial quotient).
- **(iii) is non-empty**: 4 decisions on the `0.5` parameter control,
  with 8 more frozen behind the second atom. Phase 2b stops for Ev's
  fork.

### Phase 2a: rule G's exact quotient

The one value-free (iv) decision is taken by
`SymRules::root_quotient` (`crates/geom-core/src/sym/root.rs`, its
header section "The exact quotient"). A root whose argument `N/D` has a
denominator that divides its numerator exactly is minted over the
polynomial quotient `Q` before rule G's split is asked. The quotient
comes from `Poly::div_exact` (`crates/geom-core/src/sym/form.rs`), a
leading-term division under grlex whose exact remainder
`rest = N − q·D` is its own proof, capped at the budget's term cap. The
dial is read as `canonical_root && root_quotient`, and
`SymRules::without_root_quotient` is the tier SYM-9 shipped.

**What it moves.** Measured on the nominal splits of nine documents
(the plate, the bracket, the annulus, the link, the boss, both `0.4`
D-tabs and both controls; `CAD_M10_10_RULES=no_q` against the shipped
set), the only move is the boss's `arc_span` 5/0/0/1 → 6/0/0/0, a
THEOREM, and its receipt `[374, 2, 96, 234]` → `[375, 2, 96, 233]`.

**The boss's ceiling.** It goes from `1.0309e3 · ε` to **0.5024,
0.7267 and 0.7271 of its real study** at ε = 1e-6, 1e-9 and 1e-12.
What bounds it at each row is now `dihedral_wedge`, 1/58, a real
margin, which puts it in `real-margin-dependency-widening`'s class
beside the annulus. The two parameter D-tabs' ceilings are unmoved at
all three rows: `3.52e2 · ε` and `4.34e2 · ε`, both bounded by
`carrier_matches_mapped_source`.

**The walk ledger.** On the plate's nominal replay the rule returns a
quotient on 108 of 208 root mints (ten distinct arguments), and it
changes that root's form on each. The ledger moves one digest,
`Early/Decision`, and every count is identical.

**Leaf cost** (release, best of three, off → on):

| document | off | on |
| --- | --- | --- |
| plate `1e2·ε` | 0.339 s | 0.349 s |
| plate, real study | 0.342 s | 0.346 s |
| annulus | 0.364 s | 0.364 s |
| bracket | 3.807 s | 3.823 s |
| link | 19.427 s | 19.332 s |
| pad | 144.495 s | 145.677 s |
| boss `1e2·ε` | 0.245 s | 0.250 s |

That is +0–3%, with every receipt but the boss's unmoved.

**It is a trade.** The re-keyed root no longer meets the split
spelling `sqrt(N)/sqrt(D)` of the same value. Both reviews found this,
and no measured document moves on it. Asking the split first would keep
that meeting, and it still takes the measured boss, because the split
declines on the ring there. But it loses the boss's shape wherever the
ring lets the split through (its dyadic twin keys `|p³|/p²`) and reds
four rows, so neither order keeps both. The quotient stays first
because its key does not depend on the ring width.
`the-exact-quotient-re-keys-a-root-the-split-met` (P2, filed here)
carries both shapes and the remedy.

**What it does not reach.** It does not take a shared factor where
neither half divides the other (a GCD), and it does not reach an `abs`
over such a quotient. No measured decision stands on either.

**The renders.** `m10_bulge_renders.txt` is re-taken at the fix head.
Every form attributed to (iii) or (iv) is written uncut, and evaluating
them at the points above reproduces each class. The boss's former
residual is kept as its base-tree render.

## The bulge's sign: how the tier learns it (DECIDE-4's fork)

Ruled by Ev on #3186 (2026-09-25), both decisions as written below.
Route B is DECIDE-5, its own unit, since DECIDE-4 had closed its review
before the ruling. The apothem's sign is
`the-apothems-sign-is-a-value-read`. This item closes when DECIDE-5
lands. What is left then is the ring at `fl(0.4)`, which is the ring
class and not the bulge's.

DECIDE-4's Phase 1 (on `decide/4-bulge-reach` at `200123f29`, this
item's section "What stands on DECIDE-3's and SYM-9's tree") finds the
sign of a parameter bulge still blocking. On the `0.5` parameter
control, four `carrier_matches_mapped_source` decisions are zero for
every `b > 0` and non-zero for `b < 0`: the carrier's `sqrt(1 + |b|²)`
against the pushforward's `sqrt(1 + b²)`. Eight more freeze behind the
same second atom. Nothing else on today's tree needs the sign: every
other residue is the ring, or the one value-free decision on the boss
that DECIDE-4 takes.

**The decision: the sweep spells the carrier's span from the turn it
already decided.** `placed_segment_spec` writes `4·atan(σ·b)`, where
`σ` is the turn sign `path_arc_bulge` decided, in place of
`arc_span`'s `4·atan|b|`. The carrier and the pushforward then mint
one atom for one quantity.

- **What it takes:**
  - On the control, `carrier_matches_mapped_source` goes from 16 to 6
    numeric, and `carrier_on_surface_2` from 6 to 4.
  - On R2's link, `carrier_matches_mapped_source` goes from 32 to 10.
  - The control's ceiling rises from `4.3375e2` to `5.1078e2 · ε`.
- **What it costs:** R2's link loses four `carrier_on_surface_2`
  theorems, two to the door and two to `numeric`. That is the
  non-monotone class SYM-9's ladder was built for, and it is
  re-baselined and said.
- **Leaf time:**
  - link: 19.8 → 10.7 s;
  - pad: 151 → 91 s;
  - bracket: 3.9 → 3.1 s;
  - the rest flat.
- **How the new zeros count:** they are theorems, or `registered`
  through the span identity's door. None is `sign_gated`.

**Rejected: rule C on (`signed_root`, the one shipped-off dial).** It
does fold the sign, and it takes 14 decisions on the control. But as
the dial stands, it runs ahead of the value-free folds and the door:

- it re-labels theorems `sign_gated` on every document (the control's
  `carrier_on_surface_1` goes 144/0/0/0 → 27/117/0/0);
- it loses the door's `pcurve_map_residual`;
- `numeric` RISES everywhere: plate 462 → 498, annulus 209 → 245,
  link 453 → 477, bracket 760 → 782, boss 234 → 251.
- it takes back what DECIDE-4's value-free rule gains. With that rule in,
  the boss's `arc_span` under rule C on is `[4, 2, 0, 0]`: two of its
  theorems are re-labelled `sign_gated`. This was measured by DECIDE-4's
  review on `334bb2aa2`. The cause is older: `sqrt(5p²) − √5·|p|` is a
  theorem under the shipped set and refused under `SymRules::all()`.

**Not measured, named so the choice is whole:**

- the registration variant of the recommendation,
  `register_equal(abs(b), σ·b)` stated where `path_arc_bulge` is
  decided. It leaves the constructor's spelling as it is, at the cost
  of an axiom where the spelling gives the same atom.
- a NARROWED rule C: only `abs`/`sqrt` atoms, ordered behind the door
  and every value-free fold, as the decision read is.

**A second, smaller question rides with it: the sign of the APOTHEM.**
Six decisions on the control (`line_span`, `arc_span` and
`contact_at_shared_vertex`, two each) are zero exactly where the
apothem `L(1 − b²)/(4b)` is positive. The magnitude
`sqrt(r² − (L/2)²)` stands against the signed offset. The turn `σ`
does not reach them; only a value read does. **The decision: filed as
its own DECIDE item**, with the narrowed rule C as its candidate, not
folded into DECIDE-4.

