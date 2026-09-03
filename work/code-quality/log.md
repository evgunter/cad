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
