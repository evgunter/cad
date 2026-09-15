---
id: an-announced-seam-is-recorded-only-on-the-crossing-programs-side
kind: issue
title: A program's keep_out announces its crossings into another program's files, and the owning program has no record of them
status: open
opened: 2026-09-12
---



(WIRE orchestrator, 2026-09-12) Surfaced by D364's lane (PR 2445),
which asked whether its crossing into `crates/editor-core/src/program.rs`
was "on the record, since I made it only in the PR body". It is — and
checking that turned up the general hole.

## The instance

`work/wire/program.md`'s `keep_out` says:

> `crates/editor-core/src/names/role.rs` and `node.rs` and `program.rs`
> and `edit.rs` and `persist/*` are DOCM's — **D364 and S195 reach
> `res_spec` and `res_target` in `program.rs` and are announced there
> rather than landed from here**

That is a pre-authorisation naming the two rows and the two functions,
written at WIRE's opening. The lane's PR-body announcement is the
per-crossing notice on top of it. Nothing more was owed, and the
answer to the lane is "yes".

**But `work/docm/program.md`'s `keep_out` does not name WIRE.** It
names code-quality Track V, M10, SEAT, S-BOOL, LIB, S-MATE, CHROME and
VIEW — not the program that has two rows aimed at its `program.rs`. So
a DOCM lane reading its own program file has no way to learn that
`res_target` is being rewritten this week.

## Why no check sees it

`work/README.md:196-202` has the reciprocity rule — *"Two open programs
may claim one path only if BOTH `keep_out`s name the other"* — and
`lint` measures it by intersecting every open program's **`paths`**
globs against `git ls-files`. This seam is invisible to that check for
a reason that is not a bug in the check: WIRE does not **claim**
`program.rs`. It disclaims it, in prose, and then announces that it
will edit it anyway. There is no glob overlap to measure, so `lint`
is silent, correctly by its own rule and uselessly for the hazard.

The asymmetry is the same shape as the reciprocity rule was written to
catch — *"an overlap written on one side or neither is a live conflict,
and the program that was there first is the one that cannot see it"* —
arriving through the door the rule does not watch. An announced seam is
structurally a claim; it is just spelled as a disclaimer plus an
exception.

## The scale

This is not one clause. WIRE's `keep_out` carries five such crossings
(`program.rs` for D364/S195; `crates/profile/*` for the two lift rows,
"by announced seam as EVAL and FILLET both did"; `crates/sweep/*`;
`assembly.rs`; `topo/src/source.rs`), and the EVAL and FILLET precedents
it cites mean the pattern predates this program. Other programs'
`keep_out`s read the same way — DOCM's own says *"the check-registry
subject edits `product.rs` by announced seam"*, which is a crossing
**into WIRE's `paths`** that WIRE's `keep_out` does not name either. So
the hole is symmetric and already live in both directions between these
two programs.

## What is owed

A decision about where an announced crossing is recorded, not a bigger
lint. The candidates:

- **Write the reciprocal clause when the seam is announced** — the
  crossing program's orchestrator adds a line to the owning program's
  `keep_out`. Cheapest, and it makes `lint`'s existing rule cover the
  case the day both sides are `paths`-overlapping. Costs: one-file-one-item
  means editing another program's `program.md`, which is exactly the
  crossing being recorded, recursively.
- **A `reaches:` field** on the program header — globs the program will
  edit without claiming — and teach `lint` to warn when a `reaches:`
  entry is not named in the owning program's `keep_out`. Machine-readable,
  and it distinguishes "I claim this" from "I will touch this", which
  the prose currently conflates.
- **Leave it prose and accept it**, on the grounds that orchestrators
  read the full `work/` tree and the PR body announces each crossing.
  This is today's answer by default; it is worth stating deliberately
  rather than by omission, since the reciprocity rule's own text says
  why one-sided records fail.

The middle option is the one I would argue for, but the choice binds
how every program header is written from here, so it is
`work/README.md`'s to settle rather than any one program's.
