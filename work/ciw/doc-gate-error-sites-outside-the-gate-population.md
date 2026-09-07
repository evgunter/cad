---
id: doc-gate-error-sites-outside-the-gate-population
kind: issue
title: doc-gate.sh carries 14 gate_error sites outside the gate population D109 reads
status: open
opened: 2026-09-06
---


`scripts/doc-gate.sh` sources `scripts/gates/lib.sh` — it is the gate
`scripts/gates/gate-roster.sh` counts as "1 under lib.sh's contract
sited outside it" — and carries 14 `gate_error` sites. The reading
GATES' D109 lane instrumented (PR 2038: which `gate_error` sites fire
under some `--selftest`) and the lane that drove `viewer-module-kinds.sh`
to zero unreached sites (PR 2057) both walk the declared population
`scripts/gates/*.sh` only; `doc-gate.sh`'s sites have never been read,
so whether each of its guards fires under its own self-test is unknown
in the same way the six viewer guards were until PR 2057.

Cite: `scripts/doc-gate.sh` (`grep -nE '(^|[[:space:];&|])gate_error "' scripts/doc-gate.sh`
lists the 14); `scripts/gates/gate-roster.sh` (the "sited outside it"
count). Reported by the GATES lane for PR 2057 as outside its fence;
filed here because `scripts/doc-gate.sh` is CIW's path.

Two shapes for the fix, either CIW's call: run the D109 reading over
`scripts/doc-gate.sh` too (widen the instrumentation's population to
"every script that sources lib.sh", which `gate-roster.sh` already
knows how to enumerate) and plant fixtures for what comes out
unreached; or move `doc-gate.sh` under `scripts/gates/` so the existing
reading covers it — a move GATES would need to agree to, since it puts
CIW's file in GATES' fence.
