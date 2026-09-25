---
id: sign-hull-spec-names-the-deleted-interval-feature
kind: issue
title: PROPS-SIGN-HULL-SPEC instructs --features interval and a CI-Config trailer; the first now errors and the second never did anything
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [interval-orthonormal-basis-sign-hull, ring-4-interval-feature-dropped]
---

## What

`docs/PROPS-SIGN-HULL-SPEC.md` — the live spec for
`interval-orthonormal-basis-sign-hull` — tells its implementer to run the
`m10_5_*_interval` and `onb_*_interval` rows with `--features interval` and
to put `CI-Config: lane=interval eps=default` on the commit (its
verification section, ≈`:204-205`).

RING-4 deleted the `interval` feature, so the first instruction is now a
cargo error (`the package ... does not contain this feature: interval`),
and the rows it names run in every ordinary build. The second has been
inert since 2026-09-04 (`docs/prompts/implementer-discipline.md`: nothing
in CI reads a commit trailer) and names a lane axis RING-4 also removed.
The spec is LINALG's, so RING-4 left it; the re-word is two lines — drop
the flag, drop the trailer.
