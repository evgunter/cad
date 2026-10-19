---
id: via-arrival-start-rustdoc-hides-which-direction-its-director-binds
kind: issue
title: paths: ViaArrivalStart's rustdoc does not say its director binds the arrival tangent at Start
status: open
opened: 2026-10-19
priority: P3
cost: E
---


Found while weighing the arrival-constructed arc (`arc-closer-constructed-from-arrival-tangent`); premise corrected by that design's harmony pass.

Every arc-arrival director binds the arc's END tangent, which is the landed point's INCOMING direction, and leaves the departure free. `RadiusArrivalAt` / `RadiusArrivalDir` (`crates/profile/src/path/family.rs:615`, `:638`) and `ViaArrival` (`family.rs:772`) all land `HasPos<WithIncoming>, NoAng`. `ViaArrivalStart` (`family.rs:845`, "A `Via` CLOSE: anchor at the entry, director pending.") and its `angle_kernel` (`family.rs:867`) do the same thing at `Start`: `via_close` takes the director as the arc's direction at the entry. It differs from the others only in that it closes.

None of these rustdocs says which direction the director binds. A reader who knows the line arrivals, where a director binds the side's direction and the tip stays Directed on it, can read it the wrong way. The fix is comment-only: one sentence on each arc-arrival builder saying that the director is the arc's end tangent, and the departure stays free. Once the token half of the arrival design lands, `Via { q, p: Start.arrives_tangent() }` closes at the verb with no director, and `ViaArrivalStart`'s sentence should name that as the tangent form.
