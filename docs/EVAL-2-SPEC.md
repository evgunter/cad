# EVAL-2 — the content key's tag vocabularies are declared, not counted: every tag has a named home and every home is censused over its `ALL` (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 2). **Items, one unit:**
`work/eval/node-tag-space-census-blind-to-tags-outside-sentinels.md` and
`D365` (claimed from DOCM in this unit — §Claim of D365).
**Track:** E — the CIW/CHROME posture: one implementer lane, one style
review with claims to falsify, a fix pass, record-at-merge. No A/B draw.
**Correctness arm: yes** — the unit touches what a document's memo key is,
so the review carries the key bit-identity claim as a MAJOR-class claim
(§Bit-identity), not a style note.
**Branch:** `eval/2-tag-vocabularies`. **Difficulty:** M (breadth: ~96
`write_tag` sites in one 4 600-line file; no numeric content).

## The claim

**A content key is a byte stream, and the memo serves entry X to node N
exactly when the two streams agree. A tag separates alternatives only at
the grammar position where it is read, so the property that keeps a memo
from serving another node's geometry is injectivity WITHIN each vocabulary
read at one position — never "one number space for the whole key".** The
file (`crates/editor-core/src/eval/mod.rs`) half-knows this and
half-denies it, and the denial is where the drift is:

- Two vocabularies are censused by SOURCE TEXT between sentinels
  (`NODE-TAG-SPACE`, `:2950`–`:3069`, `node_tag_space_is_injective`
  `:4422`; `SEG-TAG-SPACE`, `:4036`–`:4237`, `seg_tag_space_is_injective`
  `:4527`). The node census's doc says a tag written outside its sentinels
  "does not exist today". Outside them today: `41` (`:3122`, the lane
  program opener), `45` (`:3189`, carrier radius), `44` (`:3566`, the
  `StartArriving` target), `42` (`:3708`, the lane-scalar wrapper), `43`
  (`:3958`, the blend's flow-bearing slot), `30`–`37` (`:3592`–`:3625`
  arc modes and sides, `D365`'s), and the presence bits `0`/`1`/`2`/`3`
  in `feed_placement`, `feed_mate`, `feed_alignment`, `feed_measure_expr`,
  `naming_key`. None is a collision, because none is in the node space —
  but nothing in the tree can say that, which is the finding.
- Three site comments assert a GLOBAL append-only space and contradict
  each other: `:3186` "45: the next free number in this key's tag space
  (44 is the tangent arrival's, 43 the blend's flow-bearing slot)";
  `:3561` "the tag space is append-only and 42 was the high-water mark …
  43 held the retired STRAIGHT member and stays DEAD, never reused
  (D365)"; and `:3958` writes `43`. `43` is dead in the target vocabulary
  and live in the blend-payload vocabulary, which is fine and is exactly
  what "one space" cannot express.
- Four vocabularies already have the right shape and are the precedent:
  `verb_content_tag(VerbKind) -> Option<u8>` censused over `VerbKind::ALL`
  (`:2720`, tests `:4318`, `:4364`), `verb_tag(profile::Verb) -> u8` over
  `Verb::ALL` (`:3497`, `:4287`), `contact_class_tag` (`:3859`),
  `dimension_tag` (`:3928`). A number is a function of a closed enum, and
  a test iterates the enum's `ALL`. Nothing reads source text.

The unit makes every vocabulary take that shape, and makes the prose say
which vocabulary a tag is in instead of pretending there is one.

## What lands

1. **One declaration module for the structural tags.** Every
   `write_tag(<literal>)` that is not a function of an enum — the format
   version `6`, the openers `41`/`43`/`45`, the wrapper `42`, `LoopStart`
   `1`, the presence bits, `naming_key`'s `3`, measure-expr's `1`/`2`/`3`
   — moves to named `const`s in a `mod tag` (or `tag_space`) at the top of
   the content-key section, GROUPED by the vocabulary they are read in,
   each group a `const ALL: &[u8]`, with a one-sentence doc per group
   naming the grammar position it is read at. One test asserts injectivity
   within each group, iterating the groups' `ALL`s. **After this, `rg
   'write_tag\([0-9]' crates/editor-core/src/eval/mod.rs` returns zero
   hits outside the declaration module and the two enum matches below** —
   that grep is the acceptance and goes in the PR body with its output.
2. **The role-segment vocabulary becomes a function of `SegTag`.**
   `names/select.rs:104` already carries the payload-free mirror of
   `RoleSeg` with an exhaustive `SegTag::of(&RoleSeg)`; it is EVAL's file.
   Add `SegTag::ALL` (the `Verb::ALL` shape), write
   `seg_content_tag(SegTag) -> u8` with the numbers exactly as they are,
   have `feed_role_seg` read the tag through `SegTag::of` and then feed
   the payload as it does now, and replace `seg_tag_space_is_injective`
   with a census over `SegTag::ALL`. The `SEG-TAG-SPACE` sentinels go.
   (`C6`, DOCM's, wants the `SegTag` mirror collapsed one day by a proc
   macro; this unit builds on the mirror that exists and does not touch
   that question.)
3. **The arc-mode and side vocabularies become functions of their enums**
   (`D365`): `arc_mode_tag(ArcMode) -> u8` over `ArcMode::ALL`
   (`crates/profile/src/path/program.rs:215`, already there) for `30`–`35`;
   `ArcSide` has two variants and no `ALL` — add `ArcSide::ALL` only if
   the enum is in EVAL's ground (it is not: `crates/profile` is S-BOOL's
   glob), else declare the two side tags in the module of item 1 as a
   two-member group and say why. `Target`'s `4`/`5`/`44` are a group in
   item 1 (`Target<T>` carries payloads; there is no kind enum, and a
   three-member group needs none).
4. **The node vocabulary keeps its source-text census, narrowed and
   honest.** There is no payload-free `NodeKind::ALL` and `node.rs` is
   DOCM's, so `node_tag_space_is_injective` stays as the census of the
   one match that is a function of `Node` — but its doc and the sentinel
   comment now say precisely that: this census covers the node-kind
   vocabulary and nothing else, every other vocabulary is declared in
   `mod tag` or as an enum function with its own census, and "a tag
   outside the sentinels" is not a gap but a different vocabulary. Rename
   the sentinels if a clearer name suggests itself; do not widen the
   region.
5. **Every "one space / next free number / high-water mark / append-only
   across the key" sentence is rewritten** to name the vocabulary the tag
   is in, and the rule that holds is stated once, at the declaration
   module: *within a vocabulary, an existing number never changes meaning
   and a retired number is never reused; across vocabularies numbers are
   unrelated.* `:3561`'s "43 stays DEAD" is true of the target vocabulary
   and says so; `:3186` stops claiming 44 and 43 are its neighbours.
6. **No number moves.** Every tag keeps its value. Keys are
   process-internal and never persisted (`verb_content_tag`'s doc, and
   nothing under `persist/` names `ContentKey`), so a renumbering would
   cost nothing on disk — and it would also buy nothing, while making
   §Bit-identity unfalsifiable. The unit is a re-homing of declarations.

## Bit-identity

**Claim: every content key computed for every document in the test corpus
is bit-identical before and after.** The implementer proves it the direct
way: at the merge base, dump `(node id, content key)` for every node of
every corpus document through the existing key door (a throwaway test or a
small example binary under `/home/user/eval-2-scratch/`, not committed),
repeat at the head, `diff`. Put the row count and the diff result (zero
lines) in the PR body. The memo tests in `crates/editor-core/tests/`
(`switch_program_key.rs`, `msolve4_mate_memo.rs`, and the rest that grep
`ContentKey`) are the standing guard and must stay green untouched.

## Claim of D365

`work/docm/D365.md` ("Anchor the content-key mode tags 30-35 on
`ArcMode::ALL` in an injectivity census, as the verb tags already are") is
this unit's item 3 exactly, and sits on `eval/mod.rs`, EVAL's path; DOCM
took it on 2026-09-06 as a Track V row with the memo files, before this
program existed. Per `work/README.md`, claiming MOVES the file:
`git mv work/docm/D365.md work/eval/D365.md`, append a `## Claimed by EVAL
(date)` record naming this spec, keep the id and the `track: V` line,
set `parent:` to `node-tag-space-census-blind-to-tags-outside-sentinels`.
The orchestrator announces the claim to DOCM in the program log and on
the PR; if DOCM objects, the file moves back in a one-line PR and item 3
still lands (the code is EVAL's either way).

## Sweep

The class is "a tag literal with no declared vocabulary". The acceptance
grep in item 1 is the sweep of this file. Beyond it:

```
rg -n 'write_tag\(' crates/editor-core/src --glob '!eval/mod.rs'
rg -n 'write_u8\(|write_tag\(' crates/editor-core/src/eval/memo.rs crates/editor-core/src/eval/parts.rs
```

`eval/memo.rs` and `eval/parts.rs` are DOCM's: a hit there is reported in
the PR body with its vocabulary named, never edited. State what the
pattern cannot match (a tag written through a local helper whose name is
not `write_tag`, or a tag composed arithmetically).

## Review

One style lane, `docs/prompts/reviewer-style-lane.md` by path, and the
correctness claims below carry MAJOR weight:

1. **Bit-identity** (MAJOR if false): reproduce the key dump at merge base
   and head independently — do not trust the PR body's zero — and diff.
2. **Injectivity is total** (MAJOR if false): for every vocabulary, name
   the test that iterates its `ALL`, and try to add a duplicate number to
   each declaration and watch that test go red. A group whose test
   cannot go red is not censused.
3. The acceptance grep is what the PR body says it is: run it.
4. Q1 — is any vocabulary declared twice now (the enum function AND a
   leftover literal in a feed)? Q2 — does any surviving comment still
   assert a cross-vocabulary order? Q5 — does the census doc claim what
   it measures and no more? Q8 — read the whole content-key section end
   to end once; it is the largest thing this unit touches.
5. The `D365` move is a `git mv` with history, and the DOCM
   announcement exists.

Emphasis for this lane: the fix reproducing the defect it closes — a
declaration module can mint a THIRD spelling of a vocabulary that also
lives in an enum function — and a disclosed blind spot read as a discharge.

## Records at merge

`work/eval/log.md` entry with the DOCM announcement; both items `closed`
with `pr:`; this spec deleted per `docs/DOC-LEDGER.md`.
