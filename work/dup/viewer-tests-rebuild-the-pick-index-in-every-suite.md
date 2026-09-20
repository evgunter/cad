---
id: viewer-tests-rebuild-the-pick-index-in-every-suite
kind: issue
title: Fourteen private spellings of one PickIndex build in crates/viewer/tests
status: closed
opened: 2026-09-20
closed: 2026-09-20
branch: dup/viewer-shared-doors
pr: 2929
---


## Finding

- **Where**: `crates/viewer/tests/`, fourteen private helpers in
  thirteen files, under six names — `index_of` (`blend_authoring`,
  `common/asm`, `edge_pick`, `focus_highlight`, `frame_policy`,
  `select_pick`), `fresh_index` (`index_memo`, `pick3_acceptance`,
  `review_pick2_r1`, `review_pick_r2`), `index_at` (`review_gui2_r1`,
  `review_gui2_r2`), `indexed` (`pick_windows`, `select_pick`).
- **The construction, which is one**: `session.landed_pair()`, then
  `session.landed_generation()`, then
  `PickIndex::build(doc, eval, PictureKey::of(generation, δ), session.tol())`.
  `crates/viewer/src/pane/viewport.rs` writes the same four arguments,
  so the shipped caller is a fifteenth site and the natural home is a
  `tests/common` door over it rather than a copy of it.
- **What makes the fold a judgement and not a substitution**: the δ.
  Each suite passes its own display tolerance and several take it from
  a private `delta()` chosen for that suite's geometry, so a shared
  door takes δ as a parameter and the per-suite value stays at the call
  site — the same split this row's parent settled for the fixtures.
  Three of the fourteen also open the session themselves
  (`pick_windows::indexed`, `select_pick::indexed`,
  `review_gui2_r1::landed`), which is a second door, not this one.
- **Importance**: medium. No oracle rides on it — a bug in the shared
  build would red every pick row in the crate, which is exactly the
  measurement the fold would need.
- **Instrument, and its blind spot**: `git grep -n 'PickIndex::build'`
  over every tracked file with no path argument, then a per-function
  read of each hit. It cannot see a site that reaches the index through
  `PickIndex::build_with` (`crates/viewer/src/evalseam.rs` does), nor
  one assembled by a macro.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled fourteen times
— is S-DUP's charter. Any of the five may claim it by `git mv`.

## Closed 2026-09-20 — re-taken at `cd9fdfd6b`, folded onto `common::{index_at, index_of}`

### The census, re-taken

`git grep -n 'PickIndex::build'` over every tracked file, **no path
argument**, then a read of each enclosing function. **14 textual sites
in 13 files** — the row's count held. Two things in the row did not:

- **"six names" is wrong; there are four.** The row's own list
  enumerates `index_of`, `fresh_index`, `index_at` and `indexed` and
  then calls them six. Four is the number of distinct functions that
  CONTAIN a build; `review_gui2_r1::index_of` and
  `review_gui2_r2::landed_index` are one-line forwarders onto
  `index_at` and contain none.
- **"fourteen PRIVATE helpers" is thirteen private and one shared.**
  `common/asm.rs`'s `index_of` is `pub` in `tests/common/` already —
  it was the home for the assembly suites, with their δ baked in.

The hit list and its disposition:

| site | disposition |
| --- | --- |
| `blend_authoring::index_of` | folded → `common::index_of(session, common::pick_delta())`; the private `delta()` went with it |
| `edge_pick::index_of` | folded, same |
| `focus_highlight::index_of` | folded; its own δ (5×10⁻⁴) is not this class's value and stays |
| `frame_policy::index_of` | folded |
| `select_pick::index_of` | folded |
| `select_pick::indexed` | folded — it opened a session and then rebuilt the construction; now `index_of(&session)` |
| `pick_windows::indexed` | folded, same shape |
| `pick3_acceptance::fresh_index` | folded |
| `review_pick_r2::fresh_index` | folded |
| `review_pick2_r1::fresh_index` | folded |
| `index_memo::fresh_index` | folded onto the **`Result` arm**, `common::index_at` — its rows read the refusal, which is why the door has two arms rather than one |
| `review_gui2_r1::index_at` | folded; the suite keeps `index_at(session, δ)` as its own one-line binding |
| `review_gui2_r2::index_at` | folded, same |
| `common/asm.rs::index_of` | folded — it is now `super::index_of(session, delta())`, an adapter binding the assembly's δ |

**The blind spots the row named, run rather than restated.**
`PickIndex::build_with` — `git grep 'build_with'` over every tracked
file: no `crates/viewer/tests/` site calls it; the two hits in that tree
are comments, and the one real caller is `crates/viewer/src/evalseam.rs`.
Macro-assembled: none — every hit is textual. **The converse needle**
for a class whose members "build and take the result" is the set of
other PickIndex PRODUCERS: `git grep 'PickIndex' --
crates/viewer/tests/` read for `-> PickIndex` and `-> Result<PickIndex`
names three more (`index_memo::seam_index`, `seam_index_at`, and
`review_gui2_r1::landed`), of which the first two are the SEAM door and
not this construction, and the third is a session opener that now calls
the folded one.

### The proof

See the mutation table in
`viewer-tests-spell-one-display-tolerance-in-seven-places`. The two
plants that isolate this door:

| plant in `common::index_at` | direction | total | reds |
| --- | --- | --- | --- |
| indexes at 5×10⁻² whatever δ it was handed | coarsen at the door, so the index disagrees with what its caller keyed | 616 / 10 | `index_memo` 5, `review_gui2_r2` 2, `pick3_acceptance` 1, `review_pick_r2` 1, `select_pick` 1 |
| keys the picture on `generation.next()` | harder — the index claims to picture a run that has not happened; nothing is made easier | 620 / 6 | `review_gui2_r1` 2, `review_gui2_r2` 2, `frame_policy` 1, `select_pick` 1 |

Seven of the thirteen suites are plant-proved on the door. The other
six (`blend_authoring`, `edge_pick`, `focus_highlight`, `pick_windows`,
`review_pick2_r1`, and the assembly suites through `asm::index_of`) are
**compiler-proved only** at the door — their private helper is deleted
and the call is type-checked — though the picture they pick against is
itself live: aiming `common::down_from` under the fixture reds
`edge_pick` 2 and thirteen suites' worth besides. `focus_highlight` and
`pick_windows` appear in NO red set across thirteen plants and are
filed as
`work/tint/viewer-plate-suites-index-at-a-display-tolerance-nothing-asserts`.
