---
id: the-face-to-solid-walk-is-spelled-per-test-file
kind: issue
title: The face to shell to solid walk is spelled once per test file in topo/tests and sweep/tests
status: open
opened: 2026-09-19
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

**Two of the sites the parent row listed are NOT members** and are not
carried here: `crates/sweep/tests/revolve_ring.rs` (~:58) and
`crates/sweep/tests/verbs_tubewall.rs` (~:165) read `.solid` off a
shell key a HANDLE already holds (`t.cavities[0]`, `t.shell`). No face
is walked; the parent row's regex matched the handle field's name.
