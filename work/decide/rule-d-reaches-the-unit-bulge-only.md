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
needs_ev: true
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

## The bulge's sign: how the tier learns it (DECIDE-4's fork)

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

