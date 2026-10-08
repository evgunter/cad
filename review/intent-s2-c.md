# Review — INTENT stage 2 PR C, "the product is the world" (#4359)

Frozen head `1ed6477f2`. Scope: C's own diff, `b57e0376` (merge-base with
`origin/intent/s2-b-reads`) → head. Reviewer lane (correctness + style).
Probe sources: `review/intent-s2-c-probes.rs` (not part of any build).

## Verdict: APPROVE-WITH-FIXES

The gather, the retirements, test 6 and the re-baselines hold up under
execution. Two MAJORs, both small and local: a construction can read a
world copy (D10's "construction never reads the world", and claim C2),
and no row anywhere evaluates a placement whose pose is not the identity
(a mutant that ignores the pose survives the whole editor-core suite).

## Findings

**MAJOR-1 — A construction reads the world.** `DocEdit::place`
(`edit.rs:674`) mints the copy as an ordinary `Body` output. B's slot
door then lets any `Body` slot read it, so `Boolean { a: <copy of
PlaceInWorld>, b }` is admitted and evaluates. Its geometry moves with
the placement's pose: the boolean's digest is `b3aed905…` at dz=0 and
`69aa25f1…` at dz=0.45 (probe `rv_a_construction_reads_the_world`, run
at head). That breaks D10 ("Construction never reads the world … export
reads its coordinates and nothing else does") and the ruling that only
the gather and export read the pose.

The kernel also plans for such readers. Split's narrowed closure
(`refactor.rs:2776`, "a body a cut placement places (or that placement's
copy)") re-points a remainder read *of the copy*. The viewer avoids it on
its own: `world::seat_of` reads through to the body, which is a
convention and not a rule. The fix is a door refusal: a read of a
`PlaceInWorld` output by anything is refused at `SetParam`/insert and at
load, the shape D's `ConstructionReadsObserved` has. Drop the copy arm
of `cut_placed` along with it. If the orchestrator rules the copy
readable instead, D10's sentence needs an `[ev]` PR. **sure** (executed).

**MAJOR-2 — The pose is unexercised; a pose-blind mutant survives.**
Mutant: `wire_place_in_world` (`eval/wire.rs:4388`) evaluates the pose
and then places at the identity. Result over the editor-core `ci`
profile: 2824/2826, and the only reds are my probe and the aggregation
census for my probe file. No editor-core row, Python test
(`grep pose=` in `crates/pncad-py/tests`), viewer or tour site calls
`place` with `Some(pose)`; every placement in the tree is the identity.
The posed path is a whole feature with no row: the motion, the
copy-name graft under a moved body, and the "rigid placement keeps every
arena key" claim about contact records. Q3 says no row can go red here.
Add a posed row: copy digest equals `Transform` at the same chain, and
the copy's contact records resolve. **sure** (executed).

**MINOR-1 — Ruling (2) has no row.** Mutant: drop `if !placed { return
Ok(None) }` in `assembly::resolve_face` (`assembly.rs:~1505`). Every
`ci` row passes. The behaviour is also silent. The caller arm
`(Ok(None), Ok(_)) | (Ok(_), Ok(None)) => continue` (`assembly.rs:1392`)
records no unminted row, no finding and no trace. So a mate between an
unplaced instance and a placed one declares a contact the gate never
checks, and nothing says so. Spec Q8 and test 22 do say "mints
nothing", so the rule is right. Pin it now (test 22's second bullet,
early), or say in the PR that only F pins it. **sure** (mutant run).

**MINOR-2 — The interim positional `k` pick is not flagged as interim.**
The PR body (call 4) and the code comment at `refactor.rs:~3323`
(`split`'s `in_world`) say the `Part(Instance(k))` index is "evaluated at
the document's values" (`eval_var_count(*index, &env)`). Neither says
this is positional and interim, as the orchestrator's ruling requires.
Add one sentence to the PR body, and preferably to the comment. The rule
itself bites: with `holds` forced true, red in
`p2_face::a_face_side_on_a_pattern_copy_crosses_split_and_inline_unmoved`
and `fix_pattern_mate_crossing::an_underqualified_pattern_head_…`.
**sure**.

**MINOR-3 — Spec §4's migration exception (Q9, residue 3) is neither
built nor disclosed.** "A root InstantiatePart on the world gauge with no
placing mate: its offset moves into the placement's pose." C keeps every
offset on the instance and places it at the identity (the tour's
`assembly.rs:565/596` `SetOffset` rows). Test 6 still holds, because the
digest is the same either way. But the PR body never mentions Q9, and
the world pose now has three homes in C (instance offset, gauge
placement, `PlaceInWorld` pose) where the spec meant to move one. Either
build it or disclose it with a scheduled row. **likely**.

**MINOR-4 — Ruling (7): the viewer delete loses the world entry the add
gesture moved.** `Session::delete_node` (`viewer/src/session.rs:3263`) →
`cascade_delete_order` takes the deleted feature's placements. Adding a
fillet re-points the body's placement to the fillet. Deleting the fillet
then deletes that placement and leaves the pre-fillet body unplaced, so
the part vanishes from the viewport. Pre-C, `on_delete` re-rooted it.
Cascading is right for the kernel: re-pointing would infer a re-point,
which DM6 forbids, and leaving it would strand the placement. But the
gesture pair add→delete is no longer an identity on the world, and no
row or issue says so. File a CHROME row: the delete gesture could
explicitly re-point a feature's placements to its target, as the add
gesture did. **likely** (by reading; not executed in the viewer).

**MINOR-5 — "world's coordinates" wording, against the ruling.** It is
new in C at `refactor.rs:582` (`TwoAnchors` doc), `refactor.rs:2969`,
`refactor.rs:3591` (`places_an_instance` doc) and
`tests/p2_gauge_offsets_and_spaces.rs:960`, whose comment also says
"The transform is the root". **sure**.

**NOTE-1 — C1's "every placement appears once" has a carve-out.**
`product_in` skips a placement whose copy lives in an unplaced group's
own space (`evaluation.space(node) != space`, `product.rs:~976`).
`product()` is then `Ok` without that copy. Export refuses it
(`UnplacedHere`), and `Product::spaces` carries it to the gate. This is
A11 (2), pre-existing and stage 3's to retire, but it means an explicit
`PlaceInWorld` can be left out of `product()` silently. Say so in A10's
"Nothing else places" sentence or in the gather's doc. **likely**.

**NOTE-2 — `Doc::unplaced` lists `Body` outputs only.** A document
holding only a pattern names the prototype and not the pattern
(`unplaced = [#29]` in probe `rv_unplaced_lists_no_bodies_output`), and
the recourse "place a body" does not tell the author that a copy needs a
`Part` pick first. **sure** (executed).

**NOTE-3 — Two rigid motions, two record rules (Q1).** `wire_transform`
(`eval/wire.rs:4314`) returns `OpOut::plain` and drops a boolean's
contact records. `wire_place_in_world` carries them verbatim on the
argument that "a rigid placement keeps every arena key". That argument
is equally true of `Transform`. One of the two is wrong, or the reason
is not the one stated. **unsure**.

**NOTE-4 — The log entry is misfiled.** `work/intent/log.md:488–502`:
C's entry sits inside B's, so B's bullet "Q1/C2 as built" (line 502) now
reads as C's (a merge of B landed it there). C's "Filed:" bullet
(line 656) is appended to the stage 4 slicing entry. **sure**.

**NOTE-5 — Stale A10 prose in C-touched code.** `product.rs:611–615`
(`sources_of`: "no longer a sink, so it is no longer a root") and
`checks.rs:1325` ("root-list order"). The `debug_assert` text at
`assembly.rs:1527` says "a root's face row … every root". The PR's sweep
patterns ("root list", "sink set") did not match "a sink", "root-list"
or "a root's". **sure**.

**NOTE-6 — Every product name grows four words** ("the world copy of",
`names/words.rs`). name_words p99 is 34→38, and `OVER_BUDGET` now admits
two arms one over. Given Ev's "ceremony" worry (2026-10-03), a GUI pick
on any body now says "the world copy of" first. Consider eliding the
qualifier in words when the body has exactly one placement. Taste.
**unsure**.

**NOTE-7 — Cost.** `Doc::placements()` is a full node scan, and
`resolve_face` runs it plus one `strict_ancestors` walk per placement for
each mate side: O(mates × placements × N). That is fine at corpus size
and invisible at assembly size. **unsure**.

## The rulings

| Ruling | Verdict |
|---|---|
| Pose read only by gather and export | **Not held**: MAJOR-1 (a construction reads a copy); MAJOR-2 (the pose read is untested) |
| No "world coordinates" wording | Not held: MINOR-5 |
| No new `Transform` uses | Held in src: Duplicate's `Transform` is the existing gesture. Tests add `xform` as fixtures only |
| Instance ships one `body` port; per-placement signature filed | Held. `an-instance-defines-one-body-per-part-placement` is filed, with the four open choices |
| Split/pattern gestures leave the world; AddPart places only what is asked | Held (`add_pattern` Instances arm; `add_part` → `create_placed`). Overlap with the still-placed target is filed (CHROME) |
| Placement carries contact records and declaration rows verbatim | Held as code; untested under a pose (MAJOR-2). See NOTE-3 |
| `k` pick at document values is interim, flagged in the PR body | Built, **not flagged**: MINOR-2 |
| Driven pattern count moves no placement | Held. `place_pattern_to` places the fins; the perf12 goldens are byte-identical (below) |
| Impl (1) `RoleSeg::Placed { of }` | Right. The placement's identity rides in `node`, as `Instance` does for a pattern. Cost: NOTE-6 |
| Impl (2) unplaced mate member mints nothing | Right per spec Q8 and test 22, but silent and unpinned: MINOR-1 |
| Impl (7) delete cascades a placed feature's placement | Right for the kernel; the gesture asymmetry is unfiled: MINOR-4 |
| Impl (8) p2_split TwoAnchors face-frame arm deleted | Right. A face frame is no longer in the world, so it casts no vote. `TwoAnchors` is still pinned elsewhere, and the replacement row (a lifted copy in an unplaced group) still reaches `UnplaceableRoot` |

## Claims

- **C1 — partly falsified (NOTE-1).**
  - Order: mutant `Doc::placements` reversed → test 6 red, plus 38 others.
  - Stranded placement: mutant that skips it and continues silently → test 10 red.
  - `EmptyProduct` and `StrandedPlacement` are both reachable (test 10), and their recourses are true.
  - Nothing unplaced appears (test 9; my probe). An explicit placement in an unplaced group's space is the exception.
- **C2 — falsified (MAJOR-1).**
  - Grep: the pose is read only by `wire_place_in_world`, the content key and the frame-fault door.
  - The copy is readable by any construction, demonstrated by execution.
- **C3 — confirmed by execution.**
  - I wrote an independent probe at B's tip (`b57e0376`) that gathers every corpus document and the five committed files.
  - It reproduces all 34 `PRE_C` rows byte for byte at ε default, 1e-6 and 1e-12 (files read at the process ε, as test 6 does).
  - At head, `intent_s2_c_world` and `perf12_census` are green at 1e-6 and 1e-12 (13/13 each) and at the default.
  - The `perf12_census_1e-{6,9,12}.txt` goldens are untouched by the diff.
- **C4 — four pins checked, all moved for their stated reason.**
  - `golden.cad`: the 7 pre-C roots (Assertion, two Profiles, Chamfer, Tube, HollowTube, Shell) become 4 identity placements of the four bodies, in root order. Nothing else moved but the chain and mint rows.
  - `die_composed_tour.pncad`: `roots` is gone, and the trailing `DeleteNode` of the blank is replaced by one `PlaceInWorld`.
  - `plate_param.pncad`: one placement of the boolean output. The measure and assertion ids shift by +2.
  - `asm2a` row6 reuse 1→2 (the placement is a second memo hit).
  - Executed on both trees: the asm2a, name_words_corpus, docm4, p2_gauges, lib_g16 name-digest and perf2 keying rows (66 tests) pass at B with B's pins and at head with C's.
  - "the world copy of" is four words, matching the name-words +4.
- **C5:** see the table above.
- **C6 — four mutants run.**
  - Placement order and the stranded skip: killed.
  - The `k` pick: killed (two rows).
  - The unplaced-mate skip (MINOR-1): **survived**.
  - Pose ignored (MAJOR-2): **survived**.

**Runs:** editor-core at head, `--profile default` (slow set included):
2886/2887. The one failure is
`name_words_rows::a_large_table_of_names_alike_at_no_citation_is_said_in_bounded_time`,
which also fails at B's tip on this box, so it is a timing row and not
C's. Mutants ran under `--profile ci`.

**Not exercised:**
- the viewer, pncad, pncad-py and Python suites; test 9's viewer and Python halves; tour;
- clippy and fmt;
- the delete-gesture asymmetry (MINOR-4), which I read but did not run;
- Q9's corpus reach: I found the tour's offsets but did not probe a migrated instance.

## Style

Questions exercised: Q1 (NOTE-3; three copy qualifiers `Instance`,
`InPart` and `Placed` are not a duplication), Q2, Q3 (MAJOR-2, MINOR-1),
Q4 (NOTE-5, MINOR-5), Q5 (`product.rs` module doc against NOTE-1), Q6
(MINOR-3 undisclosed; the filed follow-ups are rows, so they count as
scheduled), Q7 (NOTE-6, NOTE-7) and Q8 (I read `product_in`,
`wire_place_in_world`, split's `in_world`, `resolve_face` and
`viewer/src/world.rs` whole).

- The fix-mints-the-defect check: the PR's sweep for "roots" left new
  "root" prose in the code it rewrote (NOTE-5, and "The transform is the
  root"). Its claim "no construction reads the world" is stated in prose
  (`world::seat_of`'s doc), while the kernel admits the read (MAJOR-1).
  Both are instances of the defect the unit closes. **likely**.
- `held_placement` (`node.rs:~4027`) gained a `PlaceInWorld` arm but has
  no caller in the workspace. Dead code grew. **sure**.
- `wire_place_in_world` re-derives the split-half port → `SplitHalf`
  mapping inline (`port == 0` twice). `wire_part` already has one. Taste.
  **unsure**.
- Test 6 rewrites each committed file's `"epsilon"` line before loading,
  so at 1e-6 it reads a 1e-9 document as a 1e-6 one. It holds (my B probe
  agrees), but the comparison is of a re-stamped file, which deserves a
  sentence at the record. **unsure**.
