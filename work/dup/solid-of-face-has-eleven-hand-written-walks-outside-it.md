---
id: solid-of-face-has-eleven-hand-written-walks-outside-it
kind: issue
title: Body::solid_of_face: the hand-written face to shell to solid walks outside it are seventeen, not eleven
status: closed
opened: 2026-09-19
closed: 2026-09-20
refs: [the-face-to-solid-walk-is-spelled-per-test-file, shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim, listing-a-solids-faces-is-spelled-four-times-in-topo-src, two-spellings-of-the-face-to-solid-owner-index]
---


## Finding

- **Where**: `crates/topo/src/shell10_r2_probes.rs` (`faces_of`, ~:39)
  and `crates/topo/src/offset_together.rs` (`faces_of`, ~:981) —
  **byte-identical helper bodies under one name in two files**;
  `crates/topo/src/props.rs` (~:3293), `crates/topo/src/seqgen.rs`
  (~:725, ~:733, ~:1594, ~:1602), `crates/topo/src/euler_ring.rs`
  (~:2755), `crates/topo/tests/bool4r1_probes.rs` (~:199),
  `crates/sweep/tests/revolve_ring.rs` (~:58),
  `crates/sweep/tests/shell5_r1_probes.rs` (~:533),
  `crates/sweep/tests/verbs_tubewall.rs` (~:165).
- **Importance**: medium
- **Confidence**: sure for the `topo/src` sites, which were read; the
  three `sweep/tests` sites are structural-regex hits not yet read.
- **Raised by**: PR #2857's fix pass, 2026-09-19.

`Body::solid_of_face(face) -> Option<SolidKey>` — face → its shell's
back-pointer → `Shell::solid` — carries the doc claim *"the one spelling
of face → shell → solid the census, the point-in-solid door and their
suites read."* That sentence is scoped and true of what it names. What
it does not say is that **eleven hand-written spellings of the same walk
sit outside it**, two of them `faces_of` helpers identical modulo
indentation in two files of the same crate.

The shapes:

| shape | sites |
| --- | --- |
| `faces_of(body, solid)` — helper, identical modulo indentation | `shell10_r2_probes.rs` (~:39), `offset_together.rs` (~:981) |
| `solid_of_face` inlined as `get_face(k).and_then(get_shell).is_some_and(...)` | `props.rs` (~:3293) |
| `get_shell(face.shell).expect(...).solid`, the half-edge data already in hand | `seqgen.rs` ×4 |
| the walk split over two `let`s | `euler_ring.rs` (~:2755) |
| test-lane `unwrap` chains | `bool4r1_probes.rs`, three `sweep/tests` files |

## Re-measured 2026-09-19 at `b932d5cac`; the class is seventeen

Five instruments were run over **every tracked file, no path
argument**, and then a sixth — a denominator-first classification —
was run over the same tree and corrected the five. What each added is
below. The class, corrected:

| bucket | sites | disposition |
| --- | --- | --- |
| `topo/src` | 12 | 8 folded onto the door, 1 kept and guarded, 3 correctly unfoldable |
| `topo/tests` | 1 | filed on tint (with tcost) |
| `sweep/tests` | 4 | filed on tint (with tcost) |
| not members | 4 | see below |

### The row's own list was off by three

- The prose said **eleven**; the list under it names **twelve** sites.
- **`crates/sweep/tests/revolve_ring.rs` (~:58) and
  `crates/sweep/tests/verbs_tubewall.rs` (~:165) are NOT members.**
  Both read `.solid` off a shell key a HANDLE already holds
  (`t.cavities[0]`, `t.shell`); no face is walked. The structural
  regex matched the handle field's NAME (`t.shell`), not a
  `Face::shell` read. This is the instrument's own blind spot, and it
  is the opposite of the one the row stated. **The same shape stands
  in `demos/tour/src/ring.rs` (~:277) and `demos/tour/src/tubewall.rs`
  (~:361)** — the only `shell -> solid` reads in any cargo root
  outside `--workspace`, and non-members for the same reason.
  Recorded here so the next lane does not re-find them as the class.
- **`crates/topo/src/offset_together.rs`'s `scope_of_moves` (~:931)
  was missed.** The regex could not see it because the walk is split
  over two `let`s.
- **`crates/sweep/tests/shell8_common.rs` (~:55),
  `shell8_r1_probes.rs` (~:40) and `shell8_r2_probes.rs` (~:61) were
  missed** — three more byte-identical `solid_of` bodies, split over
  two `let`s the same way.
- The brief for the fold said `seqgen.rs`'s four sites *"carry
  `expect(...)` with a distinct message per hop"*. **They do not.**
  Each has exactly ONE lookup and one `expect`, because the face datum
  arrives from the `body.faces()` iterator. They fold cleanly and did.

### Three more in `topo/src` that the five instruments missed

Found by the denominator-first classification below, after the fold.
**None of them can fold**, and the door's doc now says so as a shape
rather than naming `scope_of_moves` as a lone exception.

| site | shape | why the door cannot take it |
| --- | --- | --- |
| `euler_ring.rs`'s `Body::kfmrh` (~:764), walk 1 | two `let`s, 14 lines apart | hop 1 refuses `StaleKey { key: EntityId::Face(f1) }`, hop 2 `StaleKey { key: EntityId::Shell(f1_shell) }` — the refusals distinguish the hops, as `scope_of_moves`'s do |
| `euler_ring.rs`'s `Body::kfmrh` (~:772), walk 2 | the same, on `f2` | the same |
| `seqgen.rs`'s `fusion_remake_shell` (~:1216) | two `let`s, 5 lines apart | refuses uniformly (`?` on `Option` throughout), but the intermediate `shell2` is **live past the `.solid` read** — `shell_components(body, shell2)` and `shells.last() == Some(&shell2)`. Folding cannot delete the `let`; it can only add a second resolution of a face key the function has already resolved |

`kfmrh` is a **`pub fn` Euler operator**, which retires the fold PR's
claim that `scope_of_moves` was *"the only production site in the whole
class"*. The class's unfoldable members are a population of four walks
across three functions, two of them public API.

One more site is member-**shaped** and is not a member:
`euler_ring.rs` (~:2793) reads `get_face(seed.face).unwrap().shell`
and then **writes** `get_shell_mut(f1_shell).unwrap().solid = dead`.
`Body::solid_of_face` is a `&self` read; no read-door can host a write.

### What folded (8), all in `topo/src`

`props.rs` (~:3293, `and_then` chain, uniform `Option`),
`shell10_r2_probes.rs` and `offset_together.rs`'s two `faces_of`
bodies, identical modulo their leading indentation (uniform panic,
preserved as an `expect` over the door), `euler_ring.rs`'s
`fused_two_shell_body` (~:2755, two `let`s, uniform `unwrap`),
`seqgen.rs` ×4 (uniform `expect`).

### What did not (1 of the 9 the fold saw), with its posture

`crates/topo/src/offset_together.rs`'s `scope_of_moves`. Hop 1 refuses
`ReplaceFaceError::StaleFace { face }` — **the caller's own key, named
back to it**. Hop 2 refuses `ReplaceFaceError::Corrupt` — nullary,
because no key the caller holds is wrong. An `Option` door raises one
value for both.

**The fold was planted and nothing red**: `solid_of_face(face)
.ok_or(Corrupt)?` passes 4405 tests across `topo`, `sweep` and
`editor-core`. So the hole was real and the guard is the deliverable —
`offset_together::scope_walks::the_two_hops_refuse_differently` now
reds on BOTH flattenings, each on its own arm (`hop 1 names the
caller's own key`; `hop 2 is the body's own incoherence, not a stale
argument`).

Two things that guard does **not** do, disclosed at the test:

- Arm 2's `Corrupt` is also the refusal of the terminal
  `Scope::of_solids(..).ok_or(Corrupt)`, so the variant alone cannot
  say which of the two raised it. The arm brackets it — the same face
  and the same body build a scope **before** the back-pointer is
  broken and refuse after — which pins the refusal to the corruption
  but still cannot separate hop 2 from an `of_solids` the same
  corruption broke. What it guards is the flattening, and either fold
  changes the variant at one arm or the other.
- Arm 1's witness was `FaceKey::default()`, a key that never existed —
  the *foreign* case the door documents that it does **not** catch.
  It now mints a face through `mvfs` and kills it with `kvfs`, so the
  witness is the *stale* case the refusal under test is about.

### The instruments

1. **The row's structural regex** (`get_shell(… .shell …) … .solid`,
   across line breaks, every tracked file): 9 code sites. Blind to a
   walk split over two statements — which is 7 of the 17 — and it
   over-fires on any field named `shell`, which is the 4 non-members.
2. **A face-anchored window** (a face lookup, then `.shell`, then
   `.solid`, within 8 lines). Its hit list is below; **it fires on
   `seqgen.rs`'s `fusion_remake_shell`**, so the site the fold missed
   was inside this instrument's reach and was not dispositioned.
3. **A type-directed probe** (`#[deprecated]` on `Face::shell` and
   `Shell::solid`; primary spans paired within 8 lines, deduped by
   `(file, line)`, over `cargo check --workspace --all-targets
   --features topo/interval`). The denominator: **109 uses of
   `Face::shell`** in 36 files, **81 of `Shell::solid`** in 29, of
   which **24 pair**. It is the only instrument of the five that
   cannot miss a spelling inside what it compiles — but it cannot read
   a root outside `--workspace` (`scripts/doc-gate.sh --print-roots`
   names seven) or code behind a feature it was not given, and its own
   8-line pairing window is what hid `kfmrh`.
4. **A name census** (`(fn|let)\s+(solid_of|faces_of|shell_of|…)`,
   every tracked file): found a **third** `faces_of` in `topo/src`
   (`boolean/solid_contain.rs` ~:3764) whose body is a call, not a
   walk, so no structural instrument could reach it. Filed as its own
   row.
5. **A prose census** (the walk described in words): found
   `shell.rs`'s `PlanarFace::solid`, which reads through
   `offset_together::Scope::solid_of` — a **second, memoized door**
   with the same name shape and no walk in it — and
   `validate.rs`'s pass 10, which derives the shell from the
   OWNERSHIP partition on purpose and must never read the
   back-pointers it validates. Neither is a member; both would have
   read as one to a grep.

### Instrument 2's hit list — the receipt the fold owed

Reconstructed and re-run over every tracked `.rs` file at `b932d5cac`:
anchor `get_face(_mut)?\s*\(`, then the first `\.shell\b` and the first
`\.solid\b` within the 8 lines from the anchor, the `.solid` at or
below the `.shell`. **14 hits over 1771 files**, every one
dispositioned:

| hit | disposition |
| --- | --- |
| `sweep/tests/shell5_r1_probes.rs:533` | member — tint's row |
| `sweep/tests/shell8_common.rs:56` | member — tint's row |
| `sweep/tests/shell8_r1_probes.rs:41` | member — tint's row |
| `sweep/tests/shell8_r2_probes.rs:62` | member — tint's row |
| `topo/src/body.rs:875` | the door's own body |
| `topo/src/body.rs:1335` | over-fire — a back-pointer assertion; the shell key is the fixture handle's `t.shell` |
| `topo/src/euler.rs:2817` | over-fire — the same shape on `c.shell` |
| `topo/src/euler_ring.rs:2755` | member — **folded** (`fused_two_shell_body`) |
| `topo/src/euler_ring.rs:2793` | member-shaped **write** — `get_shell_mut(..).solid = dead`; a `&self` door cannot host it |
| `topo/src/offset_together.rs:932` | member — **kept and guarded** (`scope_of_moves`) |
| `topo/src/props.rs:3293` | member — **folded** |
| `topo/src/seqgen.rs:1219` | over-fire in the PAIRING — `shell1` stops at the shell (`shell_components`); the `.solid` five lines on belongs to `shell2` |
| `topo/src/seqgen.rs:1220` | **member — missed by the fold** (`fusion_remake_shell`); see above for why it stays hand-written |
| `topo/tests/bool4r1_probes.rs:198` | member — tint's row |

**What this instrument cannot see**, stated because the fold did not:

- **A walk whose face datum arrives from an iterator**, not from a
  `get_face` lookup — `body.faces()` yields `(key, &Face)` and the
  `.shell` is read straight off it. That is **6 of the 8 the fold
  folded**: both `faces_of` bodies and all four `seqgen.rs` sites.
- **Two hops more than 8 lines apart** — `kfmrh`'s are 14 (`get_face`
  at ~:768, the `.solid` at ~:787), so both its walks escape.
- It over-fires on back-pointer assertion rows, on writes, and on a
  `.solid` that belongs to a different shell binding in the window.

### The instrument this row leaves behind: denominator-first classification

**Enumerate every textual `.solid` read in every tracked `.rs` file,
then classify each one backwards by where its receiver came from.**
149 hits at `b932d5cac` — all cargo roots, including the four outside
`--workspace`, every `cfg`, every feature, because it reads text and
compiles nothing.

It is the strongest instrument this program has produced, and the
reason is structural rather than clever: **every member of this class
must terminate in a `Shell::solid` read**, so a walk it cannot see does
not exist. Where the five face-anchored instruments each ask *"is this
shape a walk?"* and miss the shapes they did not think of, this one
asks *"is this `.solid` read reached from a face?"* of a denominator
that is closed by construction. Its cost is the classification: 149
receivers read by hand, of which 132 are `.solid` on some other type
(`SolidSpec`, `MvfsCreated`, `ShellClassification`, `PlanarFace`, the
step-import specs) or on a shell key from a handle, an iterator, a
slice or an `EntityId`.

**Its blind spots, and the three it closes by measurement rather than
by argument** — each checked over the same tracked set:

- A `Shell::solid` reached through an **accessor method** rather than
  the field would not be spelled `.solid` as a field read. `git grep
  '\.solid()'` is **empty**: no such accessor exists.
- A **struct-pattern destructuring** (`Shell { solid, .. }`) binds the
  field without a `.solid` read. The only hit for `Shell\s*\{[^}]*solid`
  is `euler.rs:958`, an error-enum variant, not `Shell`.
- A helper that returns the shell (`shell_of`) and whose caller then
  reads `.solid` would still be *counted* — the `.solid` is the
  denominator entry — but its receiver traces through the helper. Eight
  `shell_of` helpers exist; none feeds a `.solid`, so none is a member.
  Five of them are one hop short of this class and are recorded below.
- **What it genuinely cannot do**: it reads text, so a walk assembled
  by a macro, or one spelled `. solid` across whitespace, is invisible.
  Neither occurs here (`git grep -E '\.\s+solid\b'` is empty), but the
  first is unfalsifiable by this method and a future macro would need a
  different instrument.

### The sibling-door re-census (the instrument that opened this row)

**Take the door a change cites as its PRECEDENT, and re-census that
door's own walk.** PR #2857 sited `Body::face_of_half_edge` on
`solid_of_face` as the model for "the one spelling of this walk". A door
held up as a model is a door somebody once folded onto — and the fold is
as old as its claim, so the claim is exactly as stale as the tree has
moved since. Running the census on the model rather than on the new door
found the same defect one door over.

**Blind spots.** It needs a cited precedent, so it says nothing about a
door with no model, and it inherits whatever blind spot the census used
on the precedent has.

### This door's precedent chain

`solid_of_face` cites no model, so the sibling-door re-census cannot
run on it. Its own provenance is in `docs/MODEL-AB-LOG.md`'s BOOL4
row: the door was minted in PR #2767's **fix pass**, from a bilateral
review finding — *"face→shell→solid spelled four times"* — and the
fold was scoped to that PR's own four sites. So the scoped doc claim
was never a survey; it was a report of one PR's reach, and the tree
had seventeen. **A door minted by a fix pass inherits that pass's
fence, and its doc sentence inherits it silently.**

**And the fix pass for that finding repeated it.** PR #2865's own
replacement sentence — *"every spelling in THIS CRATE that refuses
uniformly across the two hops reads through here"* — was false of
`seqgen::fusion_remake_shell`, in the same file as two sites that PR
folded, and its enumeration (*"the census, the point-in-solid door, and
the probe and sequence-generator suites"*) already omitted
`euler_ring::fused_two_shell_body`, one of the eight it had just
folded. The door's doc now claims a **population** and no census at
all, and points here for the measurement. The unguarded-census-sentence
class is
`work/dup/a-doors-rustdoc-carries-an-unguarded-census-sentence.md`.

## Adjacent families this sweep passed over

- **Five `shell_of` closures in `sweep/tests`** —
  `shell5_r1_probes.rs` (~:200), `shell5_r2_probes.rs` (~:100),
  `verbs_shell.rs` (~:490, ~:542), and `m3_pr3_split.rs` (~:384, from
  a vertex). Each is `body.get_face(f).expect(..).shell`: one hop
  short of this class, the same duplication one level up. Carried as
  evidence on tint's row.
- **`crates/topo/src/boolean/ops.rs` (~:1996)** spells *"the shell of
  this face"* the **reverse** way — scanning every shell's face list —
  where `Face::shell` is a back-pointer that answers in one lookup.
  Its own row on curved's slate:
  `work/curved/the-shell-of-a-face-is-scanned-for-where-a-back-pointer-answers.md`.

## What remains

| bucket | sites | owner | row |
| --- | --- | --- | --- |
| `topo/src` `scope_of_moves` | 1 | dup | THIS row, kept and guarded |
| `topo/src` `kfmrh` ×2, `fusion_remake_shell` | 3 | topo (territory) | THIS row; recorded, no work owed — see the table above |
| `topo/tests` + `sweep/tests` | 5 | tint (with tcost) | `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md` |
| the verbatim fixture family | 5 helpers | dup | `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md` |
| `faces_of` ×3 + `SolidFaces::of` | 4 | dup | `work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md` |
| the unguarded census sentence | class | dup | `work/dup/a-doors-rustdoc-carries-an-unguarded-census-sentence.md` |


## Re-taken 2026-09-20 at `b29fe8bd1`; the class did not move

Two instruments, every tracked file, **no path argument**, at the merge
base of the `faces_of` unit — three days and one merge-forward after
the census above was taken at `b932d5cac`.

- **The denominator-first classification, re-run.** Every textual
  `.solid` FIELD read (excluding `.solid(` calls): **173 occurrences on
  150 lines** in tracked `.rs`, against the 149 the row recorded. The
  figure moved; **the class did not.** Classifying each backwards by
  whether a shell resolution stands in the 16 lines above gives 75
  candidates, of which the members are the same set this row already
  names. Nothing new in `topo/src`.
- **The row's instrument 2 (face-anchored window), reconstructed and
  re-run**: **12 hits over 1780 tracked `.rs` files**, against 14 over
  1771. The two that left are `props.rs` and `euler_ring.rs`'s
  `fused_two_shell_body` — both recorded above as **folded**, so the
  delta is exactly the fold and nothing else.

### One family the earlier census passed over, now measured

`crates/topo/src/movefac.rs` carries six `.solid` reads and appears in
neither the row's list nor its non-member list. All six were read:
`move_shells_to_new_solid`'s `owner_of`, its plan/commit pair and its
rows take the shell key as an ARGUMENT or from the shell arena, never
from a face. Not members. The same for `shell.rs`'s `data.solid`
(~:1319, a `body.shells()` iterator) and `census.rs`'s
`EntityId::Shell(s) => get_shell(s).map(|d| d.solid)` (~:3286, a shell
key out of an `EntityId`). Recorded so the next lane does not re-find
them as the class, which is what this row already did for
`revolve_ring.rs` and `verbs_tubewall.rs`.

### Citations that had rotted (numbers, not names)

`kfmrh`'s two walks anchor at ~:769 and ~:774; their `.solid` reads are
at ~:787 and ~:794. The names are right and the numbers are allowed to
rot (`CLAUDE.md` §Citations); they are corrected here only because the
next lane re-running the 8-line-window instrument needs the real gaps,
which are **18 for walk 1 and 20 for walk 2** — not the one figure the
row gave (14), and not the single 17 this section gave at its first
attempt, which read walk 1's `.solid` off the `})?` line at ~:786 and
then quoted one gap for two walks that differ by two. **A correction
that is itself a number is the third of its kind on this program**;
give both, or give the larger.

## Closed

Closed 2026-09-20 with the `faces_of` unit, which was the last bucket
this row still owed. Every remaining bucket has a file of its own:

| bucket | file |
| --- | --- |
| `scope_of_moves` kept and guarded | done in this row, guarded by `offset_together::scope_walks::the_two_hops_refuse_differently` |
| `kfmrh` ×2, `fusion_remake_shell`, the `euler_ring` write | recorded above; no work owed, and `Body::solid_of_face`'s rustdoc states each as a POPULATION with no count |
| `topo/tests` + `sweep/tests` | `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md` |
| the verbatim fixture family | `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md` |
| `faces_of` ×3 + `SolidFaces::of` | `work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md` — folded onto `Body::faces_of_solid` |
| the unguarded census sentence | `work/dup/a-doors-rustdoc-carries-an-unguarded-census-sentence.md` |
| the face-to-solid OWNER INDEX, spelled twice | `work/dup/two-spellings-of-the-face-to-solid-owner-index.md` — new, found by the `faces_of` census |
