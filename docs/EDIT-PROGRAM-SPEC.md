# EDIT-PROGRAM — a live profile's program is replaced whole, and every name its reshaping touches is reported or rebound (spec)

**Unit:** `a-committed-profile-program-has-no-whole-program-edit`
(kernel unit, v6 dual, block EDIT-B2 slot 2 — implementer FABLE per
the block's draw; pre-draw fields, logged here after the block byte
and disclosed as such: difficulty **L**, task-class **STRUCTURAL** — a
new persisted `DocEdit` variant, a step-provenance map read through
the replay record, a rebind-and-report door over every name carrier,
and two ratified sentences re-worded on Ev's ruling). **Ruled by Ev
on `[ev]` PR #2904** (2026-09-20, "(B) makes sense!"): the edit
carries the new program AND each new step's provenance; the door
reports a strand for every name on a dropped or changed step and
REWRITES every name on a kept step to its new index in place,
reporting the move as its own `Maintenance` arm. V2's "structure
changes only by re-authoring" (`crates/profile/README.md`) and DM7's
"reported at the delete" (`crates/editor-core/REFERENCES.md`) both
move; that ruling is their ratification, and the re-wordings land
with this unit. Deleted at merge and recorded in `docs/DOC-LEDGER.md`.

Branch `edit/program-edit`. Read first: the row (`## What is missing`,
`## What that costs the viewer, today`, `## Put to Ev`, `## RULED`);
`crates/editor-core/src/edit.rs` — `DocEdit` (~59; `SetMembers` ~107
and its apply arm ~2698 as the shape precedent, its rustdoc's "stated
in full" argument ~89), `InsertNode`'s arm (~2585: the checks a new
profile walks), `SetParam`'s arm and `check_profile_after_slot_edit`
(~3206), `set_slot`'s `check_param_refs`, `Rebind`'s arm (~2860: the
one-shot rewrite through `Node::rebind_payload_names` and the
appearance store), `Maintenance` (~1676) and `Applied.maintenance`'s
ORDER CONTRACT (~1950), `stranded_references` (~1878),
`apply_maintaining` (one arm per variant), `LoggedEdit`/`replay`
(~2435–2560, ~3306); `node.rs` — `SlotId::Profile`'s doc (~406, the
sentence this unit re-words), `payload_names` / `rebind_payload_names`
/ `payload_read_sites`, `name_free_node!`; `doc.rs`'s `name_carriers`
(~1097) and `Doc::appearance`; `names/role.rs` — `ProfileEdgeRef`,
`ProfileVertexRef`, every `RoleSeg` arm carrying one (~582–815);
`program.rs` — `ProfileProgram` (~440), `LoopProgram` (~380,
`authored_steps`, `step_args`), `ProfilePayload` (~2201),
`ProfileProgram::check` (resolve + replay + validate), `step_segments`
/ `checked_records` (DM8's map); `crates/profile/src/structure.rs` —
`ReplayStructure.steps: Vec<StepSpan>` (segment `k` leaves vertex
`k`); `eval/anchor.rs`'s `remap_edge` (~282: names hold PROGRAM
coordinates); `persist/mod.rs` (the unversioned format, `FileBody`)
and `persist/check.rs`'s `edit_non_finite` (~631, EXHAUSTIVE on
`DocEdit`); `crates/pncad-py/src/py/doc.rs`'s `insert_node` and
`set_members` bindings and `pncad.pyi`'s `DocEdit`; `REFERENCES.md`
DM6, DM7, DM8, §0's carrier list; `crates/profile/README.md` V2;
`crates/viewer/src/session.rs` `edit_profile` (~1935),
`sketch::program_edits`, `accepted_order` (~2358),
`forms::ShapeEdits::Locked`, `session/refuse.rs`'s
`ProfileRestructure` / `ProfileEditOrder` / `ProfileEditOrderCapped`
(READ ONLY — VIEW's follow-up); `docs/prompts/implementer-discipline.md`.

## The ruling, as premises (verify each against the tree before building on it)

1. **The edit.** `DocEdit::SetProgram { node, loops: Vec<LoopProgram>,
   provenance: Vec<LoopProvenance> }` replaces a live profile node's
   loops whole. The plane is NOT carried and does not move: it is the
   profile's one DAG input (`Node::Profile(p) => p.plane_input()`),
   and DM6 rules that no edit rewires a live node's inputs — the
   variant's doc cites DM6 for the exception and `SetMembers`'s "stated
   in full, so nothing is inferred about which of the old entries
   survived" for its shape. `LoopProvenance { from: Option<u32>,
   steps: Vec<Option<u32>> }`: `from` is the OLD loop index this loop
   continues (`None` = a new loop), `steps[i]` the old step index new
   step `i` continues (`None` = a new step) — the editor knows this
   (it inserted the leg), so the door is told rather than guessing.
   The provenance's shape is checked before anything else: one entry
   per new loop, one per new step, every `from`/step index inside the
   old program, no old loop or old step named twice; each fault its
   own typed `EditError` arm, named for what is wrong (say which arms
   you add and why not one).
2. **The new program walks the insert door's own checks — the same
   functions, not mirrors** (`SetMembers`'s sentence, ~2714):
   `check_param_refs` over every slot of every loop (the load-door
   payload-refs unit's rule: a `{Slot,Payload}×{UnknownDocParam,
   DocParamDimension}` refusal at both doors), then
   `ProfileProgram::check` under the current `ParamEnv<f64>` →
   `EditError::ProfileProgramRefused { node, refusal }` — the arm
   `SetParam` already uses. A `SetProgram` whose program is
   byte-identical to the current one and whose provenance is the
   identity is legal and records nothing beyond the edit (say whether
   the record is `structural: true` and why).
3. **Which segment each old step drew, and which each new step
   draws, is READ from the replay record, never re-derived** (DM8).
   `ProfileProgram::check` runs resolve + replay + validate and
   discards the structure; factor it so the door receives the
   `ReplayStructure` per loop (a `check_returning` or a split of
   `check` — one home, `check` calling it) for BOTH the current
   program and the new one, under the same env, at f64. Then, per new
   loop with `from = Some(l)` and per new step `i` with `steps[i] =
   Some(k)`: old step `k`'s span (`replay_old[l].steps[k]`) and new
   step `i`'s span (`replay_new[..].steps[i]`) map segment-for-segment
   in order iff the spans have EQUAL length — that step is KEPT; a
   step whose span length changed (a line that became `arc_fillet`, a
   fused verb whose fit flattened) is CHANGED and treated as dropped
   for every name on it; a step with `None` is NEW and holds no
   names. Names hold PROGRAM coordinates (`eval/anchor.rs`'s
   `remap_edge` publishes them so), so no canonical permutation
   enters the map — verify that premise against `remap_edge` and the
   published `ProfileEdgeRef`s before building on it, and if it is
   false, say what coordinate the names hold and map in that one.
   A `ProfileVertexRef` at vertex `v` maps as the segment LEAVING it
   (`StepSpan`'s own convention: segment `k` leaves vertex `k`); a
   `CapEnd` is untouched.
4. **The door reports and rebinds, at the door, out of the document
   it just produced** (DM7's method, `stranded_references`' walk):
   every name carrier the document holds — `Node::payload_names` over
   every node AND the appearance store (`doc.rs`'s `name_carriers`,
   REFERENCES.md §0's two carriers) — whose `StableName.node` is this
   profile and whose `RolePath` holds a `ProfileEdgeRef` /
   `ProfileVertexRef` of a DROPPED or CHANGED step, or of a loop with
   no `from`, is reported `Maintenance::Strand { node, name }` /
   `Maintenance::StrandedAppearance { name }` exactly as a delete
   would report it (DM7 widened: the subject is "the edit that removes
   a name's referent", of which the delete is one); every such name of
   a KEPT step is REWRITTEN in place to its new `(loop, segment)`
   through `Node::rebind_payload_names` and the appearance store's
   rebind path (the `Rebind` arm's one-shot rewrite, ~2876 — factor
   the rewrite into one function both arms call; the `Rebind` arm's
   collision refusal applies here too and is refused, not auto-picked)
   and reported as a new arm **`Maintenance::Rebound { from, to }`**
   (both `StableName`s), so a moved name is visible in the accepted
   edit and never silently re-denotes. Names of OTHER nodes'
   coordinates are untouched by construction (the walk filters on
   `name.node == node`). The rewrite is deterministic (the same edit
   replays to the same document — the replay-identity row pins it).
5. **`Applied.maintenance`'s order contract gains one clause and
   stays a contract**: strands (document node order, payload order),
   then stranded appearances (store key order), then **rebounds (the
   same two orders, node carriers first)**, then orphaned declares,
   then the cluster acts. The contract paragraph is re-written in
   full (it says so itself), and `dm7_delete_strands`'s order row
   extends to the new arm. `Maintenance::Rebound`'s `Display` names
   both names in words; the F6 census gains its row; the Python
   maintenance surface (`grep OrphanedDeclare crates/pncad-py`) gains
   the arm the same way the orphan unit added its arm.
6. **Persisted like every `DocEdit`** (GQ3, `crates/viewer/GUI-DESIGN.md`):
   serde-derived, externally tagged, `deny_unknown_fields`, no version
   — an old build refuses a file carrying `SetProgram` typed
   (`PersistError::Unreadable`, the regenerate recourse); the
   save-side `edit_non_finite` walk is exhaustive on `DocEdit` and
   gains its arm (loops hold `Expr`s — finite by the construction
   door; provenance is integers). `persist::save`'s verification
   replay and `load` accept it because `apply` does. The persist
   schema is PATHS's contended ground by keep-out: announce the exact
   lines (a new variant on an unversioned format, ruled by Ev on
   #2904).
7. **The Python door**: `DocEdit.set_program(node, loops, provenance)`
   in `crates/pncad-py/src/py/doc.rs` beside `set_members`, taking the
   profile description the insert door already takes (read how
   `insert_node` receives a `Profile` payload and reuse that value —
   never a second spelling) plus a provenance list of per-loop
   `(from | None, [step | None, ...])`; `pncad.pyi` and the tag map
   (`set_program`, `rebound`, the new refusal arms) follow; the
   census rows that walk `DocEdit`'s variants gain it.
8. **Two ratified sentences are re-worded, not re-decided — Ev
   decided them on #2904.** V2 (`crates/profile/README.md` ~70):
   "step indices are stable because structure changes only by
   re-authoring" becomes "structure changes only by `SetProgram`,
   which reports every name its reshaping strands and rebinds every
   name it moves", with the Record line naming `[ev]` #2904. DM7's
   title and first paragraph: the subject widens from `DeleteNode` to
   "the edit that removes a name's referent — `DeleteNode`, and
   `SetProgram` for the steps it drops or changes", the "why not
   as-is" bullet unchanged (it is the argument this unit inherits),
   the Record line naming #2904. `SlotId::Profile`'s doc (`node.rs`
   ~406) cites the new V2 sentence. State in the PR body where you
   looked for each sentence's ratification (`git log --all -S` with
   a short phrase, per CLAUDE.md).
9. **The viewer is NOT touched by this unit.** Its lock
   (`ShapeEdits::Locked`), `sketch::program_edits`, `accepted_order`
   and the three refusals become droppable the day this merges; that
   is VIEW's follow-up, and this unit FILES the row for it
   (`python3 scripts/work.py new … --program view`, listing every
   site the survey named: `session.rs` `edit_profile` ~1935/1962–2004,
   `sketch.rs` ~538–612, `forms.rs` ~235–253, `refuse.rs` ~266–348,
   `session/op.rs` ~448–465, `profile_edit.rs`'s order-search row)
   rather than crossing into it. `crates/viewer` compiles unchanged
   (`cargo check -p viewer --all-targets`).

## Rows (each red on `origin/main` first, then green; each named with the invariant it pins)

- **The row the finding names**: a committed square profile with a
  fillet on one of its walls; `SetProgram` inserts a leg BEFORE that
  wall's segment with provenance saying so → the fillet's
  `StableName` is rebound to the new index, `Applied.maintenance`
  carries exactly one `Rebound { from, to }`, and the evaluated
  fillet is on the SAME wall (measured on the solid). The same edit
  with provenance omitting that step → `Strand` for the fillet's
  name, no `Rebound`, and the fillet refuses `NodeGone` at the next
  evaluation (DM7's diagnosis).
- A step whose span length CHANGES (a `line` re-authored as
  `arc_fillet` with a `Radius` spec — two segments for one) with
  provenance claiming it kept → its names STRAND (a changed step is
  not a kept one), reported, not rebound.
- A dropped LOOP (`from: None` for every new loop, or fewer loops):
  every name on the dropped loop strands; a loop that moved index
  (`from: Some(1)` at position 0) rebinds every name on it.
- An appearance attachment keyed on a moved wall: `StrandedAppearance`
  when dropped, `Rebound` when kept, and the attachment reachable
  under the new key afterwards.
- Every provenance shape fault refuses typed BEFORE the program is
  checked (wrong length, index out of range, an old step named twice),
  each arm's Display pinned whole in the F6 census; the program
  faults refuse `ProfileProgramRefused` with the new program never
  written (the document is unchanged on refusal — pin by `bit_eq`).
- The plane cannot move: the variant has no plane field (a
  `compile_fail` twin on the struct literal, the EDIT-DECL pattern),
  and DM6's sentence stays true.
- `check_param_refs` at this door: a new program naming an undeclared
  parameter refuses the same arm the slot door refuses.
- The identity edit (same program, identity provenance) is accepted
  and reports no maintenance.
- Order contract: a `SetProgram` that strands one name, drops one
  appearance key and rebinds another reports them in the contract's
  order (`dm7_delete_strands`'s order row extended).
- Persistence: save/load/replay-identity over a document whose log
  holds a `SetProgram` (the corpus gains ONE such document, in the
  `tour` or a new fixture — the first persisted `SetProgram` in the
  tree, say which); the old-build refusal is a typed `Unreadable`
  (author the pre-unit file's bytes as a golden, the DECL pattern);
  the non-finite walk's exhaustiveness (E0004 if the arm is removed —
  a `compile_fail` twin or the walk's own row).
- Python: `set_program` round-trips a document through the binding
  and the `rebound` maintenance surfaces; the census rows walk the
  new variant and arms.
- DM8 unchanged: `step_segments` / `segment_radii` on the edited
  profile answer the NEW program's record (the map re-derives nothing
  and the names now agree with it).
- `perf2_name_keying_differential` and every name-table hash held for
  every corpus document (no document in the corpus uses `SetProgram`
  until this unit adds one — its hashes are new rows, not moved
  ones).

## Mutants (each named with the rows it reds)

- The door rebinds by STEP index instead of the span's segments (the
  fillet lands on the wrong wall).
- A changed step treated as kept (the changed-step row).
- Rebounds reported before strands (the order row).
- The appearance store not walked (the appearance row).
- Provenance shape unchecked (the shape-fault rows; a panic instead of
  a refusal).
- The rewrite non-deterministic (a `HashMap` iteration order in the
  walk — the replay-identity row).
- `check` replaced by the OLD program's structure for the new spans
  (the changed-step row and the finding's row).
- The plane carried and rewired (the `compile_fail` twin).

## Territory

`crates/editor-core/src/{edit.rs, node.rs, doc.rs, program.rs}`,
`REFERENCES.md` DM7 (EDIT; the DM7 and V2 re-wordings ruled on
#2904); `crates/profile/README.md` V2's sentence (PATHS's page — one
sentence, ruled; announce); `persist/check.rs`'s exhaustive walk and
the persisted form (PATHS contended — announce the lines);
`crates/editor-core/tests/*`, `crates/pncad-py/tests/*` (TCOST/TINT);
`crates/pncad-py/src/py/doc.rs`, `tags.rs`, `pncad.pyi` (LIB, by
announcement); `crates/editor-core/tests/corpus/*` (one new document);
NOT `crates/viewer` (a filed row instead). Kernel unit: the v6 dual on
a frozen head, then the union fix pass.

## Verification

Green hosted CI on the pushed head (twelve `test (…)`, five `k-lint`,
the python suite — step conclusions); `cargo test -p editor-core
--test all` and `--lib`; `cargo test -p pncad --test all`; `cargo
check -p viewer --all-targets` (unchanged crate, still compiling);
`cargo check -p pncad-py --features python --all-targets`; `cargo
clippy -p editor-core --all-targets -- -D warnings`; `cargo doc -p
editor-core --no-deps` no new warning; `python3 scripts/work.py lint`.
The PR body is the record: premises verified or corrected (on this
program the implementer who measured a premise first has been right
every time — premise 3's coordinate claim and premise 2's "same
functions" are the ones to measure first), the seams announced with
lines, the two ratified sentences with where their ratification was
looked for, the sweep (`re-authoring`, `Locked`, `program_edits`,
`accepted_order`, `one argument`, `ProfileRestructure` — the viewer
hits go on the filed row, not into this diff), the mutant table, the
CI run id, every path `python3 scripts/work.py territory` reports
outside EDIT's.
