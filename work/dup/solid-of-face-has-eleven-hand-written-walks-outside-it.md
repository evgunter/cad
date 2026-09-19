---
id: solid-of-face-has-eleven-hand-written-walks-outside-it
kind: issue
title: Body::solid_of_face has eleven hand-written face to shell to solid walks outside it
status: open
opened: 2026-09-19
refs: [the-face-to-solid-walk-is-spelled-per-test-file, shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim, listing-a-solids-faces-is-spelled-four-times-in-topo-src]
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
sit outside it**, two of them byte-identical `faces_of` helpers in two
files of the same crate.

The shapes:

| shape | sites |
| --- | --- |
| `faces_of(body, solid)` — byte-identical helper | `shell10_r2_probes.rs` (~:39), `offset_together.rs` (~:981) |
| `solid_of_face` inlined as `get_face(k).and_then(get_shell).is_some_and(...)` | `props.rs` (~:3293) |
| `get_shell(face.shell).expect(...).solid`, the half-edge data already in hand | `seqgen.rs` ×4 |
| the walk split over two `let`s | `euler_ring.rs` (~:2755) |
| test-lane `unwrap` chains | `bool4r1_probes.rs`, three `sweep/tests` files |

## The instrument — sibling-door re-census

**Take the door a change cites as its PRECEDENT, and re-census that
door's own walk.** PR #2857 sited `Body::face_of_half_edge` on
`solid_of_face` as the model for "the one spelling of this walk". A door
held up as a model is a door somebody once folded onto — and the fold is
as old as its claim, so the claim is exactly as stale as the tree has
moved since. Running the census on the model rather than on the new door
found the same defect one door over, at eleven sites.

**Blind spots.** It needs a cited precedent, so it says nothing about a
door with no model, and it inherits whatever blind spot the census used
on the precedent has. The structural regex used here
(`get_shell(... .shell ...) ... .solid`) has a bounded-nesting limit and
cannot see a walk split across a helper boundary, so eleven is a floor.
`git grep -n '\.shell'` read by hand is what confirmed the `topo/src`
nine.

## Re-measured 2026-09-19 at `b932d5cac`; the class is fourteen, and two of the eleven are not members

Four instruments were run over **every tracked file, no path
argument**. What each added is below. The class, corrected:

| bucket | sites | disposition |
| --- | --- | --- |
| `topo/src` | 9 | 8 folded onto the door, 1 kept and now guarded |
| `topo/tests` | 1 | filed on tint |
| `sweep/tests` | 4 | filed on tint |
| not members | 2 | see below |

### The row's own list was off by three

- The prose said **eleven**; the list under it names **twelve** sites.
- **`crates/sweep/tests/revolve_ring.rs` (~:58) and
  `crates/sweep/tests/verbs_tubewall.rs` (~:165) are NOT members.**
  Both read `.solid` off a shell key a HANDLE already holds
  (`t.cavities[0]`, `t.shell`); no face is walked. The structural
  regex matched the handle field's NAME (`t.shell`), not a
  `Face::shell` read. This is the instrument's own blind spot, and it
  is the opposite of the one the row stated.
- **`crates/topo/src/offset_together.rs`'s `scope_of_moves` (~:931)
  was missed**, and it is the only PRODUCTION site in the class. The
  regex could not see it because the walk is split over two `let`s.
- **`crates/sweep/tests/shell8_common.rs` (~:55),
  `shell8_r1_probes.rs` (~:40) and `shell8_r2_probes.rs` (~:61) were
  missed** — three more byte-identical `solid_of` bodies, split over
  two `let`s the same way.
- The brief for the fold said `seqgen.rs`'s four sites *"carry
  `expect(...)` with a distinct message per hop"*. **They do not.**
  Each has exactly ONE lookup and one `expect`, because the face datum
  arrives from the `body.faces()` iterator. They fold cleanly and did.

### What folded (8), all in `topo/src`

`props.rs` (~:3293, `and_then` chain, uniform `Option`),
`shell10_r2_probes.rs` and `offset_together.rs`'s two byte-identical
`faces_of` bodies (uniform panic, preserved as an `expect` over the
door), `euler_ring.rs` (~:2755, two `let`s, uniform `unwrap`),
`seqgen.rs` ×4 (uniform `expect`).

### What did not (1), with its posture

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
caller's own key: Corrupt`; `hop 2 is the body's own incoherence, not
a stale argument: StaleFace { face: FaceKey(1v1) }`).

### The instruments

1. **The row's structural regex** (`get_shell(… .shell …) … .solid`,
   across line breaks, every tracked file): 9 code sites. Blind to a
   walk split over two statements — which is 4 of the 14 — and it
   over-fires on any field named `shell`, which is the 2 non-members.
2. **A face-anchored window** (a face lookup, then `.shell`, then
   `.solid`, within 8 lines): added the three `sweep/tests` `solid_of`
   helpers and `scope_of_moves`. Over-fires on back-pointer assertion
   rows (`euler.rs`, `body.rs`, `instance.rs`), which assert each hop
   against a known key and are not walks.
3. **A type-directed probe** (`#[deprecated]` on `Face::shell` and
   `Shell::solid`; primary spans paired within 8 lines, deduped by
   `(file, line)`, over `cargo check --workspace --all-targets
   --features topo/interval`). The denominator: **109 uses of
   `Face::shell`** in 36 files, **81 of `Shell::solid`** in 29, of
   which **24 pair**. It is the only instrument that cannot miss a
   spelling inside what it compiles — but it cannot read a root
   outside `--workspace` (`scripts/doc-gate.sh --print-roots` names
   seven) or code behind a feature it was not given.
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

### This door's precedent chain

`solid_of_face` cites no model, so the sibling-door re-census cannot
run on it. Its own provenance is in `docs/MODEL-AB-LOG.md`'s BOOL4
row: the door was minted in PR #2767's **fix pass**, from a bilateral
review finding — *"face→shell→solid spelled four times"* — and the
fold was scoped to that PR's own four sites. So the scoped doc claim
was never a survey; it was a report of one PR's reach, and the tree
had fourteen. **A door minted by a fix pass inherits that pass's
fence, and its doc sentence inherits it silently.** That is the
instrument this row leaves behind for the next door.

## What remains

| bucket | sites | owner | row |
| --- | --- | --- | --- |
| `topo/src` `scope_of_moves` | 1 | dup | THIS row, kept and guarded |
| `topo/tests` + `sweep/tests` | 5 | tint (with tcost) | `work/tint/the-face-to-solid-walk-is-spelled-per-test-file.md` |
| the verbatim fixture family | 5 helpers | dup | `work/dup/shell10-r2-probes-restates-the-scope-walk-fixtures-verbatim.md` |
| `faces_of` ×3 + `SolidFaces::of` | 4 | dup | `work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md` |
