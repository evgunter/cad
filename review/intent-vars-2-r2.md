# Review r2 — PR #4011 (INTENT-VARS-1 PR 2, the variable table keyed by minted id)

Frozen head `42e2330ddf`; base `git merge-base 42e2330ddf origin/main` = `8f3b475c`.
Private `CARGO_TARGET_DIR`, release profile, everything run in the foreground. I did not
read any other `review/*` branch or the PR's comments.

## Verdict: NOT-MERGEABLE-AS-IS

The design holds wherever I could test it by execution. C2, C7 and C8 survive probes,
and C1 survives a structural, id-normalized diff of both re-blessed documents. **But
the frozen head has four red editor-core tests.** All four are in the nextest `ci`
profile's slow-set exclusion (`.config/nextest.toml`), so the PR gate never runs them;
the nightly default profile will. One is exactly the runtime-only
`DeclareVar`-should-have-been-`SetVarValue` catch the spec warns of (§5). The other
three come from one root cause, which the PR found and fixed in one fixture
(`75573598`) without sweeping the class. Each fix is small.

## Findings

### MAJOR

**M1. Four tests are red at the frozen head, all hidden from the PR gate.** (Bears on
C3, C6, C9. Shown by execution: the full `tests/all.rs` run gave
`2361 passed; 6 failed`. Two of the six are `msolve8_levered_clash::c4_band_*`, which
fail only from process-global tolerance sharing and pass with `--exact`. The four
below fail in isolation.) Confidence **sure**.
- `perf12_census_goldens.rs`, `heatsink_at`: `DocEdit::DeclareVar { name: "fins", … }`
  re-declares a name the corpus heat sink already holds, so it panics
  `VarNameTaken { name: "fins", holder: … }`. This is the mechanical
  `SetDocParam → DeclareVar` rewrite applied to a re-set site. **It is a surviving
  create-or-replace call site, and C3 is false for the test tree.** With the edit
  locally rewritten to `SetVarValue { var: "fins".into(), value: Count(fins) }`, the
  test passes and its three census goldens are unchanged, which is C1 evidence for
  that golden.
- `m10_3_driver_interval.rs`, `a_band_parameter_certifies_normally_and_prices_nothing`:
  `two_param_plate(Band…)` and `two_param_plate(uniform…)` differ only in the
  distribution inside the `DeclareVar` definition. A declare now mints from its
  definition, so every later node id differs, and
  `v.witness_vector().key() == u.witness_vector().key()` fails (the keys are `1522…`
  and `2373…`). The row's premise, "the same document with a uniform in place of the
  band", no longer holds.
- `m10_4_stackup_interval.rs`, `a_stale_or_foreign_verdict_is_refused_by_content`:
  the same mechanism. `plate(Some(uniform(half/2)), …)` is now a different document
  from `doc`, so `sensitivities(&doc, …, Some(&narrow_verdict))` refuses
  `ForeignVerdict` where the row expects "a certificate over itself".
- `m10_4_stackup_interval.rs`, `the_two_hole_plate_stackup` (≈ line 500): the
  goldening form now prints `variable <full id> feeds the section of node …`, but
  the assertion still expects `hole_r feeds the section of node …`. The expectation is
  stale; nothing ran it.

  The m10_6 neck fix (`75573598`, "declare unannotated, annotate by its own door")
  is the same class as the m10_3 and first m10_4 rows. It was fixed at one instance
  and not swept: that is a half-fix in the style lane's sense. Spec §5 asks for "the
  full editor-core suite before marking the PR ready", and the full suite includes
  the slow set.

### MINOR

**m1. Three load-door refusals can be deleted and the suite stays green.** (Bears on
C4 and C9. Shown by execution, with env-switched mutants in `persist/check.rs`
`first_var_fault`.) Disabling the kind check (`VarKind`), the mint check
(`VarNotMinted`) or the live-id check (`NameOnMissingVar`) each leaves the whole
editor-core suite green: 2246 tests plus the 115 that my exclusion pattern had also
caught. The PR covers these three only as display rows in `display_contract.rs`. My
probes show each one is reachable from a hand-edited file and correct:
- `w is stored as kind angle and its definition is of kind length`
- `v is not in the document's mint log`
- `the name q is attached to variable …, which is not live`

`VarNameTwice` does have an execution row. Mutants of the diff (`diff.rs`) and of
`DefineVar`'s kind check are killed (3 and 5 tests go red). Confidence **sure**.

**m2. The load door accepts an unnamed variable, which no PR 2 door can make, and the
lanes then disagree about it.** (Bears on C4 and C5. Shown by a probe: save `twins`,
delete `v`'s `var_names` row, then load.) The load returns `Ok`. `analyzed_box`
iterates `doc.vars()` and gets 2 axes, and `sample_offsets` gives 2 draws, so the
unreadable variable consumes an RNG draw. `stackup::continuous_params` lists it too.
But `Doc::param_env` and `param_env_over` iterate `doc.var_names()` and never bind
it, and `seed_env` refuses it `UnknownParam`. So two iteration bases for "the
document's continuous variables" coexist, and only a file can make them differ today.
Walk 6 (`AnonymousVarUnread`) is PR 3's. Either PR 2 should refuse an unnamed
variable at load, or the lanes should agree on one base. Confidence **likely**.

**m3. `split` declares the part's variables in name order, so the part's id assignment
depends on names.** (Bears on C2 and C5. Shown by inspection: `refactor.rs`,
`cut_refs: BTreeMap<VarName, RecipeNodeId>` (≈2581), iterated at ≈2710 to issue
`DeclareVar`s on the part's chain.) Two parents that differ only in names can give
part documents whose `VarId`s map to the parent's variables differently. In PR 2,
node ids already carry reader names, so this is no regression. But PR 3's reader
change will not fix this site, because it iterates a name-keyed map rather than
reading a name. Confidence **likely**.

### NOTE

**n1. C7 is an ordering artifact, settled by execution.** I swapped the symbol fed
through `param_env_over → axis_of` and re-ran
`sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`:

| Symbol | `registered` (without ladder / with) |
|---|---|
| the old name hash (`ParamSymbol::of` restored by hand) | 156/162 |
| `!id` (order reversed) | 156/162 |
| `id.rotate_left(32)` | 154/160 |
| `id` (as shipped) | 154/160 |

The bracket's reach flips between two values purely with the relative order of its
symbols. No class of identities is lost. **The symbolic tier's reach is not invariant
under symbol relabelling.** That is pre-existing, but it now follows minted-id bits,
so an unrelated earlier edit can move it. Nothing records it as an open property
(I grepped `geom-core/src/sym*` for any order claim). It deserves a `work/` issue.
Confidence **sure** for the measurement, **likely** that it should be filed.

**n2. C2 and C8 hold, by probe.** Two documents declare `(w, v)` and `(zz, aa)`: same
definitions, opposite name-vs-id order, and a measure of `a + 2b`. They agree on:
- the `VarId`s;
- the `analyzed_box` axes;
- 32 MC draws, each keyed by id;
- the stackup entries, keyed by id.

The measure's node ids differ (`9410…` vs `3843…`), which is expected for PR 2:
reader names are still in the `InsertNode` preimage, and PR 3 removes them. MC draws
are a pure function of id order and sample index. Confidence **sure**.

**n3. C1 holds for both re-blessed documents, checked structurally.** I parsed old and
new `golden.cad` and `plate_param.pncad`, mapped node ids to their `order` index,
variable ids to their names and step ids to first appearance. Nodes, roots,
witnesses, metadata, appearance, the edit log and the variable definitions are
equal. The only residue is one `Chamfer` selection (node 7 of `golden.cad`), which
is set-equal but re-sorted because it sorts by step id. The `lib_g16` and k-stats
rows keep their counts and move only their digests. The other three `.pncad` files
change only `params`→`vars`. Confidence **sure** for those files. I did not check
the tour's MC sheets (they are not in the diff).

**n4. C6: putting the definition in the mint preimage is per spec, and it has a cost.**
The spec erases only the display unit (§1). A probe shows that one `bit_eq` variable
gets different ids depending on how it was written: declared annotated, or declared
bare and then annotated through `SetVarDistribution`. That is coherent with VR1
(the id is a function of the edit sequence). But it makes initial values and
annotations part of identity: every later node id and every cached verdict moves
with them, which is what broke the M1 rows. When VR6 has typed values mint
variables, every typed number will thread into the chain. The ruling is worth
stating explicitly in VARIABLES-DESIGN, not discovering fixture by fixture.
Confidence **likely**.

**n5. `DeclareVar` checks for a collision before it validates the definition**
(`edit.rs` `DeclareVar` arm, ≈4801). So an invalid definition that also collides
reports `VarIdCollides`, and a definition refusal names a `SpokenVar` whose id was
never minted. The load-door order is "definition first". Confidence **unsure**.

**n6. A seed on an unnamed variable silently loses the pinned-section guard.**
`eval/mod.rs` builds `LaneEnv.seed` as
`opts.seed.and_then(|var| Some((var, doc.var_name(var)?)))`, so for an unnamed
variable the guard in `section_of` turns off. Today nothing breaks only because
`seed_env` refuses an unnamed seed first: an ordering dependency, reachable only
through m2's file. Confidence **unsure**.

## Style

Questions exercised: Q1, Q2, Q4, Q5, Q6, Q7, and Q8 partially. Q3 is answered on the
correctness lane (m1, M1).

- **Q1. Two iteration bases for one set.** `Doc::param_env` and
  `analysis::param_env_over` walk `var_names()`. `analyzed_box`,
  `stackup::continuous_params`, `range` and `persist::check::free_vars` walk
  `vars()`. These are one concept ("the free continuous variables") read two ways,
  and only m2 makes them differ. **sure** that the two shapes exist; **likely**
  that this is worth one home.
- **Q1. Four spellings of "a variable in a refusal".** `SpokenVar` (most arms),
  `VarRef` (`UnknownVar`, `NonFiniteSite::DocParam`), bare `VarId`
  (`NameOnMissingVar.var`, `VarNameTwice.a/b`), and a local `SiteVar` wrapper in
  `check.rs`. The prose differs to match. The probe output reads
  `w is stored as kind angle` (bare name, from `SpokenVar`) beside
  `variable w, …` (from `SiteVar`). **likely**.
- **Q1. Error names straddle two vocabularies.** `UnknownVar`, `VarNameTaken` and
  `VarKindFixed` sit beside `DocParamCountHasNoUnit`, `DocParamUnitMismatch`,
  `NonFiniteDocParam`, `ContinuousParamCannotBeCount` and
  `PayloadUnknownDocParam`. Spec §3 schedules the rename for PR 3, so this is
  scheduled. **sure**.
- **Q1, constants.** `test_utils::symbol_id` is another FNV-1a 64 copy (`0xcbf2…`,
  `0x…01b3`), beside `stackup.rs:885`, `eval/memo.rs:59`, `mesh/src/memo.rs:107`,
  `test-utils/src/fuzz.rs:242` and `topo/src/seqgen.rs:2037`. None of them states
  that it duplicates the others. **sure** it is a copy; it is a pre-existing class.
- **Q1/Q7. `Doc::spoken_declare` re-runs the mint to predict a refusal's id**
  (test-support only). It is a second statement of the door's draw order, which
  breaks quietly if the door ever validates first (n5). **unsure**.
- **Q4. Invalidated premise: `FreeVar::with_value`'s rustdoc.** At `doc.rs` ≈514 it
  still says "Changing a parameter's kind is a REDECLARATION — the create-or-replace
  door". There is no such door now; VR3 fixes the kind. The same stale door is cited
  at ≈567 (`with_display_unit`) and ≈625 (`with_distribution`). The code is right
  and the doc rotted. **sure**. Sweep the rest of that class too: "param table"
  across `persist/check.rs:146–204, 530, 551, 599`, `edit.rs:3628, 3829, 3857`,
  `persist/canon.rs:20` ("nodes, order, params"). **sure**.
- **Q5. `CarryForwardDoor::Definition`.** `DefineVar` replaces a whole definition;
  it carries nothing forward, yet the enum's name and doc still say "carry-forward",
  and `UnknownVar`'s sentence says `{door} has nothing to carry forward` for it.
  **likely**.
- **Q5. `standing_var` refuses `UnknownVar` for a variable that exists but is not
  free** (`edit.rs`, `standing_var`). It is unreachable with one `VarDef` arm, but
  the name will lie the day `Defined` lands. **unsure**.
- **Q2. A comment in a fixture holds an invariant that nothing enforces.** The m10_6
  `neck_with` comment ("declared unannotated … so ids do not differ per ε row") is
  the only guard on a rule that M1 shows the other fixtures broke. **likely**.
- **Q2. A long justification beside a re-baseline.** The paragraph added in
  `sym_9_retry_interval.rs` explains the 156→154 move in prose. n1 confirms the prose
  is true, but nothing guards the claim "an ordering artifact". **likely**.
- **Q6. Disclosed but unscheduled.** The reach-depends-on-symbol-order property (n1)
  is disclosed only in a test comment, with no `work/` item. The
  `viewer-param-exists-restates-var-name-taken` issue is properly filed. **likely**.
- **Q7. Leftover duplicate check.** The viewer's `create_param` pre-checks the name
  that `DeclareVar` now refuses. It is filed, so I only note that it is a second door
  for one fact. **sure**.
- **Q8. Whole-file reads.** I read `var.rs` and `intent_vars_2_table.rs` end to end,
  and every hunk of the `mint.rs`, `doc.rs`, `persist/check.rs`, `analysis.rs`,
  `diff.rs`, `mc.rs`, `stackup.rs` and `range.rs` diffs. I did **not** read
  `edit.rs` (5830 lines, the largest file touched) end to end, only its diff and
  the variable arms. That is a declared gap.

## Claims exercised

| Claim | How | Result |
|---|---|---|
| C1 | Structural id-normalized diff of `golden.cad` and `plate_param.pncad`; lib_g16 and k-stats count check; perf12 goldens re-run with a one-line fix | Holds. Tour MC sheets not checked |
| C2 | Name-swap probe (n2); inspection of `ParamSymbol`, `report_key`, `DocDiff` | Holds for every id-keyed lane; split's name order (m3) is a residual |
| C3 | Grep and inspection of editor-core, pncad, pncad-py and viewer; full suite | **False in the test tree** (M1, perf12); the product crates are clean |
| C4 | Probes for kind, unminted, dangling name and twice-held name; old-format row passes | Refusals reachable and correct; unguarded (m1); unnamed var accepted (m2) |
| C5 | Inspection of every lane | Id-keyed; two iteration bases (m2); split (m3) |
| C6 | Probe (n4); the M1 rows | The rewrite hid the class in three slow-set rows; coherent with VR1 |
| C7 | Symbol-order mutants (n1) | An ordering artifact; reach is order-dependent |
| C8 | 32-draw name-swap probe | Holds |
| C9 | Mutants: diff and DefineVar-kind killed; three load walks survive (m1) | Partly |

Not exercised: the `pncad-py` Python tests (no maturin build), the viewer suite,
`demos/tour` and `demos/wild` clippy, and the tour's rendered MC sheets. Of the slow
set outside editor-core, I ran only sweep's `sym11_far_placement_rows`, which passes.
