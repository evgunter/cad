# Review R1 (Opus): INTENT stage 2 PR F, #4566 at `cfe1ca2115`

**Verdict: NOT-MERGEABLE-AS-IS.** MAJOR 2, MINOR 5, NOTE 4. There are also 11 style findings, which do not gate.

Both MAJORs are demonstrated by red probe rows in
`crates/editor-core/tests/r1_s2f_probes.rs` on this branch
(`review/s2-f-R1`). Both are local fixes. The rest of the PR held up
under execution. Poses are bit-equal against the base on 242 documents
at all three ε (claim 3). A9's coupling, the copy rule's Ambiguous and
unplaced arms, and the mate split paths behave as claimed.

Baseline: `fe6a5a30`, the main commit `cfe1ca2115` last merged
(`cfe1ca2115^2`). Probes ran on the branch, and P1 also ran on the
baseline, so each failure is attributed to the PR.

Lane isolation: I did not read PR comments, reviews, CI artifacts or
the other reviewer's branches. I did read the PR body. I also noticed
that the PR's head has moved to `a74be9fc` since the freeze; I reviewed
`cfe1ca2115` only.

---

## Correctness lane

### MAJOR-1: split now carries a selection read through a body no cut placement places, and moves its geometry. `inline(split(d)) ≠ d` (claim 5)

- **Where:** `crates/editor-core/src/refactor.rs`, the new crossing rule
  in `split` (the `let (body, crosses) = match doc.selection(var)` arm,
  `:2927`), with `placing_of` (`:2869`).
- **What it falsifies:** "a cut that succeeds while stranding a mate's
  selection" (the reader here is a measure), and "`inline(split(d))`
  must equal `d` wherever split accepts".
- **Mechanism.** A remainder selection now crosses when two things hold:
  its body's operation is cut, and each of its names is "held" by one
  cut placement (`placing_of`). `placing_of` walks from each cut
  placement's body down `Whole` edges and `Part` picks to the name's
  minting node. It never checks that the selection's body is on that
  walk. On main the rule was `cut_placed.contains(&select.body)`, so the
  selection's body had to be the placed body exactly.
  - Take a selection read in `t = Transform(x, +5z)` where only `x` is
    placed. Its name is minted at `x`, and `P(x)` "holds" it, so the
    selection crosses. It is then re-anchored onto the instance's copy
    of `x`, which is 5 units away from where the reader looked.
  - The reverse also holds by the same code: a selection on `x` with
    only `T(x)` placed is re-anchored onto the moved copy. This case is
    inferred, not run.
- **Demonstrated:**
  - **R1-P1** (`r1_p1_split_carries_a_selection_read_through_an_unplaced_transform`).
    A distance measure between `t`'s top cap and a kept cube `y` reads
    **5.0** before the split. Cut the closure of `t` plus `x`'s
    placement. The split is **accepted**, and the remainder's measure
    reads **0.0**. On the baseline `fe6a5a30`, the same row is refused
    `SeveredEdge { consumer: Measure, input: Transform }`, so this is a
    regression introduced by the PR.
  - **R1-P1b** (`r1_p1b_…round_trip…`). `inline(split(d))` differs from
    `d`. The measure's selection body moves from `t`'s output to `x`'s
    output.
  - **R1-P3.** The mate version of the same document is refused
    `MateFrameCrosses { side: B }`, so mates with face-relative frames
    are shielded by the frame check. Measures, fillets, chamfers, shells
    and face frames have no such check.
- **Suggested fix.** A selection should cross only when its body's
  operation lies on the chain `placing_of` walked, from the placement's
  body down to the name's node. Equivalently, its body is the placement's
  body, or a `Part` pick under it. Alternatively, keep main's
  `cut_placed.contains(&select.body)` and add the `Part`-pick extension
  explicitly. Pin both P1 rows.

### MAJOR-2: the edit door admits what `persist::save` refuses. `selection_body_kind` is not mirrored by `persist::check` (claim 9)

- **Where:** `crates/editor-core/src/edit.rs` `selection_body_kind`
  (about `:1273`, which checks only at mint time) against
  `crates/editor-core/src/persist/check.rs:618–629` (`only_mate_sides`).
- **What it falsifies:** "`persist::check` mirrors it exactly", and
  "check a selection shared by a mate side and another reader".
- **Mechanism.**
  - The edit door checks the body kind only when a selection is *minted*
    at a slot. A mate side may mint one on a pattern's `Bodies`.
  - When a second reader later reads that existing selection by var or
    by name, no body-kind check runs.
  - The persist walk instead derives the kind from the set of current
    readers. One non-side reader makes it expect `Body`, so it refuses.
- **Demonstrated: R1-P4** (`r1_p4_a_side_selection_on_a_pattern_shared_with_a_measure`).
  1. A mate's side `a` reads copy 2 of a pattern, so its body kind is
     `Bodies`. The mate-only document saves.
  2. Name the selection (`RenameVar`), then insert a `Distance` measure
     reading it by `Operand::Var`. The door **admits** it, and the
     measure evaluates with no node error.
  3. `persist::save` then refuses
     `Snapshot(SelectionBody { fault: Kind { found: Bodies } })`.

  So the editor can hold a document it cannot save.
- **Also inferred:** `only_mate_sides` is vacuously true for a selection
  nothing reads, so load admits an unread `Bodies` selection.
- **Suggested fix.** Choose one rule and give it one home:
  - either check the selection's body kind against every new reader slot
    at the edit door (`lower_operand` on an existing selection);
  - or let any reader of a face-of-a-copy read a `Bodies` body (the
    measure evaluated fine).

  Then have `persist::check` call that same function rather than
  re-deriving it.

### MINOR-1: `MintRefusal::Unevaluated` fires for a mate whose members are not placed, which the spec says mints nothing (claims 1, 2)

- **Where:** `crates/editor-core/src/assembly.rs`, `mint`. The `usable`
  check runs before `copy_face`, which is what answers "no placement
  reads it".
- **Demonstrated: R1-P2.** Two instances, a mate between them, and only
  their union is placed. While the mate is healthy, `assemble` returns
  `Ok`. Re-point side b at a ghost name:
  - `product_recorded` is still `Ok`;
  - `assemble` refuses `Mint { [Unevaluated { mate }] }`.
- **Why it is a problem.**
  - Spec §7 and test 22 say "a mate whose members are not both placed
    mints nothing … the boolean's coincidence door's business".
  - ASSEMBLY.md's new "Where a declaration is minted" paragraph says the
    same thing, and then says an unevaluated mate refuses.
  - So a broken workbench mate now blocks certification of a product it
    never spoke for.
- **Suggested fix.** Raise `Unevaluated` only when each side whose head
  still resolves has a world copy. That means asking the placement
  question first, falling back to raising when a head is gone. Or else
  state the over-approximation as the rule in ASSEMBLY.md and §7.

### MINOR-2: the severed-read refusal does not name the select or the side (claim 5; spec test 21)

- **Where:** `crates/editor-core/src/refactor.rs:575`. `SeveredEdge`
  carries `{consumer, input, consumer_is_cut}`.
- **What changed.** Spec test 21 reads "refuses the severed-read refusal
  naming the select". `OperandSeveredFromMate` named the side; its
  replacement names neither the side nor the selection variable. A mate
  whose two sides read one body gives no clue which side.
- **Demonstrated** by reading the test row
  `a_cut_severing_a_mates_read_refuses_as_a_severed_read`, which matches
  only consumer and input.
- **Suggested fix.** Add the slot (and the read) to `SeveredEdge`, as
  `UnresolvedRead { slot, var }` already carries them.

### MINOR-3: the msolve14 C5 relaxation is weaker than "a poisoned mate stands for the escalation" (claim 7)

- **Where:** `crates/editor-core/tests/msolve14_run_scalar.rs`,
  `assert_interval_structure`
  (`mate_log_escalated || mate_poisoned_through_moved`).
- **What it falsifies:** "the msolve14 C5 relaxation … hides a real
  escalation that went missing".
- **Mechanism.** The disjunct is satisfied by *any* mate poisoned
  through *any* moved node. A document where mate M1 is poisoned through
  an escalated member while M2's own escalation goes missing from M2's
  log would pass.
- **Partly mitigated.** The escalation is still asserted as the member's
  own `OffsetUnchecked` under an allowed predicate.
- **Inferred, not demonstrated:** no mutant was run.
- **Suggested fix.** Require that every escalation in the reference
  solve's log is either on an `Ok` mate's log or on a mate poisoned
  through the member whose fault carries that escalation, element by
  element, as `solve_decisions_have_one_home` does for the `Ok` mates.

### MINOR-4: stale and dangling prose describing the retired mechanisms as current; the PR body's sweep claim is inaccurate (claim 8)

- **Live code is clean.** No `src` path depends on `reading_edges`,
  `names::lift`, `MovedAbove`/`Vanished` or `OperandSeveredFromMate`, and
  the `.pyi`, tags and census agree with the Rust.
- **Test prose still describes the old mechanism as current:**
  - `tests/msolve5_operand_refusals.rs:1–33`: the module header still
    describes `RefusedRef::MovedAbove { at: T, by: P }` and `Vanished` as
    what the file measures.
  - `tests/rev_fix_xsplit_unreachable.rs:11`: an intra-doc link to the
    deleted `editor_core::reading_edges` ("re-derived here from the
    public `reading_edges`"); also `:147`, `:156`, `:213`, `:328`.
  - `tests/asm_r2a_mate_solve.rs:1137`: the assertion message says
    "the reading edges are recomputed from the name heads".
  - Also `fix_pattern_mate_crossing.rs:6, :211, :440`,
    `asm_r2b_assembly.rs:689, :974, :1019`, `mate1_member_vocab.rs:186`,
    `msolve1_transform_aware.rs:1068`, `dm7_delete_strands.rs:176`,
    `place_mate_frame_offset.rs:191` and `crates/pncad/tests/all.rs:3808`.
- **The PR body's sweep is wrong.** It says `reading_edges` remains
  only in "logs, specs, DR and AB logs, closed items and the binding
  census's historical note". `rev_fix_xsplit_unreachable.rs:11` contains
  the literal token.
- **Suggested fix.** Sweep the tests' prose with the same grep, and
  re-word the msolve5 header to the new verdicts.

### MINOR-5: a bridge miss in `copy_face` is silent (claim 2)

- **Where:** `crates/editor-core/src/assembly.rs`, at the end of
  `copy_face`: `Ok(copies(placement).and_then(|keys| keys.face(key)))`.
- **Mechanism.** A grafted copy whose bridge lacks the side's face key
  answers `Ok(None)`, the same as "no placement reads it". The mate
  then silently mints nothing. Elsewhere the product treats the bridge
  as total and refuses loudly (`ProductError::ContactLineage`).
- **Inferred, not demonstrated:** I found no document that reaches it.
- **Suggested fix.** Distinguish "the placement grafted nothing" from
  "it grafted, and the key is missing". Make the second `unreachable!`,
  or a typed refusal.

### NOTE-1: copy rule edge cases that behave as designed, recorded so the reconcile can confirm them (claim 1)

- A placement through a *different* `Part` node that picks the same copy
  as the selection's own `Part` body does not count as the side's copy.
  The mate mints nothing, although the product holds that geometry. This
  is by var identity, and consistent with "a placement of the body
  itself".
- `copies_body` judges the `Part` index at `var_env::<f64>()` (nominal),
  whatever the evaluation's lane.
- Placed twice refuses `Ambiguous { width: 2 }`, which is pinned by the
  PR's row.

### NOTE-2: the mate content key feeds side names with no per-side count (claim 6)

- **Where:** `eval/mod.rs`, the `Node::Mate` arm (`for side in [*a, *b]
  { for name in selected(..) }`).
- **Why it is safe today:** a side is a singleton `Face` selection
  (door, load and `head_of`).
- **Why the rest of claim 6 holds:**
  - Upstream keys are fed in field order (`operand_rows`), not in id
    order, so swapping which body each side reads re-keys.
  - The solve's role is in the key, and a mate's value *is* its role, so
    no stale role can be served.
- **The two expectation changes are right, not masks:**
  - The tour now expects the mates to re-key on a gauge swing. A mate's
    reads (instances) carry their gauge chain, so the mates re-key.
  - In msolve6 A5, a re-pinned part re-keys the mate, because the
    instance the mate reads re-keys.

### NOTE-3: `MintRefusal::Unevaluated.standing` is not exposed to Python

Python gets only `variant` and `str(refusal)` (`pncad.pyi` `MintRefusal`).

### NOTE-4: the PR's refusal-order change is disclosed

`TornGroup` and `PlacingMateLeft` are now checked before the closure
loop. As a result, a cut that both tears a group and severs a non-mate
read now refuses `TornGroup` where it used to refuse `SeveredEdge`. The
PR discloses this, and I agree it is the more informative refusal.

### Claims that held, with how

- **Claim 3 (poses unmoved): confirmed independently.**
  - I hooked `fixture::run`, env-gated and since reverted, to dump every
    instance's and mate's relative pose (`relative_map`, full `Debug`
    float repr), world placement, root position, role, fault presence
    and world/own space, keyed by document id and node position.
  - I ran 578 branch and 568 baseline tests matching
    `msolve|mate|asm_|p2_|intent_s2|fix_pattern|rev_fix|place_mate|edit_instance|dm7|switch_slots|part_depth|rv_dm7|refusal_concision|eval4`
    at ε unset, 1e-6 and 1e-12.
  - **There were 242 documents in common (241 at 1e-12), about 795 rows
    including about 554 posed-instance rows, and 0 differing documents
    at each ε.**
  - This is a different corpus from the PR's msolve14 probe.
  - The only non-probe failure was `asm_r2a row7e` on both trees. It is
    an artifact of my hook, which re-solves in the caller's frame.
- **Claim 4 (A9 over reads keeps the gauge edges): holds.**
  - Read: `relative_freedom_components` adds `upstream_of` edges plus
    `(instance, gauge)` and `(gauge, parent)` for live gauges.
  - The A9 and test-20 rows pass at all three ε.
  - The one new coupling is a mate with a dangling member walk, which
    still couples through its read. That is a read, so it is within the
    claim.
- **Claim 2, `in_space`: the filter itself loses nothing.** A head that
  resolves to no member is asked only in the world gather. A space with
  no placements gathers `EmptyProduct` and mints nothing, which is
  correct because nothing there is placed.

---

## Style lane (`docs/prompts/reviewer-style-lane.md`)

**Questions exercised:**
- Q1: prose grep and data grep over the touched files.
- Q2.
- Q3: the C5 row and the test-21 row.
- Q4: who cited `reading_edges` and A12.
- Q5: the ASSEMBLY.md paragraphs against `mint`.
- Q6.
- Q7.
- Q8: **only partly.** I read `assembly.rs`'s `mint` and `copy_face`
  region and the split/inline diff in full, but not the whole of
  `refactor.rs`.

**Findings:**

- **S1 (Q1, likely).** There are now three spellings of "which `Part`
  pick holds this pattern copy":
  - `assembly.rs` `copy_face`'s `copies_body`, which matches the pick
    index to `ent.body`;
  - `refactor.rs` `placing_of`/`holds`, which matches it to the name's
    leading `Instance { i }` segment;
  - `mate/member.rs` `check_reference`, whose `PartSelectsAnotherCopy`
    walks the chain.

  Each evaluates the index at `var_env::<f64>()`. The comment in
  `placing_of` already calls its match "positional and interim (#4341)".
  The gate's copy is a third, undisclosed one. Look also in
  `inline`'s `selection_heir` (`copy_of`).
- **S2 (Q2, sure).** `persist/check.rs`'s `only_mate_sides` is a
  hand-written mirror of `edit.rs` `selection_body_kind` with a
  different domain: per reader set rather than per authored slot.
  MAJOR-2 is that drift, already realised.
- **S3 (Q1, sure).** `tests/fixture/mod.rs:390` `mate_edges` is the
  deleted `reading_edges`' mate half, re-implemented in the fixture and
  called from 9 sites. `rev_fix_xsplit_unreachable.rs`'s header says its
  oracle is "re-derived from the public `reading_edges`", which no longer
  exists. The test-side copy is now the only home of a retired concept.
- **S4 (Q4, sure).** Stale prose in tests (MINOR-4's list). This is a
  class, not one instance: any test whose header narrates A12 or the
  lift.
- **S5 (Q2/Q7, likely).** `copy_face` re-resolves the side's name by
  looking it up in `head.at`'s table and `unreachable!`s on any
  non-unique answer. The doc says the face is "resolved ONCE, at the
  selection". It is resolved twice, with agreement asserted by panic.
  Passing the evaluation's resolved entity through would make that true.
- **S6 (Q7, likely).** `mint` has three `unreachable!`s that rest on
  "a mate that evaluated `Ok` has two live one-face heads". That is an
  invariant of `eval/wire.rs`'s `select`, held in another module.
- **S7 (Q3, likely).** The C5 disjunct (MINOR-3) cannot go red when one
  mate's escalation is lost while another mate is poisoned.
- **S8 (Q6, sure).** Spec test 21's "naming the select" was narrowed to
  `SeveredEdge {consumer, input}` without disclosure or a scheduled
  follow-up (MINOR-2).
- **S9 (Q5, likely).** ASSEMBLY.md A12's new text says "a cut that
  separates a mate from a body it reads is D-2's ordinary severed
  read", but the split now carries such reads when names are "held"
  (MAJOR-1). The doc states the intended invariant; the code drifted.
- **S10 (Q2, unsure).** The mate content key's names are fed with no
  per-side count (NOTE-2). It relies on an invariant nothing near the
  hasher states.
- **S11 (Q7, unsure).** `MintRefusal::Unevaluated` duplicates a fact the
  evaluation already reports: the mate's own `node_error`. Its
  `standing` field is opaque in Python. I would have expected the gate
  to cite the node error rather than re-raise it.

---

## Probes

`crates/editor-core/tests/r1_s2f_probes.rs`, registered in `tests/all.rs`.

| Row | Result on `cfe1ca2115` | Result on the baseline `fe6a5a30` |
|---|---|---|
| P1: split carries a selection through an unplaced transform | **RED** (5.0 → 0.0) | refused `SeveredEdge` (correct) |
| P1b: the round trip | **RED** (selection body moves) | n/a |
| P2: an unevaluated mate on unplaced operands at the gate | **RED** (`Unevaluated`) | not run |
| P3: a mate side through an unplaced transform | green (`MateFrameCrosses`) | not run |
| P4: a side selection on a pattern shared with a measure | **RED** (save refuses) | n/a (no `Bodies` side) |

Approximate context used: about 190k tokens. Wall time: about 1 h 45 min.
