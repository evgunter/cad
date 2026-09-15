---
id: test-utils-now-holds-two-set-difference-comparators
kind: issue
title: test-utils holds two both-direction set comparisons: census::set_difference and roster::violations_against, landed the same day by sibling units
status: open
opened: 2026-09-15
---


Found by TINT-5 on its closing base merge, reading TINT-4's landed
`crates/test-utils/src/roster.rs` beside its own new
`crates/test-utils/src/census.rs`.

## The finding

Both units landed a **both-direction set comparison** into `test-utils`
within a day of each other, and neither knew about the other's.

- `census::set_difference` (TINT-5) — declared vs witnessed, returns
  `Option<String>`, prints only the direction that failed, with the
  caller supplying a sentence per direction. Promoted out of
  editor-core's test binary, where its own doc had argued *"the one set
  comparison in this file… writing that twice is a second copy of the
  comparator kept in step by hand"*.
- `roster::violations_against` (TINT-4) — rostered vs listed, returns
  `Vec<String>`, spells both directions inline: `unrostered` filters
  `mine` against `rostered`, `absent` filters `rostered` against `mine`,
  each with its own `push(format!(…))`.

The two filters in `violations_against` are `set_difference`'s two
filters, written again.

## Why this is not simply "call the shared one"

`violations_against` is not only a set comparison and its output shape
is not `set_difference`'s:

- It reports a **Vec** of violations, so a caller can print all of them;
  `set_difference` folds both directions into one string.
- It carries two directions the comparator does not have: a **vacuity
  floor** (a roster compared against nothing) and a **nested-module**
  partition, and both are reported as peers of the two set directions.
- Its sentences are long and name recourse; `set_difference`'s are the
  caller's one-liners.

So the fix is a judgement about which shape wins, not a substitution —
either `set_difference` grows a `Vec` form that `violations_against`
folds its two set directions into, or the two stay separate and each
doc names the other so a third comparator is not written blind.

## Why it is filed rather than fixed

`roster.rs` is TINT-4's landed work, merged hours before TINT-5's close;
reshaping either function's return type is a design call, and TINT-5's
spec decided its scope by name. What makes this worth its own row rather
than a PR-body note is that it is **observation 1 in
`work/tint/process-observations.md` happening a third time** — a program
whose subject is a mechanism kept in step by hand minting a second copy
of a comparator inside its own fixes — caught this time by a sibling
lane's base merge rather than by a style reviewer.
