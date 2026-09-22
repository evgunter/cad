---
id: the-lane-register-has-no-home-after-views-directory-goes
kind: issue
title: VIEW's lane register binds four successor programs by reference and dies with work/view/plan.md
status: closed
opened: 2026-09-17
priority: P4
cost: E
closed: 2026-09-21
branch: view/delete-the-register
---


Disclosed by the 2026-09-17 re-scope, as its own file rather than as a
sentence in a plan, because `work/README.md` says the sweep sees items
and not sentences.

## The finding

`work/view/plan.md` is two documents in one file. The first is this
program's plan — status, territory, Order, exit shape. The second is a
**rule register**: roughly six hundred lines of operational discipline,
every rule of which is a named failure at a named PR (the citation
rules, the census and proxy rules, the CI-tier rules, the lane-isolation
rules, the `desired_width` rule, the δ round-trip rule). It is handed to
every lane this program dispatches and it is the reason its later waves
cost less than its early ones.

The four successor programs opened on 2026-09-17 — `vnews`, `vgeom`,
`vseam`, `vdoc` — each **inherit that register by reference**, in their
`plan.md` §The register. The choice was deliberate and its argument is
written there: four copies of a register that is re-derived every wave
give four divergent copies inside a week, which is this program's own
count-fixed-in-one-place defect applied to its own discipline; and a
rule detached from the PR that paid for it reads as a rule without its
receipt.

## Why that is a row and not a footnote

`work/README.md`: *a closed program's directory is deleted … `program.md`,
`plan.md` and `log.md` go*. So the day VIEW's exit walk is ratified,
`work/view/plan.md` goes, and four live programs are left pointing at a
path that resolves only at a SHA in `docs/DOC-LEDGER.md`. A lane told to
read its discipline out of the ledger's recoverable history will not.

**This is a precondition of VIEW's exit walk, not a follow-up to it.**
The walk cannot be ratified while four open programs depend on a file it
deletes.

## What is undecided

Where the register goes. The candidates, none of them chosen here:

- `docs/prompts/` — the standing discipline handed to every lane by
  path, which is what the register in fact is. CLAUDE.md makes that
  directory Ev's call, so this route is an `[ev]` PR.
- `crates/viewer/README.md` — CLAUDE.md's home for finished-work design
  beside the code. Wrong shape: the register is process, not design, and
  most of its rules are not about the viewer at all.
- One of the four successors' `plan.md`, with the other three
  referencing it. Moves the problem to that program's own exit.
- A fifth file under `work/` with no program — `work/README.md`'s layout
  does not admit one today, which makes this a tracker question and
  therefore META's.

Splitting it is also on the table: the rules about THIS CRATE stay with
the viewer programs, and the rules about lanes, CI tiers, citations and
censuses — which bind every program in the tree and are re-derived
independently in several plans already — go wherever the general
discipline lives.

## The archaeology — 2026-09-21, `view/cut-residue`

Measured on this branch's tree (which differs from `origin/main` only
by this branch's own rewrite of the tier rule, +2 rule paragraphs).
Every number below is printed by the command beside it, and the command
was run as printed.

**How big it is, and how many rules.**

    wc -l < work/view/plan.md
    # 1470

    # a rule is a paragraph that OPENS in bold, which is how every rule
    # in this register is written; the register runs from the first one
    # (line 165) to the line before `## Exit shape`
    awk 'NR>=165 && NR<1467 && /^\*\*/ && prev=="" {n++} {prev=$0} END{print n+0}' work/view/plan.md
    # 92

    awk 'NR<165 && /^\*\*/ && prev=="" {n++} {prev=$0} END{print n+0}' work/view/plan.md
    # 5   — the PLAN's own header paragraphs, not register rules

**What that enumeration cannot see**, stated because this register
demands it: a rule not written as a bold-opening paragraph is not
counted, and a bold-opening paragraph that is narrative rather than a
rule is. Five of the 92 are the second kind — `Fifteen units on main`,
`Twelve units on main`, `What the wave produced beyond its four diffs`
and the two 09-07 fork records — so **87 are rules and 5 are VIEW's own
wave narrative.** The first pass at this count was **97**, from an
`awk` whose `NR>=165` guard sat on the `exit` rule and not on the
counting rule, so it counted the whole file; it is quoted here because
it is the register's own *a receipt is a citation* class committed
against the register while measuring the register.

**VIEW-specific against general.** A mechanical classifier is a proxy
here and a bad one: `grep -cE 'crates/viewer|viewer|egui|emath|wgpu|WGSL'`
over the rule paragraphs returns **59**, which is an upper bound and
nothing more, because nearly every general rule in this register uses a
viewer failure as its worked example. The property is *would this rule
still be true and runnable if `crates/viewer` did not exist*. By hand,
against that test, **twelve** are VIEW-bound:

| line | rule |
|---|---|
| 374 | a viewer test command needs `--no-fail-fast` |
| 402 | `desired_width` is not a character budget |
| 441 | do not say "the ones CI runs" — the two doc-gate passes are alternatives |
| 450 | and that wording undercut the rule it introduced |
| 549 | the orchestrator's own heartbeat is a single point of failure |
| 807 | a `crates/viewer` diff's own CI cannot read the skip-mode doc pass |
| 824 | so a VIEW diff owes `--skip-viewer-toolkit` locally |
| 840 | and the skip's named backstop cannot red |
| 850 | and the cost was paid on the next viewer diff |
| 863 | running the crate's own suite is not running the suite |
| 882 | the viewer's own suite has two targets, not one |
| 892 | at wasm32 the CI row is `cargo check` |

**So 75 of the 87 rules are general** — citations, censuses, proxies,
sweeps, receipts, CI tiers, lane isolation, board reads, provenance —
and bind any program in this tree. That ratio is the single most
important input to the decision below: **six sevenths of this file has
nothing to do with the viewer.**

**Who cites it from outside `work/view/`.**

    git grep -l -F 'work/view/plan.md' origin/main -- '*.md' \
      | sed 's|origin/main:||' | grep -v '^work/view/' | grep -v '^work/STATUS.md'
    # 14 files

    git grep -n -E 'work/view/plan\.md:[0-9]' origin/main -- '*.md' | grep -v '^origin/main:work/view/'
    # 2 line-numbered citations, both in work/vdoc/stale-file-citations-after-the-split.md

Twenty citing lines in fourteen files, in four populations:

- **Structural inheritance — 10 lines in 7 files.** `work/vnews/`,
  `work/vseam/` and `work/vdoc/`'s `plan.md` §The register (2 lines
  each) and `program.md` (1 each), plus `work/vgeom/program.md`. These
  are the ones that dangle on sweep day.
- **Item bodies quoting a named rule — 7 lines in 4 files**, all
  VNEWS's and VDOC's, including the two line-numbered ones.
- **EDIT's two rows** citing this plan for an ITEM's text (the
  authored-step map), not for the register — 2 lines, and they die with
  the file too.
- `work/vnews/log.md`, narrative — 1 line.

**And one of the four inheritances is already broken.**
`work/vgeom/program.md` says *"`work/vgeom/plan.md` §The register says
why and what happens to it when VIEW's directory goes"*. There is no
such section: VGEOM's priority-seam re-cut of 2026-09-20 rewrote
`plan.md`, whose headings are now `## The slate`, `## Order` and
`## Review posture` only — §Charter went with it. So a VGEOM lane has
no route to the register today, and VGEOM has no written charter test
either. Filed as `work/vgeom/vgeom-plan-has-no-register-section-and-no-charter`.

## The options, with what each costs

`crates/viewer/README.md` stays ruled out and is now measured: 75 of 87
rules are not about the viewer, so it would put six sevenths of the
file beside code it says nothing about.

| option | who may edit it | "every rule is a named failure at a named PR" | what breaks on sweep day |
|---|---|---|---|
| **A. `docs/prompts/lane-register.md`** | **Ev signs off every change** — CLAUDE.md puts `docs/prompts/` on the sign-off list because it is the discipline handed to every lane by path. META owns the directory. | Survives: the text moves verbatim, PR numbers intact. | Nothing. The path is permanent and CLAUDE.md already hands the directory to every lane. |
| **B. `work/LANE-REGISTER.md`**, a peer of `work/README.md` with no program | Any lane, as today. | Survives verbatim. | Nothing. It belongs to no program, so no program's exit can take it. |
| **C. `work/meta/lane-register.md`**, an item in META's directory | Any lane; META custodies. The `work/meta/m6-carried-items-register` precedent is exactly this. | Survives verbatim. | Nothing until META closes, which is a larger event than VIEW's exit. |
| **D. One successor's `plan.md`**, the other three referencing it | That program's lanes. | Survives. | **Moves the problem to that program's exit**, and the four will not exit together. |
| **E. Split** (orthogonal to A–D): the twelve VIEW-bound rules to the viewer programs, the seventy-five general ones to A, B or C | Two answers, one per half. | Survives in both halves. | Nothing — but the split is a per-rule judgement that has to be re-made at every new rule's birth. |

**The costs that are not in the table.**

- **A's sign-off gate is the whole question.** This register went from
  nothing to 87 rules in eighteen days, and almost every one was
  written by an orchestrator in the hour after a lane failed — today's
  two (*a receipt that was never run in the form it was written down
  in*, and *a fence written in the same commit as the program it fences
  has no independent authority*) were written within an hour of the
  failures that produced them. Under A each of those waits on an `[ev]`
  PR round trip. That is not a small tax on a document whose value is
  that it is written while the failure is still legible.
- **B needs a two-line change to `scripts/work.py`, not just a README
  line — and the first draft of this bullet had it backwards.** I wrote
  that a file directly under `work/` is simply unparsed; it is not.
  `scripts/work.py:280` makes a top-level file that is not in
  `FREE_FILES` a lint **ERROR**: *"a file at the top of `work/` must be
  one of ['README.md', 'STATUS.md']"*. So B costs one entry in
  `FREE_FILES` and one line in `work/README.md`'s Layout block, both
  META's, both in the PR that moves the file. Read at the source rather
  than inferred from the `NARRATIVE` set, which is per-program and was
  the thing that misled the first reading.
- **C costs nothing mechanically and lies in the vocabulary**: `kind:
  issue` means *a defect or finding, not yet a unit*, and an open one
  is dispatchable work that charges META's budget forever. The
  precedent exists and is the same lie.

## Recommendation

**B, with E deferred and A put to Ev separately.**

- **B (`work/LANE-REGISTER.md`) unblocks VIEW's exit walk without
  asking anyone a governance question.** The register is read by lanes
  in `work/`, it is about how lanes work, and a top-level peer of
  `work/README.md` is where a reader would look for it. It belongs to
  no program, which is the one property every other option in `work/`
  lacks.
- **Do not split at the move.** A move whose only change is the path is
  checkable by `diff`; a move that also re-homes twelve rules is a
  judgement nobody can check against the original. Split afterwards,
  if at all — and the classification above is the input, including its
  stated blind spot.
- **A is the right long-run home for the general 75 and the wrong one
  today.** CLAUDE.md's test — *text that binds future work rather than
  describing this change* — does put the register there, and the day
  the general half stops growing is the day to move it. It is not that
  day: two rules were added this morning. Put A to Ev as its own `[ev]`
  question once B has made the path safe, and let the answer be about
  governance rather than about VIEW's exit walk, which is the coupling
  this row exists to break.
- Whatever is chosen, the move re-points **10 structural lines in 7
  files** and **7 quoting lines in 4 more**, and `work/vgeom/` needs a
  §The register section written rather than re-pointed, because it has
  none.

**This row stays open.** The decision is the orchestrator's; it closes
when the ruling lands and the move with it.


## Closed (2026-09-21) — the register is deleted, so it needs no home

Ev's ruling, in chat: lift out anything that reports an **actual
problem** AND that **a prompt update could actually fix** — not
something categorisable in retrospect that an advance warning would not
have prevented — *"otherwise it should just be deleted"*.

**Seven candidates were offered. None survived as new text.** Three
were already written down and the checking is the whole finding:

| candidate | where it already lives |
|---|---|
| verify a tier by shape, not a job total | `docs/prompts/implementer-discipline.md` §2 — *"if you cannot see twelve test jobs and five `k-lint (gate, …)` jobs on a code-tier run, something narrowed it"* |
| a receipt for a command never run in the form it was written | `implementer-discipline.md` §5 — *"a pattern with no hits recorded is a claim; a hit list is a receipt"* |
| an un-mergeable PR's CI looks like broken infrastructure | `memories/agent-lane-operations.md`, the conflicting-PR bullet |

The other four were retrospective categorisation. One **amendment**
came out of it (#3019): that memory bullet said such a PR gets NO CI
run, and its third face is a run that COMPLETES with every job dead in
2-3 seconds — a healthy docs tier except for `gate ok` — diagnosed with
`git merge-tree --write-tree`, never from the logs.

**So the row's question dissolves rather than being answered.** It
asked where 1,330 lines should live after VIEW's directory goes. They
live nowhere, VIEW's exit walk loses a precondition, and the four
successors point at `docs/prompts/` instead — which is where a lane's
standing obligations were supposed to be all along.

**What the register cost, said plainly, because it is the argument for
the ruling.** Three of the last day's failures were covered by text
read at the start of every session and not applied. The register was
not preventing those; it was recording them, and growing at about five
rules a day while four live programs were told to read it in full
before every dispatch.

Recoverable at `66d7357417`, the last commit that carried it — the
convention `docs/DOC-LEDGER.md` uses for a deleted exit walk.
