---
id: compile-fail-blocks-without-error-codes
kind: issue
title: Bare compile_fail doctest blocks accept any compile error: 16 blocks without an error code
status: open
opened: 2026-09-05
refs: [1969]
---

## What

A bare ```` ```compile_fail ```` doctest block accepts ANY compile
error: renaming the function the snippet calls keeps it green, so the
block stops pinning the property it was written for. The convention
(the Span unit; `k_stats::Bracket`'s `!Send` pin in PR #1969) is
```` ```compile_fail,E0277 ```` — the error code named — beside a legal
twin that differs in one respect and compiles, since stable rustdoc
does not verify the code annotation and the twin is what makes the
block honest.

## The list, measured on PR #1969's head

`grep -rn '\`\`\`compile_fail' crates/*/src | grep -v 'compile_fail,'`
— **8 bare fences, all in one file**:

- `crates/quantity/src/units.rs:110`, `:120`, `:352`, `:358`, `:364`,
  `:393`, `:397`, `:401`

The review that filed this counted 16 across `quantity/src/units.rs`,
`topo/src/{validate,live,review_m0_pr7}.rs` and `pncad/src/profile.rs`;
on this head the `topo` blocks carry codes (`validate.rs:2409` E0277,
`review_m0_pr7.rs:47`, `:59` E0308) and the `live.rs` / `pncad`
mentions are prose about a block, not a block. What the grep cannot
see: a fence spelled ```` ```rust,compile_fail ```` (none found), or a
block in a `#[doc = ...]` attribute string.

## Acceptance

Each bare block names its code and gains a twin, or says why it
cannot (a `pub(crate)` type a doctest cannot name — `topo/src/live.rs:29`
records that case).
