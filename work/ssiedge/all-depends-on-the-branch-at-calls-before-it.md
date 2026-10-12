---
id: all-depends-on-the-branch-at-calls-before-it
kind: issue
title: A section handle's all() depends on which branch_at calls came before it, so it is not a function of the operands alone
status: open
priority: P3
cost: M
opened: 2026-10-11
refs: [the-plane-nurbs-door-selects-its-branch-by-nearest-sample]
---

Both ℝ⁴ section handles (`geom_brep::ssi::Section` in `ssi/pair.rs`,
`geom_brep::ssi::PlaneSection` in `ssi/plane.rs`) keep the branches
`branch_at` certified, and `all()` banks them first and traces only the
crossings they do not end at (`ssi/handle.rs`, `Banked::open`). A branch
`branch_at` certified was marched both ways from its seed
(`Ends::through_seed`), while `all()` reaches the same branch from its
crossings, Hermite first (`Ends::branches`). So `all()`'s result
depends on the calls before it:

- the order of `branches`;
- each branch's orientation, and its carrier's bits;
- a closed loop's witness.

`all()` is therefore not a function of the operands alone. The
accounting still closes, so exhaustiveness holds; what varies is the
representation. Found in review of PR 4677, where the plane lane gained
the handle. The pair lane has had it since PR 4597.

Owed: either make `all()` a function of the operands (canonical order,
orientation and witness, or re-derive banked branches from their
crossings), or state the dependence as part of the contract
(`crates/geom-brep/README.md` C3) and confirm that no caller depends on
it.
