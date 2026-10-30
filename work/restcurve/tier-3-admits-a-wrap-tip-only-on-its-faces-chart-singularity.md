---
id: tier-3-admits-a-wrap-tip-only-on-its-faces-chart-singularity
kind: issue
title: Tier 2 admits a valence-1 vertex only at a wrap edge's tip, and tier 3 refuses a wrap tip off its face's chart singularity
status: parked
opened: 2026-10-29
priority: P1
cost: M
refs: [validity-refuses-an-interior-chart-singularity, import-mints-the-wrap-tip-at-a-chart-singularity, 6014]
blocked_on: [sphere-faces-take-a-per-face-chart-or-pole-vertices]
branch: restcurve/wrap-tip-tiers
---

Split out of `validity-refuses-an-interior-chart-singularity` by the RESTCURVE orchestrator (2026-10-29). `[ev]` PR 6014 (fork row 215) grew that row with the wrap tip, and DESIGN `[topology.tiers]` now states two rules for it:
- tier 2: "no valence-1 vertices but a wrap edge's tip";
- tier 3: "every wrap edge's tip lies on its face's chart singularity".

The wrap tip is the one edge the kernel adds to a chart singularity that no written edge reaches, walked out and back.

## Why this is its own row

Ev approved the wrap tip "if the complexity it adds is tolerable" and asked that the shared loop walk land first (`work/issues/loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk.md`, first in the wrap-tip sequence). Until then each loop reader (props' sphere and cone loops, which check 6 reads through, the mesh walk, and the boolean sectors) reads a tip on its own. FOREIGN's probe measured a hand-written reading at 22.81 against a true 23.86, and it passed tier 3.

Admitting the tip at tier 2 before that walk exists would let such a body through. The other half of the row, "every chart singularity in a face's closure is a vertex of it", needs no tip and lands now.

## What

- **Tier 2, check 2** (`validate.rs`, the `Strut` arm near `:5301`): a valence-1 vertex is exempt only when it is the tip of a wrap edge. That is read off the stored wrap flag (`EdgeDescriptionSpec::Chart { wrap, .. }`), so it is structural and takes no ε. Every other valence-1 vertex still refuses.
- **Tier 3**: a wrap edge's tip lies on its face's chart singularity. Read it as a certified reading of the pcurve's end on the chart's singular set, not as a point-to-pole distance at a band (FOREIGN's note on the parent row). A meridian that stops short of the pole refuses; `spur.step` is the witness, and today it passes tiers 2–3′ on the tier-2 exemption alone.
- Check 6 and the other tier-3 loop readers read the tip through the shared walk, never a reading of their own.

FOREIGN's `import-mints-the-wrap-tip-at-a-chart-singularity` waits on this row: import mints the tip only once the tiers admit it.

## Released (2026-10-30)

`loop-readers-each-derive-a-corners-turn-with-no-shared-loop-walk` closed with PR 6088. The shared structure is `Body::both_sides_face`, read through two views: `surface_cycles` (region) and `chart_image` (spur-free: integrators and chart readers). Read the tip through these, never by a turn of your own. (RESTCURVE orchestrator)

## Spec (RESTCURVE orchestrator, 2026-10-30)

Released by PR 6088. A spur, a wrap tip included, is dropped from the chart image before any integrator or chart reader sees it, so no reader reads a turn at the tip. Unit branch: `restcurve/wrap-tip-tiers`. Review: single FULL. This relaxes tier 2 for a structural class and adds one tier-3 refusal. No kernel door mints a tip yet: FOREIGN, CARVE, TESSWALK and SPLITDOOR hold those rows.

**Rides with it:** `wrap-tip-incidence-is-read-at-mint-and-again-at-tier-3`. The tip rule IS that incidence, and the designer pair on PR 6085 ruled it is read once, at the mint (`singular_at`).

1. **Tier 2, check 2** (`validate.rs`, the strut arm). A valence-1 vertex is legal only when it is the tip of a wrap edge, read off the stored flag (`EdgeDescriptionSpec::Chart { wrap: true, .. }`). That is structural, with no ε. Every other valence-1 vertex refuses as today. Define "the tip": the wrap edge's end that no other edge reaches. A wrap edge whose both ends have valence 1 is not a tip; say what it is.
2. **One incidence reading.** "This vertex lies on its chart's singular point" becomes ONE decision with one site name. `topo::pcurves::singular_at`'s analytic arm, `geom_brep::chart_singularity::locate`'s `AtVertex` placement of a singular point, and the new tip rule all cite it, at the caller's band. The spline arm (`net_reach`) stays `singular_at`'s unless it falls out naturally. If the mint's band and tier 3's band differ, say which band the one reading takes and why. The rule is that a reset joint at mint and an at-rest `AtVertex` cannot disagree on one vertex.
3. **Tier 3:** every wrap edge's tip lies on its face's chart singularity. If check 8 (`validate_pcurves`) already refuses a wrap edge whose far end is not a reset joint, then the tier-3 rule is check 8's finding: cite it, pin it, and add no second check. Otherwise add the refusal as a typed `ValidationError` that names the edge and the tip. Never read a point-to-pole distance of the tier's own.
4. **Rows (Euler-built; no door mints a tip yet):**
   - A sphere face holding a pole reached by one wrap edge walked out and back. It passes tiers 2 and 3 and measures right: volume and area against the closed form, through the chart image.
   - The same at a cone's apex.
   - A wrap edge whose tip stops short of the pole: the meridian spur, as `spur.step`. Tier 3 refuses it.
   - A valence-1 vertex on an edge that is not a wrap edge: tier 2 still refuses.
   - A mint and validate pair at an in-band vertex, showing that the one incidence reading answers both the same way.
5. **Out of scope:** the mesher (TESSWALK), the sectors (SPLITDOOR), revolve (CARVE) and import (FOREIGN). If a row's body reaches one of those, assert the refusal it makes today and name the owning row.

## Held (2026-10-30)

Re-parked on `sphere-faces-take-a-per-face-chart-or-pole-vertices` (a ruling open on `[ev]` PR 6085). Ev pointed out that a sphere face other than the whole sphere has a singularity-free chart, which may retire the pole vertex and the wrap tip for spheres. Do not build on the wrap tip until it is ruled. (RESTCURVE orchestrator)
