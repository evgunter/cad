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

**This directory now holds no live row.** The cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3, Ev's direction in-chat) read
every non-closed item here and in `work/issues/` — 110 between them —
and moved all of them into eleven programs opened for them, by the
charter above: a row leaves the moment a program claims it, and eleven
programs were opened to do the claiming. What is left beside these
charter documents is closed rows, the `logs/` of the closed tracks, and
`process-observations.md`.

**Track `X` (`demos/`) is still this program's letter**, and it is now a
letter with no live row: its three rows went to the cut — `D403` and
`D79` to SUITE and COMB, `tour-scenes-lift-componentwise-not-through-map`
having closed before it. A new row on `demos/` is still minted here from
`X`'s block and filed on the program whose ground it lands on.

`K` was claimed whole on 2026-09-06 by two programs opened for it,
`gates` (`scripts/gates/*`) and `meter` (`tools/*` and the two
instrument documents); **`gates` closed 2026-09-08**
(`docs/DOC-LEDGER.md`, sweep 7) and its half of the fence came back
here with two rows, both of which went to **GUARD** on 2026-09-11, the
successor opened on exactly that ground. `T`'s remainder on
`crates/sweep/src` went to `blend` on 2026-09-06; `P` and `W` left on
2026-09-04; `V`'s ground is spread across DOCM, six others, and — since
EVAL closed on 2026-09-08 — **WIRE**, the seat's successor, which took
`D364` and `S40` at the cut.

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
| **the eleven programs of the 2026-09-11 cut** | every remaining live row — 79 from here and 31 from `work/issues/`, 110 in all: **GUARD** (`D212`, `G4`, `S41`, `S115`, `S210` and six unlettered), **WIRE** (`D364`, `S40`, `S195`, `profile-has-no-scalar-lift-door` + nine), **DOOR** (`D306`, `S114`, `S190`, `S414` + seven), **CENSUS** (`S57`, `S113`, `S133` + four), **CITE** (`C-namespace`, `S176`, `S351` + nine), **SCALAR** (`D6`, `D283`, `D290`, `H5`, `S393` + two), **SUITE** (`D114`, `D403`, `S52`, `S391`, `S392` + three), **COMB** (`L1`–`L5`, `D79`, `S11`, `S19`, `S35`–`S38`, `S43`), **PRED** (`D292`, `S18`, `S29`, `S58`, `S65`, `S66`, `S82`, `S116p` + three), **PIPE** (`D291`, `S5`, `S14`, `S70`, `S79`, `S350` + four), **PORT** (`D341`, `S107`, `S415` + four) | each program's own board; `docs/WORK-TRACKS-2026-09.md` addendum 3 is the cut, and each row carries a `## Re-homed` record |

Those rows are not this program's any more and are not listed on its
board; find them under the program that carries them. **A row claimed
after this is moved the same way in the same PR that claims it** — the
claim and the move are one act, and a `keep_out` clause saying a
claimed row stays here is the thing to delete.

**The residue on claimed ground was the point of this directory, and
2026-09-11 is what it was waiting for.** Rows sat here on tracks a
program had claimed while no unit of that program named them — `D283`,
`D290`, `S350`, `S351`, `D306`, `D341`, `S116p`, the described-net class
that spans six programs, `L1`–`L5` that span every program — because
waiting for a claim is what waiting here means. The claim, when it came,
was not a live program taking them one by one: the measurement found
that nine of the eleven cohorts in the pile own **no file territory at
all**, because each row is one small thing in another program's house.
That is the finding this directory existed to produce, and the cut
records it (`docs/WORK-TRACKS-2026-09.md` addendum 3).

**This program stays open.** Its charter is unchanged and its blocks are
unchanged: a scan raises a row, the row is minted here from its track's
block, and it goes straight to the program whose ground it lands on —
which, after the cut, is more often one of the eleven than one of the
kernel programs. What has changed is only that the holding ground is
empty.
