---
id: test-support-convention-has-no-prose-home
kind: issue
title: The test-support feature convention is restated in four manifests and has no prose home
status: open
opened: 2026-09-17
priority: P3
cost: E
---


## Finding

- **Where**: `crates/topo/Cargo.toml`, `crates/profile/Cargo.toml`,
  `crates/sweep/Cargo.toml` and `crates/editor-core/Cargo.toml` —
  each crate's `[features]` `test-support` line and the comment block
  above it
- **Importance**: low
- **Confidence**: sure — the four comment blocks were read side by side
- **Raised by**: the fix pass of PR #2801 (`edit/raw-target-test-support`),
  whose reviewer named it S3

Four manifests declare a `test-support` cargo feature, and each one
argues the same convention from scratch in its own prose:

- **what the feature is for** — a `tests/` file is a separate crate, so
  it can name neither a `#[cfg(test)]` item nor a `pub(crate)` one, and
  without the feature every suite mints its own copy of the vocabulary
  and the copies drift (`topo`, `sweep`, `profile`, all three citing
  `S52`);
- **how it is turned on** — off by default, enabled only through a self
  dev-dependency, so it is on for the crate's own test targets and off
  for every normal build (all four);
- **when it must forward `profile/test-support`** — `sweep` and
  `editor-core`, each deriving the rule again from its own evidence
  (`sweep` from an `E0603`+`5×E0599` red on `-p verbs --all-targets`,
  `editor-core` from a census row).

There are two instruments and no prose home. The convention's
*machine-checked* half lives in
`crates/profile/tests/raw_door_census.rs`'s
`every_crate_that_names_the_door_reaches_it` (a crate whose `src/`
names profile's raw door and declares its own `test-support` forwards
profile's) and in `scripts/gates/test-features-dev-only.sh` (no non-dev
dependency edge in any manifest in the repository enables a test-only
feature, and no ordinary feature forwards to one). Its *argued* half —
why a dev-dependency edge is the sanctioned door, what
`cargo build --all-targets` and a `--workspace` test run each unify,
what a feature can and cannot claim about a consumer — is written
nowhere that either instrument points at, and so is re-derived per
manifest.

The cost is the ordinary one for a restated argument: the four copies
already differ in what they claim, and one of them was wrong. Before
this PR's fix pass, `editor-core`'s comment justified the forward with
"a feature that cannot compile the tests it is named for", which is
false — that crate's `profile` dev-dependency already carries
`test-support`, so its own test targets build either way, and what
actually reds is the census row. A convention with no home cannot be
corrected once.

## What a fix would look like

One prose home for the convention — the natural candidate is a section
in `scripts/gates/test-features-dev-only.sh`'s header, which already
carries the longest version of the argument and is cited by the gate
step — with each manifest comment reduced to what is local to that
crate (which door the feature opens, which crates' fixtures need it)
plus a pointer. Not attempted here: the four manifests belong to four
programs' ground, and this PR's fence is `editor-core`'s.

## Where it stands

No program's path list covers "the convention across four crates", so
this is filed on `work/issues/` rather than on a slate.
`python3 scripts/work.py territory --files -` reports the four
manifests under different owners.
