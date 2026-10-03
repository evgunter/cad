---
id: dev1-cylinder-sphere-circle-locus-arm
kind: issue
title: The circle loci in general form: TangentLocus as a curve carrier with tangent_at, the certified side as data, wedge routing run on the locus, banked with the kissing arm
status: parked
opened: 2026-08-23
github: 974
refs: [971, 967]
priority: P1
cost: H
blocked_on: [declared-cusps-second-order-wedge-arm]
---

## From GitHub issue 974

Opened 2026-08-23; 0 comments.

(m9-3 lane)

The M9-3 follow-up STOP-reported in PR #971: the cylinder×sphere G1 cap rim (a sphere-capped tube — the other faithful in-scope analog of the lily tube chain) needs the DEV-1 closed-form witness lane widened by a coaxial cylinder×sphere CIRCLE arm:

- `TangentLocus` gains a `Circle { center, axis, radius }` variant (rest.rs); `tangent_locus` gains the coaxial cyl×sphere arm (decisions: center-to-axis perpendicular offset, radius gap — both metre margins under the existing `tangent_locus_*` row family; non-coaxial point tangency stays `Unsupported`, a point is not a locus the C3 record supports).
- Consumers: `verify_tangent_declaration`'s witness build (a `Curve3::Circle` carrier — the circle-carrier jet certificate lane already covers cyl×sphere, built for the M5 PR-12 fillet trimlines), `tangent_lump`/`record_germ_dir` (locus tangent at the site point: `axis × (p − center)`), and the m9_2_census_door pattern matches (a second variant makes `let TangentLocus::Line {..}` refutable).
- This revises the ratified DEV-1 set (M9-1 PR-2) and PR-A's door text (#967) — a design revision to discuss, not a lane's own call.

**BLOCKING PRECONDITION (M9-3 fix-pass adjudication, M1):** the reduce rung's covered endpoint posture consumes a *separation invariant* of the witness lane — every configuration `tangent_locus` mints a locus for has each carrier wholly in ONE closed residual half-space of the other, so an on-carrier edge under declared cover never crosses the partner surface (its residual is one-signed). The convexity of a line's residual bounds its MAXIMUM at an endpoint, not its minimum, so the `(Zero, Positive)` endpoint branch is sound only through this invariant (reduce.rs, the declared-cover rung; the contract sentence lives on `tangent_locus`). The coaxial cyl×sphere pair's residuals are one-signed in OPPOSITE orientations per direction (the cylinder lies outside the sphere, the sphere inside the cylinder), so the circle arm must restate its own residual-sign story — which covered incidences are admissible on which side — before it may land.

## Home

`crates/topo/src/boolean/rest.rs` and `carrier_eq.rs` are in S-MATE's `paths:` territory, and Rest reach is its charter.

## 2026-10-02 — re-scoped (TANG, after the DEV-1 designer pair)

The BLOCKING PRECONDITION above is stale: the coaxial cylinder×sphere
separation invariant is measured and satisfied
(`crates/topo/tests/verbs_cylsph_tangent_residuals.rs`). The row's
motivating fixture is in the wrong class. A sphere-capped tube's rim is
a wedge-π seam with aligned normals, not a `Tangent` contact (C1, C4),
so a circle witness gives it nothing; its path is
`a-declared-rest-mate-does-not-license-its-rim-against-the-partner-wall`
and `pi-seam-between-two-operands-has-no-declaration`. The two
designers agreed on this disposition:

- A circle locus serves the coaxial KISS only: a ball in an
  equal-radius bore (wedge 2π), or a rod ending in an equal-radius
  spherical hollow (wedge 0). Its consumer mints the cusp edge, which is
  unbuilt for the LINE locus too (`declared-cusps-second-order-wedge-arm`
  item 3), so it banks with that arm, as MATE-7 Q2 banks the
  torus×torus witness.
- When it builds, it builds in general form. `TangentLocus` wraps a
  `Curve3` (Line | Circle) with `tangent_at(p)`: coaxial surfaces of
  revolution touch where their meridian profiles touch, swept about the
  axis, and the line arms are the same construction swept by
  translation. The certified side each carrier lies on is DATA on the
  verified declaration (see
  `covered-circle-rung-reads-a-negative-endpoint-as-a-crossing`). The
  material-wedge routing runs first, on the locus, for every shape. The
  arms read the section classifiers' tangency variants
  (`tangent-locus-re-meters-the-section-classifiers-tangency`).
- The DEV-1 set's provenance is an orchestrator's acceptance of an
  implementer deviation (M9-1 PR-2, `docs/MODEL-AB-LOG.md`), so
  widening it later reverses nothing Ev ruled.
