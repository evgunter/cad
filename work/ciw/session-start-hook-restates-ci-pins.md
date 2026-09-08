---
id: session-start-hook-restates-ci-pins
kind: issue
title: the agent-container hook restates three ci.yml pins as literals, out of every gate's reach
status: open
opened: 2026-09-06
refs: [local-half-restates-ci-pins-as-literals, ruff-pin-read-shares-the-first-match-shape]
---

`.claude/hooks/session-start.sh:104,123,124` sets `NEXTEST_VERSION=0.9.140`,
`MATURIN_VERSION=1.14.1` and `TY_VERSION=0.0.39` as shell literals, and
INSTALLS from them — the nextest tarball URLs at `:117-118` interpolate the
first one. `.github/workflows/ci.yml`'s workflow-level `env:` block is the
single source of truth for all three; nothing reconciles the hook's copies
with it.

**Why it is not closed by the unit that closed the rest of its class.**
`local-half-restates-ci-pins-as-literals` was discharged by
`check-ci-mirror-parity.py`'s pin-literal claim, which derives every version
literal under `local-scripts/` and reds when one names no pin `ci.yml` sets.
That claim cannot reach these three: every hosted job deletes `.claude/` at
checkout by design, so a claim about that file would pass on hosted CI and red
only on a developer's box — a check whose verdict depends on which half is
running it is worse than no check, and worse in the direction this whole class
is about.

**So the fix here is the other one: stop restating the value.** The hook can
read each pin with `scripts/ci-pin.py NAME`, which is what every other
programmatic reader in the tree now does. It is a separate argument because of
where it runs: this hook is the agent container's provisioner, executed before
any session does anything, and no gate of record covers it. A `ci-pin.py` call
that refuses — a workflow moved, a pin renamed, a `python3` the container does
not have on PATH yet at that point in the hook — ends the session's setup
rather than one CI row, so the change has to decide what the hook does when the
read fails, and that decision (fall back to a literal? proceed unpinned? fail
the hook loudly?) is the whole content of the item. `.claude/` is also outside
`work/ciw/program.md`'s `paths`, so whether CIW owns the edit at all is the
orchestrator's call.

**What it costs while it sits.** The day a pin is bumped, agent containers go
on installing the old cargo-nextest, maturin and `ty` — silently, and in the
one lane nothing gates. The three lines drift exactly as
`local-scripts/`'s did; the difference is only that nothing here can be made to
say so.
