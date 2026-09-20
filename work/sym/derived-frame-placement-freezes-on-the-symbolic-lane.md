---
id: derived-frame-placement-freezes-on-the-symbolic-lane
kind: issue
title: A profile placed on a derived frame whose AXES carry a widened parameter does not certify on the symbolic lane: the re-normalised stored unit vectors freeze on degree and the identity is not reached (on a purely TRANSLATED derived frame the chain is constant and rule A0 folds it)
status: open
opened: 2026-09-04
parent: SYM-5
---


## What

Found by DOCM-1's dual review (PR 1829, R1's two red probes on
`docm/1-review-r1` @3435cf61:
`r1_c7_an_extrude_on_a_widened_derived_frame_versus_the_authored_guided_twin`,
`r1_c7_the_prs_transform_lifted_shape_with_an_extrude_above_it`; both
reproduced by the orchestrator and by the fix pass at ε default, 1e-6
and 1e-12). On `Sym<Interval>` an extrude of a profile on a DERIVED
frame (`Datum::FaceFrame`, DM1c) whose body carries a widened
parameter refuses certification (`carrier_endpoint_*` /
`carrier_on_surface_1` indeterminate, enclosure 6–12× the width) at
every width from ε/8 to 0.05, under both `ProfileLift`s; the
AUTHORED-frame twin with the same widened placement certifies under
both lifts through the same `frame_plane_lane` door. On plain
`Interval` the two frame kinds agree at every width.

## Diagnosis (the fix pass, instrumented)

`SymCounts` per evaluation: derived/Pinned `frozen = 16`,
derived/Guided `37`, authored `0`. Every freeze is a budget refusal in
`geom_core::sym::form_in` (`crates/geom-core/src/sym.rs:1478–1484`,
`combine(...).filter(within(budget))` → a frozen indeterminate), never
a missing node; the frozen ops are `Powi 2` / `Mul` / `Add` on kid
forms already at rational degree 65–128. Raising `max_degree` to 4096
(`max_terms` 65536) still leaves seven freezes, the survivors at
degree 433 and 670. Mechanism: a `Sqrt` is an indeterminate keyed by
its argument's form (`sym.rs:1397`), so each normalisation
`v / sqrt(v·v)` adds a denominator and each square doubles the degree;
the `Decide for Sym` rescue (`sym.rs:1871`) fires only on an
identically-zero form, and a frozen subtree cancels nothing. An
authored frame's axes are literals normalised once; a derived frame's
axes are the kernel's ALREADY-normalised stored vectors (the cap
normal and `u_ref`, themselves rational forms from the extrude's
`w.normalize()` and the profile plane's), which the boss extrude
normalises again and squares in certification. Emitting the
already-unit `u`, `n × u` without editor-core's re-normalisation
(a local experiment) still refuses (`frozen` 9/31): editor-core's
levels are not decisive, the degree comes from the stored vectors.

## Where the fix lives

The kernel's symbolic lane — `geom_core::sym` (a `Sqrt` of a
value-exact norm minted as a degree-resetting atom, or normalisation
simplified before squaring) or `geom_core::UnitVec3` / the extrude's
certification — outside DOCM's fence and inside M10's (E12, the
symbolic identity lane; the program stays open "until certification
is parameter-aware", and this is a case where it is not). DOCM-1
merged with the f64 and plain-`Interval` behaviour pinned and this
limitation disclosed in its PR body; R1's two rows are the pin, red
until this is answered, and become unit rows when it is.

## Home

M10. Filed by DOCM at DOCM-1's merge (2026-09-04).

## Re-homed at M10's exit sweep (2026-09-13)

Here because the mechanism is the tier's: every freeze is a budget refusal in
`geom_core::sym::form_in`, where a derived frame's re-normalised stored unit vectors
double the degree per square. DOCM-1's dual review found it and DOCM owns the
derived-frame door (`Datum::FaceFrame`) the other half would change; the row follows
the mechanism and names DOCM.

From `work/m10/` at M10's close (`docs/DOC-LEDGER.md` sweep 13; the walk and the directory are recoverable at the SHA it names). The id is unchanged.

## What stands, and what moved (SYM-5, phase 1)

SYM-5 measured this row on `origin/main` at `d0d430fc7` before touching
the tier. **The measurement splits the row in two**: DOCM's own
document is the NARROW case and is answered; a derived frame whose
AXES carry the parameter is the case this row was filed about, and on
it the mechanism stands on every rung. Phase 2 (PR-2 of SYM-5) runs on
the second document.

Every count below is a DATED READING taken at `d0d430fc7` on one box,
not a pinned number: the only assertions over them are
`frozen > 0` / `frozen == 0` and the refusal's predicate and
diagnostic. Re-take them by re-running the rows named.

### DOCM's document: a pure TRANSLATION, and A0 answers it

The two DOCM R1 probes are ported unchanged as
`crates/editor-core/tests/m10_derived_frame_interval.rs`. The
transform-lifted row is GREEN at all three ε rows and is now a kernel
pin. The parity row is green on the symbolic lane at half-widths ε/8,
`1e-6` and `1e-3` under both `ProfileLift`s — the widths this row said
refused — and red at `5e-2` alone.

The rule ladder there, at the whole declared box, at the two widths
measured (`1e-3` and `5e-2`; no other width was run under `none`):

| rules | frozen | refusals |
| --- | --- | --- |
| `none` | 12 | `carrier_endpoint_start`, at both measured widths |
| `const_fold` alone (A0, replacing) | **0** | none at `1e-3`; `newell_plane_residual` **Invalid** at `5e-2` |
| `+ early`, `+ door`, `without_the_algebra`, `shipped` | 1,253 at the nominal / 369 at the box | the same one |

The document's widened parameter is the cube's HEIGHT, so every
normalised quantity in the chain is CONSTANT in it: the chain is
`sqrt(16384/256)` and `abs(128/(16·sqrt(16384/256)))` nested under a
further `sqrt` of a degree-88 polynomial in them, and M10-8's exact
constant fold collapses the lot to rationals. The freezes the shipped
set still makes in its early walk (`Powi`/`Add`/`Sub`, kids at total
degree 65–128 in 13–192 terms — the same SHAPE as SYM-1's plate table)
cost this document no decision: every predicate at the nominal is a
`Theorem`.

**What moved the parity row, and what never explained it.** DOCM's
probes were taken on `20f04189` (2026-09-04); M10-8's constant fold
landed the next day (#1828, 2026-09-05). The `none` rung reproduces
DOCM's refusal and A0 alone clears it, so A0 is what moved the row —
not M10-9's door and not M10-10's algebra, which are bit-identical to
`without_the_algebra` here. And on DOCM's own tree the BUDGET never
explained the refusal: under `none` at 4,096 / 65,536 the document
freezes 1 and still refuses with the identical enclosure, and rule A
alone (`sqrt(X)² = X`, no constant fold) does not clear it at either
budget. What stood was OPAQUE CONSTANT ATOMS, which is exactly what A0
folds
(`sym5_phase1_the_height_document_under_a_raised_budget_and_rule_a_alone`).

**What is left there is clause 1's, not the tier's.** One
`newell_plane_residual` of 29 at `5e-2` is `Invalid` — `newell_plane`
divides a cross-sum whose `y` component encloses `[-2.0507, 0.8977]`
by a length enclosing `[0, 2.0924]` — and the tier's early form for
that residual is the ZERO form. Filed as
`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`,
with the eight-call probe, the offsets that seed it and three
candidate fixes; its acceptance test is the ported parity row.

### The document this row was filed about: the AXES carry the parameter

SYM-5's review lane built it and the rows are adopted as
`crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`
(from `sym/5-review` at `a3e2b1b46`): an authored
`Datum::Frame { u: (1,0,0), v: (0,1,t) }` with `t = 0.25 ± half`, a
cube extruded from it, a `FaceFrame` on its cap, the same boss; the
twin is the boss on the tilted frame directly. The frame normal is
`(0, −t, 1)/sqrt(1 + t²)` — a stored unit vector over a `sqrt` of a
NON-CONSTANT form, which no constant fold reaches.

Measured at `half = 1e-3`, `ProfileLift::Guided`, whole declared box:

| rung | derived | authored twin |
| --- | --- | --- |
| plain `Interval` | refuses `carrier_endpoint_start` | refuses `carrier_endpoint_start` |
| `none` / `A0` / A alone | refuses `carrier_endpoint_start`, enclosure `[0, 1.7951e-2]` — 18× the half-width, the shape this row was filed on | certifies |
| `shipped` | refuses `newell_plane_residual`, a plain STRADDLE `[-6.857e-2, 6.841e-2]` (not `Invalid`), frozen 632 | certifies |

So the tier CARRIES the authored twin where the plain lane refuses it,
and does not carry the derived one. The refused residual's early form
is non-zero and carries a non-constant `sqrt(S)` with two frozen `Mul`
nodes on its path; the freezes are 642 by the profile, overwhelmingly
`Degree` — `Add` 308, `Powi` 159, `Mul` 101, `Sub` 64, on kids at
total degree 69–128. And the budget is not the lever, by execution:
raising it to 512/16,384 leaves 383 frozen and to 4,096/65,536 leaves
483, with the identical refusal at every rung.

That is the item's own mechanism, alive. A widened PROFILE (the width
parameter instead of the height) is not a third case: A0 clears it too
(frozen 0, no refusal on either lane at either width).

### Where this row stands

Open, on the TIER's mechanism — not on one clause-1 residue. SYM-5's
Phase 1 (PR-1) is the measurement and the narrow case; **Phase 2
(PR-2) runs on the tilted document**, where a unit-vector atom, a
normalisation simplified before squaring, or a degree-resetting
`sqrt` each have something to act on. The clause-1 residue on DOCM's
document is the PROPS row above and closes separately.

## What PR-2 took (SYM-5 phase 2, 2026-09-14)

**Rule E — the quotient's common factor** (`geom_core::sym::quotient`,
`SymRules::common_factor`), chosen by the measurement among the three
candidates the spec names, closes this row's mechanism. In the early
walk every form has the monomial its two halves share divided out, and
a numerator that is a rational multiple of its denominator folds to
that rational. Both are equalities of rational functions wherever the
denominator is non-zero, which clause 1 guarantees; nothing reads a
value.

**Why it is the one.** The chain the tilted document freezes on is
`sqrt(P/P)` for a degree-8 `P` in the frame's parameter — the number
ONE, carried as an opaque atom because neither half of `P/P` is a
constant for A0 to read — and above it the shared power of the
normalisation's `sqrt` atom, multiplied by every further normalisation
and doubled by every square. Candidate (a), a unit-vector atom, and
candidate (c), a degree-resetting `sqrt`, both re-key the atom and
leave that shared factor in place; candidate (b) is this one, with the
cancellation taken at every node of the early walk rather than only at
the `Powi 2`. Measured: with the rule off the derived boss refuses at
every rung under `Guided`, with it on it certifies where the authored
twin does, and raising the budget to 4,096 / 65,536 does not do it
(483 frozen, the same refusal) — so this was reach, not a cost wall.

| rung (`half = 1e-3`, `Guided`, whole box) | rule E off | rule E on |
| --- | --- | --- |
| `none` / `A0` / A alone | refuses `carrier_endpoint_start` `[0, 1.7951e-2]` | (the rule needs the early walk) |
| `shipped` | refuses `newell_plane_residual` `[-6.857e-2, 6.841e-2]`, frozen 632 | **certifies** |
| the authored twin, every rung | certifies | certifies |

**What it moves elsewhere, and what it does not.** R1's boss at
`bulge = 2`: `carrier_on_surface_2` 63/0/0/27 → 81/0/0/9 (18 numeric
decisions became THEOREMS), `witness_on_surface_2` 7/0/0/3 → 9/0/0/1,
`carrier_matches_mapped_source` 72/0/48/6 → 72/0/54/0 through the door,
and the document's whole-certifying ceiling `8.2611e2 · ε →
9.3559e2 · ε`. The D-tab's `carrier_endpoint_start` 24/0/8/4 →
24/0/12/0 on both spellings. No count falls anywhere; no ceiling on the
five measured documents moves by a digit. On M10-9's per-document pin
one of the five moves and moves UP: the ROUNDED PAD's `registered`
86 → 104, with `symbolic_zero` 695 → 858, `numeric` 1172 → 991 and
`frozen` 2750 either way — 181 decisions leave `numeric`, 163 as
theorems and 18 through the door, nothing refused or contradicted at
either dial. The other four do not move in `registered`; the link and
the bracket only shift `numeric` into `symbolic_zero` (485 → 515,
1075 → 1083). Cost on the affordability line's OWN instrument (one whole-box leaf,
`m10_10_leaf_cost_with_and_without_the_algebra`, release, off → on):
plate at `1e2·ε` 0.132 → 0.358 s, plate at its real study
0.141 → 0.340, annulus 0.120 → 0.287, bracket 0.438 → **1.699**, link
3.312 → **2.427** (cheaper, and over at both dials), pad
3.850 → **14.404**. The line is 1.6 s: the bracket, the pad and the
link are over it, disclosed. On the ceiling-bisection instrument —
not what the line is defined for — the same five read 0.23 → 0.47,
0.17 → 0.34, 0.53 → 1.23, 3.42 → 2.28, 3.73 → 10.90.

**What still stands on the tilted document, and it is not the tier's.**
At a half-width of `5e-2` the derived boss refuses with a clause-1
`Invalid` on `newell_plane_residual` — the same defect PR-1 filed as
`work/props/a-widened-derived-placement-normalises-a-straddling-newell-sum`,
at the boss's cap plane instead of its side plane. The tier has done
its work there; the value channel has not.

## Re-pointed at SYM-5's measurement (2026-09-14)

DOCM has left the tracker (`docs/DOC-LEDGER.md` sweep 14); the walk and
the directory are recoverable at the SHA that sweep names. The
references above to DOCM-1's fence, to DOCM owning the derived-frame
door and to "DOCM's diagnosis" read as history: the derived-frame door
(`Datum::FaceFrame`) has no live owner, its design is
`crates/editor-core/REFERENCES.md` (DM1–DM6), and a row on it is filed
where its mechanism lands.

## What the two reviews added (SYM-5's PR-2 fix pass, 2026-09-14)

Both reviews re-took every count and every ceiling to the digit and
neither found a box where rule E folds a non-identity. What they added
to this row:

**The reach is the DOCUMENT's, not a class.** Measured on the `Guided`
lift at half `1e-3`, rule E off → on:

| document | off | on |
| --- | --- | --- |
| tilt about `v` (this row's) | refuses `newell_plane_residual` | **certifies** |
| non-unit authored axes `u=(2,0,0)`, `v=(0,2,t)` | 1 refusal | **certifies** |
| two derived frames STACKED, `Pinned` | certifies, 219.4 s | certifies, **1.1 s** |
| two derived frames STACKED, `Guided` | 4 refusals | **1** — the boss, on the value channel's clause-1 `Invalid` |
| in-plane spin `u=(1,t,0)`, `v=(−t,1,0)` | certifies | certifies — says nothing either way |
| HALF spin `u=(1,t,0)`, `v=(0,1,0)` (R1) | refuses `carrier_endpoint_end` `[0, 1.43e-1]`, frozen 126 | **certifies**, frozen 1467 |
| tilt about `u` (`u=(1,0,t)`) | refuses `carrier_endpoint_end` | refuses identically, frozen 37 both |
| `FaceFrame` on a REVOLVED body's cap | 4 refusals, frozen 3 | 4 refusals, frozen 902, the refusal moves to `pcurve_loop_continuity` |

The tilt-about-`u` wall is the rule turning a DEGREE wall into a TERM
wall: R2 read the frozen `Powi` kid going from 606 terms at degree 60
to 440 terms at degree 28, and `440² > MAX_TERMS`. It sits behind the
`abs(1/sqrt(…))` and `copysign` atoms a `FaceFrame`'s `u_ref`
derivation mints. The rows are
`editor-core/tests/m10_derived_frame_tilted_interval`'s
`sym5_the_reach_on_documents_the_unit_did_not_build` (adopted from R2).

**The revolved cap is its own row**, filed as
`work/sym/a-face-frame-on-a-revolved-cap-refuses-on-pcurve-loop-continuity`.

**The (a)/(c) rejection is a reading, not a finding.** PR-2's body said
candidates (a) and (c) "are not it". That is not measured: (a) is an
atom PLUS the rule `Σ Uᵢ² = 1`, and that rule collapses a
re-normalisation's `S'` to the constant 1 for A0 to fold — which is a
different mechanism from re-keying alone and might reach the tilt-U
wall that rule E does not. What is measured is that rule E reaches the
documents above at the cost recorded there; (a) and (c) are unmeasured.

**Two notes from R1.** (n-2) No scheduled register re-takes the
ceilings or the leaf cost — the `#651`/`#667` shape: the numbers in
`sym.rs`'s rule-E section and in this row are re-taken by running the
evidence rows by hand, and nothing fires if they drift. (n-4)
`datum_unit_norm` is untouched by the rule on every document measured
(it is `Definite(Positive)` throughout), so the door's own unit-norm
check is not what the rule moves.

**The next shape, not taken** (R2 Q7): a canonical `Form::quotient` at
CONSTRUCTION, dial-gated, would remove the two-site convention rule E
now keeps — `form_in` cancels what the walk memoizes and
`trig::sqrt_atom` cancels what rule D builds by hand, and the two have
to agree by hand. Declined for PR-2; recorded here.

## What SYM-8 took: the tilt-`u` wall (2026-09-15)

**Rule F — the manifest sign** (`geom_core::sym::manifest`,
`SymRules::manifest_sign`) folds the two atoms the tilt-`u` wall sat
behind. `Vec3::orthonormal_basis` mints `s = 1.copysign(n.z)` and
`r = 1/(1 + |n.z|)`, and on this document `n.z` is `1/sqrt(P(t))` — an
`Inv` of a `sqrt` atom, which the FORM shows positive. In the early
walk `copysign(Y, X) → abs(Y)` and `abs(X) → X` wherever `X` is
manifestly POSITIVE; both are equalities of reals at every point
clause 1 admits and neither reads a value, so a zero reached through
them is a theorem. Strict positivity rather than the non-negativity
rule D's `atan2` fold reads: `copysign` reads a SIGN BIT, so
`copysign(1, −0.0) ≠ copysign(1, +0.0)` and at a real zero the node
denotes no function of the real value of its argument.

Measured on the tilt-`u` derived document (`u = (1,0,t)`, a cube
extruded from it, a `FaceFrame` on its cap, the boss on that —
`m10_derived_frame_tilted_interval`'s `sym8_phase1_*` rows), rule F
off → on:

| rung (`half = 1e-3`) | rule F off | rule F on |
| --- | --- | --- |
| derived, `Guided` | refuses `carrier_endpoint_end` `[0, 5.521e-2]`; the residual is `sqrt(?#…)` over a FROZEN `Powi ^2` whose kid is 440 terms at degree 27 over 298 at degree 28 (`440² > MAX_TERMS`) | that node is BUILT; `carrier_endpoint_end` 24/0/0/1 → **33/0/0/0**, every decision a theorem, and the refusal moves on to `newell_plane_residual` `[−5.744e-2, 5.744e-2]`, 662 terms and three frozen nodes on its path |
| derived, `Pinned` | certifies; `symbolic_zero` 754, `numeric` 568, 2.6 s | certifies; **876 / 446** — 122 decisions out of `numeric` — in **0.4 s** |
| derived, `Guided`, `half = 5e-2` | refuses `interval_span_forward` | identical, the same refusal |
| the authored twin, every rung and width | certifies | certifies, byte-identical counts |

So the wall this row named IS the two atoms, and it falls. The document
still does not certify under `Guided`: what stands there now is a
`newell_plane_residual` straddle the tier does not prove, filed as
`work/sym/the-tilt-u-newell-residual-is-the-next-wall`.

**What it moves elsewhere: nothing, with one disclosed exception.**
Every per-predicate split at the nominal is bit-identical with the rule
on and off on seven of the eight measured documents (the pad's nominal
split with the shape report installed exhausts the measuring box's
memory at BOTH dials and was not takeable there), and every
whole-certifying ceiling is identical to the digit on all eight, with
the over-band set at ceiling + δ identical too. The exception is the
pad at the scale it certifies whole at: `symbolic_zero` 858 → 854,
`registered` 104 → 128, `numeric` 991 → 971, `frozen` 2750 either way.
The four that left `symbolic_zero` are the ring row's class and are
recorded there. Cost on the leaf instrument: free to the measurement's
noise, and cheaper on most documents.
