---
id: two-d-director-doors-skip-the-finiteness-question
kind: issue
title: profile's 2-D director doors decide a length they never asked to be finite (the SEAT-DV overflow class, in 2-D)
status: open
opened: 2026-09-05
---


## What was measured (SEAT-DN, DN-3)

DN-3 sent the unit to `profile::path::Dir<T>::from_unit` to give it
issue-1527's treatment. Measuring its callers first — as the spec
required — found that constructor is NOT where the hole is: every ray
reaching it was already decided at the door that built it, and the
finding is one level up, at those doors.

**The three producing callers** (`crates/profile/src/path.rs:1774`):

| caller | what it holds | decided under |
|---|---|---|
| `unit_from_components` (`path.rs:2818`) | authored components | `path_director_norm` |
| `arc_fillet::carrier_tangent` (`path/arc_fillet.rs:901`) | τ·(P−O)⟂/R | `path_arc_center_radius` |
| `Dir::reversed` (`path.rs:1791`) | the negation of a stored unit ray | unit by construction |

No caller holds an ANGLE (deriving the ray from one is what the VQ4
exactness contract exists to prevent), and no caller holds an
undecided ray. So neither branch of DN-3 applied: nothing to re-spell
through the angle door, and no new K-REPORT carrier row owed. That is
stated at `from_unit` and pinned by
`path::tests::every_ray_a_director_is_built_from_was_decided_at_its_own_door`.

## The hole those two doors DO have

Both classify their length's SIGN and neither asks first whether that
length is a FINITE NUMBER — the exact order SEAT-DV's review found
wrong in 3-D and the finiteness gate fixed
(`topo::query::unit_direction`, `is_finite_length`).

Measured on this unit's branch, at `f64`, through the public door:

```
unit_from_components(1e200, 0.0, Tol::witness())
  -> Ok(Dir { unit: (0, 0), ang: 0 })
```

Components of `1e200` are finite VALUES, so the expression layer
passes them; `(dx² + dy²).sqrt()` overflows to `+∞`; an infinite
margin reads MAXIMALLY DEFINITE positive to `sign_within`, so the
door decides "this names a direction"; and `dx / ∞` then collapses
the ray to `(0, 0)` with angle `0`. A director that names no
direction is admitted out of a decided path, and every leg built on
it steps by nothing.

`PartialPath::toward(dx, dy, tol)` reaches this door
(`path/program.rs:663`, `path/verbs.rs:372`), so the reach is the
public authoring API, not an internal corner.
`carrier_tangent`'s `radius = v.norm_squared().sqrt()` has the same
shape at `|P−O| ≳ 1e154` (not executed here).

## Why SEAT-DN did not fix it

The rule has ONE spelling in the workspace — `topo::query::is_finite_length`
— and `profile` cannot call it: `crates/profile/Cargo.toml` names
`geom-core` and nothing else, while the predicate lives in `topo`,
which sits above. Closing the hole means one of

1. moving `is_finite_length` down to `geom-core` (where `Real` and
   `is_poison` live) and re-exporting it from `topo::query`, so the
   one spelling stays one; or
2. a second spelling in `profile`, which is what this family's ruling
   is against.

and then a refusal for the new arm: `PathError` has no "not a finite
length" variant, so (1) also adds a public error arm, its sentence,
its `PathErrorKind` row and whatever the python tag census pins about
that surface. That is a unit, not a rider on a ruling SEAT-DN was
told to execute faithfully, and its funnel names are `profile`'s to
place.

## Refs

- Executed by SEAT-DN (`work/seat/SEAT-DN.md`); the 3-D half is
  `topo::query::unit_direction`.
- The 3-D precedent and the class: SEAT-DV (PR #1564), whose review
  found the same order-of-questions defect at the datum door.
- **Interlocks with `work/fix/is-finite-length-homed-in-the-query-seat`**
  (FIX's slate, open): that item asks SEAT whether the predicate
  should live in `geom-core` beside `Real`/`is_poison` rather than in
  the query seat. Answering it "yes" is option (1) above and is what
  makes this hole closable without a second spelling, so the two want
  to be decided together. That item's closing line still cites issue
  1570 as the open family question; the ruling closed it, and only
  that item's owner can update it.
