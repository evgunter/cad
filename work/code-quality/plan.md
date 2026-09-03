# Code quality — the plan

Every live finding, row, ruling and sweep of this program is a file in
this directory, and `work/STATUS.md` is the board. This page is the
rules those files run on. Nothing here is ratified and nothing here is
a commitment: a finding is a *question worth answering*, not a defect;
several describe deliberate, ratified positions that a scanning agent
could not distinguish from drift, and the ratified design contract is
`docs/DESIGN.md`.

## How the numbering works

- **`S<N>` is a finding.** Ids are stable and never reused, so they can
  be cited from PRs, specs and other items. They are not contiguous:
  closed findings are gone, `S45`–`S48` are reserved and never
  allocated, and later blocks are handed out per track rather than in
  order, so a number says when a finding was *raised*, not where it
  sits.
- **`D<N>` is a schedule row** — a unit of work on a track. Rows and
  findings are different namespaces but **not disjoint**: some live rows
  are themselves numbered `S<N>`, colliding with a finding of that
  number, and others are numbered `C<N>`, `G<N>` or `H<N>`. Read a
  citation by the file it names, never by its letter. A `D<N>` cited in
  prose with no file here is a landed or retired row, and resolves the
  way a closed finding does: the merged PR is the record and git is the
  archive. One collision the tree cannot spell away: `docs/DESIGN.md`'s
  ratified decisions are also `D1`–`D9`, and most prose citations in
  that range mean *those* — `D9` is determinism, not a row.
- **`C<N>` is two things.** `process-observations.md`'s headings are
  `C1`…`C27`, and live rows are also numbered `C<N>` — `C3` is at once a
  process observation and a Track R row about `props/quad.rs`. A `C<N>`
  that is a file here is a row; a `C<N>` in prose is an observation
  unless it names a track. The collision is a ruling for Ev
  (`C-namespace`).
- **`L<N>`** is a cross-cutting sweep from *Last, deliberately* below.
- **`#NNN`** is a GitHub PR or issue.
- **A `W…` pointer is provenance only**: the retired wave numbering
  names no live item.
- **Blocks.** Each track allocates row and finding numbers from the
  block in the territories table, which `program.md` carries as well.
  Blocks are clear of every existing id and of the tree's maxima, and a
  new number is re-checked against the tree at allocation, because a
  block cannot stop a number arriving from another track. Track J's
  block (`D180`–`D199` / `S250`–`S269`) stays reserved and is not
  reissued.
- **Closing a finding owes a citation sweep** of `work/` and the tree,
  not of the file being closed. A finding's number is cited from
  wherever it was useful — another finding's argument, an observation's
  census, a row's `## What` — and lint resolves header fields only, not
  prose. `rg` the id first, re-aim or annotate every hit, and put the
  count in the closing PR.

## How to read a finding

A finding carries its evidence at `file:line` or, where a fix pass has
moved the line, at a **target name** a reader greps for. Claims, not
line numbers, are the content. A citation written inside the diff that
renumbers it is stale in the commit that writes it, so a landing lane
cites by target name, and any line number that survives says which
commit it is relative to (`S176`). A census likewise has one home and
states the sweep's definition beside its result, so a re-derivation is
checkable; every other site points at it instead of carrying a number.

**The fielded form is a minority.** Some findings carry a `**Where**`
line, a `**Confidence**` (`sure` / `likely` / `unsure`), an
`**Importance**` or a `**Raised by**`; the rest state the same things in
prose or leave them out. Read the absence of a field as an absence,
never as a claim.

**`**Verdict:**` is Ev's line**, and a blank one means unruled, not
disputed. The spellings in use — `ACCEPTED`, `ACCEPTED IN PART`,
`ACCEPTED, SORT REQUIRED`, `ACCEPTED WITH QUALIFICATIONS, row by row`,
`ACCEPTED, BUT SEQUENCED`, `ACCEPTED, AND SEPARABLE`, `ACCEPTED AND
SETTLED`, `ACCEPTED, unstaffed`, `DISPUTED`, `OPEN for the part that
matters`, `RULED`, and `_(unreviewed)_`, which means blank — are not a
taxonomy: each means what its own sentence says, and none of them is a
placement. Ordering rule 4 below decides whether a row is owed.

**No scan executed the code.** Every *"unreachable"* and *"no producer"*
claim is from reading plus `rg`, and could be wrong about a path reached
through a macro, a trait object, or a feature combination nobody
considered. That qualifies every finding, and most sharply the ones
labelled `unsure`. `C15` asks the same disclosure of every sweep a fix
lane runs; it applies first to the scans that produce findings.

**A finding phrased as an INFORMATION requirement gets discharged by
`format!` unless it is phrased as a TYPE requirement.** The shape
reaches any ask of the form *"carry the reason"*, *"say which"*,
*"record what happened"*: name the type you want, or expect the
cheapest thing that carries the information. Its audience is whoever
writes a finding, not whoever reviews the fix.

## How to read an item

A row is a file whose id is the row number and whose header carries
`track:`. Its body has four sections:

- `## What` — the work, as the row states it.
- `## Was` — where the row came from: the closed track that placed it,
  `unrowed` for a finding that never had a row anywhere, `neither` where
  two records disagreed about which track held it, `<Track> unplaced` or
  `filed` where a track placed it and never carried it into the
  partition, or the unit that filed it.
- `## Finding` — the finding's substance, where the row is the finding's
  only home.
- `## Fence` — the paths the row reaches where they differ from the
  track's territory. An exception to a fence is written here or is not
  an exception.

A live finding no row cites is a `kind: issue` file with `## Finding`
and `## Was` only. A question only Ev answers is a `kind: ruling` file
with `## Question` and `## Gates`; a ruled one carries its full ruling
text and is closed.

**A row leaves the board when it lands, and its finding leaves with
it.** The merged PR is the record, and the file goes to `closed` in the
same PR. A finding only PARTLY closed does not close: its closed members
are deleted member by member — the bullet, the table row, the paragraph
— and what stays is the open half plus whatever framing it needs to
stand alone, with no note saying the rest completed. Where the closing
PR establishes a standing rule or a correction later work depends on,
that sentence is relocated **in full** into text that survives — this
plan, `memories/`, the code itself — before the record that carries it
closes; a pointer left behind aimed at a one-line restatement is the
same loss with a longer path to it.

**A rides-along is its own file with `rides_with:` naming its
carrier.** A finding routed to an already-dispatched lane travels with
that lane's row rather than taking a row of its own, and it has no other
index — so a row may only close once every passenger has landed with it
or been re-homed: given a file of its own, with citations re-derived
against the tree as it is now. Lint refuses a live passenger on a closed
carrier, and prose elsewhere pointing at a closed row is not a schedule.
The landing report is where the passengers are listed, so it is where
the check happens. `L5` is the walk of every row struck before this
rule existed.

## The four ordering rules

1. **Decide before you delete; delete before you polish.** Comment
   trimming (`S38`) and test-suite combing (`S36`) come last — both
   operate on files whose fate earlier rows have not settled.
2. **A finding whose steelman said SURVIVES IN PART is scoped by the
   steelman, not by the original finding.** Several shrank materially
   under scrutiny.
3. **A lane's own residues are rows, not footnotes.** Many rows exist
   because a fix pass or a review found something its own PR could not
   carry. Recording them as prose inside a merged PR body is how they
   get lost.
4. **A verdict is not a placement.** A finding may leave a review with
   ACCEPTED, DISPUTED or DECIDED and no row only if the verdict is
   *closed*. Everything else owes a row, a ruling or a landed PR, written
   in the same PR that records the verdict — because accepting findings
   in batches gives the batch's leader a lane and its siblings a verdict
   and nothing else.

## The rules this partition runs on

The partition rule is the only one that matters: **no two tracks may
edit the same file**, so no branch waits on, fences against, or
re-derives another's scope. Dependencies *inside* a track are its own
orchestrator's to sequence, and there are no dependencies *between*
tracks that any lane must honour. A track can be claimed the day it is
read. `L`, `O` and `S` are not tracks: `L` is the *Last, deliberately*
rows, `S` is the finding namespace, `O` reads as a zero; `J` is retired,
its ground stated under *What this partition leaves out*.

1. **The fence is the file, not the subject.** A track owns paths. If a
   row's work reaches a path another track owns, the reaching half is
   **filed as a row on the owning track** and the first track lands
   without it. No lane ever edits across the fence, and no lane ever
   waits for the other side — filing the row *is* the handoff.
2. **Number blocks are published before any lane is given a number**
   (the territories table), and re-derived after every merge anyway: a
   block cannot stop a number arriving from another track, only
   re-checking can. A track's own rows are what other tracks fence
   against, so a row's scope is edited in the same PR as the diff that
   moves it, never after.
3. **A row leaves when it lands, and its finding leaves with it.** Both
   close and neither is annotated; the relocation rule for standing
   sentences is under *How to read an item*. Landing PRs edit files in
   this directory and conflict by construction, so within a track,
   **merge one at a time**.
4. **A style review runs on every unit** against
   `docs/prompts/reviewer-style-lane.md`, carrying the two questions the
   standing brief does not ask — *is the original problem completely
   gone*, and *was it closed in the best way available*. **Adversarial
   only where a wrong answer is reachable**; the rows that carry it are
   marked **ADV**.
5. **The one thing every closed track agreed on**, and it held on every
   unit of Tracks F and G: **the fix mints a fresh instance of the defect
   it closes**, and naming that trap in your own PR body does not prevent
   it. Only a reader who did not write the fix has ever caught it.
   Standing rules: `logs/SMELL-F-LOG.md`,
   `memories/agent-lane-operations.md`.
6. **Not in any track, and deliberately:** `L1` (`S36`, comb-and-rename
   per suite), `L2` (`S38`, comment trimming), `C2`/`H17` (`S37`'s
   rustdoc remainder, ~1115 lines across 130 files) and `C21` (two
   workspace-wide comment populations, read per item). All four are
   cross-cutting comment or naming sweeps that would collide with
   **every** track on this list. They go after the tracks empty.
7. **Not work at all:** the rulings. No lane may resolve one by
   implementing something. Where a track holds the work that *follows* a
   ruling, its row says so and the row is not takeable until the ruling
   lands.

## The territories

| Track | Territory (the fence) | Block |
|---|---|---|
| **K** | `scripts/gates/` less `gate-roster.sh` and `probe-suite-census.sh`, `tools/`, `docs/K-REPORT.md` | `D200`–`D219` / `S270`–`S289` |
| **M** | `crates/geom-core/src/{real,ring_interval,dual,interval,k_stats}.rs`, `interval-transcendentals/`, `crates/bvh/`, `crates/topo/src/props.rs` | `D220`–`D239` / `S290`–`S309` |
| **N** | `crates/geom/src/`, `crates/geom-core/src/{spline/,linalg/}` | `D240`–`D259` / `S310`–`S329` |
| **P** | `crates/topo/src/{euler.rs,euler_ring.rs,euler_kill.rs,split.rs,attach.rs,movefac.rs,revert.rs,live.rs,merge_faces.rs,seqgen.rs,validate.rs,review_d18.rs,review_d18_probes.rs,fixtures.rs,source_walk.rs}` | `D260`–`D279` / `S330`–`S349` |
| **Q** | `crates/topo/src/{boolean/,splitting/,census.rs,chord_join.rs,chart_region.rs,face_normal.rs}`, `crates/geom-brep/src/{ssi*,pcurve_cache.rs,nurbs_iso.rs,edge_nurbs.rs}`, `docs/predicate-dimension-audit.md` | `D280`–`D299` / `S350`–`S369` |
| **R** | `crates/geom-brep/src/` **less the four paths Q names**, `crates/mesh/` | `D300`–`D319` / `S370`–`S389` |
| **T** | `crates/sweep/` | `D320`–`D339` / `S390`–`S409` |
| **U** | `crates/step-import/`, `crates/step-export/`, `crates/stl/`, `crates/pncad-py/`, `crates/pncad/` | `D340`–`D359` / `S410`–`S429` |
| **V** | `crates/editor-core/`, `crates/profile/` | `D360`–`D379` / `S430`–`S449` |
| **W** | `crates/*/tests/` (all crates), `crates/test-utils/` | `D380`–`D399` / `S450`–`S469` |
| **X** | `demos/` (Rust and Markdown; its Python is not X's), `docs/DESIGN.md`'s companion table | `D400`–`D419` / `S470`–`S489` |

**Three seams are stated rather than left to be discovered**, because
each is a place where a reasonable reader would think the fence
ambiguous:

- **`crates/*/tests/` is W's in every crate, and the exception is
  written here**: Track T's fence covers the `sweep/tests/` files its
  own rows name. A track that owns a crate's `src/` does **not**
  otherwise own its `tests/`. **No lane mints an exception** — an
  exception is a fence line or a row's `## Fence`, and nothing else is
  one. Where a src change needs a test, W is not in the way: the test
  belongs to the PR that makes the change, and W's rows are about the
  *test-side mechanisms* named in them — the guards, the doctests, the
  stand-downs, the fixtures, the probe-gated suites. W files a row on
  the owning track when a mechanism reaches into `src/`, and vice versa.
  Two other fences carry a named exception the same way, in the other
  direction: Track R's `C23` reaches one line of `geom/src`, which is
  N's, and Track V's `D366` reaches the `pncad-py` tag map, which is
  U's.
- **Every `*.py` in the repo is ONE population and belongs to no
  track**, including the fixtures under `crates/*/tests/` and the
  renderer under `demos/`. Splitting it to match the Rust fences would
  put four tracks in it, and it is already linted as one — `ruff.toml`
  plus `scripts/check-python-lint.py` — so an instrument enumerates it;
  **re-derive its size rather than transcribing one.** It is unowned
  ground in the sense the `geom-brep` seam gives that phrase: a row
  whose work reaches it draws the fence first.
- **`crates/geom-brep/src/` is Track R's, less the four paths Track Q
  names** (`ssi*`, `pcurve_cache.rs`, `nurbs_iso.rs`, `edge_nurbs.rs`).
  Stated because the rest of the crate — `patch_bound.rs`,
  `offset_meters.rs`, `nurbs_hull.rs` and its siblings — is easy to read
  as nobody's: a row's work reaching an unowned path is not a licence to
  edit it, it is a fence that has not been drawn, and R takes this
  ground because `mesh` is what consumes it.

## What this partition leaves out, said explicitly

- **The rulings** — `D6`, `S14`, `S65`, `S70`, `S82`, `S107`, `S116p`
  and `C-namespace`. Most of the tracks hold work that one of these
  gates; each such row says so.
- **`L1`, `L2`, `C2`/`H17` and `C21`** — the four cross-cutting comment
  and naming sweeps, which collide with every track and go after it.
- **`L3`** — the remaining `S35` roll-up rows, lowest value density,
  several of which will be resolved incidentally by the tracks.
- **Track `J`'s ground, the track being retired with an empty table:**
  `.github/workflows/`, `local-scripts/`, `scripts/doc-gate.sh`,
  `scripts/gates/{gate-roster,probe-suite-census}.sh`, every `*.py`, and
  root `Cargo.toml`'s `[workspace.lints]`. **It is unowned, not
  finished** — the distinction the `geom-brep` seam draws. A row landing
  on it is a fence to be drawn, in the same PR that mints the row, and
  until then no lane edits there. Its number block stays reserved and is
  not reissued; the observations its lanes raised outside their rows
  are `C26`–`C27`.
- **The unscanned crates**, which are a scanning input rather than a
  work item: `crates/bvh/` and `crates/quantity/` are Track M's to
  commission. (`step-import/`, `step-export/`, `stl/`, `pncad-py/` and
  `profile/` are scanned — the SMELL-UV commission, `logs/SMELL-UV-LOG.md`,
  findings `S410`–`S415`.)

## Last, deliberately

`L1`–`L5` are the cross-cutting sweeps that go after every track
empties, each a `kind: unit` file with its own `## Why last`. The reason
is the same for all five: each is document-wide or workspace-wide — a
comment or naming pass over files whose fate earlier rows have not
settled, a walk of this program's own history, or a re-read of its own
dispositions — so it collides with every track and can be scoped to no
fence. They are not takeable while a track is open on the files they
would touch.
