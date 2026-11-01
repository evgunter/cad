---
id: sweep-suites-wrap-tol-witness-in-a-private-tol
kind: issue
title: sweep suites each define a private fn tol() returning Tol::witness() - inline Tol::witness() at the call sites
status: open
opened: 2026-09-29
priority: P4
cost: E
refs: [sweep-family-homes-fold-into-tests-common]
---

**The ruling this routes onto.** On PR 3385 (2026-09-29), Ev chose to
inline the per-suite `fn tol() -> Tol { Tol::witness() }` wrappers at
their call sites rather than home one helper: *"lean Tol::witness
rather than the helper but it's not a strong opinion"*. This follows
the f64 pass-through ruling ("(c) all the way", #3311/#3312): a
one-line wrapper around a public door is not a helper.

**The unit.** Every `fn tol() -> Tol` whose body is `Tol::witness()`
goes, and its calls read `Tol::witness()`. That includes the one in
`shell7_common.rs`, if that file still exists when this lands (see
`sweep-family-homes-fold-into-tests-common`). A `tol()` with any other
body is not a member. Name it in the PR and leave it.

**Census.** PR 3385 measured 66 copies in `crates/sweep/tests`.
`git grep -n "fn tol() -> Tol"` with no path argument (method item 3)
counted 81 definitions tree-wide on 2026-09-29. Re-take both at the
merge base, and separate the members by body.
