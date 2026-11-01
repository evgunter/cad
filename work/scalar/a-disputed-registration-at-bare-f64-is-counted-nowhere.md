---
id: a-disputed-registration-at-bare-f64-is-counted-nowhere
kind: issue
title: scalar: SymRegistration::handle says a Disputed answer is counted in the session's receipt, but at bare f64 there is no session, so an inexact witness's refusal of a registered identity is dropped uncounted
status: open
opened: 2026-10-20
---


Found by designer B on PR #5449 (the path door's arc read-back),
outside that PR's question.

**The gap.** `SymRegistration::handle` (`crates/geom-core/src/real.rs`)
documents its `Disputed` arm as "an inexact witness's refusal, counted
in the session's receipt and never asserted on", and its body drops
the arm. The count lives in `Sym::register_equal`
(`crates/geom-core/src/sym.rs`, `count_registration_refused`), so it
exists only when the lane is `Sym<f64>`. At bare `f64`,
`<f64 as Real>::register_equal` answers `Disputed` (a relative-ε
comparison, `tol.eps() * max(|a|, |b|, 1)`) and nothing counts it: a
registrant whose identity the point value disputes leaves no trace in
any receipt or K stream.

**Why it matters less after #5449, and still matters.** The path door
now reads every constructed arc's landing, and a `Via` arc's
through-point, at a point scalar and refuses a miss, so a profile arc
whose registered endpoint facts are disputed at `f64` is refused by
that read rather than lost. Every other registrant (`sweep`'s
rigidity and rim identities, `Arc2::register_tangent_at` through the
line×line fillet) has no such read, and its disputes at bare `f64` are
invisible.

**Shape.** Either `handle` counts `Disputed` itself (a thread-local
counter the receipt and the probe lane can read at every scalar), or
its doc says the count is the symbolic tier's and that bare `f64`
drops it on purpose.
