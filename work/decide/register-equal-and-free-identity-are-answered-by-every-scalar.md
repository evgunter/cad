---
id: register-equal-and-free-identity-are-answered-by-every-scalar
kind: issue
title: Real::register_equal and Decide::free_identity are methods every scalar answers, where only a symbolic scalar has a meaning for them: move them onto the expression
status: open
opened: 2026-11-23
---

Found by BOXFINISH's expression-question unit (PR 7488), which gave a
scalar's expression a type (`Real::Expression`) and made ERROR-DESIGN
E12's structural question a method of it
(`geom_core::Expression::proves_zero`), a type error at a numeric
scalar (`crates/geom-core/README.md` EX1).

Two older surfaces whose whole meaning is symbolic are still methods
every scalar answers:

- `Real::register_equal` (`crates/geom-core/src/real.rs`): at `Sym<T>`
  it records a constructor's theorem in the session's registry; at
  `f64` and `Interval` it checks the claim at the value and records
  nothing; the trait's default answers `Unwitnessed`.
- `Decide::free_identity` (`crates/geom-core/src/predicate.rs`): at
  `Sym<T>` it mints the identity's graph over a free atom; the default,
  which every numeric scalar takes, drops the identity and returns
  `self`.

Both were outside PR 7488's fence. What would close it: each becomes a
method of the expression (or takes a bound only the symbolic scalar
meets), so a numeric scalar has no impl to answer with, and the
numeric witness check `register_equal` does today is named as what it
is.
