# CITE exit walk

**Program:** `cite` — CITE, citations, numbering and the paperwork a
lane runs on. Opened 2026-09-11 in the tracker cut of that day
(`docs/WORK-TRACKS-2026-09.md` addendum 3); walked 2026-09-12.
Branch prefix `cite/`, away-channel tag `(CITE orchestrator)`,
A/B band 4000–4099 (**unused** — see *The A/B band*, below).

This walk checks the program against the charter in
`work/cite/program.md` and the slate in `work/cite/plan.md`, quoting
both. It is the done-state of record once ratified.

---

## The charter, quoted

From `work/cite/program.md`:

> The project's own text and harness rather than its kernel: citations
> that rot because nothing re-derives them, a row number that was
> reissued, the convention that would stop the first from happening
> again, and the three small paperwork rows a lane trips over (a
> scratchpad shared between worktrees, a build-slot banner that leaks a
> command line, no local script that builds all four cargo workspaces).

From `work/cite/plan.md`, the constraint that shaped everything:

> This program claims **no paths, and could not**. Its repair ground is
> `work/<program>/*.md`, which is one-file-one-item ground.

The plan predicted the program would therefore produce three things:
**a measurement, a convention, and a routing list**. Two of those are
what happened. The third did not, and the reason is the walk's main
finding.

---

## The twelve rows

| row | class at cut | disposition |
| --- | --- | --- |
| `S176` | H | **closed** — its remedy is the convention; `Verdict:` filled |
| `doc-line-citations-rot-silently` | H | **closed** — convention adopted, sweep declined |
| `tracker-file-line-citations-measured` | E | **closed** — folded; its figures carry the convention |
| `code-quality-item-quotes-a-viewer-doc-string-…` | E | **closed** — repaired in `work/door/` |
| `d107-release-profile-job-lives-in-nightly` | E | **closed** — repaired in `work/topo/` |
| `loud-skip-marker-row-cites-a-lib-paragraph-…` | M | **closed** — eight entries re-derived in `work/tint/` |
| `build-slot-banner-leaks-the-holders-command-line` | E | **closed** — script fixed and verified |
| `C-namespace` | E | **closed** — Ev ruled: leave the ambiguity |
| `S351` | E | **re-homed to TRIM** — standing watch, unfired |
| `d321-row-number-reissued` | M | **re-homed to META** — overtaken by sweep 11 |
| `no-local-script-builds-all-four-cargo-workspaces` | E | **re-homed to CIW** — never started |
| `lane-scratchpad-is-shared-between-worktrees` | M | **re-homed to META**, deferred by Ev |

Eight closed, four re-homed, none abandoned.

## What the program produced

**1. The convention, in two places.** The argument, the measurement and
the three never-repointed shapes are in `work/cite/plan.md`; the
lane-facing rule is `docs/prompts/implementer-discipline.md` **§7**, two
sentences:

> **Cite by name; line numbers rot.** A number may ride along beside the
> name and is allowed to go stale; a bare `file.rs:NNN` is not a
> citation.

**2. The measurement that made it a ratification rather than a
proposal.** 1,508 citations in 317 open rows across 22 programs;
**1,446 (96%) already name their subject beside the number**, 90–100%
per program. So the rule describes existing practice and the residual
work is the 4%.

**3. Three repaired rows** on DOOR's, TOPO's and TINT's slates, each
carrying a marked section for its owner recording what the repair put in
question — and CITE ruling on none of it.

**4. One leak closed.** `local-scripts/with-build-slot.sh` no longer
records the caller's command line in the holder file, so the banner a
waiting caller prints cannot carry another lane's test filter or scratch
path between blinded reviewers.

## What was deliberately not done

- **No sweep** of the 1,508 citations. A re-derivation would mostly
  delete redundant numbers beside anchors that already work.
- **No gate.** A line-range checker sees the 2.1% whose file or line is
  out of range; VIEW's hand-sweeps found ~75% of what they touched
  pointing at the wrong *subject*, nearly all of it inside the column
  such a check passes. The check would be silent on almost every real
  defect.
- **No `memories/` sentence** for either paperwork row (Ev,
  2026-09-11: *"no memory, just script fix"*).
- **No prefix for the `C<N>` namespace** (Ev, 2026-09-11), and sweep 11
  then retired the colliding side outright.

---

## The finding this program leaves behind

**The routing list the plan promised was never produced, and should not
have been.** The plan reasoned that one-file-one-item makes a
cross-program citation repair impossible, so CITE would hand each owner
a list. Ev overrode that at the first ask: repair in place, one PR, no
routing issues. That was right, and the evidence is in the row that
motivated the fence in the first place —
`loud-skip-marker-…` records that §6 reported the same rot **twice with
nothing filed either time**. A third routing would have been a third
nothing.

So the fence held where it mattered — CITE changed no claim, count,
title, membership or disposition belonging to another program — and gave
way where it did not. **The distinction is the transferable part:
repairing what a row POINTS AT is not the same act as ruling on what it
CLAIMS**, and only the second needs the owner.

**The program's subject happened to the program, six times.** The plan's
own table cited three rows at pre-cut paths. A row written to fix a
stale quotation cited its own replacement at the wrong file and line.
The repair PR introduced three fresh bad citations, one of them a number
it explicitly certified as re-derived. Then the merge of `main` moved
`forms.rs` under DOOR's parallel re-derivation, and `forms.rs:199` —
correct when written — came to land on `pub(crate) const ALL;`,
plausible and wrong, while CITE's name-cited repair of the same file was
untouched.

Every one of those was recovered by the **name** beside the number. That
is not a failure of the convention; it is six independent tests of it,
and it passed each time. It is also why the rule says a number is
*allowed* to rot rather than forbidding numbers: the cost of a drifted
line is a lookup, and the cost of a bare one is a dead end.

---

## Residue, and where it went

Nothing is left in the directory. Per `work/README.md`, residue is
re-homed before the sweep:

| row | to | why |
| --- | --- | --- |
| `S351` | `work/trim/` | the rule it watches is in `nurbs_iso.rs`, TRIM's `paths` |
| `d321-row-number-reissued` | `work/meta/` | both targets gone at sweep 11; the surviving `work.py` id check is META's |
| `lane-scratchpad-…` | `work/meta/` | deferred by Ev; `docs/prompts/*` is the only document left in play, and it is META's |
| `no-local-script-builds-…` | `work/ciw/` | the fix is in `local-scripts/*`, CIW's `paths` |

Each is a `git mv` with a `## Re-homed` record on the row, ids kept,
bodies unedited.

## The A/B band

**4000–4099 was allocated and never drawn from.** The review posture
(`work/cite/plan.md`) set no A/B row for this program, at Ev's
direction — infra-and-prose, one style review per unit, with a full
review reserved for the citation re-derivations, where a "repair" that
re-transcribes a stale number is the way the work fails silently. That
full review is what caught the three MAJORs in the first repair pass.
The band stays claimed in `docs/MODEL-AB-LOG.md` and no ordinal was
spent.

## Honesty notes

- **This walk quotes the charter and the convention verbatim**, and
  quotes the plan's slate as a table rather than verbatim — the plan's
  table has a `where the work lands` column that is about dispatch and
  is not a criterion. The plan itself is recoverable at the sweep SHA.
- **Four of the twelve rows were never worked by this program**, and
  three of those four were re-homed unstarted. A program that opens
  twelve rows and works eight is not a program that finished twelve;
  the four moved because they were somebody else's, not because they
  were done.
- **`S176`'s `Verdict:` was blank for three weeks and is now filled by
  this program**, on the measurement rather than on a ruling from Ev.
  The row says so in its own words.
