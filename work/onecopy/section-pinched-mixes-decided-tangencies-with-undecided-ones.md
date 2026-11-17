---
id: section-pinched-mixes-decided-tangencies-with-undecided-ones
kind: issue
title: SectionError::Pinched carries tangencies decided Zero beside SsiError tangencies whose undecided margin it drops, so it aborts as mixed
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found splitting `geom_brep::SectionError` by standing (`onecopy/section-error-standing`). `SectionError::refusal` stands `Pinched` (`crates/geom-brep/src/intersect.rs:740`) as an abort, because it mixes classes.

## What

`Pinched { predicate }` is raised for two kinds of reading:

- **Tangencies decided `Zero`, which are in band.** These are the torus arms' pinch, which is decided zero only (`pair_section/torus.rs:472`; its undecided margin already goes to `Escalated`), the cylinders' bisector (`pair_section/cylinders.rs:388`), and the spline section's decided critical values and nodes (`pair_section/spline_implicit.rs:1240`, `:1276`, `:1317`, `:1337`, `:1421`).
- **Tangencies whose undecided margin is dropped.** `whole_reading_refused` (`pair_section.rs:1299`) maps every `SsiError::tangency()` (`ssi.rs:1357`) to `Pinched`. That includes `BoundaryGraze` and `BoundaryTangent` with `BandVerdict::Undecided(cause)`, and `TransversalityBand`. Also `spline_implicit.rs:1041`, which re-reads an `Unaccounted` as a pinch. The torus arms' `Pinch::named` readings (`torus_arms.rs:391`, `:658`, `:717`, `:745`, `:762`, `:969`, `:1081`, `:1162`) take an undecided `sign_as` (`torus_arms.rs:65`, `.ok()`) as a tangency the same way.

A dropped margin may be poisoned, and a ladder must never read past a possible fault, so the variant aborts as a whole.

## What closing it takes

Route each undecided tangency to `SectionError::Escalated(cause)` where it carries its cause: `BandVerdict::Undecided`, and a `Pinch::named` read from an `Err`. Then `Pinched` is decided `Zero` only, and it stands `InBand`. The spline rows that pin `Pinched { predicate }` move to `Escalated` where their tangency was undecided. Measure which ladders reach it first. The ladder today reaches `SectionError` only through the plane-cone apex section, which never raises `Pinched`.
