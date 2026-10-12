---
id: the-dual-review-logs-shared-tally-line-conflicts-every-unit
kind: issue
title: docs/DUAL-REVIEW-LOG.md's one tally line and global DR numbering make every unit's merge conflict with every other program's
status: closed
opened: 2026-10-11
priority: P4
cost: M
closed: 2026-10-11
pr: 4647
---


Filed by the NURBS orchestrator. Friction finding, per
`memories/orchestration-model.md`: "Friction is a finding".

## What

Each dual-review unit's PR ends with a commit to `docs/DUAL-REVIEW-LOG.md`.
That commit does two things that every other program's last commit also does:

- **It takes the next global `DR-n` number,** assigned at merge time.
- **It rewrites one shared line,** the `**Tally: …**` foot. The foot carries
  the fair-pair list and the readout toward-count.

Every other program's DR commit edits that same line. So any two
in-flight units conflict there, and the loser renumbers its row and
re-adds itself to the foot.

## Measured (2026-10-10 / 11)

**Collisions.** PR 4553 (NURBS) conflicted on this file six times
between its first green CI and its merge. Its row was renumbered
DR-153 → DR-157 → DR-158 as INTENT's and GERM's rows landed. The
other NURBS units collided as well: PR 4540 went DR-148 → DR-149 → DR-150,
and PR 4518 went DR-143 → DR-144.

**Wall time.** Each collision on a code PR used to cost a full CI
run, because the docs-only exception in `orchestration-model.md`
was not being applied. The exception does let a log-only
resolution merge over a green head. Even so, each collision still
costs a fetch, a renumber, a foot re-count, a push, and a race with
the next lander.

**Damage already on main.** The foot's discipline has slipped:

- FLUX's DR-149 and DR-150 duplicate INTENT's and NURBS's numbers.
- FLUX's two rows, and GERM's DR-157 and INTENT's DR-162 / DR-163,
  sit below the ruling bullets rather than in the table.
- The toward-count has silently omitted a row at least once:
  INTENT's M-tier DR-149, which NURBS restored on 2026-10-10.

**CI time.** The per-PR `test` job ran 33–35 min on PR 4553
(geom-core, geom-brep and topo touched), against the 15-minute bar
in `orchestration-model.md`. CIW's `latency-cut.md` owns that
budget; the figure is recorded here because it multiplies every
collision above.

## Where to look (not designed)

The contention comes from two choices, and either could change:

- **A global sequence number.** Each program could number its own
  rows, with a prefix.
- **A tally stored as text in the same file as the rows.** The tally
  could instead be derived by a script from the rows, the way
  `work/STATUS.md` is rendered from the items.

A derived foot would also make the count's omissions impossible.
The protocol is `docs/DUAL-REVIEW-PROTOCOL.md`. Changing it is an
experiment-protocol change, so it waits on Ev.

## Closed

Fixed by PR 4647 (merged 2026-10-11, `0aa103bc2`), under Ev's 2026-10-11
"feel free to split into separate files". The second choice above is the
one taken. Each row is its own file,
`docs/dual-review-log/rows/PR-<n>.md`, so no two units touch one file.
Its DR number is assigned on main in merge order by the
`dual-review-log` workflow. The foot is computed from the rows' front
matter by `scripts/dual-review-log.py`, and `docs/DUAL-REVIEW-LOG.md` is
its render, which `work.py lint` refuses to see hand-edited.

The damage listed under "Measured" was migrated as follows:
- FLUX's duplicates were renumbered DR-155 and DR-159.
- The rows below the rulings now sit in the table.
- The computed foot restores the omissions: DR-123, DR-129, DR-132 and
  DR-161 among the M-tier units, and FLUX's two pairs among the fair
  pairs that found a MAJOR.

## Closed

Resolved by PR 4647 (`tooling/dual-review-log-per-row`, merged by Ev on
2026-10-11). Each row is now its own file,
`docs/dual-review-log/rows/PR-<n>.md`, so two PRs never share a file.
Its DR number is assigned on main in merge order by the
`dual-review-log` workflow. The foot is computed from the rows' front
matter, so it cannot omit a row. CI latency stays with CIW's `latency-cut`.
