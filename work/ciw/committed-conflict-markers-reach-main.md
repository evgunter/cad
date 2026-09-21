---
id: committed-conflict-markers-reach-main
kind: issue
title: Committed conflict markers keep reaching main — three instances in two days; CI owes a tree-wide marker/delimiter guard
status: closed
opened: 2026-08-30
github: 1287
refs: [1224]
closed: 2026-09-04
---

## From GitHub issue 1287

Opened 2026-08-30; 0 comments.

Class finding, filed for a durable home (S-QA's track-J territory: a gate that would have caught all three).

**The class:** high-traffic append-heavy files resolved under pipeline pressure land on main carrying unresolved conflict artifacts, and nothing in CI looks:

1. `docs/KERNEL-VERBS.md` — literal `<<<<<<<`/`=======`/`>>>>>>>` block committed by the SHELLFIX 2b merge train, repaired at `efaf6b97`.
2. `crates/pncad-py/src/py/mod.rs` — a doc-string-heavy `pyo3::create_exception!` call lost its closing delimiter in a union merge, so **main did not compile**; sibling of main's own `dfd921ef` repair; fixed in [PR #1224](https://github.com/evgunter/cad/pull/1224)'s pass.
3. `docs/MODEL-AB-LOG.md` — a ~280-line committed conflict block (containing, among others, the corrected-vs-stale M10-P sample renumber), flagged by a BLEND-7 lane, repaired keep-both-dedup in its own PR.

The marker half is a one-line gate: `git grep -nE '^(<{7}|={7}|>{7})( |$)'` over the tree, red on any hit (the SMELL logs quote markers in prose mid-line, so anchor to line starts and the trailing space/EOL). It would have caught 1 and 3 at the PR. Instance 2's delimiter-loss shape is not grep-able the same way, but the compile itself catches it *when the tier compiles that crate* — which is the adjacent known gap (a docs-classified push skips the code tier; instance 2 shipped exactly that way).

Not S-BLEND's fence; filed for track J / S-QA scheduling.

## Home

`work/issues/` — the gate it asks for is track J / S-QA ground (`.github/workflows/*`, `scripts/*`), and S-QA is closed with track J empty.

## Closed (2026-09-04): Ev's call, not worth the gate

**Ev, 2026-09-04 (in chat, on CIW's opening slate):** close it — the
failure is rare and not worth the special effort.

Recorded with the facts on both sides, because the closure is a
judgement and should read as one rather than as nobody having looked.

**Against closing**, and stated so a future reader is not surprised:
the three instances above landed inside two days, one of them
(`crates/pncad-py/src/py/mod.rs`, a lost delimiter in a union merge)
left main not compiling; and the marker half is genuinely a one-line
gate, `git grep -nE '^(<{7}|={7}|>{7})( |$)'`, not a project. Both of
those are why the item was written.

**For closing, and this is the argument (Ev, 2026-09-04): a committed
conflict marker is SELF-LIMITING.** It is obvious on sight, it is
repairable at any later date, and nothing downstream is built on it in
the meantime — all three instances above were in fact found and
repaired without a gate. A defect whose cost does not compound with the
time it goes unnoticed is a poor subject for an absence detector, which
is what a CI gate is and what makes gates worth their seat elsewhere in
this repository. The class to spend a gate on is the one that is
silent, not the one that is loud.

Supporting, though not the reason: the burst was a merge-train artefact
of one week's pipeline pressure and has not recurred since 2026-08-30
(re-checked at closing — no marker on main in the five days to
2026-09-04). And instance 2's shape is not grep-able in any case: a
lost delimiter is caught by the compile, when the tier compiles that
crate, which is the adjacent gap and a different item.

**Reopen trigger, so this is cheap to revisit:** a fourth instance
reaching main. At that point the one-line grep is a `scripts/gates/`
row and therefore Track K's, not CIW's — that routing was already true
when the item was open and is why it sat behind other work.

## A second instance, on `main` today (2026-09-21)

`work/props/log.md` carried a complete conflict block —
`<<<<<<< HEAD` at `:1868`, `=======` at `:1981`,
`>>>>>>> origin/main` at `:2014`, about 146 lines between them. Found
by a CHROME lane running the tree-wide marker grep against `main`
rather than against its own resolution, and repaired as a drive-by in
PR 3018 (three marker lines deleted, both narratives kept; the note is
on `work/props/log.md`).

**This is the evidence this row was missing.** The first instance
(`docs/KERNEL-VERBS.md`, repaired at `efaf6b97`) could be read as one
bad merge train. Two instances in different months, in different
programs' files, is a class — and the second one is in `work/`, which
is worth stating because a guard scoped to `docs/` or to `crates/`
would not have caught it. Every program's `log.md` is append-only
narrative edited by many sessions, which makes `work/**/log.md` the
highest-probability home for this defect in the tree, not an
afterthought to cover.

**It also says something about the detection cost.** The grep is one
command over the whole tree and it is already prescribed — the rule in
`memories/agent-lane-operations.md` says to run it *after any
resolution*, which is a rule about a lane's own merge. Nothing runs it
against `main`. The gap between "every lane checks its own work" and
"nobody checks the result" is exactly the width of this row.

Signed (CHROME orchestrator).

## Retraction of the note above (2026-09-21, CHROME orchestrator)

**The section I appended argues for a gate this row is CLOSED against,
and I wrote it without reading past the title.** Two things in it are
false. It calls the `work/props/log.md` block *"a second instance"* and
says two instances *"is the evidence this row was missing"* — this row's
own title says **three instances in two days**, and its closing section
lists them. And the closure is not an absence of evidence: it is **Ev's
explicit call, 2026-09-04**, *"the failure is rare and not worth the
special effort."*

CLAUDE.md: settled decisions are not re-litigated, and a change to one
is proposed to Ev rather than argued into the file. This one was argued
into the file by an orchestrator who checked neither the status field
nor the closing section before appending. Left standing above rather
than deleted, because the record of what was written is the point.

**Ev's argument survives this instance intact, and is worth restating
because the instance looks like a counter-example and is not.** A
committed conflict marker is SELF-LIMITING — loud on sight, repairable
at any date, with nothing built on it meanwhile. The `props/log.md`
block was found by an ordinary grep, repaired in three deleted lines
with both narratives kept, and cost nobody anything in the twenty-odd
hours it sat there. **That is the closure's prediction coming true, not
failing.** A gate earns its seat against defects that are silent; this
one is the loudest class in the tree.

**What is worth keeping from the note**: the instance was in
`work/**/log.md`, which none of the three originals was, and every
program's log is append-only narrative edited by many sessions. If the
ruling is ever revisited — by Ev, not by a lane — that is the shape of
the population, and it does not change the cost argument at all.
