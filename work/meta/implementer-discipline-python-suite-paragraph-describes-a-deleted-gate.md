---
id: implementer-discipline-python-suite-paragraph-describes-a-deleted-gate
kind: issue
title: implementer-discipline §2 still tells every lane the python suite is seed-gated; the gate was deleted on 2026-09-12 and it runs on every code-tier run
status: open
opened: 2026-09-13
priority: P4
cost: E
---



(VIEW) Found by the `view/numeric-field-door` lane (PR 2519) when its
own run contradicted the doc it had just been handed. Filed on META's
slate because `docs/prompts/implementer-discipline.md` is META's by
`scripts/work.py territory`, and because the text binds every lane by
path rather than describing any one program's code.

## The claim, and the run that falsifies it

§2 says, in bold:

> **The python suite runs whenever a seed is a crate a build of the
> wheel compiles** — `pncad-py`'s non-dev dependency closure, which on
> this tree is every workspace member except two: `viewer`, which sits
> above the wheel, and `test-utils` […]. A closure seeded only in one
> of those two skips it; everything else buys it. The `change filter`
> job's log prints both the seed set and `RUN_PNCAD_PY`, so a run says
> which way it went.

PR 2519's run (34777661121, head `71c1e3b9`) is a closure seeded only
in `viewer` — the `change filter` log prints `SEEDS=viewer` and
`RUN_PNCAD_PY=false` — and **`python suite (wheel + guide +
north-star)` ran, all sixteen steps, and passed.** By the paragraph it
should have skipped.

## Why

`ci.yml` deleted the gate and says so at the site:

> NO `run_pncad_py` OUTPUT. The filter still computes the key and
> echoes it with the seeds it came from […] but no job here gates on
> it: the python suite runs on every code-tier run.

The job's condition is now
`github.event_name != 'push' && needs.filter.outputs.run_build == 'true'`.

Both dates are in the tree. The doc paragraph was written
`370a7dfdb9` (2026-09-06), when the axis really did gate the job;
`b6cc8d4d2e` (2026-09-12, the TCOST-C1/C2/C3 restore) ungated it in
`ci.yml` and left the prose. Six days, and every dispatch in between
handed a lane a false statement about what its own run covers.

## Which way the error cuts

Towards a lane doing MORE than it had to, not less — this lane ran the
viewer suite twice over believing the hosted run would not carry the
python half. That is the cheap direction, but it is still a standing
doc telling lanes a green run means less than it does.

The paragraph's last sentence is separately wrong now: the `change
filter` log does print `RUN_PNCAD_PY`, but that value no longer says
"which way it went" about the job — it says which way it WOULD have
gone under an axis nothing reads.

## Not a duplicate of three neighbours

- `work/tcost/run-pncad-py-is-computed-and-gates-nothing` (open) is the
  same fact from the other side: it asks whether the computed-and-unread
  AXIS should be given a consumer or deleted. It does not touch the
  standing doc, and the doc is wrong whichever way that is decided.
- `work/ciw/python-suite-axis-skips-only-two-members` (open) asks
  whether the exception earns its machinery. Same axis, same
  independence from this row.
- `work/ciw/ciw-rows-and-ci-local-prose-rotted-by-the-c1-c3-restore`
  (open) is the prose-rot row for exactly the commit that caused this —
  but it enumerates THREE sites, all of them CIW's own
  (`local-scripts/*` and two CIW items), and says so in its own header.
  `docs/prompts/` is not among them and is not CIW's to edit.

## Fence

`docs/prompts/` waits for Ev's sign-off before merging by `CLAUDE.md`'s
own rule — it is the standing discipline handed to every lane by path.
So this is a row to be taken with that in mind, not a wording fix a
lane can land in passing.
