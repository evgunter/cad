---
id: certified-line-carrier-is-read-in-five-places
kind: issue
title: "is this edge a certified Line" is spelled separately in five splitting/join files; one helper
status: closed
opened: 2026-10-06
priority: P1
cost: E
branch: onehome/geom-readings
closed: 2026-11-02
pr: 6328
---



## What

"Is this edge's certified curve a straight `Line`" is answered inline in
each splitting file that needs it, each with its own reach into
`CurveGeom::Certified(..).carrier()`:

- `splitting/rules.rs` `straight` (the knife edge's contact site, PR 4098);
- `splitting/join.rs` `certify_section_area`'s per-half-edge `straight`
  read (the spur invariant);
- `splitting/containment.rs` (the `Line` arms that read a loop edge as a
  chord);
- `splitting/section.rs` (`HalfCarrier::Chord`);
- `splitting/classify.rs` (the crossing lane's `Line` arm).

One helper on the certified curve (or on `Body` by edge key) would answer
it once, so a carrier that later reads as straight (a degenerate conic, a
fitted line) is admitted or refused in one place.

## Found by

CLEAVE DR-4098's delta review (Style 5).


## Duplication triage (INTENT, 2026-10-26)

Classified on Ev's ask, via the CLEAVE note on INTENT's log, against main `e92fb951` and DESIGN `[one-way.places]` / `[one-way.readings]`. The four classes:
- **design signal:** DESIGN names no single home, so a design conversation is owed;
- **mechanical:** DESIGN names the home; delete the off-home copy and repoint its users;
- **D10 debris:** a D10 stage retires one copy;
- **not duplication.**

**Mechanical** (cluster D). It is now about 20 `matches!(…, Curve3::Line{..})` sites across topo, geom-brep, sweep and step-import. No copy disagrees. Fix: one predicate on `Curve3`/`EdgeCurve`. DESIGN home: `[one-way.readings]`. **Cluster D, carrier readings re-spelled outside their named home.** DESIGN names the home. Ev (2026-10-26): the first fix is making the canonical reading the path of least resistance (the API consumers need), with a gate only as a backstop. 

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P4 → P1: one question answered inline in five files: two implementations of one underlying logic, P1 by `work/README.md` Priority.
- Moved to ONEHOME (`work/onehome/`) by `git mv`, id and body unchanged: the P1 track opened for CLEAVE's duplication rows; INTENT is sorting each into design signal or a mechanical deletion (`work/intent/log.md`, 2026-10-26), and the sort goes on the row. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).

## Closed (ONEHOME, 2026-11-02, PR 6328)

`geom::Line<T> { origin, dir }` with `param`/`foot` is the one line value, and `Curve3::Line(Line<T>)` holds it, so `Curve3::as_line()` returns the held value and `is_curved()` is `as_line().is_none()`. About 45 straightness sites and the axis-foot copies are repointed (including editor-core's `mate/corner` and `emit_topo`, and `AxisLine::foot_of`). The copies disagreed on non-unit `dir`; the dividing form is kept and pinned. STYLE review FIX-THEN-MERGE: the first version's `geom::Line` repeated `Curve3::Line`'s fields, fixed by making it the variant's payload. Delta FIX-THEN-MERGE: the slow-set digests were re-blessed after a spelling-only check. Follow-ups: `local-point-direction-line-types-restate-geom-line`, `a-points-radial-offset-from-an-axis-is-spelled-inline`, `a-curves-closed-form-foot-is-re-spelled-outside-geom`, `carrier-speed-is-re-spelled-with-a-default-arm-in-import`, `a-points-foot-on-a-plane-is-spelled-inline`, and the parked `analytic-surfaces-spell-their-frame-field-by-field`.
