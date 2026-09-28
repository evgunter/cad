---
id: at-rest-body-verdict-is-not-tied-to-its-symbolic-session
kind: issue
title: AtRestBody's kept verdict is not tied to the geom_core::sym session it was derived under
status: open
opened: 2026-09-28
priority: P4
cost: M
---


Found in review of PR 3374 (`gather/assemble-single-local-battery`).

## The finding

At `Sym<T>`, a predicate's decision reads the thread-local session in
`crates/geom-core/src/sym.rs` (`SESSION`; `discharge` returns `None`
outside a session or at a zero-term budget). `SymBudget` and
`SymRules` decide which margins discharge as exact theorems and which
fall back to the numeric band, so one body's tier-3 battery can pass
under one session and escalate under another.

`topo::AtRestBody` (`crates/topo/src/validate.rs`) keeps a `Validated`
verdict with the body but not the session it was derived under.
`AtRestBody::validate_pseudomanifold` then runs the census alone. A
caller that gathered a `Sym<T>` product in one session and assembled it
in another, or outside any session, would read the first session's
battery verdict where re-running it could refuse. `AtRestBody`'s doc
says so.

## Why it is not fixed

No caller does it: `editor_core::assemble` gathers and gates in one
call, and the driver's sessions (`crates/editor-core/src/drive.rs`)
wrap evaluation rather than a product/assembly pair. The session
exposes no identity a verdict could carry. Tying the verdict to it
would need a session id, or a check that the current session's
(budget, rules) are the minting one's. That is worth doing only once a
caller splits the pair across sessions.
