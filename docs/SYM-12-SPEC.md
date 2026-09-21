# SYM-12 — the derived-frame freeze's next shape: `tiltUV` rendered, the manifest-NEGATIVE arm measured, the other `copysign` mint sites counted (spec)

**Program:** SYM (`work/sym/plan.md`, the two placement-freeze rows).
**Item:** `work/sym/derived-frame-placement-freezes-on-the-symbolic-lane.md`
(its last section: what rule F does NOT reach — the one-sided reach,
the `n.z` bare parameter over a root, and `tiltUV`, "whoever takes the
next unit on this row should render it first"). **Track:** protocol
v7 **IN** by the program's default for a unit that changes what the
tier decides on a document (the negative arm of rule F is a rule of
the atom algebra, measured against the ring item's recorded loss).
The full v6 dual. Block SYM-B3 slot 1, arm FABLE per the block's draw
(byte 178). **Pre-draw fields, logged on `sym/b3-block` before the
draw:** difficulty **H**, task-class **NUMERIC**.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (every section, in order — it is the unit's history); the
ring item `work/sym/coefficient-ring-width-is-not-monotone-in-reach.md`
(opening an atom is not free: this unit is measured against it);
`crates/geom-core/src/sym/manifest.rs` (rule F: the predicate
`positive`/`nonneg`, the two folds, its soundness argument and the
signed-zero edge — the NEGATIVE arm is the same argument reflected),
`sym.rs`'s header (`# The form-level algebra`, the rule table, the
"one-sided reach" paragraph near line 484, `# Freezing`), `sym/signed.rs`
(rule C; what a sign READ is), `sym/quotient.rs`, `sym/trig.rs`;
`crates/geom-core/src/linalg/vec.rs`'s orthonormal basis (READ-ONLY —
LINALG's/PROPS'; where `s = 1.copysign(n.z)` and `r = 1/(1 + |n.z|)`
are minted); the other `copysign` mint sites the SYM-8 body named:
`crates/geom-brep/src/implicit.rs:200` (the cone's `s_a.copysign(h)`),
`crates/profile/src/sugar.rs:1076` and `:1402` (the arc's spoke and the
fillet's apothem), `crates/profile/src/path.rs:2892` (`one().copysign(half_tan)`),
`crates/geom-core/src/linalg/svd.rs:202` (Householder's `alpha`);
the rows: `crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`
(`sym8_phase1_the_tilt_u_ladder`, `sym8_the_reviews_documents_the_unit_did_not_measure`
— the four documents and their table, `flipZ`, `tiltNZ`, the start cap,
`tiltUV`; the gating row
`m10_the_tilt_u_derived_boss_stops_on_the_newell_residual_and_names_it`),
`crates/geom-core/tests/sym_rule_f_rows.rs` and `sym_rule_f_interval_rows.rs`;
the `m10_8`/`m10_9`/`m10_10`/`m10_bulge` pins and `m10_sym_profile_interval`'s
ledger (the eight documents' acceptance); `work/sym/the-tilt-u-newell-residual-is-the-next-wall.md`
(the wall that stands after rule F — NOT this unit's: DECIDE-3's
canonical root and reads are aimed at the sign-hull frame's Newell
margin; this unit does not touch it and says so where it meets it).

## The claim

Rule F's measured reach is the tilt-`u` document alone. Three things
stand beside it, none measured: (1) **`tiltUV`** (`n.z = 1/sqrt(1 + 2t²)`),
which both SYM-8 reviews predicted as the shape rule F folds and which
moves not one count at either lift — the fold either never fires on
that document's `n.z` form or fires without reaching a decision
(SYM-11's R1 guessed the subform freezes before the `abs`/`copysign`
node combines: the budget, not the predicate); (2) **the reach is
one-sided**: a frame whose `n.z` is `−1/sqrt(P)` (the start cap, the
`FlipZ` frame) is declined by a predicate that refuses a negative
coefficient outright, and the manifest-NEGATIVE arm — `abs(−X) = X` and
`copysign(Y, −X) = −abs(Y)` for a manifestly positive `X`, identities of
reals exactly as the folded ones are — was NOT taken by SYM-8 because
it doubles the shapes the early walk opens and opening an atom is not
free; (3) **five other `copysign` mint sites** exist beside the
orthonormal basis, and nothing says whether any reaches a decision the
tier is asked on a measured document. **The unit's thesis: measure all
three before writing anything, and take the negative arm only if the
measurement says it buys decisions on a document without losing any
anywhere — the ring item's acceptance, stated before the code.**

**Ratified and not re-litigated:** E12; rules A–F and their order (F
before C at the `abs` node); the freezing budget; the signed-zero
edge (strict positivity); the ring item's finding; SYM-8's pad-four
ruling; the acceptance "no split or ceiling moves down anywhere".

## Phase 1 — before touching anything (the measurement)

1. **`tiltUV`, rendered.** On the tilt-`uv` derived document at
   `half = 1e-3` under `Guided` and `Pinned`, rule F off and on:
   render the refused residual and the `n.z` subform at `explain_depth`
   6 (the shape report; the pad's OOM does not apply — this is a cube);
   say which of the three is true — the fold never fires (the
   `copysign`/`abs` argument is not manifestly positive as a FORM: say
   what its form is), it fires but the node is frozen before it combines
   (the budget: name the node and its term count), or it fires and the
   decision was never asked (the refusal is elsewhere). One table; the
   answer recorded on the item whichever it is. If it is the budget,
   say what budget would reach it and what it would cost — measured, not
   guessed — and do NOT move the budget in this unit.
2. **The `copysign` census.** For each of the five other mint sites:
   what quantity is signed, whether it runs at `Sym<Interval>` on any
   of the eight measured documents (or on the reviews' four), and
   whether the signed argument's form is manifestly positive, manifestly
   negative, or neither at the decisions the tier is asked. Instrument:
   the shape report's atom census (`copysign`/`abs` atoms per residual)
   on each document at the nominal. One table; sites the tier never
   sees are said to be so.
3. **The negative arm, hand-planted.** `abs(−X) = X` and
   `copysign(Y, −X) = −abs(Y)` where `−X` is manifestly negative (the
   predicate reflected: every coefficient non-positive, one strictly
   negative term of manifestly positive indeterminates, over a
   manifestly non-negative denominator — write it down before coding
   it, as SYM-8 did its positive half; the signed-zero edge is the same
   and closed the same way), planted in `manifest.rs` behind a local
   dial and REVERTED after the table. Measured on: the start-cap and
   `FlipZ` documents (the one-sided reach), the tilt-`u` document (must
   be unmoved), the eight measured documents' splits at the nominal and
   whole-certifying ceilings on both instruments (must be unmoved or
   UP), the walk ledger, and whatever Phase 1.2 found reaching the
   tier. One table, off → on, per document: split, ceiling, frozen,
   `sign_gated`, first refusal by name.
4. **The stop.** If the negative arm moves no decision on any document
   (Phase 1.3 all identical), or moves a split or ceiling DOWN
   anywhere it does not fix elsewhere, Phase 2 is not taken: the item
   records the three tables and the arm stays a known shape; the PR is
   the record and the unit's whole deliverable.

## Phase 2 — the negative arm (only if Phase 1.3 earns it)

- `manifest::negative` beside `positive`, sharing the term-wise
  machinery (one home; the reflection stated once); the two folds under
  `SymRules::manifest_sign` (the same dial — one rule, two arms; say
  why not a second dial, or give it one if the census differentials
  need to tell them apart, and then the census counts it).
- The soundness argument in `manifest.rs`'s header, reflected, with the
  signed-zero edge closed by strict negativity.
- Rows: a theorem row per fold at the scalar door; the negatives (a
  form with a non-negative term declines; a sum of squares negated
  declines — it can be zero); the ordering against rule C pinned by a
  row that reds under the other order (SYM-8's two rows are the
  precedent — do not re-use the row that cannot red); the start-cap /
  `FlipZ` documents' rows re-cut with what moved, by name.
- The acceptance re-taken as gating pins where a count moved, and every
  pin that did not move left bit-identical.

## Scope

- Files: `crates/geom-core/src/sym/manifest.rs`, `sym.rs` (the rule
  table, the header's reach paragraph), tests under
  `crates/geom-core/tests/sym_rule_f*` and
  `crates/editor-core/tests/m10_derived_frame_tilted_interval.rs`
  (the announced tests-family overlap). `linalg/vec.rs`, the five
  other mint sites and the Newell wall are READ-ONLY.
- No new value read; no budget change; no new tolerance.

## Acceptance

- Phase 1's three tables in the PR body, the `tiltUV` answer stated in
  one sentence and recorded on the item, the census's "never reaches
  the tier" sites named as such.
- If Phase 2 is taken: no split or ceiling moves DOWN anywhere (the
  eight documents on both instruments, the walk ledger); what moves UP
  is pinned by name; the tilt-`u` row bit-identical; cost on the leaf
  instrument disclosed against the line.
- Local checks green: `cargo fmt --all -- --check`; clippy `-D warnings`
  on `geom-core` at default, `interval`, `interval,sym-profile-testing`
  (all targets) and `editor-core --features interval` (all targets);
  `scripts/doc-gate.sh`; `python3 scripts/work.py lint`. The hosted
  matrix is the verification of record.

## Review

Protocol v7 IN: the full v6 dual — the arm per block SYM-B3's draw
(FABLE at slot 1), two blinded reviewers on a frozen green head
(ordinal claimed on `main` at dispatch, SYM's band 4700–4799), the
union fix pass on the implementer's lane, a delta by R1, the row at
merge.

## Landing

PR against `main`; the spec deleted at merge with its
`docs/DOC-LEDGER.md` entry; the item's "what rule F does not reach"
section answered; the SYM log carries the verdict. Branch
`sym/12-negative-arm`.
