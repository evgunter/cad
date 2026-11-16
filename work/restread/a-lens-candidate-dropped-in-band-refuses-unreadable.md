---
id: a-lens-candidate-dropped-in-band-refuses-unreadable
kind: issue
title: Self-touch refusals that name the wrong cause, in-band readings as Unreadable and a limit as InBand, from pair's dropped candidate on
status: open
opened: 2026-11-12
priority: P3
cost: E
---


Found by the remainder-refuses lane (PR 6854), sweeping for band
readings that end in `Unreadable`. Priority P3: only the refusal's
stated cause, and so its recourse, is wrong. The point refuses either
way.

## What

`second_order::pair` (`crates/topo/src/link/second_order.rs:622`) takes
`[i, j]` as a candidate only where `j`'s rays read on `i`'s by
`super::on_ray` (`:643`). A ray in band of the other's reads "not on
it", so the pair is dropped as no candidate (`None`), rather than
refused in band.

The two sheets then stay on each other. `one_material` notes them
coincident (`crates/topo/src/link.rs:5013`) and refuses
`Curved(Unreadable)` (`:5023`), telling the user the shape is not read
yet when the true recourse is the tolerance.

## The fix

Make `pair`'s candidacy three-valued. Where `on_ray` or `wedge_side`
reads in band, refuse `TouchUnread::InBand`, as `kind` now does for its
own signs. Pin it with a lens whose second ray is turned off the first
by a hair between the band's zero and its escalation.

## The same class elsewhere (PR 6854's fix pass)

PR 6854's sweep matched a `Curved(Unreadable)` reached from a band
reading. It could not match a band reading that is passed over and
refused later for another reason, nor a limit mapped to `InBand`. Its
review found these instances:

- **`second_order::parts`** (`crates/topo/src/link/second_order.rs:370`,
  the `Place::Bound` arm). A bound whose side or ahead reading is in
  band is `continue`d. `in_band` is consulted only where no sheet of
  that sector was pushed. Where another bound of the sector does push
  one, the in-band bound drops out of the picture, and a later refusal
  names `Unreadable` (or the sheets read without it).
- **`second_order::thorns`' `next`** (`second_order.rs:122`). With two
  candidates decided ahead and a third in band, it reports `InBand`,
  though the ambiguity is structural: two decided candidates already
  make the ring not one cycle.
- **`material_round(..).ok_or(InBand)`**:
  - `second_order.rs:187` (`thorns`);
  - `crates/topo/src/link.rs:4823` (`rest::at_apex`, via `None =>
    InBand`);
  - `link.rs:5318` (`rest::lensed`).

  `Cone::reads_over` drops its `Unread` (`link.rs:1048`), so
  `Unread::NoReference`, a limit (`Refusal::Limit`, `link.rs:1067`),
  reads as `InBand`. The fix: return the `Unread` from `material_round`
  and map it through `Unread::refusal`, or through a `TouchUnread`
  conversion, at each site.

## One more instance (branch `restread/self-touch-valid-bodies`)

`second_order::along_a_line` takes a sheet on a planar face's whole
plane as a candidate only where `rest::in_plane_of` reads every ray of
it on the plane and `wedge_side` reads the two the other way round. A
reading in band drops the candidate as `pair`'s does, and the sheets
then refuse `Curved(Unreadable)` as lying on each other.
