# PR 4527 (FORK-DM4 unit 1) — review r2 (Opus, concurrent dual, unit size H)

- **Frozen head:** `3cbd7403ab`. **Base:** merge-base with `origin/main`, `cb4172aa9f`.
- **Lane isolation:** I read no other `review/dm4-unit1-*` branch, no
  other reviewer's scratchpad and none of the PR's comments. No glimpse
  to disclose.
- **Read:** `CLAUDE.md`; the PR body; `work/intent/a-union-member-is-keyed-by-its-read.md`;
  REFERENCES DM3, DM4, DM5 (`crates/editor-core/REFERENCES.md`);
  `docs/prompts/reviewer-style-lane.md` in full; the diff of
  `operand.rs`, `eval/wire.rs` (the boolean section, ~2860–3780, end to
  end), `names/emit_union.rs`, `names/role.rs`, `names/emit_topo.rs`'s
  read-keyed hunks, `refactor.rs`, `edit.rs`, `node.rs`'s fault arms,
  `coincide.rs`, `mate/member.rs`, `resolve/mod.rs`, `names/words.rs`,
  `eval/mod.rs`'s keys and tags, `viewer/src/combine.rs`,
  `viewer/src/session.rs`, `session/author.rs`, `pncad.pyi`, and the
  one-shot `tests/dm4_migration_one_shot.rs`.
- **Wall-clock:** about 1 h 30 min. Tokens: not instrumented in this
  session.

## Verdict: **APPROVE-WITH-FIXES**

1 MAJOR (demonstrated by execution), 3 MINOR (1 partly demonstrated, 2 by
inspection), 6 NOTE, plus the style lane (22 items).

The MAJOR is a real defect in the new indexed-read surface (a subtract
whose two seats read one family refuses with an emission bug where the
same geometry through two variables builds), but it fails loud, it is
local (one sided-by-read decision in the pair emitter), and the fix is
well specified. Everything else I could execute held: the CI set passes,
geometry did not move under the id-masked fence, the one-shot passes
against the base and catches a planted wrong seat, and a corpus-wide
probe found every one of ~76 800 carry segments keyed by a seat read of
its minter.

## Runs (executed)

All on `3cbd7403ab`, cloud session, 4 vCPU, `CARGO_INCREMENTAL=0`.

| Run | Result |
|---|---|
| `cargo nextest run -p editor-core --profile ci` | 2895/2896 pass. The one failure, `every_suite_file_is_aggregated`, was **my own** untracked probe file not yet listed in `all.rs` at the time; it is not the PR's. Includes `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` (pass, and its constant is untouched by the diff), `lib_g16_corpus_name_digests`, `m4_pr4_ci::diagnosis_corpus_is_golden`, `name_words_corpus`, `dm3_indexed_read::*`, `dm4_reads_key_booleans::*`. |
| `dm4_migration_one_shot` with `DM4_BASE_TREE` = the base's three stored files | pass, all three files. |
| one-shot with a planted `FromA`→`FromB` in the **base** copy of `die_composed_tour.pncad` | fails: `$.edits[31]…Fillet.selection[0].path[0]: read 29:…, but seat 1 of 255:… reads 254:…`. The map check can go red. |
| r2 probes (`review/dm4-unit1-r2/r2_dm4_probe.rs`, 10 rows) | see findings; 9 pass as written, one row is a measurement that prints. |
| r2 base probe (`review/dm4-unit1-r2/r2_base_probe.rs`, on a base worktree) | see NOTE N4. |
| mutants (`review/dm4-unit1-r2/mutate.py`, `mutants.sh`) | see the table at the end. |

Not executed: the viewer crate's tests, the pncad-py Rust tests, the
Python suite and `ty`, and the slow (non-`ci`) editor-core set. Claims 9
and 10 are therefore accepted **by inspection** only.

## MAJOR

### M1. A subtract whose two seats read one family refuses with an emission bug — the pair emitter sides incident edges by read (claims 2, 6)

**Demonstrated by execution** (`r2_a_subtract_of_two_members_of_one_family`).

A linear pattern `xs` of two unit cubes along `(1,1,1)` at spacing 0.5
(overlapping, no coplanar faces):

```
diag: xs[1] - xs[0]:           Naming(SeamVertexParentage { vertex: VertexKey(16v1) })
diag: xs[0] - xs[1]:           Naming(SeamVertexParentage { vertex: VertexKey(16v1) })
diag: xs[1] - cube:            builds, volume 0.640082, 45 names
diag: Part(xs,1) - Part(xs,0): builds, volume 0.640082, 45 names
diag: [xs[1], xs[0]]:          builds, volume 1.640082, 63 names
```

`Part(xs,1) − Part(xs,0)` is the same geometry through two variables
and builds; `xs[1] − xs[0]` (the spelling DM3 makes the new authoring
form) refuses. The cause: both seats are keyed by the one read `xs`
(DM4, correctly), and `emit_topo` decides which operand an incident
edge belongs to by comparing its `From` read against the two sides'
reads, A first:

- `crates/editor-core/src/names/emit_topo.rs:1974-1975` —
  `Some(RoleSeg::From { read, of: x }) if *read == a.read => a_edges…`,
  then `if *read == b.read => b_edges…`. With `a.read == b.read` every
  tool edge lands in `a_edges`, and the seam vertex's parentage match
  (`emit_topo.rs` ~2045–2078) falls into `([_], [], _, _) =>
  SeamVertexParentage`.

The read is the name's key; it is not a side, and the PR's own premise
says so ("which seat a read sits in is the node's to say, never the
name's", `role.rs`'s `RoleSeg::From` doc). The union fold escapes this
because its sides are the `FOLD_A`/`FOLD_B` sentinels; the subtract
does not. The PR's only row with two indexed seats on one family
(`dm3_indexed_read::an_indexed_read_at_a_body_seat_reads_that_member`)
uses **disjoint** instances, so no seam exists and the defect cannot
show.

Sweep owed (class, not instance): every place the subtract tells its two
seats apart by read rather than by position — `side_by_operand`
(`eval/wire.rs:3592`, which handles `(true, true)` by table lookup, so
it survives this case), `refusal_menu` (`wire.rs:4116`, whose finding
sites both sides at `xs`), and `coincide::RowInputs { a: (a_read, …),
b: Some((b_read, …)) }` (`wire.rs:3062`). The seam-vertex classifier
is the one I showed failing.

## MINOR

### m1. The load door no longer holds a carry's read to the document (claim 6; Q4)

**By inspection** (my execution probe was defeated: rewriting a stored
read also re-mints the replayed ids, so the file refused at edit replay
for an unrelated reason — recorded in the probe output, not counted).

At the base, `derivation_nodes` included `FromMember`'s member id
(`member_edge`), and the load door checked every id it returns against
the mint log (`persist/check.rs:1918`). At the head
`derivation_nodes` (`resolve/mod.rs:2330`) no longer yields the carried
id, and the read is returned by a new `derivation_reads`
(`resolve/mod.rs:2344`) that **no caller in the workspace uses**
(re-exported from `lib.rs` only). So a stored name's `From { read }`
is checked by nothing at load, while `role.rs`'s `read_edge` doc still
promises it is "held to the document's variables when a file is read".
The same loss reaches `refactor.rs`'s `derivation_nodes(..).is_subset(cut)`
checks (`refactor.rs:3052, 3251, 3321, 3345, 3629, 4178`): the member's
operation is no longer in the set, and the miss is caught later and
less specifically by the `ReadMap` (`Unmapped::Read` → `NameStraddlesCut
{ missing: None }`).

### m2. The pairwise judgement's log is dropped whole wherever the node admits — not only where the fold re-decides it (claim 5)

**By inspection**, with the code's own premise contradicted by DM4's text.

`eval/wire.rs:3218-3266`: the judgement runs under `k_stats::detached`
and is spliced only when it refuses or finds an intersection empty. The
comment justifies dropping it on admission because "the fold re-decides
every contact the body rests on". For two members that is true (fold
step 0 is the judged pair). For three or more it is not: DM4 says a
contact "a third member covers … is still a contact of its pair and
needs the declaration", and such a pair is certified **only** by the
judgement — the fold never meets it (it is "satisfied, not refused").
The admission rests on that certification, and its verdicts are not on
the node's log, so the ε audit (`SetTolerance`) and the verdict diff
cannot see a flip there. The `probe`-feature K samples of every
admitted judgement are dropped too (`detached` swaps the sink, and an
unspliced run reaches no sink), where at the base they were recorded.
`splice_superseded` (keep verdicts and samples, drop escalations) is the
door the k_stats docs name for "an outcome the op went on to overrule";
the choice here drops all three channels.

My execution attempt at a covered-contact three-member union hit a
**pre-existing** emission refusal (N4) before reaching the log, so this
stays by inspection.

### m3. The one-shot cannot check a declaration, and none of the three files has one (claim 7)

**Demonstrated by execution for what it covers; the gap by inspection.**

The walk catches a wrong seat (planted swap above, red) and a
mis-mapped node (the bijection). But all three regenerated files hold
`"declare": []` only (counted: 1/2/1 empty lists, 0 non-empty), so
the declared-pair map — a site that was a **node id** becoming a
**read** — was never exercised; and as written
(`dm4_migration_one_shot.rs:231, 243`, `self.value(&old["declare"],
&new["declare"], …)`) it would pair the old site's node id with the
new site's var id in the one bijection that already pairs node ids with
node ids, so a non-empty declaration would refuse spuriously or
mis-pair. "It would catch a dropped declaration" holds only as an
array-length check.

Two survivors (M6, M7) are test gaps on claims 3 and 6: no row pins the
same-read declared-pair skip in the pairwise judgement, and no row lifts
a name through a subtract's seat, so a wrong read there (the shape of
M1's defect) would ship green. Recorded as NOTE N6.

## NOTE

- **N6. Mutant survivors (test gap, demonstrated by execution).** M6 and
  M7 above survive their targeted batteries.

- **N1. `IndexRank` refuses any rank but one, while DM3 writes `xs[i, j]`.**
  `node.rs:1693-1702, 3704-3716` refuse `at.len() > 1` regardless of the
  family; the fault's doc says "other than its family's rank: every
  family is indexed by one `Count` (REFERENCES DM3)", which misquotes
  DM3 ("or `xs[i, j]` for a family keyed by two indices"). A pattern of
  patterns is read by one flat index (`j·M + i`, `instance_of`,
  `wire.rs:2828`), and evaluation reads only `SlotId::Index { seat, k: 0 }`
  (`wire.rs:2918`). A narrowing of the ratified text, not disclosed in
  the PR body as one, and not scheduled. Verified the refusal by
  execution (`r2_the_load_door_refuses_bad_indices`: "the read at its
  from carries 2 indices — a family is indexed by one count").
- **N2. Doors (claim 2), demonstrated:** a negative index refuses
  `InstanceOutOfRange { index: -1, count: 3 }`; no `Index` slot exists
  at a family argument (`UnknownSlot`); `SetMembers` refuses an indexed
  family argument (`IndexedRead { IndexedFamily }`) and a family in a
  member place (`SlotVarKind { found: Bodies }`); the load door refuses a
  rank-two index, an indexed body, and a junk key in the `{read, at}`
  form. All as claimed.
- **N3. DM5 (claim 3), demonstrated:** `[xs[0], xs[0]]` and
  `Intersect([xs[1], xs[1]])` build volume 1, `xs[2] − xs[2]` is the
  typed empty body; a declared pair sited at a read spelled twice builds
  in all three orders `[a,a,b]`, `[a,b,a]`, `[b,a,a]` (volume 2), and
  undeclared refuses `UndeclaredCoincidence`. That pinned row of the
  work item ("a declared pair sited at a read spelled twice sites both
  members") has **no test in the PR**; my probe is the only execution
  of it. The unglued-cell `DuplicateName` refusal is pinned at the
  collapse by a unit row only (`emit_union.rs`
  `an_unglued_twin_refuses_as_a_duplicate_name`), never end to end.
- **N4. Pre-existing:** a three-member union `[a, b, c]` with `b`
  covering the `a|c` contact (all pairs declared) refuses
  `Naming(Emission { "a piece of a face held as several borders no
  recorded discard between them" })` at the head **and identically at
  the base** (`r2_base_probe.rs`). Not this PR's; I found no work item
  naming it (not exhaustively searched).
- **N5. Claim 1 holds by execution and by the stored bytes:** `die_tool`
  re-saves to its own bytes (`a_document_of_plain_reads_saves_to_its_own_bytes`);
  in `plate_param.pncad` the mint log is identical through entry 65
  (every node before the first boolean), so no plain node id moved.

## Claims, one by one

1. **No plain document moves** — holds (N5; mutant M11 below).
2. **Union(xs) ≡ Union([xs…]); indexed read; doors** — the doors and
   the naming identity hold (N2, `dm3_indexed_read` green); **"an
   indexed read evaluates to the right instance" fails at a subtract of
   two overlapping members of one family** (MAJOR M1).
3. **A repeated read glues** — holds (N3); end-to-end `DuplicateName`
   pin absent.
4. **Geometry did not move** — holds: the masked fence passes and its
   constant is untouched; the f64/Interval constants moved by ids, as
   the header says.
5. **The verdict log** — the two-member case holds; the N ≥ 3 case
   drops decisions the outcome rests on (MINOR m2).
6. **The read key is consistent everywhere** — on the corpus, yes:
   `r2_every_carry_is_keyed_by_a_seat_read_of_its_minter` walked every
   published table of all 29 corpus documents, 76 792 carry segments,
   each keyed by one of its minting node's seat reads; no fold sentinel
   reached a table. Off the corpus, the read is mistaken for a side
   (MAJOR M1) and is no longer held at load (MINOR m1).
7. **The one-shot** — holds for seats and nodes (planted swap red);
   declarations unexercised and mis-mapped as written (MINOR m3).
8. **Words** — accepted by inspection; the ratchet rises are disclosed
   with a reason at the constant (style S14 on their size).
9. **Viewer** — by inspection: `combine_in_world` dedups operands and
   re-points the first placed, deletes the rest, in one staged run
   (`viewer/src/session.rs` `combine_in_world`); not executed.
10. **Python** — by inspection: the stub carries `union`, `intersect`,
    `subtract(from_, tool)`, `IndexedRead`; no retired name survives in
    `pncad.pyi`; `BooleanOp` remains only as the census's
    `SHAPE` row with its reason. Not executed (no `ty` run).
11. **Content tags** — `From` 52, `Intersect` 38 present
    (`eval/mod.rs` `seg_content_tag`, node tags); the retired
    `16/17/41/28` are gone from the match. Injectivity accepted on the
    PR's own `*_tags_are_injective` rows, which passed in the CI set.

## Mutants

| Mutant | What it breaks | Expected red | Result |
|---|---|---|---|
| M1 | judgement log always spliced (`wire.rs:3262`) | m4_pr4 `PredicateFlip` row | **red**: `m4_pr4_ci::diagnosis_corpus_is_golden` (the PR's fix is pinned) |
| M3 | `member_of` ignores which member holds the name (`emit_union.rs`) | family-read union rows | **red**: `dm3_indexed_read::a_union_of_a_family_and_of_its_members_spelled_name_and_build_alike`, `set_members_writes_indexed_members` |
| M6 | a same-read declared pair is judged as a cross pair (drop `if r1.at == r2.at { continue; }`, `wire.rs`) | DM5 declared-pair rows | **survives** (177 rows incl. my `r2_a_declared_pair_at_a_read_spelled_twice`): the skip is unpinned, possibly equivalent on every fixture |
| M7 | `lift` keys a subtract's `from` survivor by the tool's read (`role.rs`) | mate/refactor rows reading `lift` | **survives** (481 rows over mate/asm/lift/inline/split/refactor): no row lifts through a subtract seat |
| M9 | the rewriters leave a carry's read unmapped (`refactor.rs` `Remapping::read`) | split/inline/remap rows | **red**: 6 rows (`remap_reorders_ids::*`, `asm4_split_inline::inline_name_refusals_fire_typed_and_name_their_subjects`, two `refactor` unit rows) |
| M11 | a plain read serializes in the struct form (`operand.rs`) | byte/golden rows | **red**: 17 rows incl. `a_document_of_plain_reads_saves_to_its_own_bytes`, `m4_pr6_golden::golden_bytes_are_frozen` |

## Style

Questions exercised: Q1 (prose sweep and a constants/spelling sweep over
the touched src), Q2 (justification-length read of `wire_combine`, the
`detached` comment, `member_of`), Q3 (on the new rows), Q4 (who cited
`FromMember`/`FromTarget`/`member_edge`/`derivation_nodes`), Q5 (module
docs of `emit_topo.rs`, `mate/member.rs`, `merged.rs`), Q6 (disclosed
deviations), Q7, Q8 (read `wire.rs`'s boolean section end to end, not
the whole 5000-line file — **not** the whole largest file).

- **S1** (Q4, sure) — stale `FromA`/`FromB`/`FromMember`/`FromTarget`
  prose survives in shipped source: `names/emit_topo.rs:4, 1267, 1811,
  3986-3987, 4160`; `mate/member.rs:36, 345, 356, 374`;
  `names/merged.rs:438-439, 491`; `pncad/src/select.rs:64`;
  `viewer/src/marks.rs:673`; `viewer/src/session/select.rs:51`.
  `mate/member.rs`'s module docs still describe the union walk as
  "qualifies `FromMember`". A sweep the PR body does not claim, but the
  class is "a retired segment named in prose"; also look in `docs/`
  (PERF-2-SPEC, MODEL-AB-LOG) if those are live.
- **S2** (Q4, sure) — `derivation_nodes`' doc (`resolve/mod.rs:2321-2328`)
  still says it includes "every node id a SEGMENT carries in its own
  right … the localization set of N7", and `derivation_reads` was
  written to restore what it dropped but has no caller (m1). The doc
  rotted and the code drifted; I think it is the second kind (an
  intended invariant lost).
- **S3** (Q1, sure) — one seat, four spellings: the field `tool`, the
  enum `OperandSlot::Cut`, its label `"tool"` (`operand.rs:133`, the
  same label `OperandSlot::Tool` speaks for a split's plane), and the
  Python slot word `"cut"` (`pncad-py/src/slot_word.rs:101`). Two
  `OperandSlot`s now render as "tool".
- **S4** (Q2, likely) — `remap_declared` (`refactor.rs`) speaks a
  union's site miss as the subtract's `from` seat ("A site is not an
  operand slot; it is read as the subtract's `from` is for the miss's
  sentence"): the comment admits the sentence is wrong for a union.
- **S5** (Q1, likely) — `wire_combine`'s family arm re-implements the
  projection `reads_projected` does for an indexed read (`project(i)`,
  `check_total`, then a gate), but gates with `T::gate_at_rest_kept`
  where the spelled arm goes through `finished_operand`. "Union(xs) and
  Union([xs…]) build alike" therefore rests on two code paths agreeing;
  one test pins it.
- **S6** (Q7, likely) — `wire_combine` opens with `let members = bodies;
  use crate::OperandSlot as O;` inside a block whose result is then
  rebound twice (`let members = members?; let members =
  members.as_slice();`). Reads as a rename left over from a refactor.
- **S7** (Q2, likely) — the `detached` comment (`wire.rs:3218-3224`)
  is a justification longer than the four lines it governs and states
  something false for N ≥ 3 (m2).
- **S8** (Q3, sure) — `words.rs`
  `by_tag_each_carry_says_its_read_and_the_feature_is_said` asserts
  "through read 000000000000 … through read 000000000000" for two
  **different** reads (`VarId::new(1, 10)` and `VarId::new(1, 77)`): the
  tag prints the digest head, so the row cannot fail if the words say
  the wrong read. The fixtures elsewhere in the file share the shape.
- **S9** (Q3, sure) — `dm4_reads_key_booleans::a_three_member_intersect_with_coincident_faces_folds`
  (the work item's named "review check") asserts volume only; the names
  the intersect lane publishes under a fold are not looked at.
- **S10** (Q3, likely) — `an_indexed_read_at_a_body_seat_reads_that_member`
  uses disjoint instances, a premise that excludes the seam case M1
  fails on.
- **S11** (Q4, sure) — `intent_s2_b_reads::a_pair_declared_across_one_splits_halves_is_sided_by_table`
  keeps its name and its doc ("Both halves are read at the split's one
  site", "the pair boolean of them"), though the work item says it is
  renamed and the "sided by table" inference is what this unit removes;
  the closure building the union is still called `boolean`.
- **S12** (Q1, unsure) — the fold sentinels double as generic test
  reads: `nest.rs`, `emit_topo.rs` and `emit_union.rs` unit tests key
  ordinary subtract operands by `FOLD_A`/`FOLD_B`, which
  `is_fold_side` treats as the fold's internal space. The subtract path
  under unit test therefore never runs with two real reads (nor with two
  equal ones, M1).
- **S13** (Q5, likely) — `lone_member` returns `ValuePayload::Body`
  while every other union/intersect outcome is `ValuePayload::Boolean`
  (`Body {..}` or `Empty`), and it drops the member's carried contacts.
  A downstream reader matching on the boolean payload sees two shapes
  for "a union's result".
- **S14** (Q6, likely) — the `NAME_WORDS` rise is large (scoped p99
  38 → 57, total 49 844 → 55 693; full p90 106 → 123) and justified at
  length as "the reading, not a regression"; per the stance, length of
  justification is mild evidence. Every union member now says its join,
  including where nothing is disambiguated.
- **S15** (Q1, likely) — `Bodies::rows`, `rows_mut`, `try_map` and
  `BodyRead::rows`, `rows_mut` repeat the same
  `u32::try_from(i).unwrap_or(u32::MAX)` / `u8::try_from(k).unwrap_or(u8::MAX)`
  enumeration five times (`operand.rs`); a saturating cast that would
  silently alias seat 4 294 967 295.
- **S16** (Q7, unsure) — `BodyRead`'s hand-written `Deserialize`
  (`operand.rs`, ~110 lines with a re-prepending `MapAccess` and a
  string re-deserializer) carries the "bare when plain" rule; an
  untagged enum or a `#[serde(untagged)]` wrapper would say it in a
  few lines. It also rejects `at: []` on read, which is right, but a
  hand-built map whose first key is neither `read` nor `at` is handed to
  the bare read's deserializer — a future `Operand` variant named
  `read` would be misparsed.
- **S17** (Q4, likely) — `coincide::carried_read` now answers
  `(RoleSeg::From { read, .. }, _) => Ok(*read)` for any node, where the
  base refused a carry segment at a node of the wrong kind (`Misplaced`);
  with m1, a stored name with a foreign read walks to whatever operation
  defines it.
- **S18** (Q6, likely) — the viewer's boolean tool picks **nodes**, so
  it cannot author the unit's headline case, a union of a split's two
  halves (two reads of one node), nor an indexed read; the DM4 text's
  "a union or intersect seat of N body picks" is met in count only. Not
  scheduled.
- **S19** (Q1, unsure) — `side_by_operand`'s `(true, true)` arm and
  `member_sites`' `holding`/`held` fallback are two spellings of "which
  of several members sharing a read holds this name", with different
  answers on a tie (refuse vs. site all).
- **S20** (Q2, unsure) — `member_of` (`emit_union.rs`) falls back to the
  first member sharing the read when no member's table holds the name
  (`.unwrap_or(first)`), silently; for a family read whole that is a
  wrong body, not an emission bug. Mutant M3 tests whether anything
  notices.
- **S21** (Q8, likely) — `wire.rs`'s boolean section keeps
  `#[allow(clippy::too_many_arguments)]` on both lowerings and threads
  tuples-of-tuples (`(op, node, bodies)`, `(results, vals)`) to stay
  under the lint; the arity is being hidden rather than reduced.
- **S22** (Q6, likely) — the PR body's "Known limits" records the
  family-wide declared-pair site and the mate lift as filed items, good;
  the rank-one narrowing (N1) and the dropped K samples (m2) are not
  among them.
