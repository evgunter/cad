# CITE — citations, numbering and the paperwork (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3).

Branch prefix: **`cite/`**. Away-channel tag `(CITE orchestrator)`.
A/B ordinal band **CITE = 4000–4099**.

## Charter

Two subjects that are one habit.

**Citations.** `work/` and `docs/` cite the tree by `file.rs:NNN`, the
tree moves, and nothing re-derives the number — so a citation written
inside the diff that renumbers it is stale in the commit that writes it
(`S176`'s finding, and the convention code-quality's plan already
stated before it left the tracker). Four rows here are that class at
four scales: one sentence
mis-quoted from a rewritten doc string, one measured count of how big
the population is, the repair sweep itself, and the cite-by-name
convention that would end it.

**Numbering and paperwork.** A row number reissued after retirement, the
`C<N>` namespace collision that only Ev can rule, and three rows about
the harness a lane runs inside.

## Territory — none, and this one is a constraint, not a shrug

This program claims **no paths, and could not**. Its repair ground is
`work/<program>/*.md`, which is one-file-one-item ground: every item file
belongs to the program whose directory it sits in. META's `keep_out`
already writes the rule down — *a stale citation in another program's
slate is routed to its owner and never fixed across the fence* — and
`stale-track-t-citations-in-fillet-and-cert` is the standing instance.

So the shape of the biggest row here is not what it looks like.
**`doc-line-citations-rot-silently` cannot be landed as one sweep.** What
this program produces is:

1. the **measurement** (`tracker-file-line-citations-measured` already
   carries one and asks for no unit);
2. the **convention** (`S176`), landed in text that survives — this plan
   and, where it binds every lane, `docs/prompts/` by META;
3. a **routing list**: per owning program, the citations in its slate
   that are stale, filed as a row on that program.

Anything else is this program editing another program's board, which is
the rule the tracker is built on.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `code-quality-item-quotes-a-viewer-doc-string-that-was-rewritten` | **E** | Re-quote one sentence in one tracker file; replacement text already identified | `work/door/viewer-pathverb-all-hand-written-seventeen.md` (:25, :29); reads `crates/viewer/src/forms.rs:190` |
| `tracker-file-line-citations-measured` | **E** | Pure sizing note; nothing to edit beyond folding the number into the owning row | — (measurement only; it explicitly asks for no unit — the repair lives in `work/cite/doc-line-citations-rot-silently.md`) |
| `no-local-script-builds-all-four-cargo-workspaces` | **E** | Loop `cargo` over five roots in an existing script plus a discipline sentence; fix is stated | `local-scripts/ci-local.sh`, `local-scripts/test-fast.sh`, `docs/prompts/implementer-discipline.md` |
| `build-slot-banner-leaks-the-holders-command-line` | **E** | Fix stated: banner prints pid and slot only, one script | `local-scripts/with-build-slot.sh` (`describe_holder`, `note_holder`), possibly `memories/orchestration-model.md` |
| `d107-release-profile-job-lives-in-nightly` | **E** | Whole fix is one stale sentence in D107; optional one-line `--nocapture` | `work/topo/D107.md:33` (TOPO's board), optionally `.github/workflows/nightly.yml` |
| `S351` | **E** | Standing watch; if triggered, re-aim two pointers. Explicitly no work today | `crates/geom-brep/src/nurbs_iso.rs`, `crates/geom/src/lib.rs`, `crates/geom/src/surfaces/nurbs.rs` |
| `C-namespace` | **E** | One sentence from Ev picks a prefix; ids stable, no renumbering, no code | — |
| `d321-row-number-reissued` | **M→overtaken** | Both halves lost their target at sweep 11 (see Order) — read it before staffing it | nothing live; possibly `scripts/work.py` lint if a stable-ids check is still wanted |
| `loud-skip-marker-row-cites-a-lib-paragraph-that-was-reversed` | **M** | Re-derive eight citations across three programs' files; decide if a reversed member is dead | `work/tcost/loud-skip-marker-is-a-hand-kept-idiom.md` (8 citations), `crates/viewer/src/lib.rs`, `crates/viewer/tests/{error_display,chrome_labels,panel_display}.rs`, `crates/sweep`+`crates/topo` markers; maybe `docs/prompts/implementer-discipline.md` §6 |
| `lane-scratchpad-is-shared-between-worktrees` | **M** | One-sentence rule, but owner undecided, memories needs Ev, deciding fact unverified | `docs/prompts/implementer-discipline.md`, `memories/agent-lane-operations.md` |
| `S176` | **H** | Register-wide citation sweep plus a standing convention; blank Ev verdict | `work/**/*.md` citations (cite-by-name sweep), convention text in `docs/prompts/*` (META's, filed there) |
| `doc-line-citations-rot-silently` | **H** | Tree-wide sweep of every `file.rs:NNN` in docs, plus live-pointer vs narrative judgement per hit | `work/*/log.md` (esp. `work/lib/log.md`), `work/*/plan.md`, `docs/KERNEL-VERBS.md`, `docs/*` less `docs/DESIGN.md` |

## Order

Five of the seven E rows first and in any order; four of them are one
file each, and the fifth is a routing (below). The other two E rows are
E in a qualified way — `C-namespace` is Ev's and `S351` is a watch — and
are handled at the end of this order.
`d107-release-profile-job-lives-in-nightly` is a single stale sentence in
`D107`, which lives on TOPO's board (`work/topo/D107.md`) — so it is
filed there, not edited from here, and this row closes by routing.

`S351` is a **standing watch**, not work: it fires only if the placement
rule it guards moves. It is on the slate so the watch has a home, and a
lane that finds the trigger has not fired closes nothing and touches
nothing.

`C-namespace` is a ruling: one sentence from Ev picks a prefix, nothing
renumbers, and it goes on the next `[ev]` sitting.

Then `d321-row-number-reissued`, which **sweep 11 overtook on both
halves** (`docs/DOC-LEDGER.md`, 2026-09-11) — read it before staffing
it. It asked for two things. A *retired-id rule* for code-quality's
numbering: there is no longer a number ledger to write one in, because
the tracker mints ids from item names and not from per-track blocks,
and the blocks left with the directory. And *citation disambiguation*
inside `SMELL-T-LOG.md` and `SMELL-KPW-LOG.md`: those went to the
archive at the sweep's SHA, so there is nothing live to edit. What may
still be worth doing is the third thing the row implies but does not
ask for — a `work.py` check that an id is never reissued — and that is
one line filed on META, not this row. `S176` is in the same position
for the `SMELL-G-LOG.md` census it cites; its convention half is
untouched and is still the row's point.

Then `loud-skip-marker-…` and `lane-scratchpad-…`. The second of those wants
a sentence in `memories/`, which is **Ev's call and waits on an `[ev]`
PR** (CLAUDE.md); the `docs/prompts/` half is META's to land.

`S176` and `doc-line-citations-rot-silently` go last and in that order:
state the convention, then sweep against it.

## Review posture

Infra-and-prose, the META posture: one style review per unit, no A/B
row. No unit here changes a kernel behaviour; the one thing a reviewer
must check is that a "repair" is a re-derivation against the tree and
not a re-transcription of the same stale number.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
