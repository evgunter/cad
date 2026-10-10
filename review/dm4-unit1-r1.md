# FORK-DM4 unit 1 — review R1 (Opus), PR #4527

- **Frozen head:** `3cbd7403ab`. **Base:** `cb4172aa9f` (merge-base with `origin/main`).
- **Protocol:** `docs/DUAL-REVIEW-PROTOCOL.md`, CONCURRENT pair (unit H). Lane R1.
- **Lane isolation:** I read no other `review/dm4-unit1-*` branch, no other scratchpad and no PR comments. I fetched only `intent/dm4-names-keyed-by-read` and `main`. I read the PR **body** through the GitHub API, as the brief directs. No glimpse to disclose.
- **Cost:** about 290k tokens (harness counter delta); about 60 min wall-clock (10:55 → ~11:55 UTC).
- **Method:** I built at the head (`CARGO_INCREMENTAL=0`, a private target dir) and wrote 15 probe rows (`review/dm4-unit1-r1-probes.rs.txt`, run as `crates/editor-core/tests/r1_probes.rs`). I made a base worktree for the one-shot and for a base twin of one probe (`review/dm4-unit1-r1-base-probe.rs.txt`), and ran mutants on the regenerated files, on `refactor.rs` and on `emit_topo.rs`.

## Verdict: **APPROVE-WITH-FIXES**

Counts: **1 MAJOR · 3 MINOR · 4 NOTE**, plus 9 style items.

The MAJOR is local: one guard, plus a row. It is fail-loud, since it surfaces as a typed emission refusal and never as a silent table. But it sits in functionality the PR claims as built ("the indexed read `xs[i]`, at every body seat"), so it should be fixed before merge.

## Runs at the head

| What | Result |
|---|---|
| editor-core `--profile ci` | 2905 run. 200 first "failed" with `failed to exec … No such file`: my own rebuild replaced the binary mid-run. Re-run, all 200 pass except `every_suite_file_is_aggregated`, which fails on my unregistered probe file only. **Green at the head.** |
| viewer + pncad | 996/996 |
| Python (`run-python-tests.sh`, `ty`/stub tests included) | 961 OK. Caveat: the cdylib build overlapped a short window in which my `emit_topo` patch (below) was applied. Only the same-read subtract path differs, and no Python row reaches it. |
| `dm4_migration_one_shot` against a base worktree (`DM4_BASE_TREE`) | all three files pass |
| `m10_p_fence::the_corpus_geometry_is_bit_identical_with_ids_masked` | passes. Its constant is not touched by the diff. Only the f64/Interval/Probe numbers moved. |
| kernel crates (`topo`, `geom-core`, `sweep`) | untouched by the diff |

## MAJOR

### M1. `Subtract { from: xs[i], tool: xs[j] }` refuses with an emission bug for any seam vertex: the subtract emitter tells its two seats apart by read, and an indexed read puts one read in both seats

- **Demonstrated by execution** (red probe, green twins, root cause confirmed by a patch).
- **Probe** (`p12_subtract_of_two_members_of_one_family`, `p14`): a unit block, `xs = Pattern(Linear [1, 0.7, 0.4] spacing 0.6, count 2/3)`.
  - `Subtract { from: xs[0], tool: xs[1] }` and `{ xs[1], xs[0] }` both refuse `Naming(SeamVertexParentage { vertex: 16v1 })`.
  - The same geometry builds through two distinct reads: through `Part(xs, 0)`/`Part(xs, 1)`, and through two `Transform`s, both vol 0.708353.
  - `Union([xs[0], xs[1]])` builds (1.708353), as does `Intersect([xs[0], xs[1]])` (0.291647).
  - P14 maps the shapes: disjoint members build, touching members refuse `UndeclaredCoincidence` correctly, and a general-position overlap refuses the emission bug.
- **Root cause:** `crates/editor-core/src/names/emit_topo.rs:1974-1975`:
  ```rust
  Some(RoleSeg::From { read, of: x }) if *read == a.read => a_edges.push(x.clone()),
  Some(RoleSeg::From { read, of: x }) if *read == b.read => b_edges.push(x.clone()),
  ```
  When `a.read == b.read` (both seats read the family `xs`), every incident edge lands in `a_edges`, `b_edges` stays empty, and the vertex falls into the `([_], [], _, _) => SeamVertexParentage` arm (`emit_topo.rs:2127`).
  - On the base, the positional `FromA`/`FromB` could not collide.
  - The union fold is immune because it passes the `FOLD_A`/`FOLD_B` sentinels (`wire.rs`, `wire_combine`), never the members' reads.
- **Confirmation:** a three-line patch reads the side by which seat's table holds `x` when the reads coincide. That is the rule `side_by_operand` already applies, at `eval/wire.rs:3603-3612`, `(true, true) if holds(a_table) != holds(b_table)`. With it, both orders of `Subtract { xs[0], xs[1] }` build at vol 0.708353, and P14's diagonal overlap builds. I reverted the patch; the head is unchanged.
- **Claims broken:**
  - Claim 6: the read key is not consistent at the subtract emitter. Two seats can hold one read, and the emitter assumes they cannot.
  - Claim 2: "an indexed read evaluates to the right instance … at every body seat" fails at a subtract of two members of one family.
- **Class, not instance.** The PR knew the case: the declaration path handles one read at both seats, but its sibling in the emitter does not. That is a Q4 sibling-sweep miss.
  - Sweep every site that tells a pair's two sides apart by `read`, not only this one. Candidates: `coincide::name_rows`'s `RowInputs { a, b }` for subtract, `refusal_menu`'s two sites, and the blend's and shell's single-seat emitters (no second seat, so likely fine).
  - My grep (`rg '\.read\b' … '==|!='` over `names/`, `coincide.rs`, `eval/wire.rs`) found only these two lines and `emit_union.rs:472`. The last one already handles a shared read deliberately (`member_of`).
- **Missing row:** no row pins a subtract of two indexed reads of one family. The DM3 suite's subtract coverage is `from: A, tool: A` and single indexed seats.

## MINOR

### m1. Forward stage specs still cite the retired vocabulary (stale premise, Q4)

- **Accepted by inspection.**
- `docs/INTENT-STAGE4-SPEC.md:38,381` names the payload to retire as `Node::Boolean.declare` (`node.rs:2383`) beside `Node::Union.declare`.
- `docs/INTENT-STAGE4-SPEC.md:400` lists `Node.boolean(declare=)`.
- `:586` reads roles off `FromA`/`FromB`.
- `docs/INTENT-STAGE5-SPEC.md:131` says the result's faces carry `FromA`/`FromB`.
- These specs bind work not yet built (stage 4 F retires declarations over exactly these nodes). The PR also changes their line anchors. Per CLAUDE.md, a re-wording forced by an approved code change lands with that change. `PERF-2-SPEC.md` is historical and can stay.

### m2. `xs[i, j]` is ratified text but refused by code that cites the same clause for the opposite (Q5)

- **Accepted by inspection.**
- `REFERENCES.md` DM3 (line 126): "`xs[i]`, or `xs[i, j]` for a family keyed by two indices". `docs/DESIGN.md:1336`: "`xs[i, j]` reads one member".
- `node.rs:1694-1697` (`InputFault::IndexRank`): "every family is indexed by one `Count` (REFERENCES DM3)". Its Display says "a family is indexed by one count", and `input_fault` refuses any `at.len() > 1` (`node.rs:3712-3717`).
- DM3's *Built* line ("the indexed read at every body seat") does not say that rank two is unbuilt. A nested pattern evaluates as a flat list, so today `xs[k]` on a nested pattern reads the flat index and `xs[i, j]` is refused.
- Either the code doc mis-cites DM3, or DM3's *Built* line owes "rank one only" with a schedule.

### m3. The one-shot's "total map" has no rule for `SitedRef.at`, and the corpus exercises no declaration, so claim 7's "would catch a dropped declaration" is vacuous here

- **Demonstrated in part by execution.**
- None of the three regenerated files carries a declared pair (`"declare": []` throughout).
- `Walk` (`tests/dm4_migration_one_shot.rs`) has no arm mapping an old `at: RecipeNodeId` to a new `at: VarId`. It would pair the node id with the var id in the bijection and most likely go red spuriously on any document with declarations.
- My mutants did go red:
  - swapping the snapshot `Subtract`'s `from`/`tool` in `die_tool.pncad` → "29:… reads as 62:…, earlier as 29:…";
  - four mutations of the tour's edit log (swapping the subtract seats, swapping two union members, re-pointing a `From` read at another member or seat). Each was caught, but by the edit-log replay at `load` (`OperandUnresolved` / `SelectionNotCanonical`), before the map ran.
- So for the tour, the map itself is not what catches a mis-seated read. The digest-chained edit log is. That is fine as a guard, but it is not what the PR body describes.

## NOTE

### n1. Pre-existing and order-dependent: a union whose declared contact a third member covers refuses `Naming(Emission)`

- **Demonstrated by execution, on the head and on the base.**
- The fixture: `A = [0,1]³`, `C = [1,2]×[0,1]²` touching A at x = 1, with the A–C pair declared via `find_flush_candidates`, and `M = [0.5,1.5]×[−0.5,1.5]²` covering the contact.
- `Union` over the six orders: `amc`, `mac`, `cma`, `mca` refuse "a piece of a face held as several borders no recorded discard between them". `acm` and `cam` build. The base gives the identical six results (`Boolean`-era `Union`).
- So this is not this PR's defect. But it is a three-box reproducer of what looks like the open P3 issue `work/emit/a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere.md`, which says "No corpus case reaches this". It also contradicts DM4's "refuses … in every member order".
- Worth attaching to that issue and re-weighing its priority. It also blocked my claim-5 probe (n2).

### n2. Claim 5 holds by inspection. The one boundary is unexercisable today.

- **Accepted by inspection, plus a partial probe.**
- What holds:
  - `wire_combine` splices the detached judgement exactly when it is `Err` or `Ok(None)` (refusal, empty intersect).
  - A fold-step refusal is already in the node's frame.
  - A two-member union re-runs the same pair at step 0, so the log is the pair's.
- The boundary: for three or more members, the judgement of a non-adjacent pair (0,2) is decided only in the judgement. Its verdicts are set aside on admission, while `UnionLinks` (the published names) rest on that judgement.
- I tried to build a case where the admission rests on a judged pair the fold never meets (a declared contact covered by a third member, `p08`/`p13`, three widths of M). Every order that skips the A–C step hits n1's emission bug. So no building document shows the dropped verdicts mattering. `unsure` whether it matters at all.

### n3. The fold sentinels rest on "no mint draws ordinal 0", which `MintId::parse` does not hold

- **Accepted by inspection.**
- `role.rs:1154-1161`: `FOLD_A = VarId::new(0, 0)`, `FOLD_B = VarId::new(0, 1)` — "a read no document mints, since mint ordinals count from one".
- `mint.rs:88-109`: the ordinal doc says "No mint draws ordinal 0", but `parse` accepts `"0"` as an ordinal.
- Whether the load door re-derives a var id from its preimage, which would refuse `0:0000000000000000`, I did not check. My probes never saw a sentinel in a published table: `no_fold_sentinel` over every union, intersect and subtract in P02–P05.

### n4. What held (evidence per claim)

1. **No plain document moves.** `BodyRead` serializes bare (`operand.rs:385-395`). The content key adds `tag::index::READS` only when some read is indexed (`eval/mod.rs:6100-6113`). `golden/mint_node_ids.txt` moved only the three boolean kinds, and `slot_tables.txt` only the boolean shapes. `a_document_of_plain_reads_saves_to_its_own_bytes` passes.
2. **Family and spelled name alike.** P11 deep-renames the node id through every nested name, over a *overlapping* diagonal family (99 union rows, 27 intersect rows). `Union(xs)` = `Union([xs[0], xs[1], xs[2]])` and `Intersect(xs)` = `Intersect([…])`. Also:
   - A family of one equals its member spelled (P04).
   - The door refuses `Family(xs[0])` (`IndexedFamily`), a mix (`SlotVarKind`), and an index on a body read at a subtract seat (`SlotVarKind`, found Body, expected Bodies) (P06).
   - Out-of-range and index-edit rows pass in the PR's suite. The exception is M1.
3. **A repeated read glues.**
   - `Union`/`Intersect([U, U])` over a seamed body `U` publish `[U]`'s table, and `Subtract{U, U}` is empty (P02).
   - `[A, A, A]`, `[A, far, A]`, `Intersect([A, B, A])` and `Union([A, B, A])` publish the unrepeated list's table (P03).
   - `[xs[1], xs[1]]` equals `[xs[1]]` (P04).
   - Member order does not move the table: `[B, A]` and `[A, A, B]` equal `[A, B]` (P07).
   - The `DuplicateName` pin is a unit row on `collapse_table` (`emit_union.rs:3467`).
4. **Geometry did not move.** Per the runs table.
5. **The verdict log.** See n2.
6. **The read key.** The refactor read-map mutant (`Remapping::read` → identity) is killed by 6 rows: `remap_reorders_ids` ×3, `refactor::…` ×2, and `asm4_split_inline::inline_name_refusals_fire_typed…`. No fold sentinel leaked in any probe. The exception is **M1**.
7. **The one-shot.** Passes, and goes red under the mutants. See m3 for its limits.
8. **Words.** The census "read alike" check (full, by tag, scoped) found nothing on two overlapping placements of one body under union, intersect and both subtract directions; on a diagonal family spelled and whole; and on `[t1, t1]` (P10). A split's halves are said "from Split …'s above / below" (P15), so the words do say the read.
9. **The viewer.** The viewer suites pass (996 with pncad). `combine_in_world` (`session.rs:3439-3470`) dedups operands, re-points the first placed operand and deletes the rest in one `Recording`. Accepted by inspection plus the suite.
10. **Python.** 961 OK, `ty`/stub tests included. `BooleanOp` survives only as the binding census's SHAPE entry for the kernel type, with its reason given.
11. **Content tags.** `seg_content_tags_are_injective` pins `From` = 52. On current `origin/main`, the segment tags top out at 51 (`Placed`) and the node tags at 37 (`PlaceInWorld`), so 52 and 38 collide with nothing. The retired 16/17/28/41 are not reused.

## Style

**Questions exercised:**
- Q1 (prose sweep, then a constants/literal sweep).
- Q2 (the justification-phrase sweep over added lines: nothing hit).
- Q3, Q4, Q5, Q6, Q7.
- Q8 only partly: I did not read `eval/wire.rs` (6122 lines) end to end. I read `reads_projected`, `wire_subtract`, `wire_combine`, `lone_member` and `judge_pairwise_contact` (about 700 lines), and the whole new section of `operand.rs`.

- **S1 — `sure`.** `names/emit_topo.rs:1974-1975` against `eval/wire.rs:3603-3612`. The one-read-at-two-seats case is answered in the declaration path and not in its sibling in the emitter. This is M1's class (Q4: an invariant established in one place and not swept to its siblings). Where else to look: anything that sorts a pair's rows into A and B by `read` (`coincide::name_rows`, `refusal_menu`, `resolve`'s subtract arms).
- **S2 — `likely`.** The member slot for list position `i` is derived four times: `OperandSlot::Member(u32::try_from(i).unwrap_or(u32::MAX))` at `operand.rs:227`, `:244`, `:279` and `eval/wire.rs:3136`. `wire_combine`'s spelled arm re-derives it instead of using `Bodies::rows()`/`try_map`, and the same `u32::try_from(i).unwrap_or(u32::MAX)` literal appears ten times across `wire.rs`, `node.rs`, `edit.rs` and `operand.rs` (Q1, data-level copy).
- **S3 — `likely`.** `reads_projected` (`eval/wire.rs:2878-2965`) clones the whole `Results` map (every node's entry) whenever it projects. `wire_combine` calls it once per spelled member, so `Union([xs[0], …, xs[N−1]])` clones `Results` N times. That is O(N · nodes) for a list the family form reads in O(N). Unmeasured.
- **S4 — `likely`.** `dm3_indexed_read.rs::a_union_of_a_family_and_of_its_members_spelled_name_and_build_alike` compares tables through `fixture::renoded`, which rewrites only the top-level node (`fixture/mod.rs:720`), and its fixture is disjoint cubes. Its premise excludes the case that differs: nested names that cite the union node (seams, crossings). It would fail spuriously on any overlapping family. My deep-rename P11 shows the claim does hold there, so the gap is the row's, not the code's (Q3).
- **S5 — `likely`.** `eval/wire.rs:3125-3127`: `let members = bodies;` and an inner `use … as O` re-bind inside a block that then shadows `members` twice more (`let members = members?; let members = members.as_slice();`). These look like leftovers of a refactor.
- **S6 — `unsure`.** `wire_combine`'s family arm destructures `BodyRead { read, at: _ }` (`eval/wire.rs:3149`) and silently ignores an index. It relies on every door holding `IndexedFamily`. A fail-loud evaluator would refuse there rather than union the whole family.
- **S7 — `likely`.** `name_words_corpus.rs` NAME_WORDS: scoped p99 went 38 → 57 (+50%), full p90 106 → 123, total words +12%. The `OVER_BUDGET` rows were raised (76 → 82, 76 → 80). Each rise carries several sentences of prose calling it "the reading, not a regression". Per the stance, a long justification for a ratchet raised in the same PR is mild evidence. Taste: "joined at Union d1aa from Extrude e548" on every member is a lot of words, and the scoped form grew most.
- **S8 — `unsure`.** `wire_combine`'s doc comment (`eval/wire.rs:3052-3108`) runs to about 55 lines over a body of about 290. Much of it restates DM4. That is Q2's shape, though the prose I checked is accurate.
- **S9 — `sure`.** n3's premise ("no document mints ordinal 0", `role.rs:1154`) is held by convention at the mint, while the parser that admits ids accepts ordinal 0 (`mint.rs:105-109`). It is a comment asserting an invariant that nothing at the door enforces (Q2).
