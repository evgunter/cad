---
id: restating-a-chart-image-across-an-authority-has-several-spellings
kind: issue
title: restating an edge as a chart image while carrying its declared authority across is spelled at three sites
status: open
opened: 2026-10-06
priority: P1
cost: E
---



## What

Three sites build "a derived chart image in chart K, keeping the edge's declared authority if it
has one" by hand. Each matches on `EdgeAuthority` and calls `EdgeDescriptionSpec::declared_by`:

- `splitting::finish::section_plane_restatements`: `EdgeDescriptionSpec::chart(chart)`, then
  `Declared(mc) => image.declared_by(mc)`, `Derived => image`.
- `splitting::finish::describe_section_boundary`'s smooth arm: a conventional
  `EdgeDescriptionSpec::chart(s_self)`, then `if let Some(Declared(mc)) = existing.authority()`
  re-declares it.
- `transform.rs`'s chart arm (`EdgeDescription::Chart`): the stated image is kept, and the
  authority travels through `map_mapped_curve`. That makes it a variant, not a copy. The others
  carry the authority verbatim.

`EdgeCurve::restated_description` (`geom-brep/src/certify.rs`) is the existing home for "this
edge's description as a construction states it". A sibling there could take the chart it is
re-imaged into, for example `EdgeCurve::reimaged_in(chart) -> EdgeDescriptionSpec`, and serve
the first two sites. `step-import`'s `adopt.rs` (`chart(chart).declared_by(mapped)`) declares a
fresh authority rather than carrying one, so it is outside this row.

## Owed

One home for the carry, or a reason the three spellings differ. Not measured: whether any two
sites disagree on an input.

## Found by

The review of PR 4158 (S5).


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical, mostly stale.** `EdgeCurve::restated_in_chart` now exists (#5426, #5484). One hand-roll is left: `finish.rs` `describe_section_boundary`. Its `!intrinsic` guard decides between a repoint and a new argument on the helper. DESIGN home: `[edges]`.

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P3 → P1: three hand-built restatements of one carry: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
