---
id: the-face-to-solid-walk-is-spelled-per-test-file
kind: issue
title: The face to shell to solid walk is spelled once per test file in topo/tests and sweep/tests (tint and tcost ground)
status: open
opened: 2026-09-19
priority: P4
cost: E
---


## Finding

- **Where**: `crates/topo/tests/bool4r1_probes.rs` (~:199),
  `crates/sweep/tests/shell5_r1_probes.rs` (~:533),
  `crates/sweep/tests/shell8_common.rs` (`solid_of`, ~:55),
  `crates/sweep/tests/shell8_r1_probes.rs` (`solid_of`, ~:40),
  `crates/sweep/tests/shell8_r2_probes.rs` (`solid_of`, ~:61) — the
  last three are **byte-identical two-line function bodies under one
  name in three files of one suite**.
- **Importance**: medium
- **Confidence**: sure. All five were read; `shell8_common.rs`,
  `shell8_r1_probes.rs` and `shell8_r2_probes.rs` were read against
  each other.
- **Raised by**: the `solid_of_face` fold, 2026-09-19, announced by
  seam from
  `work/dup/solid-of-face-has-eleven-hand-written-walks-outside-it.md`.

One walk — face → its `Face::shell` → that shell's `Shell::solid`.
`pncad::topo::Body::solid_of_face` is `pub`, total and `#[must_use]`,
and `topo/src` now reads through it. The suites still spell it out,
under `unwrap` where a stale key is a test failure and no refusal is
distinguished, so the posture question that keeps
`offset_together::scope_of_moves` out of the fold does not arise at any
of these five.

`shell8_common.rs` is already the suite's shared module and already
carries `solid_of`; `shell8_r1_probes.rs` and `shell8_r2_probes.rs`
each restate it rather than importing it. The `faces_of(body, solid)`
helper beside it is copied the same three times.

## Blind spot of the census that produced this

A type-directed probe (`#[deprecated]` on `Face::shell` and
`Shell::solid`, primary warning spans paired within 8 lines, deduped
by `(file, line)`, `--workspace --all-targets --features topo/interval`)
plus two structural regexes and a name census over every tracked file.
The probe cannot see a root outside `--workspace`, and the regexes
cannot see a walk split across a function boundary. Five is a floor.

## Territory: tint AND tcost

`scripts/work.py territory` puts all five paths on **tcost** as well
as tint (`verbs_shell.rs` adds shell). The row is filed on tint's
slate because the subject is duplicated test text, but a tcost lane
opening any of these files owns the same ground and should read it.

## One hop short: five `shell_of` closures in the same suites

`sweep/tests/shell5_r1_probes.rs` (~:200),
`sweep/tests/shell5_r2_probes.rs` (~:100), `sweep/tests/verbs_shell.rs`
(~:490 and ~:542) and `topo/tests/m3_pr3_split.rs` (~:384) each spell
`body.get_face(f).expect(..).shell` as a local closure — the same
duplication one hop short of this class, and no door exists for that
hop either (`Face::shell` is a plain field read after `get_face`).
Recorded here rather than as a sixth row: the fix for the five
`solid_of`/walk sites and the fix for these five closures is the same
decision about where a suite's per-file topology helpers live.

**Two of the sites the parent row listed are NOT members** and are not
carried here: `crates/sweep/tests/revolve_ring.rs` (~:58) and
`crates/sweep/tests/verbs_tubewall.rs` (~:165) read `.solid` off a
shell key a HANDLE already holds (`t.cavities[0]`, `t.shell`). No face
is walked; the parent row's regex matched the handle field's name.

## 2026-09-20: the `faces_of` half of this row now has a door

`Body::faces_of_solid(solid) -> Option<Vec<FaceKey>>` landed on the
dup side with `work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`,
and it is `pub` on `topo::Body` like `solid_of_face`. So the
`faces_of(body, solid)` helper this row records beside `solid_of` in
`shell8_common.rs`, `shell8_r1_probes.rs` and `shell8_r2_probes.rs` is
no longer a "which file hosts the shared copy" question: all three
bodies are the door, and the fix is to delete all three and call it.

Two things a lane folding them should know, both measured on the
`topo/src` side of the same fold:

- The door answers **`None` for a solid key the body does not hold**,
  so each call site grows an `.expect`, and deleting a one-line
  wrapper multiplies the unwrap it used to hold once. Where a row asks
  for the same solid's faces twice, bind it once rather than
  re-asking. What that cost on the `topo/src` side is counted in
  `work/dup/listing-a-solids-faces-is-spelled-four-times-in-topo-src.md`'s
  X4 section, dated; it is not restated here.
- The `topo/src` copies carried `.expect("a live face names its solid")`
  *inside* the filter, so a face whose shell had gone would panic;
  these three compare `solid_of(body, k) == solid` and would silently
  skip it. The door takes the second reading — the face belongs to no
  solid, which is what `solid_of_face` already answers about it — and
  no row in `topo` depended on the first.

`crates/sweep/tests` was another lane's live ground on 2026-09-20, so
the dup unit did not touch these three.

**Fix pass (2026-09-26, PR #3284).** The `shell8_r1_probes` and `shell8_r2_probes` copies of
`solid_of` (and of `faces_of` / `charts_of`) are gone; both files
import `shell8_common`'s, and `shell8_common::solid_of` / `faces_of`
now read `Body::solid_of_face` / `Body::faces_of_solid` (a panic
planted in each door for that caller reds 4 and 16 of the `shell8`,
`shell9` and `shell10` rows). No `shell8` spelling of the walk
remains; the other members of this row are untouched.
