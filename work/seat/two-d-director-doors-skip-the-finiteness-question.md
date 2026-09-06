---
id: two-d-director-doors-skip-the-finiteness-question
kind: issue
title: four direction doors decide a length they never asked to be finite — the SEAT-DV overflow class, swept (profile ×2, sweep, geom-core, topo)
status: open
opened: 2026-09-05
refs: [1564, 1738, is-finite-length-homed-in-the-query-seat]
---

## The class

A direction door that classifies its length's SIGN without first
asking whether that length is a FINITE NUMBER admits a `1e200`
component out of a DECIDED path: the norm overflows to `+∞`, an
infinite margin reads maximally definite POSITIVE to `sign_within`,
and the division that follows collapses the direction to zero. The
door then reports success. SEAT-DV's review (PR #1564) found this at
the datum constructor and fixed it there; PR #1738 fixed the same
order-of-questions at `editor-core`'s `unit()`; SEAT-DN collapsed
those two into one body, `topo::query::decide_unit_direction`, which
asks finiteness first through `topo::query::is_finite_length`.

**Every remaining instance is below.** They were measured while
executing SEAT-DN's DN-3, which sent that unit to
`profile::path::Dir::from_unit` — the constructor turned out not to be
the hole (every ray reaching it is already decided or an exact
negation, stated and pinned there); its two producing DOORS are, and
so are three more the sweep found.

| # | door | margin | measured |
|---|---|---|---|
| 1 | `geom-core` `linalg::frame::definitely_positive` (`frame.rs:205`), normalizing at `:287`, `:291`, `:330`, `:443` | `Margin::of(len)` | `mirror_across_plane(origin, (1e200,0,0), Tol::witness())` → `Ok(IDENTITY)` — a mirror that mirrors nothing, definite and silent |
| 2 | `sweep` `revolve::axis::AxisFrame::build` (`revolve/axis.rs:55`) | `Margin::norm2(axis.dir)` | `decide("revolve_axis_direction", Margin::norm2((1e200,0)))` = `Ok(Positive)`, `normalize()` → `(0,0)`; the frame builds with `dir_sk = (0,0)` |
| 3 | `topo` `sector_shape` (`sector_shape.rs:223`) | `Margin::of(min(arm))` | same arithmetic: both arms normalize to the zero vector after a definite-positive decision |
| 4 | `profile` `unit_from_components` (`path.rs:2818`) | `path_director_norm`, hand norm | `unit_from_components(1e200, 0.0, Tol::witness())` → `Ok(Dir { unit: (0,0), ang: 0 })` |
| 5 | `profile` `arc_fillet::carrier_tangent` (`arc_fillet.rs:901`) | `path_arc_center_radius` | same shape at `|P−O| ≳ 1e154` (not executed) |

Rows 1, 2 and 4 were executed; row 3's arithmetic is row 2's with a
3-D norm; row 5's is row 2's.

**Two of these are reachable from a public door with no
pre-validation**: `pncad-py`'s `Frame.mirror_across_plane`
(`crates/pncad-py/src/py/place.rs:162`) and `RevolveAxis` through
`pncad::prelude` (`prelude.rs:202`). The recipe road into the revolve
is guarded, but upstream and for another reason —
`editor-core`'s `wire.rs:937-948` validates `lift(plane_dir)` through
the datum door first, so the sweep's own door is never reached with a
non-finite direction from that road. That is #1738's shape exactly:
the right outcome, from a guard that does not know it is the guard.

## What separates them, and why SEAT-DN fixed none of them

The rule has ONE spelling — `topo::query::is_finite_length` — and
reachability decides the cost:

- **`sweep` (row 2) can call it today.** `sweep` depends on `topo`.
  One line before the decide, plus a `RevolveError` arm for the new
  refusal (and its census/K consequence, which is that door's owner's
  to place).
- **`topo` (row 3) is already in the crate that holds it.**
- **`geom-core` (row 1) is BELOW it**, and is the crate FIX's
  `work/fix/is-finite-length-homed-in-the-query-seat` proposes as the
  predicate's natural home (`Real`, `is_poison` and
  `Vec3::normalize`'s own overflow note are there).
- **`profile` (rows 4, 5) cannot reach it at all**: `profile` depends
  on `geom-core` alone. Closing those two means either the move above
  or a second spelling of the rule, which is what this family's ruling
  is against. Both also need a new public `PathError` arm, its
  sentence, its `PathErrorKind` row, and whatever the python tag
  census pins about that surface.

So `is-finite-length-homed-in-the-query-seat` is not a neighbouring
question — **it is the ruling that unlocks four of these five rows at
once**. Answering it "`geom-core`" makes every door above able to ask
in the one spelling; answering it "stays in the query seat" leaves
rows 1, 4 and 5 needing a second spelling or a layering change, and
that answer should be given knowing there is live silently-wrong
behaviour behind it.

SEAT-DN touched none of them: each door belongs to another program's
territory, and the unit's spec was a faithful execution of a ruling
about the direction FAMILY's home, not a licence to edit five crates.

## What this asks

One unit, after the homing question is answered: put the finiteness
question in front of every decide-then-normalize direction door in the
workspace, in one spelling, with a typed refusal per door and the
K/census consequence stated per site. Red-first per row — each of the
five reproductions above is a test.
