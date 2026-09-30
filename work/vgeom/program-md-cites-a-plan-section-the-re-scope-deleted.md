---
id: program-md-cites-a-plan-section-the-re-scope-deleted
kind: issue
title: program.md points lanes at plan.md's §The register, which the 2026-09-20 re-scope removed
status: closed
opened: 2026-09-21
priority: P3
cost: E
closed: 2026-09-21
---


## Finding

Found by the `vgeom/refusal-floor` lane while reading its brief's
pointers.

`work/vgeom/program.md`'s closing paragraph says:

> The lane register that binds every lane dispatched here is
> `work/view/plan.md`'s, inherited by reference and not copied —
> `work/vgeom/plan.md` §The register says why and what happens to it
> when VIEW's directory goes.

**`work/vgeom/plan.md` has no §The register.** Its sections are
`## The slate`, `## Order` and `## Review posture`. The re-scope of
2026-09-20 (`fb899b019c`, *"work: cut SHELL, PATHS, WIRE, VGEOM and
SYM on their priority seams"*) rewrote the file and the section did not
survive; `program.md` was not re-read against it.

Two things ride on that sentence and are currently unreachable from
the tracker:

- **why** the register is inherited by reference rather than copied,
  which is the answer to a lane asking whether `work/view/plan.md`
  binds it;
- **what happens to it when VIEW's directory goes**, which
  `work/view/plan.md` itself flags as a precondition of VIEW's exit
  walk (`the-lane-register-has-no-home-after-views-directory-goes`).

The register itself is fine and is where the sentence says it is. What
is missing is the paragraph that placed it.

## Also missing, and it cost a dispatch

The same re-scope removed whatever text named CHROME's
`chrome/datums-substitution-sweep` (#2644) as the shape precedent for
this program's substitution class. A dispatch written today quoted
`work/vgeom/plan.md` as saying it *"plainly"*; it does not say it at
all. The lane found #2644 anyway from the PR number in the brief and
the precedent is the right one — but the citation was to a file that
does not carry it, which is the register's own *a board read out of the
working tree is read at whatever commit that tree is pinned to* class
with the pin one re-scope stale.

## Fence

`work/vgeom/` — this program's own.

## Closed (2026-09-21, VGEOM orchestrator state-sync)

Both halves discharged, in two commits rather than one, and the row
stayed open across the gap because only the first had landed.

**The pointer resolves again.** `work/vgeom/plan.md` §The register was
restored on 2026-09-21 alongside §Charter, by the same commit that
closed `vgeom-plan-has-no-register-section-and-no-charter`, so
`program.md`'s closing paragraph now points at a section that exists.
`program.md` itself needed no edit: the sentence it carries was correct
about where the register lives and about what the section says, and
what had gone missing was the section, not the citation. Verified by
reading both files at `origin/main` rather than out of a working tree —
the register's own rule, and the rule this row's second half is an
instance of.

**The shape precedent is back, re-derived rather than restored
verbatim.** The deleted sentence lived inside the pre-cut Order's unit
1, which named `chrome/datums-substitution-sweep` as the same class in
`datums.rs` and said *"announce before taking any of these, and read
that unit's shape rather than inventing a second one."* Recovered from
`git show fb899b01 -- work/vgeom/plan.md`. It is not restored where it
stood, because the unit it was attached to has since landed (#3000) and
a sentence re-attached to a discharged unit would be a second stale
citation. It now stands in §Order as **The shape precedent**, naming
both #2644 and this program's own instance #3000, stating the shape in
one sentence so a lane need not read two PRs to learn it, and keeping
CHROME's announce clause.

**PR numbers verified, not quoted**: `git log origin/main --grep` finds
`2d2bfe0d Merge pull request #2644 from evgunter/chrome/datums-substitution-sweep`
and `c481b481 Merge pull request #3000 from evgunter/vgeom/refusal-floor`.

**Ratification check, per CLAUDE.md**: none is owed. `work/vgeom/` is
this program's own tracker directory, not `docs/DESIGN.md`, not a page
`docs/DESIGN.md`'s companion table lists, not `memories/` and not
`docs/prompts/`. The pickaxe was run anyway for the deleted sentence
and returned the cut's own commit `fb899b01` and nothing behind it, so
the text it removed had no sign-off to honour either — and the history
in this checkout does not reach `initial commit`
(`git log origin/main --format=%h | tail -1` gives `b55ff18e`, a merge),
so a nil result here would have been worth nothing. This one was not
nil.
