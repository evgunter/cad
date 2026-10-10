# Review R2 — INTENT stage 2 PR F (#4566), frozen head `cfe1ca2115`

**Verdict: NOT-MERGEABLE-AS-IS**

**Counts: 3 MAJOR · 7 MINOR · 7 NOTE**, plus the style section.

Reviewer R2 (Opus), in the concurrent dual review. I reviewed exactly
`cfe1ca211595f5441babc4e0b58f0c2e0059d86f`; the PR head has since moved
to `a74be9fc`, which I did not read. **Lane isolation:** I read the PR
body and nothing else on the PR. I did not open its comments, its
reviews, CI artifacts, or the other reviewer's branches. No accidental
glimpses.

The probe rows are in `crates/editor-core/tests/r2_s2f_probes.rs` on
`review/s2-f-R2`, wired into `tests/all.rs`. Run them with
`cargo test -p editor-core --test all r2_s2f_probes`. **Four are red**
(P1, P5, P7, P9b) and six are green (P2, P3, P4, P6a, P6b, P9a).

---

## MAJOR

### M1 — A split now accepts a selection crossing a posed cut placement, and the remainder's value changes

- **Where:** `crates/editor-core/src/refactor.rs:2950-2956`. The
  `world_heir` identity check now skips selection reads, because "a
  selection names the copy it reads, so it asks no such thing". The
  crossing rule at `:2927-2935` admits the selection.
- **Claim falsified:** 5. "Split and inline refuse or carry every
  crossing correctly", and "`inline(split(d))` must equal `d` wherever
  split accepts".
- **Demonstrated** by probe row `p7_a_selection_crossing_a_posed_cut_placement_keeps_its_value`:
  - The document: body B, placed at a pose of z+10, and body C, placed
    at the identity. A kept `Measure` reads `distance(B.cap(End), C.cap(End))`.
    The cut is B (its frame, profile and extrude) plus B's placement.
  - **Main** (the same row, run in a main worktree) refuses
    `RemainderReadUncarried { why: Posed { placement } }`.
  - **The branch** accepts the cut. The remainder's measure reads
    **9.0, where it read 1.0 before the split**. The re-anchored
    selection now reads B's posed world copy, not B where it is built.
  - `inline` on the branch's split output then refuses
    `InstanceReadUncarried { why: Posed }`, so the round trip fails
    where split accepted.
- This is a lost refusal and a silent wrong value. It reaches every
  selection reader (measures, fillets, shells, face frames), not only
  mate sides. A mate side crossing a posed placement takes the same
  path; I infer that but did not run it.
- **Suggested fix:** keep the identity check for selection crossings
  too, or re-anchor the selection so it still reads the unposed body.
  Inline's `selection_heir` already refuses `Posed`, so the first option
  matches it. Add P7 as a row.

### M2 — The edit door builds a document the save and load doors refuse (a `Bodies`-bodied selection shared with a measure)

- **Where:**
  - The edit door's rule is per minting slot: `crates/editor-core/src/edit.rs:1273`
    (`selection_body_kind`) admits `Bodies` only when the selection is
    minted at a mate `Side`.
  - The load door's rule is per reader set: `crates/editor-core/src/persist/check.rs:620`
    (`only_mate_sides`) admits `Bodies` only when every reader of the
    selection is a mate side.
  - These are different predicates. Nothing re-checks a selection's body
    kind when a second reader reads it by `Operand::Var` or by name.
- **Claim falsified:** 9 ("`persist::check` mirrors it exactly").
- **Demonstrated** by probe row `p9b_a_bodies_selection_shared_with_a_measure_agrees_at_both_doors`:
  1. A mate's `a` side is selected on a pattern's `Bodies` (accepted).
  2. Both side selections are named with `RenameVar`.
  3. `editor_core::measure` adds a `Distance` reading the two selections
     by `Operand::Var`. The edit door **accepts** it.
  4. `save` then refuses with
     `Snapshot(SelectionBody { var: "leg_top", fault: Kind { found: Bodies } })`.
- So an accepted edit leaves a document that cannot be saved. The
  opposite hole also exists: a selection with no readers left passes
  `only_mate_sides` vacuously, because `all` over an empty set is true.
- **Suggested fix:** give the rule one home. Either check the body kind
  at every read of an existing selection by a non-side slot, or make the
  admission a property of the selection rather than of its readers.
  Then make the load walk call that same function. Add P9b.

### M3 — A failed mate on unplaced members now fails the at-rest gate, though spec and ASSEMBLY.md say such a mate mints nothing

- **Where:** `crates/editor-core/src/assembly.rs:1366-1373`. The
  `evaluation.usable(id)` check, which raises `MintRefusal::Unevaluated`,
  runs **before** `copy_face`'s placement test at `:1576-1578`. That
  test is what decides that a side whose body no placement reads mints
  nothing.
- **Claims falsified:**
  - Claim 2's other half: a mate is "skipped only because its member is
    unplaced". Here an unplaced mate is not skipped; it refuses.
  - Spec §7 and test 22: a mate whose members are not both placed "mints
    nothing ... not an at-rest fact of the product".
  - Spec §1: "a Measure, Assertion or Mate can never gate it".
  - ASSEMBLY.md's new paragraph, "Where a declaration is minted", makes
    both promises, and they contradict each other for this document.
- **Demonstrated** by probe row `p1_a_broken_mate_on_unplaced_operands_does_not_gate_the_product`:
  - The PR's own test-22 document: a mate on two instances, their union
    placed and the instances not placed. `assemble` passes.
  - Re-point side B at a face name the instance does not carry. The mate
    fails (`SelectResolve`), and `assemble` now refuses
    `Mint { refusals: [Unevaluated { mate, standing: Failed }] }`.
  - `product_recorded` still succeeds. Only the A5 gate refuses.
  - On main, a mate with no value was passed over (`continue`), so the
    gate passed.
- Call 1 (`Unevaluated`) was accepted by the orchestrator. The defect is
  the order of the checks, not the call. A mate that never spoke for the
  product cannot "leave the gate certifying a product it no longer
  speaks for", which is the call's own rationale.
- **Suggested fix:** decide statically whether any world placement reads
  each side's body, or a `Part` of it, before the `usable` check. Raise
  `Unevaluated` only for a mate that would otherwise mint. Add P1.

---

## MINOR

### m1 — A face-based side whose name vanishes still refuses as `MateFault::FaceUnresolved`, not as the selection's error

- **Where:** `src/mate/solve.rs:1442` (`face_base`), and the early
  return for a solve-faulted mate in `src/eval/mod.rs:4210-4219`, which
  runs before the mate's `select()`.
- **Claims:** 7, and spec §7 ("`FaceUnresolved` becomes the select's
  resolve error").
- **Demonstrated** by probe row `p5_...`. With `MateFrame::from_face()`
  on both sides and side B re-pointed at a ghost name, `node_error(mate)`
  is `Mate(FaceUnresolved { refusal: Reach { refusal: NoSuchName } })`,
  not `SelectResolve`.
- So the user sees one of two error families for the same fact (a side's
  name stopped resolving), chosen by the frame's base word. The
  selection's `resolve::resolve` diagnosis is never the one shown.
- The PR body discloses "`FaceUnresolved` is kept" but gives no reason
  it beats the spec and no schedule.
- **Fix:** let a `SelectResolve` on a side take precedence over a
  `FaceUnresolved` whose cause is the same name. Otherwise write the
  deviation into the spec, or file it.

### m2 — The `MovedAbove` recourse is gone, and nothing replaces it: a mate read below a placer now silently says nothing

- **Where:** `src/assembly.rs:1576-1578`; the rows the PR renamed are
  msolve13 A1(a)/(b), msolve2, msolve5 and mate1.
- **Claim:** 7 ("a diagnostic users relied on is lost").
- **Demonstrated by the PR's own rows.** msolve13 A1(a)'s document used
  to mint both contacts and pass the gate. Now it mints nothing, and the
  gate refuses with every finding `Attribution::Unattributed`. Nothing
  points at the mate, or says the mate was not consulted because its
  member is not placed.
- The behaviour is the spec's. The finding is the lost pointer: before,
  `MovedAbove` told the user to "re-pick the face on X".
- **Fix:** add a non-gating row or finding attribution, such as "mate M
  mints nothing: no placement reads the body its side names (placement P
  reads its consumer C)".

### m3 — Poisoning a determining mate through the member it places loses its role and log

- **Where:** call 3; `src/eval/mod.rs:4210` and the general poison path.
  The PR edited `tests/msolve14_run_scalar.rs:1330` (A2) from "the mate
  evaluates" to "the mate is `Poisoned` through the bolt".
- **Claim:** 7.
- **Shown by the PR's own edit.** A mate that solved fine is now
  poisoned by the bolt's placement-door refusal, although the mate
  *determines* that bolt's pose. The bolt's failure is downstream of the
  mate's answer, so poisoning the cause by its effect drops the role and
  the solve log. That log is the diagnostic explaining where the bolt
  was put.
- **Fix:** don't poison a mate through an instance it places, or keep
  the solve log on a poisoned mate.

### m4 — The C5 relaxation is existential over all mates

- **Where:** `tests/msolve14_run_scalar.rs:2043-2052`.
- **Claim:** 7 (the C5 relaxation "hides a real escalation that went
  missing").
- `mate_poisoned_through_moved` passes when **any** mate is poisoned
  through **any** escalated node. In a scene with two mates, one
  escalation could be lost from mate X's log while mate Y is poisoned
  through a moved member, and the assertion still passes. This is
  inferred, not demonstrated.
- **Fix:** key it per moved node. Each moved node needs a mate log
  escalation naming it, or a mate poisoned through that exact node.

### m5 — The work item the PR body schedules for the tour's re-keying interim does not exist at the frozen head

- **Where:** `demos/tour/src/assembly.rs:1167-1198`, and the PR body's
  `work/intent/a-gauge-swing-re-keys-the-mates-reading-its-instances.md`.
- **Claims:** 6, and style Q6.
- **Demonstrated:** `ls work/intent | grep swing` is empty, and `rg`
  finds the name nowhere in the tree. The disclosed interim is
  therefore unscheduled at this head.
- The re-keying itself is right: the mate's key reads its instances'
  keys, and those carry the gauge chain. msolve6 A5's re-pin moving the
  mate key is also right.
- **Fix:** commit the item, and cite it at the tour's assertion.

### m6 — A Python test's module docstring still describes the retired arms as current

- **Where:** `crates/pncad-py/tests/test_assembly_author.py:43-70`.
- **Claim:** 8.
- **Demonstrated by grep.** The docstring says "`ref_moved_above` is
  reached below ... the gate names the operand and the placed union". It
  explains why `ref_vanished` is not reached, and says `ref_ambiguous`
  is unreachable. The first two arms are deleted; Ambiguous is now
  reachable by placing twice.
- **Fix:** rewrite the docstring.

### m7 — `copy_face` answers "mints nothing" when the graft lacks the face, where main debug-asserted

- **Where:** `src/assembly.rs:1590`.
- **Claim:** 2.
- In a world gather, a world placement's copy that lacks the entity's
  face key breaks this PR's own invariant: "a placement moves its body
  and re-mints nothing". `copies(placement).and_then(|k| k.face(key))`
  turns that into `Ok(None)`, so the mate drops out of the gate
  silently. Main's `resolve_face` debug-asserted the analogous row.
- My P2 rows show the keys line up for pattern picks in every order: the
  gate certifies the top resting on the right copy. So this is inferred
  only.
- **Fix:** `unreachable!` or `debug_assert!` when `copies(placement)` is
  `Some` and the face is missing.

---

## NOTE

- **n1 — Claim 3 holds; I reproduced it independently on a wider
  corpus.**
  - **Method:** I added an identical, env-gated dump to
    `solve_document`, `solve_document_at` and `solve_with_env` in a main
    worktree (`fe6a5a30`, the merge base) and a branch worktree. It
    records each instance's `Pose` (Debug, exact f64), its group root,
    unplaced causes, roles and fault class, all keyed by document
    position.
  - **Corpus:** every test matching
    `msolve|mate|asm_|fix_pattern|rv_|part_depth|switch_slots|dm7|place_|p2_|display_contract|edit_|intent_s2`,
    run one process per test. That is 849 tests on main and 857 on the
    branch, at ε unset, 1e-6 and 1e-12. ε took effect: dumps differ
    across ε on 10 and 12 tests.
  - **Result:** all tests passed on both sides at all three ε. Of the
    469 shared tests that solve, every main solve line appears bit-equal
    on the branch. The 7 that differ differ **only** in the fault class:
    `DanglingHead` became `SideUnresolved` for a deleted side's body, as
    disclosed. Two of them carry extra branch solves from added
    assertions.
  - The 7 renamed rows, such as `*_refuses_moved_above` becoming
    `*_mints_nothing`, are also bit-equal pair for pair at all three ε.
    That includes msolve1 `two_operands`, whose fixture was re-spelled to
    read at the union.
  - Four msolve13 lift rows (chamfer, shell, pair boolean, vanished) were
    deleted, not renamed, so their documents are not compared.
- **n2 — The dispatch's and the spec's "Q8" names no section.**
  `docs/INTENT-STAGE2-SPEC.md` cites "Q8" four times (lines 160, 350,
  466, 553), but §11 has no Q8. The copy rule this PR implements is
  stated only in the PR body and in ASSEMBLY.md. *Sure.*
- **n3 — Claim 4 holds.** By reasoning and by the PR's rows: every
  former reading edge `(mate, member instance)` is now a path through
  the side's body operation. One difference: a mate whose walk fails
  (`DanglingHead`) now couples to the body it reads, where it used to
  couple nothing. That is "reads couple", so it is consistent, but A9's
  sentence "relatively unconstrained exactly when..." now counts a mate
  that constrains nothing. *Likely.*
- **n4 — AQ8's new wording does not match the code.**
  `refactor.rs:3487` says "only a mate whose two face reads resolve can
  cross", but the collector gates on `head_of` plus `member_of`, which
  are structural. A mate whose side's name no longer resolves, but whose
  head still walks to a member, still contributes a crossing. *Likely*,
  not run.
- **n5 — The mate content key has no per-side length prefix.**
  `eval/mod.rs:5573` feeds `[a, b]`'s names with no count per side,
  unlike `feed_scalar_join`. One-name sides make it unambiguous, and an
  empty side is refused before keying, so I found no reachable
  collision. *Unsure.*
- **n6 — Two quadratic lookups.** `product.rs:1138`'s `copies` is a
  linear `find` over the grafted copies per side. `copy_face` builds
  `doc.var_env::<f64>()` per side per mate (`assembly.rs:1555`).
  *Sure*, minor cost.
- **n7 — A stale comment:** `crates/pncad/tests/all.rs:3809` says "which
  is exactly what the reading edge is recomputed from". *Sure.*

## Claims summary

| # | Claim | Result |
|---|---|---|
| 1 | Copy rule right and total | **Held** for patterns, pick order, missing pick, a double pick, and round trips (P2, P3, P4, P6a, P6b green). Transforms are not followed, by design; see S1. No `unreachable!` reached. |
| 2 | No mate silently dropped | **Partly fails:** M3 (an unplaced mate refuses rather than being skipped); m7 (a missing graft face drops silently, inferred). |
| 3 | Poses unmoved | **Held**, independently (n1). |
| 4 | A9 keeps the gauge edges | **Held** (n3). |
| 5 | Split/inline carry or refuse correctly | **Fails:** M1. |
| 6 | Content key sound | **Held.** The re-keying is right; the work item is missing (m5). |
| 7 | Poisoning and diagnostics | **Partly fails:** m1 demonstrated; m2, m3, m4. |
| 8 | Retirement complete | Code is clean. Stale text: m6, n7. |
| 9 | Edit and persist doors consistent | **Fails:** M2. |

---

## Style

Questions exercised: **Q1, Q2, Q3, Q4, Q5, Q6, Q7**. **Q8** only
partly: I read `assembly.rs`'s mint/copy_face section and `refactor.rs`'s
split prologue whole, but not all ~5k lines of `refactor.rs`.

- **S1 (Q1) — Three spellings of "which world placement holds this
  entity's copy", with different coverage.**
  - `assembly.rs:1556-1567` (`copies_body`): the body itself, or one
    `Part` pick.
  - `refactor.rs:2869` (`placing_of`/`holds`): also follows N1 `Whole`
    verbatim edges, so transforms count.
  - `refactor.rs:4493` (`selection_heir`): the name's `copy_of`, plus a
    "headed at the pattern" preference.
  - The split therefore treats a name as held by a placement of `T(B)`,
    where the gate says B has no world copy. Where else to look: the
    member walk's own `Part` handling. *Likely.*
- **S2 (Q1) — The persist mirror is a second rule, not a mirror.**
  `persist/check.rs:620` against `edit.rs:1273` is M2's root. The
  comment "mirrored in `persist::check`" in the PR body is the only
  thing tying them together. *Sure.*
- **S3 (Q7) — A node-kind special case in the generic evaluation
  path.** `eval/mod.rs:4210` checks `matches!(node, Node::Mate { .. })`
  in front of poison propagation. It is the only per-kind arm there, and
  it is what produces both m1 and m3. *Likely.*
- **S4 (Q7) — Six `unreachable!`s in `mint` and `copy_face`.** All rest
  on one cross-module invariant: "the mate evaluated, so its selection
  resolved to a Unique face of `head.at`'s table". That is two modules
  apart, held by convention rather than by a type; `usable` could hand
  back the resolved entities. I reached none of them. *Likely.*
- **S5 (Q3) — Test 22's unplaced row cannot see M3.**
  `intent_s2_f_mates.rs::a_mate_on_unplaced_operands_mints_nothing`
  uses a healthy mate only. The failing mode is a mate that fails, which
  the row's premise excludes. *Sure.*
- **S6 (Q3)** — m4's existential assertion. *Likely.*
- **S7 (Q7) — `RefusedRef` is a one-variant enum** wrapped in
  `MintRefusal::Reference { why }`. The wrapper now carries no choice.
  *Unsure* whether more arms are planned.
- **S8 (Q4) — ASSEMBLY.md's "Where a declaration is minted" states two
  rules that conflict on one document** (M3): "mints nothing" and
  "refuses `Unevaluated`". I think this is the code drifting from the
  intended invariant ("not an at-rest fact of the product"), not the doc
  rotting. *Likely.*
- **S9 (Q6)** — m5's interim with no schedule. m1's disclosed deviation
  has no schedule either. *Sure.*
- **S10 (Q2) — Long justification for a skipped check.** The split's
  `world_heir` skip is justified in one line ("A selection names the
  copy it reads, so it asks no such thing"), and that line is false for
  a posed copy (M1). *Sure.*
- **S11 — Comment lines left over-long after edits.** `edit.rs:437` ("(declared
  pairs, an instance's crossings, appearance keys). A name is scoped ...")
  and `msolve14_run_scalar.rs:1985` were not re-wrapped. *Sure*, trivial.
- **S12 (Q5) — The `mate.rs` module doc oversells the change.** It says
  "a mate depends on what it reads exactly as every other reader does",
  but mates are the one reader with a pre-poison fault override (S3).
  *Likely.*

---

About 300k context tokens; about 2 h 15 min wall time.
