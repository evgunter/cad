---
id: two-face-sphere-split-measures-zero-volume
kind: issue
title: "props: a closed sphere split into two faces by the same two meridian arcs measures volume 0.0 — one parse hands both faces the same levels"
status: open
opened: 2026-09-02
github: 1598
refs: [723, 1571, 1565]
---

## From GitHub issue 1598

opened 2026-09-02, 1 comment.

**Found by MESH-7's R1 review (the pole-crossing probe, issue 1571's props-side finding), mechanism measured by MESH-11 — filed as the durable home. Pre-existing.**

Build a unit sphere as TWO faces bounded by the SAME two edges (a rim at latitude asin 0.5 and one great-circle meridian arc), each face traversing the shared edges in opposite directions — the "half-cap" and its L-shaped complement in the chart. Both faces pass `require_iso_rectangle`; `mass_properties` returns `Ok { volume: 0.0 }` on a closed sphere. The three-face split of the same sphere measures 4π/3 exactly.

**Mechanism** (MESH-11's reading): the per-kind parse hands both faces the same levels (the rim's level and the meridian's span-derived extent), `linear_rim_side` then gives opposite `s_f` for the two traversal directions, and the closed-form flux contributions are equal and opposite. Tier 3 catches the body only through check 6 (`CurvedSenseInverted`), not through the closed form.

**Why MESH-11 does not close it:** its one-chart-branch predicate is cited by `mesh` only; citing it from `curved_face` would refuse the pole-crossing arcs that CERT-1's three exact rows (`a_pole_crossing_meridian_arc_measures_the_half_cap_exactly`, `the_rimless_hemisphere_split_off_its_poles_still_measures`, `a_multi_wrap_span_covers_both_poles`) admit on purpose. The defect is in how the parse's extent is assigned to a face whose meridian arc crosses a pole and whose complement shares its edges — the closed form's second premise (issue 723's class) on the L-shaped face, not the half-cap.

**Owed:** a row per direction on the two-face body (refuse typed or measure the true partition), with the CERT-1 rows unmoved; Track R / S-CERT ground.

Refs #723, #1571, MESH-7 (#1565), MESH-11.

## Comments

**2026-09-02** — orchestrator:

(S-MESH orchestrator) (S-CERT orchestrator) — handing this to S-CERT's slate. It is the closed form's extent premise on the sphere (issue 723's class, which CERT-1 owned): the parse hands the half-cap and its L-shaped complement the same levels, so their flux contributions cancel on a closed sphere. S-MESH's MESH-11 measured the mechanism and pinned the defect in two rows (`mass_properties_still_answers_zero_on_the_half_cap` and the re-aimed `mesh7r1_probes` row) without closing it, because the fix cannot be the branch door (citing it from `curved_face` would retract CERT-1's exact pole rows). MESH-12 takes the neighbouring #1601 (the saturated span) and will not touch this one. Ev deferred scheduling to the orchestrators (in chat, 2026-09-02).

## Home

`work/cert/` — `crates/geom-brep/src/props/*` is an S-CERT territory glob, and the S-MESH/S-CERT orchestrator comment explicitly hands it to S-CERT's slate.
