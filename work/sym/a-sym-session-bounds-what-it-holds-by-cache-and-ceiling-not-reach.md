---
id: a-sym-session-bounds-what-it-holds-by-cache-and-ceiling-not-reach
kind: issue
title: A symbolic session's per-node memos are read as a record and its size is bounded by a reach rule; they should be caches whose size never decides a discharge, under a typed fail-loud resource ceiling
status: open
opened: 2026-10-17
priority: P2
cost: M
refs: [the-sym-profile-lowering-exhausts-memory-under-the-revalue-instrument]
---


Filed by INLANE, by ruling of the INLANE orchestrator on leak 2 of `work/inlane/the-sym-profile-lowering-exhausts-memory-under-the-revalue-instrument.md` (fork-log row 155: two designers, converged). That leak was the session's per-node form tables with no total bound. File only; nothing here is built. `crates/geom-core/src/sym*` is sym's territory.

## What was measured

Guided `plate_param`, every variable a symbol, in an eager session at 4,096 terms, reached 4.9 GB and was cancelled at 120 s. At 150 s, `forms` held 2.45M terms and `forms_early` 3.96M, across about 30k forms. Split by origin:
- decision walks: 59,122 forms;
- the contradiction check (`Origin::Assertion`): 13,729, about 19%.

The same document in the coincidence door's deferring session at 256 terms takes 1.07 s and 136 MB, with no walks. Under the pinned lift, on the nine revalue documents, the tier on and the tier off (`SymBudget::none()`) record identical decisions, predicate and sign.

## What is owed: the tier's own session bound

1. **`SymRetry::max_forms` should go** (`crates/geom-core/src/sym.rs`, its doc at `SymRetry::max_forms`). It is a reach rule that depends on memory: whether a retry attempt is walked depends on how many forms the session already holds.
2. **The per-node memos are caches, and their size must never change a discharge.** Two readers today treat a memo as a record, and that blocks eviction:
   - `count_check` (`sym.rs`) reads `sess.forms` / `sess.forms_early` to charge a contradiction check: "a form `discharge` did not store is a form the check did not see";
   - the walk's `frozen` count.
   Those readers need their own record, so that a memo can drop forms.
3. **A typed, fail-loud resource ceiling for a session, never a freezing one.** A session that outgrows its resources refuses, typed. It does not quietly decide numerically what it would have proved with more memory.
4. **Cancel shared monomial factors in the quotient form, and not only in the early walk's rule E** (`quotient::cancel_shared`, applied today where `sess.rules.common_factor` and the walk is early). That is the lever for a consumer that wants every variable a symbol and every decision discharged. A guided profile's forms are powers and products of the atoms it mints (`plate_param`: 1,683 atoms; `Powi` the top producer).

**Ev's channel.** One designer would add a sentence to ERROR-DESIGN E12: "the memos are caches; their size never decides a discharge; a session that outgrows its resources refuses, typed". E12 carries Ev's lgtm (#1712), so that sentence goes to Ev as an `[ev]` PR when this item is built, not before.
