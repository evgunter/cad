---
id: shellcheck-is-not-run
kind: issue
title: no shell linter runs anywhere: 20+ `# shellcheck` markers are unverifiable claims
status: open
opened: 2026-09-10
refs: [apt-preamble-bypass-is-unguarded, pipestatus-after-assignment-in-ci-yml]
---

Disclosed by the `pipestatus-after-assignment-in-ci-yml` sweep, which had
to settle "would `shellcheck` have caught this?" and measured the answer
instead of assuming it. `apt-preamble-bypass-is-unguarded` names the same
absence in one clause ("there is no shell linter in the hosted gate at
all"); this is that clause with the measurements attached, so the shape
of the row is decidable.

## What is unguarded

`shellcheck` is invoked by **nothing** — not `.github/workflows/*.yml`,
not `local-scripts/ci-local.sh`, not `scripts/`. The tree nevertheless
carries the annotations of a repo that runs it: `# shellcheck
source=…`, `# shellcheck shell=bash`, and about twenty `# shellcheck
disable=SC…` markers across `local-scripts/ci-local.sh`,
`local-scripts/render-hosted.sh`, `scripts/apt-install.sh`,
`scripts/doc-gate.sh` and `scripts/gates/probe-suite-census.sh`.

A `# shellcheck disable=` is a claim that a check exists and was
consciously overridden. With no checker it is an unverifiable claim that
decays silently — the exact defect `ruff.toml`'s header records for the
twelve `# noqa` markers it found, five of which had already decayed.

## What it would find today

Measured on shellcheck 0.9.0 over `local-scripts/*.sh scripts/*.sh
scripts/gates/*.sh demos/*.sh`, 2026-09-10: **496 findings — 7 errors,
36 warnings, 453 notes.** All 7 errors are SC1087 (`$var[idx]` inside a
string) in `scripts/gates/gate-roster.sh` and
`scripts/gates/panic-free-macro-bodies.sh`, which are GATES' files, not
CIW's. The note tier is dominated by SC2086 word-splitting, which this
tree does deliberately in places and already suppresses at eleven sites.

So the row is not "turn shellcheck on" — a 496-finding gate is not
landable as one commit. It is a severity selection with a stated
rationale, in the shape `ruff.toml` already models for Python: an
`--severity` floor or an explicit enable/disable list, each entry
carrying its reason, plus the reconciliation that makes the linted
population derived rather than assumed.

## The specific hole this leaves open

`scripts/check-status-capture.py` guards `PIPESTATUS` reads. Its sibling
class is the `$?` read taken after an intervening command, which
shellcheck **does** catch (SC2319 "this `$?` refers to a condition, not
a command"; SC2320 "…refers to echo/printf…") and which
`check-status-capture.py` deliberately does not — reimplementing
SC2319/SC2320 in-repo would be a second, worse copy of a rule that
already exists.

The tree is clean of SC2319/SC2320 **today** (verified in the same run:
zero findings of either), so nothing is broken right now. Nothing keeps
it that way, and the `$?` shape fails exactly as silently as the
`PIPESTATUS` one did: a `status` that is always `0` and a `case` whose
non-zero arms are unreachable.

`shellcheck` would also not have caught the `PIPESTATUS` shape itself —
0.9.0 reports nothing at all on the five-line reproduction in
`pipestatus-after-assignment-in-ci-yml`. That is why the guard exists as
its own script rather than as a shellcheck row, and it is the reason
this item is a residue and not a supersession.
