---
id: territory-base-main-reads-a-stale-local-ref-and-answers-confidently-wrong
kind: issue
title: work/README.md documents 'territory --base main', but in this repo's working model the local main ref is reliably stale, so the documented invocation answers confidently wrong instead of refusing
status: open
opened: 2026-09-15
priority: P3
cost: E
---


## Finding

`work/README.md` documents the territory check twice, both times against
the bare ref `main`:

- `:190` — *"`scripts/work.py territory --base main` reads a branch's
  prefix and its diff and names every path another program owns"*
- `:257` — the script's usage block, `territory --base <ref>`

**In this repo's working model that ref is reliably stale.** Agents work
in ephemeral worktrees and remote checkouts and never check out `main`,
so the local `main` ref is whatever it was when the checkout was made and
is never advanced. Measured in this session's checkout on 2026-09-15:

```
main        0312083aa   2026-09-12 19:17:36
origin/main 385c01b33   2026-09-15 00:22:07
git rev-list --count main..origin/main   ->  12068
git rev-list --count origin/main..main   ->  0
```

Three days and twelve thousand commits behind, and a strict ancestor —
so nothing is diverged, the ref is simply old.

## Why this is worth a row rather than a habit

**The wrong answer is plausible, not obviously wrong.** Diffing a branch
against a three-day-old base makes the branch look like it contains every
change that landed in between, so `territory` names ~200 paths across
most open programs. That reads exactly like *"your branch crosses
everyone's territory"* — a scary, actionable-looking result — rather than
like *"your base is wrong"*. A lane that follows the documented
invocation and then believes the output will either announce seams it
never crossed, or (more likely) conclude the check is noise and stop
running it.

Found independently twice on 2026-09-15: by a WIRE implementer lane,
which caught it and used `origin/main` instead, and by the WIRE
orchestrator verifying that lane's report. The lane got it right, which
is the good case; the row exists because nothing in the tooling or the
contract would have stopped it getting it wrong.

## What a taker owes

A decision between three shapes, and the last is the cheapest:

1. **Refuse a stale base.** `territory` compares the given base against
   its remote-tracking counterpart and refuses when the base is behind,
   naming both SHAs. Fail-loud, which is this project's rule, and it
   catches every future spelling of the mistake.
2. **Default the base.** Make `--base` optional and resolve it to
   `origin/<default-branch>` when omitted, so the right thing is the
   thing you get for free.
3. **Fix the documentation only** — `work/README.md` says `origin/main`
   in both places. Cheapest, and strictly weaker: it does nothing for a
   lane that types `main` from habit, and habit is where this came from.

Whichever lands, `work/README.md:190` and `:257` move together, and the
`--selftest` gains a row: `scripts/*.py` is exactly the change set the
per-PR gate runs on (`docs/prompts/implementer-discipline.md`), so the
guard belongs there rather than in a nightly.

## Filed from outside the fence

Filed by the WIRE orchestrator under
`docs/prompts/implementer-discipline.md` §6. `scripts/work.py` and
`work/README.md` are both in META's `paths`, so this is squarely META's
ground rather than a routing guess.
