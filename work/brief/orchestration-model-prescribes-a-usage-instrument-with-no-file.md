---
id: orchestration-model-prescribes-a-usage-instrument-with-no-file
kind: issue
title: memories/orchestration-model.md prescribes the two-window usage check by a path that does not exist on at least two agent layouts, so the rule reads as followed when it cannot be
status: open
opened: 2026-09-15
priority: P4
cost: E
---


## Finding

`memories/orchestration-model.md`'s usage-alert bullet forbids inferring
from the alert event and prescribes reading the truth instead:

> `tail -1 <agent-dir>/events/claude/usage/events.jsonl` carries
> `rate_limits.five_hour` and `.seven_day` percentages together.

The reason it gives is good and is why the rule exists: a RESET event may
name only the 7d window while the 5h one is still full, so one window's
number is not evidence about the other.

**The file it names is not there.** Two layouts observed:

- WIRE's orchestrator session of 2026-09-14, alerted at 90% of its own
  5h window, found no such file under its agent directory and no usage
  jsonl written in the preceding two hours. It acted on the alert's
  single number knowingly and said so (`work/wire/log.md`, *"usage
  wind-down at 90% of the 5h window, and an instrument that is not
  there"*).
- WIRE's successor orchestrator, 2026-09-15, on a hosted remote box:
  there is no agent-directory layout at all — no
  `~/.local/share/cad-work`, and a bounded `find / -maxdepth 6 -name
  events.jsonl -path '*usage*'` returns nothing.

## Why it is worth a row rather than a shrug

**A rule that cannot be followed is worse than one that is merely
unwritten, because it reads as having been followed.** The memory is read
at the start of every orchestrator session and its instruction is
imperative and specific; an orchestrator that does not find the file has
no prescribed fallback, and the honest thing it can do — act on the
alert's one number and disclose it — is exactly what the bullet was
written to forbid. Nothing distinguishes that session's log from one that
made the check.

The same bullet's other half is unaffected: *act only on alerts naming
your own account*, and the account resolution from the agent dir, are
independent of this file.

## What a taker owes

A decision, and it is `memories/`, so it is Ev's (CLAUDE.md: *"PRs that
add to or change `memories/` … wait for Ev's sign-off"*) — which is the
reason this is a row and not a patch:

- whether the path is simply stale and a current one exists (find it and
  re-cite it), or
- whether the instrument is layout-dependent, in which case the bullet
  owes a **stated fallback** for the layouts without it, so that acting
  on one number is a described state rather than a silent violation.

Either way the bullet should say how an orchestrator knows which case it
is in, since today it cannot tell "no file" from "wrong path".

## Filed from outside the fence

Filed by the WIRE orchestrator under `docs/prompts/implementer-discipline.md`
§6. `memories/` is in no program's `paths` (`work.py territory --files -`
returns no owner), and this is a process instrument that binds every
session by path, which is META's charter. Re-home if that call is wrong.
