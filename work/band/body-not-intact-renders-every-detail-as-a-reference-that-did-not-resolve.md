---
id: body-not-intact-renders-every-detail-as-a-reference-that-did-not-resolve
kind: issue
title: blend: BodyNotIntact's rendering says '<at> did not resolve' for every Row-1 detail, including a cycle or verdict disagreement where the key resolved
status: closed
opened: 2026-09-25
priority: P2
cost: E
pr: 5021
branch: band/body-not-intact-rendering
closed: 2026-10-13
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

## Closed

`BlendError::BodyNotIntact` carries a `breach: Breach` beside `at` and
`detail` (`crates/sweep/src/blend/mod.rs`), and its `Display` renders
the cause the breach names: `Unresolved` — "{at} did not resolve";
`Unwalked` — "the walk from {at} did not close"; `Disagrees` — "{at}
resolved, but its structure disagrees with the blend's reading". The
one constructor `not_intact` became three (`unresolved`, `unwalked`,
`disagrees`, `crates/sweep/src/blend/surgery.rs`), and every raise
site was classified by what its `None` or check actually meant: 129
unresolved, 27 unwalked, 48 disagrees; nine sites that folded two
causes into one `Option` (a walk then a match, a read then a shape
check, `flank`, `trim_chords`) were split so each cause refuses as
itself.

Rows: `body_not_intact_renders_each_breach_as_its_own_cause` pins the
three texts exactly; `each_breach_refuses_and_renders_its_own_cause`
reaches one real refusal per kind at crate-internal sites (a stale
support at `chain_shell`, a lone-vertex cycle at `chord_site`, a link
across two shells); the Interval sheared-box row
(`band_turn_overrun_fillet`) is the one public-door `Disagrees`, now
asserting its breach and that it no longer says "did not resolve".
`Unresolved` and `Unwalked` are not reachable through a public door:
every door hands the blend a tier-1/tier-2-valid body.
