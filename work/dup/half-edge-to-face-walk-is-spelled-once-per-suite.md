---
id: half-edge-to-face-walk-is-spelled-once-per-suite
kind: issue
title: The half-edge → loop → face walk is spelled once per file across topo, sweep and their suites
status: closed
opened: 2026-09-19
refs: [blend-spells-the-half-edge-to-face-walk-five-times, the-half-edge-to-face-walk-is-spelled-per-test-file, names-emit-keeps-an-unguarded-two-refusal-walk, chords-spells-the-half-edge-to-face-walk-twice, step-lanes-spell-the-half-edge-to-face-walk, demos-tour-spells-the-half-edge-to-face-walk-three-times, solid-of-face-has-eleven-hand-written-walks-outside-it]
priority: P4
cost: E
closed: 2026-09-26
pr: 3284
---


## Finding

- **Where**: `crates/topo/src/shell.rs` (`offending_face`'s `face_of_he`
  closure) and `crates/topo/src/replace_face.rs` (`edge_faces`'s
  `face_of` closure) — **byte-identical closure bodies under two
  names, both in `topo/src`**; `crates/topo/src/query.rs`
  (`face_kind_across`), `crates/topo/src/splitting/join.rs`
  (`he_face`), `crates/sweep/src/blend/surgery.rs` (`face_of_half`),
  `crates/sweep/src/blend/battery.rs` (`face_of`),
  `crates/topo/src/test_support_fixtures.rs` (`face_surface_of_he`,
  the one PR 2842 folded four spellings onto), and ~49 more files.
- **Importance**: medium
- **Confidence**: sure. The two `topo/src` closures were read against
  each other; `query.rs`, `join.rs`, `blend/surgery.rs` and
  `blend/battery.rs` were read individually. The tail was measured, not
  read.
- **Raised by**: the PR 2842 review, recorded by that PR's fix pass,
  2026-09-19.

One walk: half-edge → its `parent_loop` → that loop's `.face`, and in
about half the spellings one more hop to `.surface`. It is three lines
and it is written out per file.

## The measurement (2026-09-19, at PR 2842's head)

A regex for the SHAPE — `get_loop(… parent_loop …) … .face`, across
line breaks, over every tracked `.rs` file — matches **56 files**:

| bucket | files |
| --- | --- |
| `topo/src` | 15 |
| `topo/tests` | 2 |
| `sweep/src` | 4 |
| `sweep/tests` | 31 |
| `editor-core/src` | 2 |
| `mesh/tests` | 1 |
| `step-export/tests` | 1 |

`parent_loop` appears on 335 tracked lines in all, so the walk is the
bulk of every read of that field.

**The class is not uniform, and the fold is not one function.** Three
error postures are in use and they are not interchangeable:

- `Option`-returning (`shell.rs`, `replace_face.rs`,
  `blend/surgery.rs`, `blend/battery.rs`, `query.rs`) — a stale key is
  an honest `None`;
- typed-`Result`-returning (`splitting/join.rs`'s `he_face` →
  `SplitJoinError`, `sweep/src/swept.rs`'s `face_surface_key` →
  `EulerOpError::StaleKey`) — a stale key is a refusal;
- `unwrap`/`expect` (most of `sweep/tests`) — a stale key is a test
  failure.

So the unit is one `Option`-returning door on `Body` with the
`Result` spellings as thin wrappers over it, not a single signature
imposed on 56 call sites.

## Why this is filed and not closed

PR 2842 folded **four** spellings of `face_surface_of_he` onto one
exported function and its body called that *"what the row asked for"*.
The row it cites is a **name** census, and a name census closes name
collisions. This class is the opposite shape — **one thing under many
names** — so four folded spellings is a half-fix of it, and PR 2842's
body is corrected to say so.

## Cheapest next pair

`crates/topo/src/shell.rs`'s `face_of_he` and
`crates/topo/src/replace_face.rs`'s `face_of`. Both are in `topo/src`,
both are `Option`-returning, both are local `let`-bound closures over
`body`, and their bodies are **byte-identical**:

```rust
|he| -> Option<FaceKey> { Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face) }
```

Neither was disclosed by any census in S-DUP's links 1–3. Folding them
onto one `pub(crate) fn` in `topo` costs two call sites and settles the
signature question for the rest of the class.

## Re-measured 2026-09-19; `topo/src` has ONE door and a stated residue

`Body::face_of_half_edge` is the door: `&self`, `Option<FaceKey>`,
immediately after `Body::solid_of_face`. `topo/src` is folded onto it —
**24 call sites in 15 files**.

**The walk is NOT spelled once in `topo/src` after the fold, and that
sentence is retracted.** What is true is narrower: every `topo/src`
spelling whose refusal is UNIFORM across the two hops, and which stops
at the face, reads through the door. Twenty-two do not, and they are
listed below rather than counted. The first version of this row said
"once"; the number was an artifact of the instrument that produced it
(below), not a fact about the tree.

### `topo/src` residue — the hit list, one disposition each

**Kept: the refusal distinguishes the hops (16).** Folding any of these
replaces a refusal that identifies an entity with one that does not, and
`the_walk_consumers_keep_their_own_refusal` plus
`he_face_names_the_key_that_went_stale` are the guards that red on it.
This is the population the door's doc points at.

| site | what its refusal names per hop |
| --- | --- |
| `splitting/join.rs` `he_face` (~:169) | `EntityId::HalfEdge` / `EntityId::Loop` — the guarded one |
| `shell.rs` `face_neighbours` (~:2765) | `corrupt(EntityId::HalfEdge)` / `corrupt(EntityId::Loop)` |
| `movefac.rs` (~:111) | `StaleKey{HalfEdge}` / `StaleKey{Loop}` |
| `attach.rs` `face_surface` (~:274) | `StaleKey{Loop}` / `StaleKey{Face}` |
| `query.rs` `surface_across` (~:786) | three entities: `HalfEdge` / `Loop` / `Face` |
| `merge_faces.rs` `half_edge_facts` (~:1610) | `StaleKey{HalfEdge}` / `StaleKey{Loop}` |
| `sector_face.rs` (~:139) | `Corrupt(HalfEdge)` / `Corrupt(Loop)` |
| `chord_join.rs` `owning_face` (~:1496) | `corrupt_he` / `corrupt_loop` |
| `chord_join.rs` (~:1848) | `corrupt_he` / `corrupt_loop` |
| `chord_join.rs` (~:1985) | `corrupt_he` / `corrupt_loop` |
| `boolean/join.rs` (~:1747) | `desync`, a distinct message per hop |
| `boolean/rest.rs` (~:1237) | `desync`, per hop |
| `boolean/rest.rs` (~:1340) | `desync`, per hop |
| `boolean/rest.rs` (~:1498) | `desync`, per hop |
| `seqgen.rs` (~:836) | `expect("half resolves")` / `expect("loop resolves")` |
| `seqgen.rs` (~:1010 + :1018) | same pair, split across eight lines |

**Foldable, NOT this unit (6).** Each is posture-free — one uniform
refusal across both hops — so the door's own obligation does not protect
it. Each carries a further hop PAST the face (`.surface`, `Face::shell`)
or reads the arenas directly, so folding it is the surface-hop decision
this unit refused, taken at six sites; it wants one PR, not a rider.

| site | posture | hop past the face |
| --- | --- | --- |
| `boolean/ops.rs` (~:935) | plain `Option` closure | `.surface` |
| `pcurves.rs` `half_edge_surface` (~:565) | uniform `PcurveMintError::Corrupt` | `.surface` |
| `pcurves.rs` `half_edge_surface_key` (~:583) | uniform `PcurveMintError::Corrupt` | `.surface` |
| `validate.rs` (~:5404) | arena-direct `?` closure | `.surface` |
| `validate.rs` (~:6228) | arena-direct `?` closure | `Face::shell` |
| `splitting/finish.rs` (~:423) | nullary `corrupt` | `.surface` |

**Folded by this unit, beyond the first pass (4).** All four are bare
`he -> FaceKey` with a uniform refusal, which is the door's exact
signature: `boolean/reduce.rs` (~:547, a plain `Option` `face_of`
closure with no posture argument at all), `tier3_tests.rs` (~:653),
`iso.rs` (~:737), `review_m1_pr4.rs` (~:1044).

**Not members (4).** `review_m1_pr4.rs` (~:1684) compares the two LOOP
keys and only then their faces — the loop keys are its subject;
`euler_ring.rs` (~:1684), `review_m1_pr2/cube_independent.rs` (~:250)
and `review_m1_pr3.rs` (~:228) assert `parent_loop` against a known loop
key and never reach a face.

### The entity-naming class is sixteen in `topo/src`, not two

The first version of this row presented the class as `splitting/join.rs`
plus `editor-core/src/names/emit.rs`. It is the table above plus
`emit.rs`, and **exactly one member is guarded**. That matters more than
the count: this unit's central result — that folding this posture onto
the `Option` door changes a verdict with the whole 727-test suite green
— applies to every one of them. The door's doc now points at this list
and the two guards point at the door's doc, so a lane meets the
population rather than the argument restated.

### `emit.rs` is safe by accident, and this row said otherwise

The earlier sentence — *"Those sites keep their own walk; a guard now
reds on the fold"* — reads as plural with the guard attached to both. It
attaches to one. What is actually true of
`editor-core/src/names/emit.rs`:

- **No test anywhere distinguishes `DANGLING_MATE` from
  `DANGLING_LOOP`.** Both constants are used once each (~:766, ~:769)
  and named nowhere else in the tree. A fold onto the door is invisible
  there, exactly as it was at `splitting/join.rs`.
- What blocks the fold at `rim_between` today is a **data dependency,
  not a guard**: `mate_he` is reused at ~:772 for `.edge`. Remove that
  and the fold becomes available and silent again.
- `emit.rs` (~:799) carries a third `dangling` refusal, in
  `face_half_edges`, that no census in this program has named. It is
  `face -> loop` — the opposite direction — so it is NOT a member of
  this class, but it is the same unguarded shape and belongs in the
  `wire` row filed with this PR.

### `shell.rs` does NOT already hold the half-edge data

The PR's first body claimed `shell.rs`, `chord_join.rs` and `seqgen.rs`
"hold the half-edge data already and spell only `loop -> face`". Read
against the tree:

- **`chord_join.rs`: true.** `he_data` is fetched at ~:1496 and used for
  `.edge` at ~:1498 as well as `.parent_loop`.
- **`seqgen.rs`: half-true.** ~:1014 is genuinely 2-hop (the half-edge
  comes from the iterator); ~:1010 + ~:1018 is a full 3-hop.
- **`shell.rs`: false.** `mate_data` is fetched one line earlier
  (~:2765) and used for nothing but `.parent_loop`. It is a 3-hop site,
  and it is in the kept table above for its refusal, not for its data.

## The instruments, re-run with their blind spots

**1. The row's regex** (`get_loop(... parent_loop ...) ... .face`,
across line breaks, every tracked `.rs` file): 56 files / 73 sites at
PR 2842's head.

**2. The type-directed probe** (`#[deprecated]` on
`HalfEdge::parent_loop` and `Loop::face`; pair the warning spans within
+/-3 lines over `cargo check --workspace --all-targets`).

- **Re-run with `(file, line)` dedupe**, which the first run did not
  state. `--all-targets` compiles a lib crate twice (lib and lib-test)
  and a deprecated field use in non-test lib code warns in both: at the
  merge base the raw primary-span count is **795**, and
  `(file, line, message)` dedupe takes it to **588**. So duplication is
  real, at about 1.35x — but the **paired** figure was already in the
  deduped range. Deduped and paired: **142 sites / 104 files** at the
  merge base (`f4d6bbee3`), **125 / 97** on this branch. The first run's
  145 / 103 was therefore NOT a doubled count and nothing derived from
  it halves; it is within 3 of the deduped number, and the residual
  difference is pairing tie-breaks.
- **The +/-3 window is sound, and for a better reason than first
  given.** rustc pulls both spans to the START of their expressions, and
  the `parent_loop` expression NESTS inside the `.face` expression, so
  `face_line <= parent_loop_line` bounded by receiver-chain length. The
  window is not a heuristic about formatting.
- **Blind spot A — `--workspace` is not every cargo root.**
  `scripts/doc-gate.sh --print-roots` derives **seven** roots outside
  it (`benches`, `demos/tour`, `demos/wild`, `interval-transcendentals`,
  `tools/k-lint`, `tools/tess-lint`, `tools/tess-meter`) — not four.
  `grep` over all seven: `demos/tour` holds **3** sites, every other
  root **0**. See the `work/issues/` row filed with this PR.
- **Blind spot B — feature-gated code never type-checks, so it never
  warns.** `crates/topo/tests/trim_3_chart_bound.rs` is
  `#![cfg(feature = "interval")]`; adding `--features topo/interval`
  takes `topo/tests` from 7 to 8. `demos/tour/src/bodies.rs` (~:601)
  is inside `#[cfg(feature = "probe")] fn bud_rim` and is invisible to
  a default-feature probe of that root too. Both numbers above already
  carry `--features topo/interval`.
- **Consequence: the regex is NOT a subset of the probe, and the
  floor/ceiling nesting the row claimed is unproven.** 142 is a middle
  estimate, not a ceiling.

**3. A prose census** (the walk described in words, every tracked file
including `.md`, no path argument) — it found the fourth error posture
(`emit.rs`'s `ok_or_else(bug)`), the byte-identical twins in
`sweep/src`, the arena-direct spellings, and the walk written out as
prose in `merge_faces.rs`'s doc comment. Blind spot: it reads intent,
so it over-fires on any prose about loops and faces and cannot be
counted, only followed.

**4. A 3-hop structural regex** (a loop lookup whose argument chains
through a half-edge lookup). **Its "1 hit in `topo/src` afterwards" was
an artifact and is retracted.** Re-derived here it still gives 1, and
`boolean/rest.rs` (~:1498) and `boolean/reduce.rs` (~:547) are both
3-hop sites it cannot see: the first nests three levels of parentheses
deeper than any bounded-nesting regex reaches, the second splits the
walk across two statements. Anything this instrument reports is a
floor and never a residue count.

**5. Sibling-door re-census** (new). Take the door a change cites as
its PRECEDENT and re-census that door's own walk. `Body::solid_of_face`
is this PR's model and has eleven hand-written `face -> shell -> solid`
walks outside it, two of them byte-identical helpers. Filed as its own
row on this program's slate. Blind spot: it needs a cited precedent, so
it says nothing about a door with no model.

**6. Closure-name census** (new): `let (face_of|surface_of|...) = |` over
every tracked file, 36 hits repo-wide. It over-fires (any closure whose
name starts with a topology noun) and says nothing about a walk written
inline. What it buys is reach: it is text, so it reads `demos/` and
feature-gated files the probe cannot compile, and it is what surfaced
the `topo/tests` undercount and the `demos/tour` twin pair.

## What remains — by seam, each with a filed row

| bucket | sites | owner | row |
| --- | --- | --- | --- |
| `topo/src` | 16 kept + 6 foldable | dup | THIS row, stays `open` |
| `sweep/tests` | 72 | tint (with tcost) | `work/tint/the-half-edge-to-face-walk-is-spelled-per-test-file.md` |
| `topo/tests` | 8 | tint (with tcost) | same row |
| `mesh/tests` | 5 | tint (with tcost) | same row |
| `editor-core/tests` | 3 | tint (with tcost) | same row |
| `step-export/tests`, `step-import/tests` | 2 | tint (with tcost) | same row |
| `sweep/src` + `sweep/examples` | 4 + 3 | carve | `work/carve/blend-spells-the-half-edge-to-face-walk-five-times.md` |
| `editor-core/src` | 3 | emit | `work/emit/names-emit-keeps-an-unguarded-two-refusal-walk.md` |
| `mesh/src` | 2 | tess | `work/tess/chords-spells-the-half-edge-to-face-walk-twice.md` |
| `step-import/src`, `step-export/examples` | 4 | exch | `work/exch/step-lanes-spell-the-half-edge-to-face-walk.md` |
| `demos/tour` | 3 | nobody | `work/issues/demos-tour-spells-the-half-edge-to-face-walk-three-times.md` |

Every row above was filed in PR #2857, not disclosed here: a residue
disclosed inside another item's prose reads as a record of work done,
is invisible to the re-homing sweep and dies with the directory
(`work/README.md`). The argument that separate rows would mint the
duplicate this program exists to prevent is an argument for ONE row per
seam owner, which is what these are.

One more row came out of the re-census and is not a bucket of this
class: `work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`
— the door this one cites as its precedent has the same defect, at
eleven sites.

**`topo/tests` is 8, not the 1 this row first said**, and the number
inherited the probe's blind spot: `trim_3_chart_bound.rs` (~:582) is
`interval`-gated and the default-feature probe could not see it. The
other seven are `issue86_double_subtract.rs` (~:110),
`readback_sense_kind.rs` (~:254), `review_m9_1_r2_probes.rs` (~:234),
`m3_pr2_reduce.rs` (~:338), `m3_pr3_split.rs` (~:385),
`verbs_f7_collinear_seam.rs` (~:84) and `bool4r1_probes.rs` (~:198).

## Arithmetic: which denominator

The probe's 142 counts field WRITES (the euler operators set both
fields) and 2-hop reads alongside 3-hop read walks, so "20 of 145" was
a fold count over a population that is not the same class, and "the
row's instrument found about half the class" (73 against 145) compared
the same two mismatched populations. Both are withdrawn. What the
instruments support:

- **Probe, like for like:** 142 sites -> 125, i.e. **17 of the 142
  paired sites** the probe reaches, all 17 in `topo/src` (44 -> 27).
- **`topo/src` walk sites, read by hand rather than matched:** 24
  folded onto the door, **16 kept because their refusal distinguishes
  the hops**, 6 deferred with the surface-hop decision. The fold is
  24 of 46, and the 22 that remain are each named above with a
  reason.

## Closed (2026-09-26, PR #3284)

**`topo` has the door** — `Body::face_of_half_edge` — so no door was
added for the walk. The row's own residue was the six
"foldable, NOT this unit" sites that carry one more hop; they now read
the door and then take their own hop with their own refusal:
`boolean/ops.rs` (~:947, `.surface`, `Option`), `pcurves.rs`
`half_edge_surface_key` (~:583, uniform `Corrupt`), `validate.rs`
`edge_adjacent_faces` (~:6917, arena-direct `?`) and the component
walk (~:7738, `Face::shell`), `splitting/finish.rs` (~:429, nullary
`corrupt`). `pcurves.rs` `half_edge_surface` was the key twin's walk
spelled again with a `get_surface` on the end; it now calls
`half_edge_surface_key`. One site no census here had named folds with
them: `pcurves.rs` `mate_surface` (~:557), a two-hop read of an
already-resolved half-edge, `Option`-uniform.

**The surface hop gets no door.** A `Body::surface_of_half_edge` would
be one line, but it would compose three lookups under one `None`,
which is exactly the refusal-posture question this row's kept table is
about, and the four sites that take the hop refuse three different
ways (`Option`, `PcurveMintError::Corrupt`, a `corrupt()` closure);
`validate` needs the face key as well. Folding the two-hop walk onto
the existing door leaves each of them one door call and one lookup of
its own.

**Re-measured after the fold** with the type-directed probe
(`#[deprecated]` on `HalfEdge::parent_loop` and `Loop::face`,
`cargo check -p topo --all-targets`, spans paired within ±3 lines,
`(file, line)` deduped; restored byte-exact after). `topo/src` pairs:
the door itself; the kept table's distinguishing-refusal sites
(`attach`, `boolean/rest`, `chord_join` ×2, `merge_faces`, `movefac`,
`query`, `seqgen` ×3, `shell`); `review_m1_pr4` (loop keys compared,
a non-member); and field WRITES in test code (`euler_kill` ×2,
`validate` ~:8448) and an `euler_ring` loop comparison. Then every
unpaired `parent_loop` read in `topo/src` (141) read at its line: none
reaches a face under a uniform refusal except the kept sites whose two
hops sit more than three lines apart (`boolean/join`, `boolean/rest`
×2, `chord_join` ×2, `sector_face`, `splitting/join`). **No foldable
site remains in `topo/src`.** Blind spot, as before: a macro-assembled
walk, and feature-gated code (the probe ran default features).

**Plants** (`face_of_half_edge` with `#[track_caller]`, acting only
at the six folded call lines; whole `topo` crate, 1467 rows):

| plant | reds | per site |
| --- | --- | --- |
| panic at `validate` ~:7738 (with the rest) | 979 / 1467 | it runs inside every validation, so it masks the others |
| panic at `ops` ~:947, `pcurves` ~:557, ~:585, `finish` ~:429 | 135 / 1467 | 103, 8, 3, 21 |
| panic at `validate` ~:6917 | 227 / 1467 | 227 |
| the MATE's face answered at all six | 891 / 1467 | — |

Three rows printed the plant and passed in the first run: the
`probe_support::try_wall_sheet` stand-downs and
`r2_diag_mintable_tilts`, whose `catch_unwind` is the design (they
build a cylinder sheet whose mint reaches the validator).

The other buckets of this class remain on the rows filed in PR #2857
(`helper`, `strut`, `emit`, `chord`, `export`, `issues`); none was in
this unit. `Body::face_of_half_edge`'s rustdoc points here for the
kept population, which the table above still is.
