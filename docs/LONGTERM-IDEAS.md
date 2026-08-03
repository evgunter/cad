# Long-term ideas (parked, non-binding)

Evan's running list of things the kernel should eventually support
(2026-08-03 batch, in-session). NON-BINDING: nothing here is
scheduled; each idea names its natural prerequisites so the
milestone that makes one cheap can pick it up deliberately. GUI-
specific ideas live at the bottom of GUI-DESIGN.md instead.

## I1 — Custom "unit tests" for parts

A part carries user-defined checks that run like the kernel's own
validity gates — certified where the geometry supports it, honest
heuristics where it doesn't (labeled as such, never silently
mixed). Named instances Evan wants eventually:

- **(0) The scale-relative sliver lint — the FIRST and easiest
  member (Evan, 2026-08-03, resolving #89's display half).** A
  margin that is numerically definite (≫ Kε) yet below
  display-distinguishability at model scale is *probably* a
  mistake — warn, never refuse; the modeler confirms intent ("the
  2 µm step is deliberate"). Easiest because no new geometry is
  needed: the margins are already recorded (the K-funnel
  Probe/verdict telemetry), so this is a threshold sweep over
  existing data plus a warn channel — and it is the PROTOTYPE of
  this whole lane's shape (per-part, advisory, honest about being
  a heuristic threshold). #89's kernel half (the K value itself)
  stays with the exit-walk K-snapshot decision, separately.
- **(a) Injection-molding / draft**: the shape is 1-1 along the
  pull direction (a function), with derivative everywhere below a
  max — equivalently minimum draft angle everywhere. This is a
  certified-geometry check (hull bounds on surface normals vs the
  pull direction — C9-class machinery; the tessellation/normal
  enclosures built in M5 are the substrate). Nearest-term of the
  four.
- **(b) Thermal expansion**: materials in metadata; at target
  temperatures, scale each part by its CTE and re-run the
  constraint/clearance checks — do all mates/fits still hold?
  Prerequisite: M8 signed clearance + material metadata vocabulary
  (and the interference-fit declarations, which are temperature-
  dependent by nature — see #161's declared-contact doc).
- **(c) Tool access**: can a hand holding a screwdriver reach each
  screw (wrench equivalents etc.)? Swept-volume / clearance-corridor
  queries against the assembly — needs assemblies, swept volumes,
  and a tool-envelope library; genuinely far out, parked so the
  assembly design leaves room for volume queries.
- **(d) Machinability heuristics** per machining method: relies on
  heuristics we do not currently know; explicitly the
  labeled-heuristic class (never certified language).

## I2 — Design-for-measurement

Make it natural for designs to favor easy measurement:
- **(a)** once interval tolerancing exists (M8), make "the required
  tolerance at the places that will actually be MEASURED" a
  first-class query — the designer sees measurable-point tolerances,
  not just whole-feature ones.
- **(b)** bridge the gap between how a machinist measures (calipers
  at one or two points on a width) and what the design needs (the
  tolerance holding across the whole surface): pick point-tolerances
  that suffice for the whole-surface requirement under declared
  assumptions about surface variation (a form-error budget). This is
  a certified-inference feature: point bound + variation assumption
  ⇒ surface bound, with the assumption recorded as a declared input
  (D4-honest: the assumption is data, not a guess).

## I3 — Handbook-lookup definitions

Define standard engineering choices (the named case: interference
fits) by lookup against machinist's-handbook norms keyed on
materials/sizes in question, rather than hand-entered numbers —
i.e. `fit: H7/p6-per-handbook(steel, 12mm)` as a declared,
versioned data source. Prerequisite: the declared-contact/
interference vocabulary (#161, rides the M6-era census/contact
design doc) + a data-provenance story (the handbook table is an
input with a version, like a tolerance). Pairs naturally with
I1(b) and I2.

## Process note

Items graduate from this file by being written into a milestone
plan with Evan's sign-off; the file records the idea's origin date
and any design-fact dependencies discovered since parking.
