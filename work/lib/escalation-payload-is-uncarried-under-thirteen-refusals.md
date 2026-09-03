---
id: escalation-payload-is-uncarried-under-thirteen-refusals
kind: issue
title: Indeterminate is the most-carried uncurated payload in the tree, and MarginDiag under it
status: open
opened: 2026-09-03
refs: [LIB-CUR4]
---


Found by LIB-CUR4's struct-payload sweep — the one-rung extension that
closes CUR3's own named blind spot (a): "payload types that are STRUCTS
rather than enums — the scan indexes `pub enum` only". `Indeterminate`
is exactly that shape, and it is the largest instance in the tree.

`geom_core::predicate::Indeterminate` (`crates/geom-core/src/predicate.rs`)
is the two-tolerance escalation payload (D4 ¶1 addendum). It is carried
by **thirteen** curated refusals and is on no curated list:

`BlendError`, `BooleanError`, `ContactRefusal`, `ExtrudeError`,
`LoftError`, `MateFault`, `PathError`, `ProfileError`, `RevolveError`,
`SelectRefusal`, `TubeError`, `UnitVec3Error`, `ValidationError`.

By comparison the payload CUR3 carried had ONE carrier and the four
LIB-CUR4 carried had one apiece. `Indeterminate` is reachable at
`pncad::geom_core::Indeterminate` (contract clause 1 is met — and
`crates/pncad/tests/all.rs` already names it twice, at `contain_payload`
and `ellipse_payload`, by that module path), so this is the curated-list
half only: a prelude consumer holding a prelude-carried `Escalated` arm
cannot name what it holds without a module hop.

One rung under it, and the reason this is filed as its own item rather
than swept into LIB-CUR4: `Indeterminate`'s own public field type
`geom_core::predicate::MarginDiag` (an enum, `predicate.rs:612`) is
uncurated too, so the escalation payload is the head of a two-deep
carriage question rather than a single name. LIB-CUR4 stopped at one
rung by its brief and banked this.

**What a unit closing it would have to decide.** Whether the escalation
channel is curated surface at all. The argument for is that thirteen
prelude refusals carry it, which is corpus-wide by any measure; the
argument against is that an escalation is a "the kernel could not
decide" report whose recourse is identical everywhere (tighten ε, or
accept the ambiguity), so the discriminant may be constant at the
curated boundary the way LIB-CUR4 measured `BandField`'s to be. That
measurement — is `MarginDiag`'s discriminant something a caller
branches on, or prose? — is the unit's first job, not an assumption.
