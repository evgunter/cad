---
id: compile-fail-blocks-without-error-codes
kind: issue
title: Bare compile_fail doctest blocks accept any compile error: 16 blocks without an error code
status: review
opened: 2026-09-05
refs: [1969]
branch: fix/compile-fail-error-codes
pr: 2335
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

## Re-homed (2026-09-06)

Moved from `work/issues/` to `work/fix/` in the tracker-wide cut of 2026-09-06 (Ev's direction, in-chat), which read every open `work/issues/` file and every open code-quality row against every live program's `paths` and opened four programs for the ground none covered. Id, body and header are unchanged except as noted; the directory is the claim (`work/README.md`). All eight bare fences are in `crates/quantity/src/units.rs`, FIX's glob; one PR with the fix written.

## What landed

All eight bare fences in `crates/quantity/src/units.rs` now name the
code `rustc` 1.97.0 actually emits for their snippet, each measured by
compiling the snippet standalone against the built `quantity` rlib
rather than guessed:

- `UnitDef` struct literal, and the struct-update escape — `E0451`
  (*fields `symbol`, `quantity` and `factor` of struct `UnitDef` are
  private*).
- `LengthUnit(0)` / `AngleUnit(4)` — `E0423` (*cannot initialize a
  tuple struct which contains private fields*), not `E0451`: a tuple
  struct's private field refuses at the constructor path.
- `MM.0` / `DEG.0` read, and the `mm.0 = 4` / `deg.0 = 5` write —
  `E0616` (*field `0` … is private*).

Each block already had a legal twin (one per group of blocks, differing
in exactly one respect: the twin never names a field), so no twin was
added; the surrounding prose now says that stable rustdoc does not
verify the annotation, which is what the twin is for, and names the
codes it was previously asserting only as "a PRIVACY refusal". No block
needed the "cannot" escape.
