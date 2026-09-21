---
id: program-md-cites-a-plan-section-the-re-scope-deleted
kind: issue
title: program.md points lanes at plan.md's §The register, which the 2026-09-20 re-scope removed
status: open
opened: 2026-09-21
priority: P3
cost: E
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
