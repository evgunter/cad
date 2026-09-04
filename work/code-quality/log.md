# Code quality — the log

Append-only. The tail is the program's story; the slate is the item
files in this directory and `work/STATUS.md`.

## Tracker migration (2026-09-03)

`docs/SMELL-SCAN-2026-08.md` — the findings register and its schedule
— is migrated into this directory and deleted in the same PR; the
deletion is recorded in `docs/DOC-LEDGER.md`.

- Every live Track K–X row is a file here, id = row id, `track:` set;
  a rides-along is its own file with `rides_with:` naming its carrier.
- The decisions for Ev — `D6`, `S14`, `S22-row-1`, `S65`, `S70`, `S82`,
  `S107`, `S116p`, `scaled-square`, `C-namespace` — are `kind: ruling`
  files. The two already ruled (`S22-row-1`, `scaled-square`) carry
  their full ruling text and are closed; `scaled-square` is that
  ruling's only home.
- The *Last, deliberately* rows are `L1`–`L5`, `kind: unit`.
- Every live finding no row cites is a `kind: issue` file with
  `## Was: unrowed`; a finding whose own record says it is closed is
  dropped, and git keeps it. The census of every finding heading and
  its disposition is `logs/migration-census-2026-09-03.md`.
- The rules that survive the document are `plan.md`. §C's process
  observations are `process-observations.md`, verbatim.
- The nine closed-track execution logs
  (`SMELL-{C,E,F,G,H,I,KPW,T,UV}-LOG.md`) move unedited from `docs/` to
  `logs/`.

## The tracks this program no longer executes (2026-09-04)

Eight of the eleven tracks are ground other programs own, and four of
them have a named unit elsewhere carrying the rows — `cert`'s `CERT-M3`
and `CERT-N3`, `bool`'s `BOOL-Q`, `mesh`'s `MESH-R`, plus `fillet`,
`exch`, `trim` and `lib` claiming T, U and Q's geom-brep files in their
`keep_out`s. Every one of those claims was written on the CLAIMING
side. Nothing here pointed back, so from this board a claimed row reads
`open` and unclaimed, and a session reading only `work/code-quality/`
concludes the tracks are idle. One did: it redid `D244`, which
`cert/n3-track-n-remainder` already carried.

- `program.md` now names the three tracks this program still dispatches
  (`K`, `P`, `X` — the only three no program's `paths:` reaches) and
  tables the other eight against their owners and executors.
- `plan.md`'s territories table carries an **Executed by** column, and
  *"a track can be claimed the day it is read"* is written in the past
  tense, because eight of them since have been.
- The table is maintained by hand and is the only pointer back: no row
  header says which unit carries it, so neither `status` nor `lint` can
  show or check it. A `carried_by:` key would make it lintable and is
  not proposed here.
- **A fourth seam is stated**: `crates/test-utils/src/source.rs` and
  `reader_census.rs` are W's, and `D261` (P), `D287` (Q) and `D386` (W)
  all land in them. The two conversions lower one hand-synced ceiling,
  so it is re-derived at the landing rather than lowered by each row's
  own count; both rows carry the note.
- **Track `P` runs as three sub-lanes** — Euler surgery and validation,
  the review and fixture readers, liveness and the generator — on three
  disjoint file sets, no new letters and no new blocks.
