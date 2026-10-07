# Review r1 — PR #4277, INTENT-LITERALS PR D: `Expr` holds no float

Head `1c47c8984a3664b3f9016c3149299b9d1c0b1725`, base `9d2b780fe6` (merge-base with main).
Reviewer lane r1 (correctness + style). I did not read PR comments, other `review/*` branches or another reviewer's output.

## Verdict: APPROVE-WITH-FIXES

The representation change is sound. `Expr`'s leaf set is §1's, and no document `Expr` holds an f64. Geometry did not move at
`f64` or at `Interval`, which I measured on base and head. Ratio semantics hold in every tier, and the new rows bite. One MAJOR
is a value bug in ruling 3: unspaced `INT/INT` lexing breaks left-associativity, so text that used to be refused now gets a
wrong value. One MINOR is ruling 1's inexact fallback, which mints a hidden variable inside a formula, against VR6. Fix the
MAJOR before merge. The MINOR needs a fix or an Ev-ratified spec revision.

## Findings

**MAJOR-1. Ruling 3 makes `w/2/3` equal `1.5·w`: whitespace overrides left-associativity.** `parse.rs:389`, the lexer's
`Tok::Fraction`. The lexer reads an integer, an unspaced `/` and an integer as one ratio token wherever they appear, including
right after a `/`. So `w/2/3` parses as `w ÷ (2/3)`. Claims C6 and ruling 3. Demonstrated by execution: a scratch row parsed,
resolved and evaluated each text at f64 with `w = 1`.

| text | value |
|---|---|
| `w/2/3` | 1.5 |
| `w / 2/3` | 1.5 |
| `w / 2 / 3` | 0.1667 |
| `w/2 / 3` | 0.1667 |
| `turn/2/3` | 9.4248 (1.5 turns) |
| `turn / 2 / 3` | 1.0472 (turn/6) |

The module doc still says `/` is left-associative. Before D, every one of these texts refused, since `Length/Count` and
`Angle/Count` are refused, so this admits previously refused text to a value no reader of `a/b/c` expects. That is exactly
what C6 asks a reviewer to look for. `w*2/3` happens to be right by value, but its tree is `w * (2/3)`, not `(w*2)/3`. No row
pins a chained division with an unspaced right-hand pair. Confidence: **sure**.

**MINOR-1. Ruling 1's inexact fallback mints an anonymous variable inside a formula, against VR6.** `parse.rs:991`
(`Formula::scalar` fallback) and `formula.rs:182`. Claims C2/C4/C6 and ruling 1. Demonstrated by execution: I declared
`h := <text>` and counted the variables the edit added.

| `h :=` | variables added | stored definition | `Doc::unparse` |
|---|---|---|---|
| `w * 0.1` | 1 | `w * 0.1` (a constant) | — |
| `w * 1/3` | 1 | `w * 1/3` (a constant) | — |
| `w * 1e-20` | 2 | `w * #3:…` | `w * 1e-20` |
| `w * 0.30000000000000004` | 2 | `w * #3:…` | `w * 0.30000000000000004` |

So for two bare numbers in the same position, one is a constant and the other an editable, offerable anonymous variable. Which
one you get turns on whether the decimal's reduced denominator fits under 2^53. VR6 says "A number inside an operator tree is
a constant", and §1 says `from_decimal` *refuses* out of range (`ConstantOutOfRange`). The ruling does neither. It also changes
identity: two slots `w * 1e-20` lower to distinct tokens, while two slots `w * 0.1` lower equal. `Doc::unparse` hides the
difference. Not a geometry bug, since an untoleranced free variable is a constant in every analysis lane (§11). But it is a
semantic fork in ratified text that the PR body files as a lane ruling, not as a question for Ev. Confidence: **sure** (on the
behaviour); **likely** (that it needs Ev).

**MINOR-2. A lone `p/q` at a slot root is rounded to a double, and reads back as a different text.** `formula.rs:410`
(`slot_root`, `K::Ratio(ratio) => … value: ratio.eval()`) and `doc.rs:1812`. Claim C6. Demonstrated by inspection only. Typing
`1/3` into a Scalar slot mints a free `Scalar` holding `0.3333333333333333`, which is fine under Q1. But `Doc::written` reads
it back as `Formula::number(0.3333333333333333)`. That number has no in-range exact ratio, so it reads back as the written
value `0.3333333333333333`, not as `1/3`. The text the person typed does not round-trip through the slot. That is benign by
value, but it is a consequence of rulings 1 and 3 together that nobody has stated. Confidence: **likely**.

**NOTE-1. Row 13's symbolic half is not "from two slots".** `intent_literals_d_constants.rs:179`. Claim C3/C9. Demonstrated
by inspection, plus my own Sym probe. The row evaluates a free-standing `Expr` twice, so no document lowering is on the path:
a lowering that turned `turn/4` into a variable would still pass. Its "90 deg" half binds two *toleranced* parameters by hand.
That is the §11-correct reading, since untoleranced written values are now constants in Sym, so §8's literal wording no longer
holds, but the row does not say that. My own probe confirms the tier is exact for non-dyadic ratios too: `1/10·3 − 3/10` is
`5.55e-17` at f64 and decides `Zero` at `Sym<Interval>`, with `symbolic_zero` = 2. Confidence: **sure**.

**NOTE-2. The new error arms' `Display` is pinned nowhere in Rust.** `ConstantOutOfRange`, `RatioNotReduced` and
`EvalError::Unlowered` (`expr.rs:1795`) have no `assert_f6` row in `display_contract.rs`. The EvalError roster in
`refusal_concision_chains.rs:2997` lacks `Unlowered`. That is defensible, because no `NodeErrorKind` can carry it: a lowered
document holds no authored leaf. But the roster is not exhaustive by construction either. Python tags are covered
(`pncad-py/src/tests.rs`). Claim C7. Demonstrated by grep and inspection. Confidence: **sure**.

**NOTE-3. Two `unreachable!` arms stand in for type facts.** `edit.rs:759` (`quantity_unminted`, raised if the door lowers a
quantity it minted nothing for) and the `DistributionRefusal::NotAWrittenValue` arm in `SetVarDistribution`. Both are true by
construction today: `mint_quantities` and `lower_with` read the same pre-order `quantities()`. Two walks are kept in step by
being the same visitor. Claim C7. Demonstrated by inspection. Confidence: **likely** (that this is fine).

**NOTE-4. Python `Formula.ratio(num: i64, den: u64)`** (`pncad-py/src/py/expr.rs:271`). A negative `den` or an `|num| ≥ 2^63`
raises PyO3's `OverflowError`, not `LiteralError` `constant_out_of_range`. For `den = 0` the error's `value` field is `inf`/NaN,
computed as `num as f64 / den as f64`. The `.pyi` promises `LiteralError` "for a zero denominator, or … past 2^53". Claim C7.
Demonstrated by inspection. Confidence: **likely**.

## The four spec-undecided rulings

1. **A dimensionless value is the bare number its text is.** *Sound for the exact case and consistent with VR6's text rule.*
   It contradicts §1's letter ("`Formula::scalar(f64)` writes a `Scalar` quantity"). Even so, it is the better reading: with it
   `unparse` round-trips, and the slot root is unaffected (Q1 mints either way). The **inexact fallback contradicts ratified
   text**: VR6 says a number in a tree is a constant, and §1 says `from_decimal` refuses out of range (MINOR-1). A different
   reading is forced for that half. Either refuse `ConstantOutOfRange` inside a formula, as the spec says, or take the
   "written value inside a formula" reading to Ev as a VR6 revision. A lane cannot rule on it.
2. **A bare integer beside a non-count operand reads as a scalar.** *Forced, and sound.* §1 spells a right angle `turn/4`, and
   §1 also makes `4` an `Integer`, which F1 refuses as `Angle / Count`. The spec contradicts itself, and some resolution is
   required. Scoping the fix to the infix fold is the narrowest one. Execution confirms that `w + 2` and `w - 1/2` still refuse,
   and that a count expression still refuses. It contradicts no ratified text. Quirk (Style S2): it is order-dependent.
   `w*2*3` is admitted, but `2*3*w` refuses, because `2*3` folds to a Count first.
3. **Unspaced `INT/INT` is one ratio token.** *Unsound as built (MAJOR-1).* Some spelling of a non-terminating ratio is forced,
   because `unparse` must write `Ratio(1/3)` somehow. Lexing it as a token anywhere is what breaks `a/b/c`. Different readings
   would not break it. One option is to lex the token only where it is not the right operand of `/`. Another is to have
   `unparse` bracket it, `(1/3)`, and refuse `x/p/q`. A third is a spelling the grammar cannot confuse. Note that `1/3.5` now
   refuses with "`.` is outside this grammar's alphabet", and `1/3e2` with "a unit suffixes a decimal". Both are odd messages
   (inspection plus execution).
4. **`turn` is a reserved keyword.** *Sound.* §1 states it, VR9 covers the load refusal ("a file this build cannot read
   refuses typed"), and no corpus file uses the name. It contradicts nothing. Only `turn` not followed by `(` is the constant,
   which is consistent with `sin` and the other calls.

## Claims

- **C1. Exercised, holds.** I ran the id-free walk (`m10_p_fence::walk`) at f64 and at `Interval`, dumping every outcome and
  every point's bits (lo/hi for `Interval`), on base and on head with one scratch file. The dumps are byte-identical:
  1562 lines each. That settles the brief's either/or. The corpus's only definition is `kitchen_sink`'s `x * 1.0`, which is
  dyadic, so no corpus value reaches a non-dyadic `Ratio`, and the id-free `Interval` claim is correct. Re-blessed pins:
  checked against the table by reading their diffs, not re-derived.
- **C2. Exercised (inspection), holds.** `ExprKind` leaves are `Ratio`, `Integer`, `Turn`, `Var` and `Leaf(StoredLeaf=!)`.
  `Ratio` is `i64`/`u64`. `Expr::try_from(&Formula)` refuses a quantity (`LowerFault::Quantity`), and a row covers it.
  `GeomPred::DatumDistance.value: Formula` is not serde and not stored. Survivor: `ExprTree::literal_bits` remains callable on
  `Expr`, where it is always empty (Style S4).
- **C3. Exercised, holds.** I read `ratio.rs` in full. `0.1` gives 1/10, `2^54` refuses, and `den = 0` refuses. There is no
  rational arithmetic: Mul/Div stay tree nodes. The f64 bits equal the decimal parse because `p/q` is exact and correctly
  rounded. At `Interval`, `lo < hi`. At `Sym`, the value is exact (my probe above). `turn/4` gives `FRAC_PI_2`'s bits.
- **C4. Exercised in part.** The PR's mint-order and retire rows pass. I did not exercise undo beyond reading: the viewer's
  history restores snapshots. VR1 holds: `5 mm + 5 mm` gives two variables, and that row passes. MINOR-1 is the exception to
  "a number in a tree is a constant".
- **C5. Exercised in part.** `WireExpr`/`WireFormula` match §5. A tampered `Literal` refuses as `Unreadable`, and that row
  passes. Mutant M2 (the load door accepting an unreduced ratio) went red in
  `m4_pr6_refusal::dimension_refusals_cross_the_load_door_whole`. I did not re-derive the wire blesses' bytes. I read
  `golden.cad`'s one-line diff, which is spelling only.
- **C6. Exercised.** `unparse ∘ parse` is bit-equal on 20 constant forms I wrote: `±1/3`, `-(1/3)`, `--1/3`, `turn / 1/3`,
  `turn / (2.0 / 3.0)`, `1/2^53` as a long decimal, `-2^53`, `-0.0`, `1e-20`, `1e300`, `-(3)` and `scalar(1) / 3.0`. MAJOR-1,
  MINOR-1 and MINOR-2 are the counterexamples to "no wrong value / no refusal moved".
- **C7. Exercised (inspection plus the suite).** Ruling 5 keeps live arms: each old arm is still raised by `Formula`'s quantity
  constructors. NOTE-2, NOTE-3 and NOTE-4 apply. Python `ratio`/`turn`/`count` are present, and the census has `Quantity`,
  `Ratio` and `EvalError::Unlowered`. I did not run the Python suite.
- **C8. Exercised (inspection), holds.** The VS-Q4 wording, the `expr.rs`/`parse.rs` module docs and DESIGN.md's companion
  row all say what the code does. The one exception is that `parse.rs`'s left-associativity claim is now false for unspaced
  pairs (MAJOR-1).
- **C9. Exercised: three mutants, each red.** M1: `Ratio::eval` as `from_f64(num/den)` turned
  `intent_literals_d_constants::a_constant_is_its_exact_value` red. M2 is above. M3: drop the parser's integer coercion turned
  `a_constant_is_its_exact_value` and `u8a_parse::every_dimension_error_reaches_through_the_parser` red. No row would go red
  under MAJOR-1's shape.

**Runs** (private `CARGO_TARGET_DIR`, foreground, or background blocked on to completion): editor-core `--profile default` gave
2881/2881 on the head, with my scratch probes included and removed afterwards. Not run: the workspace matrix, the ε rows, the
viewer, Python, tour and doctests. Known main reds: none of the editor-core ones reproduced in this run; I did not run sweep.

## Style

Questions exercised: Q1, Q2, Q3, Q5, Q6, Q7. I partly read the whole of `formula.rs` (Q8). I did not read `expr.rs`
end to end.

- **S1 (Q1).** `formula.rs:208` `Formula::scalar` and `formula.rs:226` `Formula::number` are one function under two names:
  `number` calls `scalar`. Each has its own doc paragraph saying the same thing. Grep for other new aliases in this PR.
  **sure**
- **S2 (Q7).** `parse.rs:662`: the integer coercion runs only when a fold sees a lone integer. So `w*2*3` is admitted, while
  `2*3*w` and `2*(3)*w`-style texts refuse. Commutative text has an order-dependent fate. **sure**
- **S3 (Q1).** Mint order disagrees between two doors. A slot's definition mints its quantities *before* the defined variable
  (`edit.rs`, `Lowering::slot`). `DeclareVar` mints its own id *first*, then its quantities (`edit.rs:6007`). Each order is
  justified locally, so the two are near-parallel rules. A third minting door would have to pick one. **likely**
- **S4 (Q5).** `expr.rs:1332`: `ExprTree::literal_bits` stays public and generic. On `Expr`, its documented contract is "pushes
  nothing", and `bit_eq` on `Expr` is now `==` plus an empty comparison. The spec lists `literal_bits` among what leaves `Expr`.
  Same class: `Formula::literal_value`/`display_unit` now answer for a lone `Ratio` too, so "literal" names three things.
  **likely**
- **S5 (Q2).** `formula.rs:328`: `with_distribution` turns a lone `Ratio` into a written quantity even when the distribution is
  `None`. A "clear the distribution" call therefore turns a constant into a variable. No caller does this today. **unsure**
- **S6 (Q3).** No row pins MAJOR-1's shape. `u8a_parse`'s round-trip rows run only `unparse` output, and that output never
  chains `a / p/q / r` unbracketed, so the parse-side ambiguity is invisible to them. **sure**
- **S7 (Q6).** Ruling 1's fallback (MINOR-1) and ruling 3 are disclosed in the PR body but scheduled nowhere: no `work/` issue
  and no `needs_ev`. Per the lane, a deviation that is not plainly better owes a schedule. **likely**
- **S8 (Q2).** `doc.rs` (`unread_anonymous_vars`): its doc paragraph is glued above `is_typed_value`'s, so one fn carries two
  docs and the other none. It predates this PR (it is on base); noted because the PR edits the neighbourhood. **sure**
