# Review r1 — PR #4011 (INTENT-VARS-1 PR 2: the variable table keyed by minted id)

Frozen head `42e2330ddf`, base `8f3b475c3c` (merge-base with `origin/main`).
Reviewer lane r1 (correctness + style). Private target dir, everything run in
the foreground. I read no other `review/*` branch, no other reviewer's output
and none of the PR's comments.

## Verdict: APPROVE-WITH-FIXES

I falsified none of the nine claims. Execution confirms the central ones:
- No pose moved (an id-free digest is identical at base and head).
- Names enter no id-keyed output (two documents that differ only in their
  names agree bit for bit).
- The sym_9 drop is a symbol-order artifact (restoring the name-hash symbols
  restores 156/162 exactly).

The fixes are MINOR: three new load refusals that no test can turn red, stale
"create-or-replace" prose, and a rename-invariance promise that a derived
`PartialEq` breaks. Nothing blocks the merge.

## Findings

**MINOR-1. Three of the four new load refusals have no test that can go red (C4, C9).**
`persist/check.rs` `first_var_fault`: `VarKind`, `VarNotMinted` and
`NameOnMissingVar`.
- *Reachable and correct.* My probe mutated a saved file three ways:
  - flipped `"kind"` → `Snapshot(VarKind{.., kind: Angle, def: Length})`;
  - re-keyed `w`'s id to one the log doesn't hold → `VarNotMinted`;
  - added a name on a dead id → `NameOnMissingVar`.
- *Unguarded.* I deleted each check and ran the 139 load, persistence and
  round-trip tests (`intent_vars_2 load_door corruptions round_trip r1_m10_1
  persist unreadable`). All three mutants **survive**.
- The only other place these variants appear is `display_contract.rs`, which
  constructs them by hand.
- By contrast, `VarNameTwice` (M6), the declare door's `VarNameTaken` (M7)
  and the `DefineVar` kind check (M8) are each caught.
- Demonstrated by execution. Confidence: **sure**.

**MINOR-2. Stale prose still describes a create-or-replace door (C3, style Q4).**
The code has no create-or-replace path left; only the prose does:
- `doc.rs:516`, `doc.rs:567`, `doc.rs:625`: the rustdoc of `FreeVar::with_value`,
  `with_display_unit` and `with_distribution` names "the create-or-replace door".
- `expr.rs:471` cites `write_doc_param` "at the create-or-replace door". That
  function is now `check_var_def`.
- `viewer/src/session.rs:1975`: `create_param` "narrows the edit's
  create-or-replace semantics". `DeclareVar` already refuses a taken name.
- `editor-core/tests/edit_doc_param_unit.rs:165,170`: `.expect("create-or-replace
  accepts it")` now wraps a `DefineVar`.

This is the "doc rotted, code right" case. Demonstrated by inspection
(grep). Confidence: **sure**.

**MINOR-3. The rename-invariance promise vs derived equality (C5).**
- `analysis.rs` `AnalyzedBox` documents "a rename moves none". It derives
  `PartialEq` over `spoken: BTreeMap<VarId, SpokenVar>`, and `SpokenVar`
  (whose `PartialEq` is derived) carries the name.
- `report.rs` `MassBasis::Forced { by: Vec<SpokenVar> }` is the same shape.
  §1 of the spec says `report.rs::by` is re-keyed by `VarId`.
- In PR 3, two boxes or bases taken across a `RenameVar` will compare unequal.
  Nothing in PR 2 can observe this, because there is no rename door yet.
- Demonstrated by inspection. Confidence: **likely**.

**NOTE-1. An unnamed, unread variable loads and becomes an analysis axis (C4).**
- I dropped `v`'s `var_names` row from a two-declare save. It loads, and
  `analyzed_box` then has 2 axes and `sample_offsets` 2 draws (probe).
- Load walk 6 (`AnonymousVarUnread`) is PR 3 per spec §3, so this is a
  scheduled gap, not a defect.
- One consequence while the gap is open: a phantom law consumes an rng draw
  in `VarId` order, so it shifts the draws of every later variable.
- No door can produce this state.
- Demonstrated by execution. Confidence: **sure** for the fact, **likely**
  for the consequence.

**NOTE-2. Ids now depend on a declare's value and distribution (C6).**
- `mint.rs` `MintingEdit::declare` hashes the definition with display units
  erased. So every node minted after a declare moves with the declared value,
  and with the annotation too.
- The `m10_6` neck fixture's rewrite (declare unannotated, then
  `SetVarDistribution`) exists only to stop the goldened keys varying per ε row.
- The rewrite hides no defect: the claim under test is unchanged.
- It is coherent with VR1/D9 and with the existing precedent that an insert
  hashes its literals.
- One consequence: `Doc::diff` of two separately authored documents that differ
  only in a declared nominal now reports every downstream node as
  removed/added, rather than as one `vars` entry. Two declares of different
  values used to be one `SetDocParam` that minted nothing.
- Worth knowing when PR 3 makes slots read ids. Demonstrated by inspection.
  Confidence: **likely**.

**NOTE-3. The symbolic tier's reach depends on symbol order (C7, measured).**
Method: I re-keyed the symbol in `param_env_over` and re-ran
`sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`. The
r2_filleted_bracket `registered` counts (without / with the ladder):

| Symbol keying | registered |
|---|---|
| old name hash (`ParamSymbol::of`, restored) | 156 / 162 (the old pins, exactly) |
| `!id` (reverses the order) | 156 / 162 |
| `rotate_left(17) ^ k` | 154 / 160 (the new pins) |
| raw `VarId` (head) | 154 / 160 |

- So the drop is purely a symbol-order artifact, and no class of identities
  was lost. The other four documents' rows were unchanged under every keying.
- The tier's reach is **not** invariant under relabelling, though: it swings
  by ±2 registrations with an arbitrary permutation of the ids.
- So these pins are hostage to the variable mint preimage, and any future
  change to what `DeclareVar` hashes will move them again. No issue tracks
  this order-sensitivity.
- Confidence: **sure** on the measurement, **unsure** on whether invariance
  is intended.

**NOTE-4. The declare door hashes before it validates.**
- `edit.rs` `DeclareVar` draws `mint.declare(def)` before `check_var_def`.
- `mint.rs` `canonical_bytes` justifies its `unreachable!` with "every field
  is … a finite float". That premise is false for the declare arm: a NaN
  definition is hashed first (serde_json writes `null`).
- My probe confirms the refusal is still typed (`NonFiniteDocParam`), with no
  panic.
- A secondary effect: `VarIdCollides` outranks a definition fault.
- Demonstrated by execution. Confidence: **sure**.

**NOTE-5. A silent drop in the seed's section check, latent until PR 3.**
- `eval/mod.rs` builds `LaneEnv::seed` as
  `opts.seed.and_then(|var| Some((var, doc.var_name(var)?)))`. A seed on an
  unnamed variable therefore skips `section_of`'s `SeedPinnedSection` guard
  without a word.
- It is harmless while readers read names, because an unnamed variable has
  no readers.
- PR 3 must not inherit it. Demonstrated by inspection. Confidence: **likely**.

### Claims confirmed by execution
- **C1, golden.** `golden.cad`, base vs head, with ids (≥12-digit integers and
  12–64-hex strings) normalized: the multiset of lines differs **only** in the
  table's shape (`params` → `vars` / `var_names` / `kind` / `def` / `Free`,
  plus 2 `var` mint entries). Every value line is identical.
- **C1, msolve14.** I wrote an id-free digest of the `msolve14` corpus and ran
  it in a base worktree and at head. Per node, indexed by document order, it
  takes role, fault, root and space (ids replaced by order index), the bits of
  the relative pose and the placement, and the f64 node error. The outputs are
  **byte-identical** (65 rows). The re-pinned `MAIN_CORPUS_DIGEST` therefore
  moved only by ids, as claimed.
- **C2 and C8.** I built one document with names `w, v` and one with `alpha,
  beta`, with equal definitions declared in the same order. The probe found:
  - The `VarId`s are equal.
  - The `analyzed_box` keys are equal.
  - `sample_offsets` agree bit for bit for draws 0..16, and repeat exactly
    (deterministic).
  - The stackup partials per `VarId` agree bit for bit.
  - The measure's node id **differs**. That is expected in PR 2, because the
    reader `ExprKind::Param` still carries the name into the insert preimage.
    It is not an id this PR keys.
- **C4, replay.** A log of `DeclareVar`, `SetVarValue`, `DeclareVar` saves,
  loads and replays to a `bit_eq` document. Appending a redeclare of a taken
  name refuses at save, with
  `EditReplay { index: 3, error: VarNameTaken {..} }`.
- **C9.** Of the eight mutants: the declare door's name check (M7), the
  `DefineVar` kind check (M8), the load walk's twice-held name (M6) and a mint
  that depends on the name (M9) are each caught by `intent_vars_2_table`. The
  `mint::tests` pin and collision rows cover the mint arm. M3–M5 survive
  (MINOR-1).

## Style

- **Q1: almost-parallel.** `Refusal::ParamExists`
  (`viewer/src/session/refuse.rs:298`) restates `EditError::VarNameTaken`.
  The PR files it (`work/intent/viewer-param-exists-restates-var-name-taken.md`),
  so it is scheduled. *sure*
- **Q1.** `Doc::spoken_declare` (`doc.rs`) is a second caller of
  `Mint::declare`, which re-draws the id to speak a refusal. Test-support
  only, but it is a parallel path that has to keep agreeing with the door's
  draw. *unsure*
- **Q1.** `standing_var` (`edit.rs`) answers `UnknownVar` both for "no such
  variable" and for "live but not free". One refusal for two facts:
  unreachable today, and real once `Defined` lands. *likely*
- **Q7.** `Var` stores `kind` beside a `def` that determines it, then
  re-checks the pairing at load (`kind_holds`, `SnapshotError::VarKind`). With
  one arm this is an invariant held by convention that a type, or a computed
  `kind()`, would make unbreakable. It is also the walk MINOR-1 found
  untested. *likely*
- **Q3.** In `analysis_is_keyed_by_var_id`, `assert_ne!(draws[&w], draws[&v])`
  cannot detect a swap of which id receives which draw: both laws are equal.
  The ∂/∂w=1, ∂/∂v=2 row is the one that can go red; the MC row cannot.
  *likely*
- **Q4.** `MAIN_CORPUS_DIGEST`'s rustdoc says it was "measured on main's own
  tree", and A3 claims to be "byte-identical to the pre-generic tree's". After
  the re-pin, the constant is the PR's own output, so A3 no longer anchors to
  an independent tree. My id-free digest shows nothing hid behind it this
  time, but the fence's premise changed silently. *likely*
- **Q5.** The `canonical_bytes` "finite float" premise (NOTE-4) and the
  `AnalyzedBox` "a rename moves none" promise (MINOR-3). Both are doc claims
  the code doesn't keep. *sure* / *likely*
- **Q2.** The `FreeValue` / `FreeVar` rustdoc in `doc.rs` still argues at
  length against a door that is gone (MINOR-2). It belongs to the same class
  as the stale prose there: sweep `doc.rs`'s whole `FreeVar` impl block, not
  only the three lines cited. *sure*
- **Q6.** The sym_9 rustdoc discloses the order-dependence as a re-baseline
  but schedules nothing (NOTE-3). The behaviour it measures stays unguarded.
  *likely*
- **Q8.** I read `var.rs` and `mint.rs` end to end. In `edit.rs` (5830 lines)
  I read only the variable doors, `check_var_def`, `write_free`,
  `standing_var` and `check_node_slots`, not the whole file. *n/a*

## Claims exercised / not exercised

| Claim | Status |
|---|---|
| C1 | Exercised: golden normalized-diff and msolve14 id-free pose digest, base vs head. The `m10_6` certifying-key move and the "two Monte-Carlo sheets" not individually re-derived. |
| C2 | Exercised: two-document probe. |
| C3 | Inspection plus grep across editor-core, pncad, pncad-py (`.rs`/`.py`/`.pyi`) and the viewer. No path emulates create-or-replace. |
| C4 | Exercised: probes for every new refusal, replay, a NaN declare and an unnamed variable. |
| C5 | Inspection of every lane's diff. No name-keyed map or hash survives in analysis, stackup, mc, drive, clearance, report, range or diff. The only remaining name→id hop is `seed_env` / `param_env_over` through `var_names`, as PR 2 intends. |
| C6 | Inspection. |
| C7 | Measured: three re-keyings. |
| C8 | Exercised: two-document probe. |
| C9 | Eight mutants. |
| Not run | pncad-py (maturin) tests, viewer tests, the demos clippy, the full editor-core suite. |
