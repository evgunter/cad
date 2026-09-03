---
id: code-quality
kind: program
title: Code quality — the structural findings register and its tracks
status: open
opened: 2026-08-18
area: infra
prefix: smell/
tag: (SMELL orchestrator)
blocks: [K D200-D219 S270-S289, M D220-D239 S290-S309, N D240-D259 S310-S329, P D260-D279 S330-S349, Q D280-D299 S350-S369, R D300-D319 S370-S389, T D320-D339 S390-S409, U D340-D359 S410-S429, V D360-D379 S430-S449, W D380-D399 S450-S469, X D400-D419 S470-S489]
---

# Code quality

The register of structural findings about the kernel — parts that play
almost-but-not-quite parallel roles, code more complex or more indirect
than the job needs, things that do not look like the way you would do
it — and the tracks that land them. Scans and the lanes that run off
them raise findings; Tracks K–X, partitioned by file territory so that
no two tracks edit one file, carry them. Nothing here is ratified and
nothing here is a commitment: a finding is a question worth answering,
not a defect, and the ratified design contract is `docs/DESIGN.md`.

## How a finding lives here

- **One file per row or finding.** The file's id is the row id it is
  cited by (`D102`, `S330`, `C15`), and a row's header carries its
  `track:` letter. A live finding no row cites is a `kind: issue` file;
  a question only Ev answers is a `kind: ruling` file and is never work.
- **A finding leaves the tracker only by landing.** The merged PR is its
  record and the file closes in the same PR. A finding only partly
  closed stays open with its closed members deleted; a note saying the
  rest completed is itself a thing to delete.
- **A finding routed to an already-dispatched lane is its own file**
  with `rides_with:` naming the carrier row, and the carrier cannot
  close while it is live.
- **The rules are `plan.md`**: the numbering, how to read a finding and
  an item, the ordering rules, the partition rules, the territories and
  blocks, the stated seams, what the partition leaves out, and the
  sweeps that go last. Process observations `C1`–`C27` are
  `process-observations.md`; the closed tracks' execution records are
  under `logs/`.

## Territory and branches

This program works inside every other program's territory, under the
per-track fences in `plan.md`, so it claims no paths of its own. Its
branch prefix is `smell/`. The retired spellings `smellc/`, `smellh/`
and `smelluv/` name closed tracks' branches and are not reused. Row and
finding numbers come from the per-track blocks in the header; Track J's
block (`D180`–`D199` / `S250`–`S269`) is reserved and not reissued.
