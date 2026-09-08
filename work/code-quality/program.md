---
id: code-quality
kind: program
title: Code quality — where a structural finding waits until a program claims it
status: open
opened: 2026-08-18
area: infra
prefix: smell/
tag: (SMELL orchestrator)
blocks: [K D200-D219 S270-S289, M D220-D239 S290-S309, N D240-D259 S310-S329, P D260-D279 S330-S349, Q D280-D299 S350-S369, R D300-D319 S370-S389, T D320-D339 S390-S409, U D340-D359 S410-S429, V D360-D379 S430-S449, W D380-D399 S450-S469, X D400-D419 S470-S489]
---

# Code quality

**Where a structural finding waits until a program claims it.** Parts
that play almost-but-not-quite parallel roles, code more complex or more
indirect than the job needs, things that do not look like the way you
would do it: scans and the lanes that run off them raise these, and they
land here because at the moment they are raised no program owns them.
This directory is that holding ground and nothing more — **it is not a
register, and a row does not live here for its lifetime**. Tracks K–X
group the rows by file territory so that a program can see at a glance
which of them are on its ground.

**A row leaves the moment a program claims it**, by moving into that
program's directory keeping its id, `track:` letter and body, with
`parent:` naming the unit that carries it where the program has cut
one. From then on the claiming program's board is where it is open,
dispatched and closed, and this directory does not track it at all.
What stays here is what nobody has claimed.

Nothing here is ratified and nothing here is a commitment: a finding is
a question worth answering, not a defect, and the ratified design
contract is `docs/DESIGN.md`.

## How a finding lives here

- **One file per row or finding.** The file's id is the row id it is
  cited by (`D102`, `S330`, `C15`), and a row's header carries its
  `track:` letter. A live finding no row cites is a `kind: issue` file;
  a question only Ev answers is a `kind: ruling` file and is never work.
- **A finding leaves this directory two ways: claimed, or landed.**
  Claimed, it MOVES — see the charter above; the claiming program owns
  it from then on and closes it on its own board. Landed from here, the
  merged PR is its record and the file closes in the same PR. A finding
  only partly closed stays open with its closed members deleted; a note
  saying the rest completed is itself a thing to delete.
- **A finding routed to an already-dispatched lane is its own file**
  with `rides_with:` naming the carrier row, and the carrier cannot
  close while it is live.
- **The rules are `plan.md`**: the numbering, how to read a finding and
  an item, the ordering rules, the partition rules, the territories and
  blocks, the stated seams, what the partition leaves out, and the
  sweeps that go last. Process observations `C1`–`C27` are
  `process-observations.md`; the closed tracks' execution records are
  under `logs/`.

## Territory, and what has been claimed away

This program claims no paths of its own: its per-track fences in
`plan.md` are drawn inside other programs' territory, and where a fence
and a program's `paths:` disagree, the program wins. Its branch prefix
is `smell/`. The retired spellings `smellc/`, `smellh/` and `smelluv/`
name closed tracks' branches and are not reused. Row and finding
numbers come from the per-track blocks in the header — **the block
ledger stays here after a row leaves**, so a program minting a new row
on a track it has claimed takes the next number from that track's block
and files the row in its own directory. Track J's block (`D180`–`D199`
/ `S250`–`S269`) is reserved and not reissued.

**One track is still this program's to dispatch — `X`** (`demos/`;
three rows, one parked on `L2`). `K` was claimed whole on 2026-09-06
by two programs opened for it, `gates` (`scripts/gates/*`) and
`meter` (`tools/*` and the two instrument documents); **`gates` closed
2026-09-08** (`docs/DOC-LEDGER.md`, sweep 7) and its half of the fence,
`scripts/gates/*`, is this program's again, with two rows re-homed here
(`D212`, riding `G4`; `directory-prefix-skips-have-no-subject-check`,
open on an `[ev]` ruling); `T`'s remainder
on `crates/sweep/src` went to `blend`, FILLET's successor, the same
day; `P` and `W` left on 2026-09-04. `V`'s ground is spread across
DOCM, EVAL (the eval seat, opened 2026-09-06) and six others, and its
rows on S-BOOL's `crates/profile` glob (`D364`, `G4`) and DOCM's
`assembly.rs` (`S190`, whose trigger #855 has fired) still sit here.

**Six programs have claimed rows off this board and now carry them**
(2026-09-04, 35 rows):

| program | rows | carried by |
|---|---|---|
| `cert` | `S235`, `D31`, `D98`, `D244`, `C24` (N); `H5` (M) | `CERT-N3`, `CERT-M3` |
| `bool` | `G9`, `S173`, `H11`, `S234`, `D95`, `D280`, `D66`, `D284`, `D287`, `D57`, `D46`, `D281` (Q) | `BOOL-Q` |
| `mesh` | `S28`, `S236`, `S237`, `D300`, `D303`, `D304`, `C23`, `C3`, `D30` (R) | `MESH-R` |
| `trim` | `D36`, `S83`, `S394`, `D305` | riders on its units |
| `fillet` | `D322`, `D325`, `D326` (T) | riders on its units |
| `exch` | `D343` (U) | its `§E` unit |
| `gates` (2026-09-06) | `D102`, `D103`, `D109`, `D211`, `D212`, `bit-identity-debug-only-gate-ends-an-item-at-a-semicolon`, `debug-only-counters-have-no-gate`, `trait-generic-sole-bracket`, `unanchored-definition-skip`, `clippy-panic-gate-blind-in-macros` (K); `gate-mod-path-resolved-textually`, `bounds-tripwire-blind-to-named-alias`, `S13`, `S49` | Track K's `scripts/gates/*` half, claimed whole at the program's opening |
| `meter` (2026-09-06) | `C15`, `D201`, `D203`, `D206`, `D213`, `D214`, `tess-budget-doc-finding-block-stale`, `k-report-baseline-fold-cert1-roster` (K); `tess-lint-face-ordinal-join`, `cut-prefix-three-unpinned-spellings`, `k-lint-predicate-roster-unpinned` | Track K's `tools/*` half, claimed whole at the program's opening |
| `blend` (2026-09-06) | `S90-impl` (M); `sweep-doc-comments-cite-tests-unenforced` | Track T's remainder, claimed at the program's opening |
| `eval` (2026-09-06) | `D360`, `D367`, `D368`, `emit-blend-restates-the-kernels-own-arguments` (V) | the eval seat's rows, claimed at the program's opening |

Those rows are not this program's any more and are not listed on its
board; find them under the program that carries them. **A row claimed
after this is moved the same way in the same PR that claims it** — the
claim and the move are one act, and a `keep_out` clause saying a
claimed row stays here is the thing to delete.

**The residue on claimed ground is the point of this directory, not an
oversight.** Rows sit on tracks a program has claimed while no unit of
that program names them: `D283` (Q — Ev's ruling, never work), `D290`,
`S350`, `S351` (Q), `D306` (R), `D341` (U — `exch` says the `pncad-py`
rows are LIB's, and LIB has not claimed them), `S116p` (T — a ruling),
and two findings whose class spans six programs
(`described-net-two-state-reads-hand-a-poisoned-net-the-described-arm`)
and every program (`L1`–`L5`). They wait here for a claim, which is
what waiting here means.
