---
id: shell-doors-take-tolerance-beside-tol
kind: issue
title: shell / shell_open take a raw tolerance: f64 beside tol: Tol, and the acceptance that no verb takes a Band beside a Tol has no mechanical guard
status: open
opened: 2026-08-31
github: 1409
refs: [1399]
---

## From GitHub issue 1409

Opened 2026-08-31; 0 comments.

(SEAT orchestrator) Two related findings from SEAT-1's dual review (PR #1399), filed as one issue because both are about the verb doors' tolerance vocabulary.

**1. The instance.** `topo::shell` and `topo::shell_open` still take a raw `tolerance: f64` parameter beside the `tol: Tol` witness (`crates/topo/src/shell.rs`, the doors' signatures). That is the exact shape SEAT-1 just removed for `Band`: a second tolerance-flavored argument whose relationship to the committed global is the caller's problem. Whether this `tolerance` is genuinely independent (a per-call offset fitting budget, say) or another derivable-from-`Tol` value needs measuring before any change — SEAT-1's rule was "drop the parameter only after proving every caller passes the canonical derivation". Out of SEAT-1's ratified scope (VERB-SEAT-DESIGN §1 S4 names `Band` only), so recorded here rather than widened into that unit.

**2. The guard gap.** SEAT-1's acceptance clause — *no public kernel verb takes a `Band` beside a `Tol`* — is now a measurement with no mechanical guard: a future verb could reintroduce the parameter and nothing goes red. If the parameter-lint machinery (`docs/PARAM-LINT-SPEC.md` / `tools/k-lint`) grows signature rules at some point, this is a candidate row; until then this issue is the durable record that the invariant is unguarded by design, not by oversight.

## Home

`work/seat/` — the verb doors' tolerance vocabulary is SEAT's §1 ground (band derivation at operation entry) and both halves are SEAT-1's own disclosed residue.
