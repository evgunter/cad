---
id: gate-mod-path-resolved-textually
kind: issue
title: interval-square-allowlist resolves mod declarations to a sibling path, so it drops production files and scans test-only ones
status: closed
opened: 2026-09-03
branch: gates/mod-path-resolution
pr: 2033
refs: [test-module-resolution-has-three-homes]
closed: 2026-09-06
---


## Finding

`scripts/gates/interval-square-allowlist.sh:186-192` builds its test-only
exclusion set by string concatenation from the **declaring file's own
directory**:

```bash
dir=${decl%%:*}; dir=${dir%/*}
name=${decl##*:}
excl+=("$dir/$name.rs" "$dir/$name/")
```

`rustc` does not resolve `mod bar;` that way. Inside a non-`mod.rs`,
non-crate-root file `dir/foo.rs`, the declaration resolves to
`dir/foo/bar.rs`; the gate excludes the **sibling** `dir/bar.rs`. Both
directions of that mismatch are live, measured 2026-09-03 under a scratch
`--root` (four files: `crates/x/src/lib.rs`, `crates/x/src/foo.rs`
carrying `#[cfg(test)]\nmod bar;`, `crates/x/src/bar.rs`, and
`crates/x/src/foo/bar.rs`):

- **Under-scan, and it is the silent one.** `crates/x/src/bar.rs` is
  ordinary production code declared test-only by nothing, and it is
  dropped: with `pub fn sq(v: f64) -> f64 { v * v }` in it the gate
  reports `OK … (3 source files scanned)`. Delete the declaration in
  `foo.rs` and the same file reds. So a production file leaves the scan
  because an unrelated sibling declared a test module of the same name.
- **Over-scan.** The file the declaration actually names,
  `crates/x/src/foo/bar.rs`, is **not** excluded, so its test-only square
  is read as production and the gate cries wolf on it.

The exclusion also has no check that the target differs from the
declarer, which is why one file declaring a module of its own name
excludes itself — the half that reached the `every source … is
test-only` guard (`D109(d)`).

**Not repaired where it was found.** Resolving the declaration properly
changes what the gate scans on the live tree, so it owes a hit-set diff
in both directions and a fixture per direction — its own row, not a
fixture's side effect. The population on the tree today is worth
measuring first: every `#[cfg(test)] mod` declaration that is not in a
`mod.rs`, a `lib.rs` or a `main.rs`.

## Was

`D109(d)`'s fixture pass — found while reaching the guard the row
recorded as unreachable.

## Landed (PR 2033)

Resolution now follows rustc: `#[path = "P"]` wins (relative to the
declaring file's directory at top level, to the inline module's
directory inside one); a crate root or `mod.rs` declarer resolves to
the sibling; any other declarer `dir/foo.rs` into `dir/foo/`; an
enclosing inline `mod y { … }` adds `y/`; an inline `mod x { … }`
mounts no file; a declaration naming its own file excludes nothing;
and a declaration the reader cannot place is refused loudly rather
than guessed at. The shape is read from the code-only view and only a
`#[path]` payload from the raw line the view names. The exclusion
filter is anchored (whole path for a file entry, prefix for a
directory) instead of `grep -F`'s substring. Live hit set: 397 → 395
scanned, the two files being
`crates/topo/src/boolean/solid_contain/r1_generic_poses.rs` and
`crates/topo/src/chart_region_r2_probes.rs`, both test modules their
declarations name; nothing entered the scan and the gate's verdict is
unchanged. The `every source … is test-only` guard is reached by
`plant_every_source_excluded_by_a_sibling` now that self-exclusion is
guarded.

### Residue

- The two other textual copies of this resolution, and the excluded-path
  filter's third home, are `test-module-resolution-has-three-homes`.
- Known gaps, each stated with its direction in the gate's header: a
  declaration split over lines (over-scan, and refused loudly once its
  file is a candidate), `#[cfg_attr(…, path = …)]` (both directions),
  and a declaration written by a macro or `include!`d (over-scan).

## Claimed by GATES (2026-09-06)

Moved from `work/code-quality/` to `work/gates/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, `track:` letter and body unchanged. Unlettered, on K's fence: `interval-square-allowlist.sh`.
