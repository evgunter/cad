---
id: lane-cross-program-filing-two-binding-docs-conflict
kind: ruling
title: May a lane file on another program's slate? implementer-discipline SS6 forbids it and work/README requires it
status: closed
opened: 2026-09-12
closed: 2026-09-12
---


Found by the style review of PR #2411 (grid-pitch), whose lane filed
`work/chrome/metres-per-pixel-swallows-a-nan-depth.md` from a DOOR unit
branch. The filing was correct and non-duplicative; the question is
whether the lane was allowed to make it.

## The two texts, on exactly this case

**`docs/prompts/implementer-discipline.md` §6 forbids it**, and states
its reason:

> They go in your report and your PR description — **not into another
> program's tracker directory.** … **you cannot tell whether the item
> already exists.** Two lanes in one session filed the same inherited CI
> red into two different programs' directories … The orchestrator could.
> Report it; let the party with the whole board place it. … **Outside it,
> reporting IS the filing act** — you hand it over and the orchestrator
> writes the file.

**`work/README.md` requires it**, and states its reason:

> `work/code-quality/` used to be where one waited for a claim; it left
> the tracker on 2026-09-11 … so **a finding now goes straight onto the
> slate of the program whose ground it lands on** (`:117`)
>
> When the owning program is clear, file the item straight onto that
> program's slate — **a lane does not need the owner's permission** to
> put a finding where it belongs, and routing it through `issues/` only
> delays the owner seeing it. (`:146`, Ev, 2026-09-04)

Both are binding on every lane. A lane that reads both cannot act.

## They are closer than they look, and the residue is narrow

The two rules answer **different questions** and only one clause
actually collides:

- §6 is about **who files** — the orchestrator, because only they can
  see a duplicate.
- `work/README.md` is about **where it lands** — the owner's slate, not
  `work/issues/`, because `issues/` is a waiting room and
  `code-quality/` is gone.

Those compose: **the lane reports, the orchestrator files on the owner's
slate.** §6 even anticipates the destination rule — *"the orchestrator
writes the file, in `work/issues/` **when no program obviously owns
it**"* — which implies the owner's slate when one does.

What genuinely collides is one clause: *"a lane does not need the
owner's permission"*. Read strictly it authorises the lane to skip the
orchestrator hop, which is the one thing §6 exists to prevent.

## Evidence from today, both directions

- **The lane-direct path worked, twice.** #2406 filed on `work/exch/`
  and #2411 on `work/chrome/`; the orchestrator checked both for
  duplicates and correctness and both were clean.
- **§6's cited failure is real and this program reproduced its
  conditions.** Five of DOOR's eleven opening rows were already closed
  or half-closed by FIX and VIEW — duplication the lanes could not see
  and the orchestrator could. That is exactly §6's argument, measured on
  this program.

## The orchestrator's read

**§6's mechanism, `work/README.md`'s destination.** A lane reports in
its PR body; the orchestrator files on the owner's slate in the upkeep
PR that follows. It costs one hop and buys the duplicate check that this
program has already needed five times in a day.

The one-clause fix would be to narrow *"a lane does not need the owner's
permission"* to say that no program's consent gates a finding reaching
its slate — which is what it is for — without making the lane the party
that writes the file.

**Not acted on unilaterally**: closing this means editing
`docs/prompts/implementer-discipline.md` or `work/README.md`, and
`CLAUDE.md`'s approval rule now names `docs/prompts/` explicitly.

## Ruled (Ev, 2026-09-12, on PR #2421)

> `work/README.md:117,146` is right; the prompt is wrong and should be
> updated to agree with the README to have no reservations about filing
> directly to other programs.

**The orchestrator's read was the losing one** — it proposed §6's
mechanism and Ev took the other side outright. §6 now directs lanes to
file on the owner's slate in the same PR, without permission and without
routing; the old reasoning is gone rather than footnoted, surviving only
as a line telling a lane to grep the target directory first.

Preserved, because they were never in dispute: a disclosed residue owes
a file rather than a PR-body sentence, and a lane reports which rows it
filed where.

The five cross-program files this session's lanes wrote — one on
`work/exch/`, four on `work/chrome/` — were correct under the rule as it
now reads.

## Disclosure

The orchestrator instructed lanes the `work/README.md` way all day
(#2406 to `work/exch/`, #2394's residue to `work/scalar/`, #2411 to
`work/chrome/`), having read §6 and not noticed the collision. Those
three files are on the right slates and stay; the instruction was one
side of an unresolved conflict rather than a ruling.
