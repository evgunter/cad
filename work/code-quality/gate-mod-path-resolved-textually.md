---
id: gate-mod-path-resolved-textually
kind: issue
title: interval-square-allowlist resolves mod declarations to a sibling path, so it drops production files and scans test-only ones
status: open
opened: 2026-09-03
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
excludes itself — that half is planted as
`plant_every_source_excluded_by_itself` and is what makes the
`every source … is test-only` guard reachable (`D109(d)`).

**Not repaired where it was found.** Resolving the declaration properly
changes what the gate scans on the live tree, so it owes a hit-set diff
in both directions and a fixture per direction — its own row, not a
fixture's side effect. The population on the tree today is worth
measuring first: every `#[cfg(test)] mod` declaration that is not in a
`mod.rs`, a `lib.rs` or a `main.rs`.

## Was

`D109(d)`'s fixture pass — found while reaching the guard the row
recorded as unreachable.
