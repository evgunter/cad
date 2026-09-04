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

## Track T, first pass (2026-09-04)

Track T (`crates/sweep/`) was carrying six rows. Four landed; three are
parked, one of them new.

**Landed.** `D320` and `D321` on PR 1782 (`smell/t1-scalar-lifts`) —
`skin.rs`'s `lift_surface` and `loft.rs`'s `lift_affine` deleted for
`NurbsSurface::map_scalar` and `Affine3::map`, with `loft.rs`'s
`LoftError::Skin(SkinError::Structure)` mapping and four test callers'
`.expect("lifts")`. `D323` and `D324` on PR 1783
(`smell/t2-blend-naming`) — the *"What consumes these rows"* paragraph
made true of `emit_blend`, and `Retired`'s absent face channel settled:
**no channel is owed**, argued from the operator set rather than from
fixtures. The surgery destroys through `kev` (kills no face) and `kef`
(kills the face of the half-edge it is handed), and every `kef` in
`surgery.rs` is handed a half of a face that surgery's own `mef` minted,
because a carve splits a support into the shrunk face plus its strips
and the shrunk face keeps its source key. That argument now lives at
`Retired` itself.

**Parked, on the two FILLET lanes live in `blend/surgery.rs`** (PRs 1763,
1752) — `T-R1`'s class, and the reason the rows were held rather than
dispatched: `D322` (gate `ring_clearance` behind `test-support`), `D325`
(the corner fusion's `first_arc`), and **`D326`, new**, raised by the T-2
style review as the row-0 answer `D323` did not take: `topo`'s
`canonicalize_chart` PICKS the dying face and structurally refuses to
hand the anchor to `kef` (`shell.rs:1398-1406`), where the blend surgery
establishes the same invariant six times by six unrelated local
arguments and states it at none of its call sites. With that carried,
`Retired`'s doc is two sentences instead of fifteen lines.

**Reviews were style-only this pass** (Ev's call for a track of this
size), against `docs/prompts/reviewer-style-lane.md`, one per unit, no
A/B arms. Both found real work: T-1's review established from
`validate_counts` and the `Vec<f64>` weights field that the deleted
re-validation could never have fired — the one thing that would have
been a MAJOR — and caught that a `map_scalar(f64::from_f64)` at the
`f64` test sites is a deep clone spelled as a lift; T-2's review
mutation-tested both face assertions to prove they can go red, and found
the `ShellRetired` contrast false in both halves (the chart reduction
runs `kef`, not `kfmrh`; `ShellRetired` records RESULT keys where
`blend::Retired` records SOURCE keys). Neither fix pass was optional and
both rows' PRs carry them.

**Filed out of the two reviews**, because a residue disclosed in a PR
body is filed nowhere: `d321-row-number-reissued` (the id was reused —
`plan.md` says ids never are), `sweep-doc-comments-cite-tests-unenforced`
(~16 doc comments naming a test file, nothing resolving them),
`profile-has-no-scalar-lift-door` and
`sweep-test-rebuilds-validated-net-for-v-reversal` (T-1's two residues),
`work/issues/stale-track-t-citations-in-fillet-and-cert`,
`work/issues/emit-blend-restates-the-kernels-own-arguments` (Track V's),
and `work/issues/two-green-prs-merge-into-a-red-main`.

**Main was red for the second time in one day** while this track ran —
`crates/viewer`'s deliberately exhaustive `blamed_mates` against a
`MateFault` variant added twenty-two minutes later by a concurrently
gated PR. Repaired by PR 1792 rather than handed back, per the standing
argument that main red blocks every program and a compile repair carries
no design content. The mechanism is the last issue above.

## Claimed rows leave this directory (2026-09-04)

Ev: this is a slush heap for findings nothing has been assigned yet, not
a long-lived register. So a row does not wait here for its lifetime — it
waits here for a claim, and the claim moves it out. Yesterday's entry
built a hand-maintained table pointing at the programs that carry
claimed rows; that was the wrong fix, because it made the tracker better
at describing a state that should not exist.

- **35 rows moved** into the six programs whose units carry them —
  `cert` (6, under `CERT-N3` and `CERT-M3`), `bool` (12, `BOOL-Q`),
  `mesh` (9, `MESH-R`), `trim` (4), `fillet` (3), `exch` (1) — keeping
  their ids, `track:` letters and bodies, with `parent:` set to the
  carrying unit where one exists. `by_id` is global in `work.py`, so no
  reference broke; ids resolve wherever the file sits.
- **The four `keep_out` clauses that said the rows stay** (`fillet`,
  `props`, `trim`, `exch`) are rewritten. A claim and a move are now one
  act in one PR, and a clause saying a claimed row stays here is the
  thing to delete.
- `program.md` says what this directory is, and stops calling itself a
  register. `plan.md`'s opening no longer claims every live row is a
  file here.
- **What stays is what nobody claimed**: `K`, `P` and `X` whole (no
  other program's `paths:` reaches their ground); `W` and `V`, whose
  ground is claimed but whose rows no unit names; and seven rows sitting
  on claimed tracks that no unit named — `S90-impl`, `D283`, `D290`,
  `S350`, `S351`, `D306`, `D341`. That residue is this directory doing
  its job, not a backlog of unfinished bookkeeping.
- **The `carried_by:` key is not needed and is not added.** `parent:`
  already says it, lint already resolves it, and a row in the right
  directory needs no pointer at all.
