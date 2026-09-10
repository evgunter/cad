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
36 warnings, 453 notes.**

The distribution is what decides the shape of the row, and it is not
what a first glance suggests. The note tier is **SC2317 (264,
"unreachable command") and SC2016 (168, "expressions do not expand in
single quotes")** — two codes that between them are 87% of every
finding. SC2086 word-splitting, the code this tree already suppresses at
eleven sites, is **10**. So the cost is concentrated in two codes with
one decision each, not spread thin across the tree.

**The 7 errors are all false positives, and were verified as such before
anything was routed anywhere.** All seven are SC1087 over
`$esc[[:space:]]` in `scripts/gates/gate-roster.sh` (`:160`, `:164`,
`:279`, `:284`, `:288`, `:292`) and
`scripts/gates/panic-free-macro-bodies.sh:143`. shellcheck reads the
`[` as an array subscript on `$esc`; it is a POSIX character class
inside a `grep -E` pattern, `[` cannot be part of an identifier, so the
class survives literally and the greps match as intended. The code is
correct and belongs to GATES, not to CIW.

They matter to this item for one reason: **they are exactly the findings
a `--severity=error` floor would hit first**, so the naive enablement row
opens with seven phantom to-dos in another program's files. Any severity
selection has to start by disposing of them — a `# shellcheck disable`
with the reason at each site, or an explicitly excluded code.

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
