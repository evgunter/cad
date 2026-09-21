# SYM-8 fix pass — the adjudicated union of both reviews

PR #2616, reviewed head `b47d4ab62`; the branch has since merged `main`
(`04c2b4760`, the 2026-09-20 cut: `work/sym/SYM-8.md` is now
`work/decide/SYM-8.md` with `priority`/`cost` fields) — build on that
head. Two blinded reviews ran on the frozen head: R1 (probe branch
`sym/8-review-r1` @ `1be1f3099`: `crates/geom-core/tests/sym8_r1_probes.rs`,
rows in `m10_9_pins_interval.rs` and `m10_derived_frame_tilted_interval.rs`)
MERGEABLE-AFTER-FIXES 2 MAJOR / 3 MINOR / 4 NOTE / 10 style; R2
(`sym/8-review-r2` @ `3efcba844`: `crates/geom-core/tests/sym_rule_f_r2_probes.rs`,
`crates/editor-core/tests/sym8_r2_probes_interval.rs`)
MERGEABLE-AFTER-FIXES 1 MAJOR / 4 MINOR / 7 NOTE. Both reports are in
`work/sym/logs/fork-state-local.md` on branch `sym/fork-state-local`
(read them whole). Neither could break rule F's soundness (claim 1
stands, by execution on both sides); both found the SAME two defects
independently. Neither could run the editor-core re-takes (the shared
box could not link the binary), so every editor-core number below is
verified by hosted CI or not at all — say which in the PR body.
Fetch both probe branches; adopted rows are credited (`R1`/`R2`) and
moved into the unit's suites under the suite's naming; the probe
branches are not merged.

Every item is TAKEN unless marked DECLINED.

## A — rule F leaked into the "bit for bit" differentials (R1 M2, R2 MAJOR-1; `sure`; R2 DEMONSTRATED)

`SymRules::without_the_algebra()` (`sym.rs:~1477`), `shipped_without_the_door()`
(`:~1493`) and `without_rule_e()` (`:~1505`) are `..Self::shipped()` and
now carry `manifest_sign: true`; their docs say "M10-9's / M10-8's /
M10-10's tier exactly, bit for bit" — all three false, undisclosed.
Consequences: `m10_10_pins_interval::m10_10_the_shipped_set_carries_the_algebra`
("five algebra dials and nothing else") passes only because both sides
carry F; `m10_8_pins`'s `a0_alone()` is "A0 alone" with F on and its
census never names F or E; every M10-8/M10-9 pin row and the leaf-cost
row's "algebra OFF (M10-9)" column run with F on; the M10-10
differential no longer isolates the algebra. Take: `manifest_sign:
false` in all three (the line SYM-5 added for `common_factor` in
`b9c4187f2` is the precedent), the algebra census says SIX dials, the
`m10_10` "nothing else" row counts F; adopt R2's
`r2_the_algebra_off_differential_is_documented_as_m10_9s_tier` (red on
the head, green after); sweep every other `SymRules` builder and every
`..Self::shipped()`; re-read `m10_10_evidence_interval`'s `no_f` rung
against the new `without_the_algebra`. State in the PR body whether any
pinned count moves (the reviewers expect none — rule F is early-only);
list this as a formerly SILENT deviation.

## B — the F→C ordering pin cannot red (R1 M1 DEMONSTRATED, R2 MINOR-1)

`sym_rule_f_rows::the_manifest_sign_lands_in_symbolic_zero_and_not_sign_gated`
is named as the pin at three sites (PR body, `sym.rs:~432-436`,
`manifest.rs:~136-138`) and is invariant under the order: its rows use
`Sym::param` (no bracket — `signed::fold` declines on
`params.is_empty()`) and the argument carries a `sqrt` atom rule C
cannot enclose. R1 planted C-before-F: all four PR rows pass. Take:
adopt R2's `r2_the_order_against_rule_c_is_pinned_by_a_residual_rule_c_would_take`
(`abs(1 + t²) − (1 + t²)` with `param_over` over `[0.2, 0.3]`, C on:
`theorem` shipped, `sign_gated` with F shut) AND R1's
`r1_a_shape_both_rules_take_is_what_pins_the_order` (`abs(2/t²)` over
`[0.3, 0.5]`); plant C-before-F yourself and quote the red in the PR
body; re-word the three sites to name the row that actually pins the
order; the existing row's doc says what it does pin (the fold lands in
`symbolic_zero`, not the order). Disclosed deviation 5 stays (the walk
ledger is silent on F).

## C — the pad's four: RULED, and guarded (R1 m3, R2 MINOR-4)

Both reviewers read the spec's Phase 1.3 stop clause as literally
tripped (`symbolic_zero` 858 → 854 on the pad, four theorems weakened
to axioms through the door; `registered` 104 → 128, `numeric` 991 →
971, no decision lost, no ceiling moved), and both judged the ship-on
defensible. **The orchestrator's ruling: rule F ships ON; the clause
is a spec deviation RATIFIED here**, on the record: no decision is
lost, no per-predicate total moves, no ceiling moves, the four are
re-taken by the registry, and the lane showed narrowing cannot
separate the pad's atom from tilt-U's (both are `abs(n.z)` of one
construction). Take: (1) say exactly that in the PR body's deviation
list and on `work/decide/SYM-8.md` — "spec clause not met, ruled by the
SYM orchestrator, reason", not "disclosure"; (2) GUARD what moved:
`m10_9_pins_interval`'s `Study` gains `symbolic_zero` (or the pad's row
asserts 854 beside 128), so a later change costing four more theorems
reds — today only `registered` is pinned and the 858 → 854 lives in a
comment (R1: the #651 shape); (3) adopt R2's
`sym8_r2_the_pads_four_re_taken` as the row that measures both dials
(gated if it runs in the pins file's budget, `#[ignore]`d with the
reason if not — it is one replay over the analyzed box, which the pins
row already pays); (4) the item's sentence "rule E shipped with the
same hazard" is corrected: rule E's loss was demonstrated at the
SCALAR, rule F's is realised on a MEASURED DOCUMENT (R1).

## D — stale citations of a moved symbol (R1 m4, R2 MINOR-2; `sure`)

`trig::manifestly_nonneg` no longer exists; cited at `sym.rs:~210`,
`sym.rs:~2182` (inside the `combine` arm this PR edited),
`quotient.rs:~325`, `tests/sym_rule_e_rows.rs:~344,~395`,
`work/decide/rule-d-reaches-the-unit-bulge-only.md:~173,~333`. Sweep
the whole tree for the old name and fix every site.

## E — the wrong reason (R1 m5, R2 style; `sure`)

`manifest.rs:~63-66` says `D` needs only non-negativity "because a
point where `D` vanishes is a point the value channel divided by zero
at" — the sentence `quotient.rs`'s header names as "the mistake this
paragraph replaces" (a denominator has FOUR sources; (ii)–(iv) are
non-zero for range reasons). The conclusion (`D ≠ 0` on an admitted
box) survives; the reason does not. Point at quotient's four-source
argument and say F mints no new denominator (R2's reading). The same
sentence in the PR body and the item.

## F — no gating tilt-U row (R2 MINOR-3, SILENT)

The acceptance asked for the tilt-U parity row green OR the width it
stops at pinned by name; every `Base::TiltU` row is `#[ignore]`d. Add
ONE gating row: at `half = 1e-3`, `Guided`, derived, `carrier_endpoint_end`
33/0/0/0 with F on (24/0/0/1 with `without_rule_f`) and the refusal
now `newell_plane_residual`, asserted by name — if it runs under ~30 s
in the test profile; else `#[ignore]` it and say the cost at the path.

## G — the f64 lift and the adversary (R2 NOTE + R1 n6 residue)

R2's `E = (x+1)² − x² − 2x − 1 + 1e-30(1+y²)` is manifestly positive
as a form but the f64 channel evaluates it to −1 at `x ~ 1e8`, so
`copysign(1, E) − 1` is a DEFINITE −2 while the tier says the identity
is zero: `Sym<f64>::sign_within`'s contradiction `debug_assert!` FIRES
(6 of 6 points) and the panic leaves the session installed. Not rule
F's unsoundness (`E > 0` as reals; the f64 lift is not an enclosure),
but rule F is the first rule that turns a one-ulp sign error into a
2.0 disagreement. R1's residue: `copysign(1, 1/(t−1)²) − 1` at `t = 1`
answers `theorem` at f64 where the function is undefined. Take: adopt
R2's adversary row and R1's `r1_a_manifestly_positive_form_undefined_inside_the_box`
(the D = 0 edge: `refused Invalid` at both dials on a box holding the
pole; `theorem` clear of it) as rows; record both shapes on
`work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert.md`
as a second mechanism (the f64 lift has no clause 1); note at
`sym_rule_f_rows::sound` that it trusts the f64 value at the point and
would call a true theorem unsound under cancellation. No change to the
assert (that row is SYM's, not this unit's).

## H — structure (R1 S1–S10, R2 style)

1. `m10_derived_frame_tilted_interval.rs`'s `fn with_rule_f() -> SymRules
   { SymRules::shipped() }` — a name that lies the day F leaves the
   shipped set; call it what it is.
2. `manifest::magnitude` mints its `Abs` atom with `sess.atoms.entry(..)`
   and a hard-coded payload — a third spelling of atom minting beside
   `mint_atom` and `trig::sqrt_atom`, equivalent only because F is
   early-only (`plain_atoms` bookkeeping skipped otherwise). Use
   `mint_atom`; note the class (`trig.rs` twice) on TIER's
   `sym-rs-is-one-file-…` or a new row.
3. `ATOM_DEPTH`'s comment argues 2–3 and sets 8, says "fixed cost per
   node" but bounds depth not breadth with no memo. Make it true.
4. `sym_rule_f_rows:~251-263` (the mint-site row) asserts nothing about
   the rule (`theorem` at both dials): say so in its doc or make it
   discriminate.
5. `label()` reads `theorem` only at `symbolic_zero == 1`; a row
   discharging twice reads `numeric Zero` and passes every `assert_ne!`
   — count-robust.
6. `positive_poly`'s `!c.is_zero()` is dead by `Poly`'s invariant —
   remove or say it is documentation.
7. The leaf-cost numbers in three places (PR body, `sym.rs:~495-501`,
   `sym.rs:~1431`) — one home.
8. **The motivation prose** (`manifest.rs:~10-23`, `sym.rs:~421-437`):
   written around Duff's `s = 1.copysign(n.z)`, which PROPS's #2468
   deletes. Write the INVARIANT first (an `abs`/`copysign` atom over a
   form the syntax shows positive is the number the form denotes) and
   Duff as the mint site that motivated it, noting the sign-hull frame's
   `|n.z|` is the next; the `copysign` arm's other mint sites
   (`implicit.rs`, `curved.rs`, `sugar.rs`, `path.rs`, `svd.rs`) named
   so the arm is not read as orphaned. This is the prose SYM-8 owes
   PROPS for landing first.
9. The pad's nominal split OOM (deviation 4) is disclosed but
   unscheduled: one line on TIER's cost row
   (`work/tier/symbolic-tier-costs-95-percent-of-the-m10-3-drive.md`).
10. R1's and R2's e2e document rows (`r1_sym8_three_documents_the_unit_did_not_measure`,
    R2's tilt-uv / start-cap / z-touches-zero) were written and never
    run: adopt as `#[ignore]`d evidence rows whose doc says "written by
    the review, not run; run once by the fix pass if affordable" — run
    them if hosted CI or your box allows and record what they show.

## DECLINED (recorded, not taken)

- Shipping the dial OFF (ruled in C).
- Changing `Sym<f64>::sign_within`'s assert or the f64 lift (G): SYM's
  row, not this unit's.

## Checks and report

`cargo fmt --all`; clippy `-D warnings` on `geom-core` (default,
`interval`, `interval,sym-profile-testing`, all targets) and
`editor-core` (`interval`, all targets); `scripts/doc-gate.sh`;
`python3 scripts/work.py lint`; territory listed. Run `sym_rule_f_rows`,
the adopted geom-core rows and the planted order locally; the
editor-core rows ride the hosted run on your fixed head, which must be
green on the full matrix before you report (twelve `test (…)`, five
`k-lint (gate, …)`). Report ≤120 lines: per item TAKEN with the commit
and the evidence (the reds before / greens after for A and B, the
plant's red for B), which editor-core numbers CI verified and which
remain unverified, anything you disagree with STATED rather than
smoothed. Do not post on the PR beyond the body edit.
