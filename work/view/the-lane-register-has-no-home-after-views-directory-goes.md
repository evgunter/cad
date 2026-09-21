---
id: the-lane-register-has-no-home-after-views-directory-goes
kind: issue
title: VIEW's lane register binds four successor programs by reference and dies with work/view/plan.md
status: open
opened: 2026-09-17
priority: P4
cost: E
---


Disclosed by the 2026-09-17 re-scope, as its own file rather than as a
sentence in a plan, because `work/README.md` says the sweep sees items
and not sentences.

## The finding

`work/view/plan.md` is two documents in one file. The first is this
program's plan — status, territory, Order, exit shape. The second is a
**rule register**: roughly six hundred lines of operational discipline,
every rule of which is a named failure at a named PR (the citation
rules, the census and proxy rules, the CI-tier rules, the lane-isolation
rules, the `desired_width` rule, the δ round-trip rule). It is handed to
every lane this program dispatches and it is the reason its later waves
cost less than its early ones.

The four successor programs opened on 2026-09-17 — `vnews`, `vgeom`,
`vseam`, `vdoc` — each **inherit that register by reference**, in their
`plan.md` §The register. The choice was deliberate and its argument is
written there: four copies of a register that is re-derived every wave
give four divergent copies inside a week, which is this program's own
count-fixed-in-one-place defect applied to its own discipline; and a
rule detached from the PR that paid for it reads as a rule without its
receipt.

## Why that is a row and not a footnote

`work/README.md`: *a closed program's directory is deleted … `program.md`,
`plan.md` and `log.md` go*. So the day VIEW's exit walk is ratified,
`work/view/plan.md` goes, and four live programs are left pointing at a
path that resolves only at a SHA in `docs/DOC-LEDGER.md`. A lane told to
read its discipline out of the ledger's recoverable history will not.

**This is a precondition of VIEW's exit walk, not a follow-up to it.**
The walk cannot be ratified while four open programs depend on a file it
deletes.

## What is undecided

Where the register goes. The candidates, none of them chosen here:

- `docs/prompts/` — the standing discipline handed to every lane by
  path, which is what the register in fact is. CLAUDE.md makes that
  directory Ev's call, so this route is an `[ev]` PR.
- `crates/viewer/README.md` — CLAUDE.md's home for finished-work design
  beside the code. Wrong shape: the register is process, not design, and
  most of its rules are not about the viewer at all.
- One of the four successors' `plan.md`, with the other three
  referencing it. Moves the problem to that program's own exit.
- A fifth file under `work/` with no program — `work/README.md`'s layout
  does not admit one today, which makes this a tracker question and
  therefore META's.

Splitting it is also on the table: the rules about THIS CRATE stay with
the viewer programs, and the rules about lanes, CI tiers, citations and
censuses — which bind every program in the tree and are re-derived
independently in several plans already — go wherever the general
discipline lives.
