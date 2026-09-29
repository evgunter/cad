---
id: body-not-intact-renders-every-detail-as-a-reference-that-did-not-resolve
kind: issue
title: blend: BodyNotIntact's rendering says '<at> did not resolve' for every Row-1 detail, including a cycle or verdict disagreement where the key resolved
status: open
opened: 2026-09-25
priority: P2
cost: E
---


## Finding

`BlendError::BodyNotIntact`'s `Display` (`crates/sweep/src/blend/mod.rs`,
the `Self::BodyNotIntact { at, detail }` arm) renders every detail as

    {detail} — {at} did not resolve, so the body is not intact there.

but its one constructor, `not_intact` (`crates/sweep/src/blend/surgery.rs`),
documents Row 1 as three causes: a stored reference that did not
resolve, a cycle that did not close, **or a verdict whose keys disagree
with the body's structure**. Most call sites are the second and third
(`ruled.rs`'s "a cap's cycle around the old vertex is not flanked by the
two feet just split", "a ruled link's end is not among the transverse
caps the verdict classified", `chord_site`'s cycle read), and for them
the rendered sentence asserts a false fact: the named key resolved.

Measured on the D-shaped through-hole before
`band/ruled-d-hole-ring-crease`: the refusal read "a face's outer cycle
does not walk, or does not carry the half-edge the carve keys on — face
FaceKey(2v1) did not resolve", for a face that resolved and whose cycle
walked.

## What the taker owes

Render the detail and the site without asserting a cause the detail
does not state (e.g. "— at {at}: the body does not hold together
there"), and re-baseline any row that asserts the old tail.
