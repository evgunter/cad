---
id: doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module
kind: issue
title: doc-gate.sh's header does not say that #[cfg(test)] modules are outside its population
status: open
opened: 2026-09-16
priority: P4
cost: E
---

`scripts/doc-gate.sh` builds rustdoc at the host cfg, so no `#[cfg(test)]`
module is ever compiled by any of its passes, and an intra-doc link written
inside one is never resolved. The witness is `crates/pncad-py/src/tests.rs`
(a `#[cfg(test)]` module), where a dangling name sat for two commits with the
gate green (CENSUS-ARRIVAL-RESIDUE, 2026-09-16). The header already names this
as a *second* reason some `cfg(not(debug_assertions))` prose is unread
(`scripts/doc-gate.sh:358-381`), but nowhere states it as the gate's scope, so
a green reads as "every doc link resolves".

A `--cfg test` rustdoc pass is not the answer: it is a distinct feature
unification sharing almost no artifacts with pass 1 (the header's own pass-3
costing, `scripts/doc-gate.sh:383-395`, measured +33 to +58 s for a smaller
pass), and pass 1 runs on every PR in `ci.yml`'s lint job (`ci.yml:322-335`).
No incident has followed the 2026-09-16 one.

The repair is one paragraph in the header, beside the `not(feature)` blind
spot it is the twin of (`scripts/doc-gate.sh:299-309`): `#[cfg(test)]`
modules are outside the gate's population, links in them are unchecked, and
why (the cost above), plus the same sentence at the gate's `gate_ok` line if
it names its scope.

What would close it: the header paragraph stating the `cfg(test)` scope and its reason, landed in `scripts/doc-gate.sh`.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Pass 1 runs on every PR (`ci.yml:322-335`). A `--cfg test` pass is a distinct unification that shares no artifacts.
The header prices a smaller extra pass at +33 to +58 s (`scripts/doc-gate.sh:383-395`).
One incident (2026-09-16, two commits of a dangling name in a test module), none since.
Replacement: say the scope in the header. See `reform/doc-gate-cannot-see-a-broken-link-inside-a-cfg-test-module.md`.
