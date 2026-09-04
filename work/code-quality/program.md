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
no two tracks edit one file, carry them. **Three of those tracks are
still this program's to dispatch; the other eight are executed by the
programs that own their ground, and the table under *Territory* below
says which.** Nothing here is ratified and
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

## Territory, and what this program does not execute

This program works inside every other program's territory, under the
per-track fences in `plan.md`, so it claims no paths of its own — and
where a fence and a program's `paths:` disagree, the program wins. Its
branch prefix is `smell/`. The retired spellings `smellc/`, `smellh/`
and `smelluv/` name closed tracks' branches and are not reused. Row and
finding numbers come from the per-track blocks in the header; Track J's
block (`D180`–`D199` / `S250`–`S269`) is reserved and not reissued.

**Three tracks are still this program's to dispatch — `K`, `P` and
`X`** — and they are the only three whose ground no other program's
`paths:` reaches. Measured 2026-09-04 against `git ls-files` and every
`work/*/program.md`: `K` 0 of 42 tracked files claimed, `P` 0 of 15,
`X` 11 of 144 (those eleven being `ciw`'s `demos/*.sh` and
`demos/*.py`). Re-derive it rather than trusting the figures; the point
they carry is the shape, not the numbers.

**The other eight tracks are ground this program does not own.** The
rows stay here — every claiming program says so in its own words — but
the lane that lands them is elsewhere, and a `smell/` branch on that
ground is a collision, not a lane:

| track | whose ground it is now | who executes the rows |
|---|---|---|
| `M` | `cert` (`geom-core/src/*`, `bvh/src/*`); `dual.rs` also `m10`'s, `k_stats.rs` also `props`' | `cert` — tracks M and N "claimed whole"; unit `CERT-M3` (dispatched) carries `H5` |
| `N` | `cert`, wholly — 33 of 33 files | `cert`'s `CERT-N3` (dispatched): `S235`, `D31`, `D98`, `D244`, `C24` |
| `Q` | `bool` (`boolean/`, `splitting/`), `verbs` (`ssi*`), `trim` (`pcurve_cache.rs`, `nurbs_iso.rs`, `edge_nurbs.rs`), `curved` and `mate` (`census.rs`) | `bool`'s `BOOL-Q` for the topo rows; `trim` for `D36`, `S394`, `S83`, `D305` as riders |
| `R` | `mesh` (`crates/mesh/` wholly), `cert`, `shell`, `curved`, `verbs` | `mesh`'s `MESH-R`; `props` claims R and N "at opening" as a successor and dispatches nothing until S-CERT's exit walk is ratified |
| `T` | `fillet` (`blend/`, `fillet.rs`, `chamfer.rs`, `extrude.rs`), `verbs` (`revolve/`), `bool` (`loft.rs`) | `fillet` — "Track T is claimed whole", rows `D320`–`D325` land as riders on its units |
| `U` | `exch` and `lib`, wholly — 285 of 285 files | `exch` for the STEP and STL rows (`D343`, `C13`, `C14`); the `pncad` and `pncad-py` rows are `lib`'s |
| `V` | `bool`, `m10`, `docm`, `seat`, `fix`, `fillet`, `shell`, `props` | no single unit; `docm` took `C6`, `D365`, `D366` and the `debug-in-prose` finding outright (2026-09-03) |
| `W` | `tcost`, wholly (`crates/*/tests/`, `crates/test-utils/`) | no unit claims the rows; the ground is `tcost`'s |

**A claim is recorded on the claiming side, and nowhere else.** Each
program above writes it in its own `keep_out` or in a named unit; no
row header here carries a field saying so, `work.py status` cannot show
it and `work.py lint` cannot check it. So a row inside a dispatched
unit still reads `open` on this board, and **the table above is the
only pointer back**. Read it before taking any row on a track it names,
and keep it current by hand when a program claims or releases ground —
the tracker has no mechanism that will do it for you.

`crates/sweep/src/skin.rs`, `swept.rs`, `test_support.rs` and `lib.rs`
are the residue of Track T's fence that no program's `paths:` reaches;
`D320` is the one row that lands there, and it also edits `loft.rs`,
which is `bool`'s.
