# Review: INTENT stage 2 PR A (#4295), frozen head `e7609c813`

Base `427fbdcd6`. Correctness lane plus the style lane (`docs/prompts/reviewer-style-lane.md`).
I did not read the PR's comments.

## Verdict: APPROVE-WITH-FIXES

The representation step is real:
- Every operation defines its outputs at insert.
- The load walk holds a file's variable table to the nodes' signatures.
- The id-bijection check is a working instrument. I re-ran it against main, it passed, and it refused every mutant I gave it.
- Geometry did not move.

None of the findings is MAJOR. The fixes owed are listed below:
- tests that exercise the signatures themselves;
- the two re-frozen older-shaped rows, plus the persist-doc premise they protected;
- the silent name loss on inline;
- surfacing the `SubgroupFamily`-vs-A11 (1) question to Ev.

## What I ran (private `CARGO_TARGET_DIR`)

- **Full suite.** `cargo nextest run -p editor-core --profile default` (slow set) at the head: 2903 run, 2902 passed. The one failure is `name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time` (11.07 s against a 10 s bound). It fails identically on main `427fbdcd6` (10.57 s) and builds no document, so it is this machine's speed and not the PR. `pncad` + `pncad-py` Rust: 209/209.
- **The one-shot bijection check.** I ran it against a main worktree:
  - golden, die_tool, plate_param and gallery_ring were equal up to ids as committed on main.
  - The tour corpus failed as committed on main (`$.edits[0]…Frame.u[0]: key Ratio against Quantity`).
  - I regenerated it with main's own `demo-tour die-corpus`. Main's output differs from main's committed bytes at byte 1142, which confirms the stale-corpus finding. Compared against that regeneration, all five documents are equal up to ids.
  - The head's own `die-corpus` output is byte-equal to the head's committed tour file.
- **Probes** (scratch test module, not pushed): displays, measure-output reads, inline, comparator mutants and mint-log order (results inline below).
- **Six env-gated source mutants**, each run through the whole editor-core suite (`--profile ci`). The table is under C8.

## Findings

**MINOR-1. Signatures are mostly guarded by incidental digests, not by a signature row** (C1, C8). Execution; sure.
- `every_operation_defines_its_signature_at_insert` (`intent_s2_a_outputs.rs:60`) covers Frame, AxisInPlane, Profile, Extrude, Revolve, Split, Pattern and Transform.
- No row asserts these signatures: `Datum::Plane`/`Point`/`Axis`/`FaceFrame`, `Measure`, `Union`, `PlacedUnion`, `InstantiatePart`, `Loft`/`Sweep`/`Fillet`/`Chamfer`/`Shell`/`Part`, or the empty `Gauge`/`Mate`/`Assertion`.
- Mutants in `Node::outputs` (node.rs, `fn outputs`):
  - `Datum::Plane` → `Point` went red only on `perf2_name_keying_differential::…persisted_text_are_pinned`, a whole-text digest.
  - `InstantiatePart` defining nothing went red only on `msolve14_run_scalar::a3_…mate_corpus`, an id-feeding digest.
  - `Mate` defining a `body` went red only on that same `msolve14` digest.

  A legitimate re-bless of those pins would carry any of these mutants silently.
- The empty signatures (`Gauge`/`Mate`/`Assertion`) are the ruling's "defines nothing", and nothing names them.

**MINOR-2. `VarKind::symmetry` is untested and unused** (C7, C8). Execution; sure.
- `VarKind::symmetry` is at `var.rs:101`. Mutant: Plane → `Cylindrical`. The whole suite stayed green apart from the suite-aggregation row that my probe file tripped.
- No test, caller or Python surface references `symmetry()`.
- The mapping as written is right (my reading of D10 :1229 and A11 (1)):
  - Frame → trivial.
  - Plane → planar (3-dim: in-plane motion).
  - Axis → cylindrical (2-dim: slide and spin).
  - Point and Direction → none: SO(3) and the 4-dim translation-plus-spin group are not in the family.

  Nothing holds it there, though.

**MINOR-3. `SubgroupFamily` is a second type beside `Subgroup`, against A11 (1)'s "one `Subgroup` type"** (C7). Inspection; likely.
- ASSEMBLY.md :438 (ratified, Ev's addition on #4222) reads: "one `Subgroup` type is both a pose's symmetry and what a mate folds: `Subgroup::{Se3, …, Empty}`, which grows a `Point`'s and a `Direction`'s when a reader needs them".
- The PR adds a parallel `mate::SubgroupFamily` (`coset.rs:188`) with the same six variants, and `VarKind::symmetry` returns that, not `Subgroup`.
- One direction is fenced: adding a `Subgroup` variant fails `Subgroup::family` (`coset.rs:233`) until the family grows. The other is not: a `SubgroupFamily` variant can grow with no `Subgroup` twin, and when Point's subgroup arrives, both enums must grow by hand.
- The PR body calls this "one subgroup vocabulary". Whether that satisfies "one type" is Ev's call, so it should be asked as a question rather than settled in the body.

**MINOR-4. The two "older-shaped document loads" rows were inverted instead of re-frozen, so the additive-growth half is no longer measured** (C4). Inspection plus the full run; sure.
- Both `OLDER_SHAPED` docs instruct: "It is re-frozen, by today's writer, at each such break" (`bool13_r1_probes.rs:393`, `unreadable_by_this_build.rs:167`). Their whole purpose is the ADDITIVE half: a document lacking every later arm loads.
- The PR rewrote both into `OutputSignature::Missing` refusals instead. So no row now asserts that additive growth leaves an older document loadable.
- `persist/mod.rs:24` still states that premise ("An OLDER document lacking vocabulary … loads — additive growth invalidates nothing").
- On the question asked: refusing a pre-A snapshot is right. The module doc's own rule is "no version, no migration … regenerate", the recourse is `REGENERATE_RECOURSE`, and stage 1's breaks refused typed in the same way (`bool13r2_probes::real_historical_documents_refuse_typed…`). Load should not upgrade.
- What is owed:
  - re-freeze both exemplars with today's writer, so they carry output rows and still load;
  - keep one older-bytes refusal row, which the inverted rows can become.
- Also: the pre-A refusal lands on `SnapshotError` rather than the vocabulary door `PersistError::Unreadable` that the module doc names as the one door for "a file this build cannot read". It carries the right recourse, but it is a second door.

**MINOR-5. Inline silently drops a name on the instance's own output** (C2, the implementer's ruling on names). Execution; sure.
- Probe: a host instance whose `body` output is renamed `bracket`, then `inline` it. The result has `var_named("bracket") == None` and `maintenance == []`.
- Split carries a named output onto its node's new output (`refactor.rs:383`), and inline carries the part's named outputs. But the inlined instance's own named output is deleted with the instance, and nothing is reported.
- This is a name a user set, dropped without a word. DM7 reports strands, and VR2 lets a name sit on an output.
- So the PR's "named outputs keep their names through split and inline" is false for this case.

**MINOR-6. A slot reading a `Measure`'s output passes the door and the load, then refuses with a false recourse** (C6). Execution; sure.
- Probe: name a measure's `Length` output `gap`, then `SetParam` an extrude's distance to `gap`:
  - accepted at the door;
  - `save`→`load` is `Ok`;
  - `var_env` does not panic;
  - evaluation refuses: "variable #32:… has no binding in the evaluation environment — it was deleted, or never declared here; point the reader at a live variable" (`expr.rs:1765`).
- The variable is live and declared, so the recourse is false.
- The body discloses the gap, and D's `ConstructionReadsObserved` is the scheduled door. Until D lands, the refusal should not tell the user the variable was deleted.
- `Doc::var_scope`, which lists scalars only, still includes such outputs. That is consistent with the disclosed scope of A.

**NOTE-1. `SeedOnNonFreeVar`'s sentence is false for a reference-kind output** (C6). Execution; sure.
- On an extrude's body (`analysis.rs:665`) it reads: "…a variable that is not free, whose derivative is its inputs' pushforward rather than an axis of its own".
- A body has no derivative. The recourse "seed a free variable it is a function of" is acceptable.

**NOTE-2. The tour corpus's bijection check compares an edit log only** (C3). Execution; sure.
- `die_composed_tour.pncad` has an empty snapshot (0 nodes, 0 vars) and only `edits`, so its "equal up to ids" compares the log.
- Spec §2 asks for the replayed document. The comparator compares stored bodies, which matches the spec's intent for the other four files.

**NOTE-3. The comparator is a statement** (C3). Execution; sure.
- Beyond the PR's three mutants, I tried two more on `golden.cad`:
  - re-pointing an extrude's `distance` at another live length variable: refused ("…reads as 149:…, and earlier as 101:…");
  - renaming one variable: refused (`"zz_renamed" against "depth"`).
- Because the mint log is walked positionally before nodes and vars, the log fixes the map. A reading or a value cannot move undetected.
- Content keys: I did not re-dump them. I rely on the PR body's 330-node dump plus the unchanged ids-masked geometry fence.

**NOTE-4. Geometry is unmoved** (C4). Execution; sure. All three checks passed in my full run with their constants untouched by the diff:
- `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`;
- `asm2b_multisolid` row 5's volume bits and solid count (only `SINGLE_SOLID_NAMES_DIGEST` moved);
- `emit_union_rim_piece_ranks::KNOWN_ABSENT`'s counts (6, 6), with only the digests moved.

In every re-pinned row I read, the moved constant is a digest that feeds ids or names.

**NOTE-5. Minting and lifecycle hold** (C2). Execution; sure.
- Mint log at an extrude insert: `var 27` (its slot), `node 28`, `var 29` (its output). The output follows its node.
- The `gc` mutant (outputs swept as unread anonymous variables) turned many rows red.
- The `names` mutant (delete keeps an output's name) turned `an_insert_reports_its_outputs…` and `a_split_carries…` red.
- Save/load `bit_eq` is asserted at `intent_s2_a_outputs.rs:113`.
- Undo is "keep the prior doc", so it needs nothing.
- `SetProgram`/`SetMembers` return `outputs: Vec::new()` (inspection).

**NOTE-6. Python `Doc.output` returns `None` for a port outside the signature** (C6). Inspection; unsure.
- It returns `None` rather than refusing. A port outside `u8` (300 or −1) raises pyo3's `OverflowError`, which I did not run.
- The brief's "refuses a bad port typed" holds only for the wrong Python type. I did not build the Python extension.

## The implementer's own rulings

1. **Value-free `SubgroupFamily`:** disagree as a settlement; see MINOR-3. It is a fine shape if Ev accepts "one vocabulary" for "one type".
2. **`VarIsAnOutput` on every edit but rename:** agree. Executed for value, notation, annotation, delete and definition. Each display and recourse ("edit or delete Extrude …") is true.
3. **Kind faults retyped to `VarKind`:** agree. It is what lets "is a body, read here as a length" be said. The scalar tags are unchanged (`var_kind_tags_are_stable`).
4. **`Doc::var_scope` lists scalars only:** agree for A. See MINOR-6 for the scalar output that still enters it.
5. **Named outputs keep their names through split and inline:** half-true. Split yes (tested); inline drops the instance's own named output silently (MINOR-5).
6. **Python gets `Doc.output` and no `Var.kind`:** acceptable, but spec §2.7's `Var.kind` is now a disclosed narrowing with no schedule. Python cannot tell an output's kind, so it owes a unit or a filed issue.
7. **The tour re-bless closes the stale-corpus finding:** agree. Confirmed by execution (above).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7; Q8 for `var.rs` only.

- **S1 (Q1).** `VarKind::article` (`var.rs:123`) is a fifth article function, self-declared "`Dimension::article`'s twin". It sits beside `expr.rs:123`, `sentence::article` (`sentence.rs:85`), `names/role.rs:360` and `eval/mod.rs:1232`. Its scalar arms restate `Dimension::article`'s. — sure
- **S2 (Q1).** Two "same up to ids" comparators now exist. The new `tests/wire/up_to_ids.rs` builds its map from wire JSON; the old `tests/fixture/round_trip.rs::same_up_to_ids` (:242) takes a map over `Debug` renderings. Neither cites the other. — likely
- **S3 (Q1).** The viewer builds `EditError::VarIsAnOutput` by hand (`viewer/src/session.rs:2132`) instead of asking the kernel door. That is a third spelling, after `edit.rs:4904` `refuse_output` and the inline arm in `standing_var`. The two in `edit.rs` are themselves two spellings of one refusal. — likely
- **S4 (Q5).** `var.rs:5`'s module doc and the `VarId` doc still say an id is "minted by `DeclareVar`". `mint.rs` was updated to say outputs are minted by `InsertNode`, but `var.rs` was not. — sure
- **S5 (Q4).** `persist/mod.rs:24`'s additive-growth premise, and the `OLDER_SHAPED` docs' "so the load itself is asserted on every row", are now false at their sites (MINOR-4). The `bool13_r1_probes` doc above the inverted row still describes a load. — sure
- **S6 (Q7).** "An output's kind is its port's" is held by convention in three places: `Var::new` panics on an `Output` (`unreachable!`), `Held::of` does the same, and `VarDef::kind()` returns `Option`, so every caller unwraps. A type split (an output is not a `VarDef` arm) would carry the invariant. — likely
- **S7 (Q7).** A `Transform` output's kind is stored at insert and then re-derived from the live operand chain at load (`doc.rs:1536` `port_kind`). That is two homes for one fact. No door changes a transform's operand today, so they agree, but B's slot door will. — unsure
- **S8 (Q7).** `Doc::output` and `Doc::outputs` (`doc.rs:1492`, `:1502`) scan the whole variable table on each call, and `remove_unread` calls them per deleted node. With no index, a cascade delete is quadratic in table size. — unsure
- **S9 (Q6).** Spec §2.7's `Var.kind` was dropped (ruling 6). The narrowing is disclosed but unscheduled. — sure
- **S10 (Q3).** `the_up_to_ids_comparator_holds_a_document_and_refuses_each_mutant` tests the comparator only on `golden.cad`. That file has a snapshot, while the tour file the one-shot also accepts has edits only (NOTE-2). — unsure
- **S11 (Q2).** `first_output_fault`'s doc (`check.rs:503`) says a placer whose chain is not live is "the structural walk's". `Walk::ORDER` runs `OutputSignature` before that walk, so the claim leans on a later walk to cover a skip. I did not check which walk refuses it. — unsure
- **Q8.** I read `var.rs` end to end. I did not read `edit.rs` (≈7k lines) whole.

## Claims

- **Exercised by execution:** C1 (by mutation), C2, C3 (re-run against main, plus two new comparator mutants), C4 (one-shot, geometry pins, tour regeneration on both trees), C5 (each fault's row read, and its doctoring is distinct), C6 (Rust displays and recourses; pncad-py Rust census and tags 209/209), C7 (by mutation), C8 (six mutants).
- **Not exercised:**
  - the Python extension (`Doc.output` and the `.pyi`, via maturin);
  - the viewer suite;
  - a re-dump of content keys;
  - CI profiles at ε = 1e-6 and 1e-12.
