---
id: param-span-lever-says-vanishingly-short-of-a-reversed-span
kind: issue
title: geom-brep: CertCheck::ParamSpan's lever says "not vanishingly short" of a span reversed by a definite length
status: open
opened: 2026-10-14
priority: P3
cost: E
---

(Filed by the ENCL implementer of `param-span-reversed-at-a-build-tells-two-stories`, from the PR 5023 style review, Q4.)

## What

`CertCheck::ParamSpan`'s lever (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`) is "move the geometry so this edge is not vanishingly short". That fits a span at or near zero. It does not fit a span reversed by a definite length, such as `Refused::Negative(-1e-3)` in `certify::tests::each_split_verdict_ends_as_its_own_arm`: that edge is not short, it runs backwards.

Since PR 5023, the sign-certain arm at a build ends in this lever plus the decision's note, "an edge of no length, or a reversed one, is one no kernel construction mints, so this is a kernel defect worth reporting". So the lever now sits beside a note that names a reversed edge, and tells the user to lengthen it.

## Question

Should the lever cover both readings the refusal carries, for example "so this edge runs forward over a length that is not vanishingly short"? Or does a reversed span, a kernel defect at a build, want no geometric lever at all? Keep every render within the editor-core `refusal_concision*` budgets.

