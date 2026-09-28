---
id: CONTACT-3
kind: unit
title: a curved wall's chart trim stops answering for a class it misstates: the ray lane decides a non-iso wall exactly or refuses, never from the vertex rectangle
status: closed
opened: 2026-09-26
priority: P0
cost: H
branch: contact/3-wall-trim
closed: 2026-09-28
---


Carries `cylinder-wall-trim-overcovers-a-tilted-section`. Spec: `docs/CONTACT-3-SPEC.md`.

Review tier: **dual**. It changes a certified walk that answers wrongly today, and it makes the exact-or-refuse class decision that later walls inherit.

## Closed

The ray lane reads a cylinder wall face against its outline
(`wall_outline`), resolved lazily on a hit in the face's window.
- **`Rectangle`** (rims and meridians on two levels): read as before.
- **`Chart`** (every edge a meridian, a rim or an on-axis planar
  section, with the window under a period): read by parity along the
  ruling. The side is the perpendicular distance in metres, and a
  junction is decided at the junction.
- **`Unsupported`**: everything else, refused typed and confined.

Across every reviewer shape a door can build, the head answers with 0
wrong. Examples, answered / wrong from base to head:
- slab at tilt 1.0: 594/230 → 386/0
- tilt 0.9, below: 527/212 → 305/0
- the V valley and ridge (two splits): 0 wrong at tilts 0.4 and 1.1

The face door shares the class predicate. `iso_bounded_wall` is gone.

Review:
- A **dual** on `0901f45` (DR row in `docs/DUAL-REVIEW-LOG.md`). Both
  reviewers returned APPROVE-WITH-FIXES with no MAJOR, and both found
  the refusal far coarser than the spec asked.
- The fix pass generalised the exact arm, where the spec had only
  asked to confine the refusal. A single delta review approved it.
- A last pass pinned per-shape answered floors and escalation caps.

Landed alone, not combined with CONTACT-4. ATREST-12 reached main
after CONTACT-4 branched, and the two decided the same four questions,
so CONTACT-4 is reconciling first (see the log).

Filed:
- `cone-chart-trim-reads-a-tilted-section-as-its-vertex-window`
- `sphere-chart-trim-folds-any-number-of-rim-levels`
- `point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity`
- `work/shell/shell-clearance-footprint-reads-vertices-not-arcs`
- `work/issues/subtract-v-notch-from-seam-aligned-cylinder-escalates-stale-halfedge`
