# INTENT stage 2 PR E (#4523): review r1

Frozen head `17f07b62b669f897f13b02a7869145774985f1f2`, merge base
`5d7934ae45`. CONCURRENT arm, H tier, label r1 (Opus).

Lane isolation: I read no other `review/intent-s2-e-*` branch, no other
reviewer's scratchpad, and none of the PR's comments or reviews. I read
the PR description only (`pull_request_read get`). No glimpses.

## Verdict: APPROVE-WITH-FIXES

Seven of the eight claims hold under execution or inspection. The
exception is claim 5: split breaks the sharing of a **named**
selection. MAJOR-1 below shows it by execution. The fix is local to
`refactor.rs`'s two re-authoring loops (split's `crossing_reads` and
inline's `readers`). It needs no design change, which is why the verdict
is not NOT-MERGEABLE-AS-IS. It is still a real defect: a saved document
ends up with an orphaned, dead named variable, and readers that no longer
share what they shared before the split.

Counts: **1 MAJOR, 3 MINOR, 4 NIT**, plus style-lane findings.

## Findings

### MAJOR-1: split splits a shared named selection into anonymous copies and orphans the name (DEMONSTRATED BY EXECUTION)

`crates/editor-core/src/refactor.rs`, `split`'s loop over
`crossing_reads` (the `for &(node, slot, var) in &crossing_reads` block
that authors `Operand::select(Operand::Var(instance_body), …)`). By
inspection, `inline`'s heir loop over `readers` (the
`if let Some(heir) = heir(current.doc())` block) has the same shape.

Both loops re-author a remainder (or host) reader's selection **once per
(reader, slot)** as a fresh `Operand::Select`. The door mints one
anonymous variable per seat. Nothing consults whether the source
selection was a *named* variable read by several readers, which is spec
row 18's sharing. The cut-side carry (`vars.map` plus the `RenameVar`
after the insert) does handle that case. The crossing side does not.

**Scenario (probe `r1_split_shared_named_selection`):**

1. Prism P (placed) is the cut; prism Q is outside it.
2. Measure `m1 = distance(face of P, face of Q)`. Its first selection is
   renamed `s`.
3. Measure `m2 = distance(Operand::Name("s"), another face of Q)`. Now
   `m1` and `m2` read **one** variable `s`, and the PR's own
   `a_selection_authored_twice_is_two_variables_and_a_named_one_is_shared`
   pins that a rebind of `s` reaches both.
4. Split the cut. In the remainder:
   - `m1` reads `VarId #74` and `m2` reads `VarId #75`; `shared = false`.
   - `var_named("s")` still exists:
     `Select { body: VarId #29 (the cut extrude's output, gone from the remainder), names: [Face of Extrude #28 (now in the part)] }`,
     with **no readers**.
   - `remainder_maintenance` reports
     `StrandedSelection { var: s, readers: [], took: Node }`. That row
     says a name was stranded, but what actually happened is that the
     variable was detached from its readers.
   - `save(remainder)` is `Ok`: the orphaned, dead named selection is
     stored in the file.

**Consequences:**

- After the split, a `Rebind` (or a re-point of `s`) reaches neither
  reader.
- A third reader authored by name binds to a selection of a body that no
  longer exists.
- The user's name `s` now denotes garbage, and the readers they named it
  for carry anonymous copies.

This falsifies claim 5's "nothing is silently dropped", and breaks the
row-18 invariant across a refactor. The same shape almost certainly
applies to inline: a named selection of the instance's body read by two
host readers gets two anonymous heirs (by inspection; not executed,
because it needs a part store).

### MINOR-1: an insert whose authored selection names a node off its body is silent; the same selection through `SetParam` is reported, with a false sentence (DEMONSTRATED BY EXECUTION)

`edit.rs` `mint_selection` (it checks only that each name's node is
live) and `stranded_by_repoint` ("…or where the edit authored the
selection").

- Probe `r1_insert_selection_off_body`:
  `InsertNode(Node::fillet(Q, …, [edge of P]))` is accepted with
  `maintenance = []`.
- Probe `r1_setparam_selection_off_body`: writing the same
  `Operand::select(Q, [edge of P])` into an existing fillet's `selection`
  through `SetParam` is accepted with a `StrandedSelection { took: Reach }`
  row, which reads: *"this edit re-pointed a read, so Extrude …, which
  minted the name, is **no longer** upstream of it"*.
- The name was never upstream, so "no longer" is false. The two doors
  that author the same selection also disagree on whether it is worth a
  row.

Both evaluate to `SelectResolve`, so nothing is silent at evaluation.
The finding is that two doors give inconsistent and misleading reports.

### MINOR-2: comments still cite the measure-site premise VERSION 12 retired (inspection, sure)

The PR removes measure sites from the content key and from
`UpstreamRead`. Four comments still describe them:

- `eval/mod.rs` (`evaluate_node`'s "What the keys read of upstream"
  comment): "…then the sites a measure reads at". The `reads` vector
  below it has no such entries now.
- `eval/mod.rs` `UpstreamRead`'s doc: "the port read (`None` for a
  measure's site …)". Every `UpstreamRead` is now built as
  `(at, Some(port))`, so the `Option` is vestigial and the doc describes
  a case that no longer exists.
- `eval/mod.rs` `feed_declared`'s doc: "as a measure's reference feeds
  both … it is fed for the measure's reason". A measure reference no
  longer feeds a site.
- `measure.rs` `MeasurePrimitive`'s doc: "A `Node::Measure` holds one
  over `SitedRef`s". It holds one over reads (`S::Read`); `SitedRef` is
  authoring input only.

These are style-lane Q4's "doc rotted, code right" case. They sit in the
key code, where they will mislead the next person who bumps the key
format.

### MINOR-3: the SlotVarKind door is not pinned at the slot door (test gap; door itself DEMONSTRATED correct)

Claim 3 names the slot door, but `intent_s2_e_select.rs` pins only the
edit door (`each_primitive_admits_its_kinds_at_the_edit_door`) and the
load door. My probe `r1_slot_door_refuses_unadmitted_measure_kind` shows
the slot door refuses correctly:

- `SetParam` re-pointing `min_clearance`'s reference 0 to an authored
  edge selection refuses `SlotVarKind { found: Edge, expected: Measured(MinClearance) }`.
- The same refusal comes when the edge selection is an existing variable
  read by id.

No row in the PR would go red if `SetParam`'s path stopped passing
`slot.kind()` as `expected`. That path is the one route by which
`Measured::clearance_operand`'s `unreachable!` could become reachable.
I suggest pinning it beside the edit- and load-door rows.

### NIT-1: user-facing recourse names Rust constructors

`EditError::SelectionShape`'s Display says *"Recourse: build it through
`Node::fillet`, `Node::chamfer` or `Node::shell`…"* (probe
`r1_edit_door_noncanonical_authored_set`). A Python or viewer caller sees
Rust API names. (sure)

### NIT-2: "so … so" in the re-point strand sentence

The re-point strand sentence reads *"this edit re-pointed a read, so
Extrude …, which minted the name, is no longer upstream of it, so every
reader of e refuses …"* (`Took::Reach`'s `said` together with
`StrandedSelection`'s Display). (sure)

### NIT-3: `EditError::DeclareNamesMissingNode` reused for a selection

`mint_selection` refuses a name whose node is not live with
`DeclareNamesMissingNode`, Declare's refusal, although no declaration is
involved. (likely; it may be a deliberate "one refusal per fact".)

### NIT-4: history in pin headers

`m10_p_fence.rs` ("ALL THREE NUMBERS MOVED WHEN A SELECTION BECAME A
VARIABLE (INTENT stage 2 PR E) … Merged with stage 5 A's `relation` …")
and `lib_g16_corpus_name_digests.rs` ("Re-pinned for INTENT stage 2 PR
E") state history. Both files already keep such a register, and the
house convention for these pin files permits it. I flag it only because
claim 8 asked for no history. (sure that it is history; likely that it
is acceptable here.)

## Claims

1. **Holds (inspection plus a grep sweep).**
   - `select` (`eval/wire.rs`) is the only `ladder::resolve_in` caller.
     Its readers are fillet, chamfer and shell (`O::Selection`/`O::Open`),
     `Datum::FaceFrame` and `wire_measure`.
   - Each consumes `Selected.ents` and never re-reads a name:
     `wire_blend` filters `ents` to edge keys, `FaceFrame` reads
     `ents.first()`, and `clearance_operand` projects `key`.
   - The other `ladder::` users are the declared-pair door
     (`resolve_declarations` / the `SidedName` helper). Everything else
     that calls `.lookup(` sits in names emission, in diagnosis
     (`resolve/`), in appearance, in `clearance::FaceScope::Named` (API
     input), or in mates and crossings (PR F's and stage 4's).
2. **Holds (executed).**
   - The edit door refuses an out-of-order edge set (`NotCanonical`) and
     a repeated face (`Repeated`).
   - The load door refuses a saved fillet's two-name set swapped by hand:
     `SnapshotError::SelectionShape { fault: NotCanonical { at: 0 } }`.
   - `Rebind { body: Some }` keeps every reader's variable id (probes,
     plus the PR's row 17) and re-canonicalizes: rebinding `e0→e1` on
     `{e0, e1}` shrinks the set to one.
   - `Rebind { body: None }` of a name that only a selection holds
     refuses `RebindNoReferences`.
   - `body: Some(<the selection var itself>)` also refuses
     `RebindNoReferences`, which is reasonable, though the error does not
     say the address was wrong.
3. **Holds (executed).**
   - Edit door and load door: the PR's rows, re-run green.
   - Slot door: my probe (MINOR-3).
   - `clearance_operand`'s `unreachable!`: every route that writes a
     measure read goes through `check_read` → `Doc::read_fault` →
     `SlotKind::Measured(verb).admits`. That covers insert, `SetParam`,
     split/inline (`refactor::carry` re-inserts through the door) and
     load (log replay through the doors, plus the snapshot walk).
     `DefineVar`/`DeclareVar` take only scalar `VarDecl`s, and `Rebind`
     cannot change a selection's kind (`RebindKindMismatch`, and the
     kind is stored on the `Var`). I found no route.
4. **Holds (executed).**
   - Deleting a fillet's body reports `StrandedRead` (the fillet's body
     read) plus one `StrandedSelection` per selected name. That matches
     base's `StrandedRead` plus payload `Strand`s.
   - A named selection read by two measures reports one row with
     `readers: [m1, m2]`.
   - A re-point taking two names out of reach of a selection that two
     blends read (named `e`) reports exactly 2 rows, not 4.
   - Caveat: MINOR-1's insert-versus-`SetParam` asymmetry.
5. **Falsified for named selections** (MAJOR-1). The anonymous paths
   hold: `asm4_split_inline`'s cut-side refusal and the crossing rows are
   green, and anonymous crossing reads re-author correctly.
6. **Holds (executed, probe `r1_content_key_tracks_names_not_ids`).**
   - Two chamfers authored apart (two selection variables, two distance
     variables) on the same names have equal content keys. A chamfer on
     another edge differs.
   - After `Rebind e0→e1` the first chamfer's key moves and equals the
     third's.
   - Two measures authored apart on the same references have equal
     keys; a third on another face differs.
   - Note (pre-existing, not this PR): two **fillets** authored apart
     have *different* keys. This is because the radius is flow-bearing
     and `feed_scalar_join` feeds its expression through
     `param_source::feed_var`, which is SEAT-6's v4 design, not a
     selection effect.
7. **Holds (executed against the merge base).** In a worktree at
   `5d7934ae45`, `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked`
   and `lib_g16_corpus_name_digests` pass. The masked constant
   `(0x2b03_cbac_a56c_3ad0, 0x6637_f0f0_be0b_f72c)` is unchanged in the
   diff and passes at the head too, so the id-masked geometry is
   bit-identical across the PR. `lib_g16`'s diff moves exactly the seven
   named rows.
8. **Mostly holds.**
   - The grep for every retired identifier (`BlendSelection*`,
     `ShellOpen*`, `FaceFrameResolve/Kind`, `MeasureRefResolve`,
     `UnresolvedSite`, `MeasureSelectionKind`, `resolve_selection`,
     `resolve_open_faces`, `named_entity`, `measure_selection_kind`,
     `MeasuresWorldCopy`, `measure_sites`) is clean outside logs and
     specs.
   - The stub's `rebind(from_name, to_name, body=None)`, the five new
     tags and the measure-kind docs match the kernel.
   - Python maps a `StrandedSelection` with no readers to `node = None`,
     which is the case MAJOR-1 produces.
   - History: only NIT-4. Stale prose: MINOR-2.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q7. Q8 was only partly
exercised: I skimmed `node.rs` (5014 lines) around the constructors and
the slot walks, not end to end. Q6 was not exercised beyond the PR's
rulings.

- **S1 (Q1, likely): canonicalization has five homes, plus a door that
  refuses rather than normalizes.**
  - `Select::canonical` is called at `node.rs` (`edges_of`,
    `Node::shell`), `doc.rs` (`rewrite_selection_names`) and three times
    in `refactor.rs`.
  - Meanwhile `mint_selection` *refuses* a non-canonical authored list.
  - Every author therefore has to remember to canonicalize before the
    door. The Python and viewer authors do so through the constructors;
    a new author will meet `SelectionShape` instead.
  - Two policies (normalize at the builder, refuse at the door) for one
    stored form is the shape that drifts.
- **S2 (Q7, sure): inventing defaults on paths the code believes
  unreachable.**
  - `refactor.rs` has `doc.var(var).map_or(crate::VarKind::Faces, Var::kind)`
    three times; the selection is known to exist, so a missing var
    silently becomes a face set.
  - `edit.rs` `mint_selection` and `selection_into` have
    `names.first().map_or(EntityKind::Face, …)`, so an empty authoring
    is treated as faces.
  - Fail-loud would say `unreachable!` or refuse.
- **S3 (Q2/Q4, unsure): `Rebind`'s comment is stale.** The comment in
  `edit.rs`'s `DocEdit::Rebind` arm ("`Node::payload_names` is the list
  of the first and `Node::rebind_payload_names` its rewriting twin")
  still reads as if payload names covered the selection arm. The `Some`
  arm goes through `rewrite_selection_names` instead, and `payload_names`
  holds no selection.
- **S4 (Q3, likely): the PR's FORK-VTX door rows cannot detect a
  slot-door regression** (MINOR-3). The load row also re-spells only
  `Distance→MinClearance` for a vertex, which is one cell of the table's
  sixteen.
- **S5 (Q7, unsure): `UpstreamRead.port: Option<u8>` is now always
  `Some`.** An `Option` with one inhabited arm is an invariant held by
  convention where the type could hold it (related to MINOR-2).
- **S6 (Q1, unsure): a measure's body index is spelled three ways.**
  `wire_measure` builds `EntityRef { body: 0, key: Body }` by hand for a
  body read, `clearance::Selection::body_of` spells the same thing again
  (ruling 4), and a third convention covers selection reads.
- **S7 (Q5, likely): the `wire_measure` docs and the `select` docs
  disagree about Vertex.** `wire_measure`'s docs say "a `Face` or `Edge`
  read is a selection", and its `unreachable!("a face or edge selection
  holds one entity")` says the same, yet `distance` admits a `Vertex`
  selection that goes through the same arm.
- **S8 (Q7, unsure): `split` of a cut leaves a dead named selection in
  the remainder with no error** (MAJOR-1's residue). The save door
  accepts a named selection whose body is a dead output and whose names
  all live in another document. Whether the load walk should call that
  `SelectionBody` is a question for the fix pass.

## What I executed

Build: `CARGO_TARGET_DIR=/tmp/…/r1-target CARGO_INCREMENTAL=0`, outside
the worktree.

- `cargo nextest run -p editor-core --profile ci` at the frozen head:
  **2886/2886 passed** (173 skipped), 420 s.
- Probes in `crates/editor-core/tests/r1_probes.rs` (on this branch,
  aggregated into `all.rs`, **not for merge**), 10 rows, all executed:
  - `r1_slot_door_refuses_unadmitted_measure_kind`
  - `r1_delete_strand_rows`
  - `r1_split_shared_named_selection` (MAJOR-1)
  - `r1_load_door_noncanonical_edge_set`
  - `r1_rebind_shapes`
  - `r1_content_key_tracks_names_not_ids`
  - `r1_edit_door_noncanonical_authored_set`
  - `r1_repoint_strands_once_per_name`
  - `r1_insert_selection_off_body` (MINOR-1)
  - `r1_setparam_selection_off_body` (MINOR-1)
- Merge base `5d7934ae45` in a separate worktree:
  `m10_p_fence::*` and `lib_g16_corpus_name_digests::*`, 4/4 passed
  (claim 7).
- `crates/pncad-py/run-python-tests.sh` at the head: **957 tests OK** (330 s).
- `python3 scripts/work.py lint`: ok.
- Not run: viewer, tour, clippy, ε sweeps.

Tokens and wall-clock: not recorded by this lane (cloud session); about
1h15m wall-clock.
