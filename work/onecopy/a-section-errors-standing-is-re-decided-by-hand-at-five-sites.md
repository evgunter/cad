---
id: a-section-errors-standing-is-re-decided-by-hand-at-five-sites
kind: issue
title: A SectionError's standing is re-decided by hand at five sites, and they disagree with SectionError::refusal()
status: open
opened: 2026-11-16
priority: P2
cost: M
refs: [7034]
---

Found by PR 7034's review (`onecopy/section-error-standing`, finding 1). Ruled out of that PR, because onecopy is winding down.

## What

`geom_brep::SectionError::refusal()` (`crates/geom-brep/src/intersect.rs`) is the one exhaustive standing of a section refusal. Its escalations, ranked by margin, are `Escalated`, `RadiusEscalated`, `Carrier(EllipseInvalid::Escalated)` and `Spiric(SpiricInvalid::Escalated)`. Its in-band causes include `ConeAtTheBand`.

Five sites still decide by hand which section refusal escalates, each over a different subset, and they disagree with it:

| site | escalates | everything else becomes | so |
|---|---|---|---|
| `crates/topo/src/offset_axial.rs:2603` (`plane_cone_section` for a cone cap) | `Escalated`, `RadiusEscalated` | `ReplaceFaceError::TogetherAxialEdge` | it aborts, where `refusal()` says margin for `Carrier(Escalated)` and in band for `ConeAtTheBand` |
| `crates/topo/src/offset_axial.rs:2362` (the spiric section) | `Escalated`, `Spiric(Escalated)` | refusals of its own | |
| `crates/topo/src/offset_derive.rs:674` | `Escalated`, `RadiusEscalated` | `SectionVerdict::Closed` | `Carrier(Escalated)` and `Spiric(Escalated)` end as `EdgeSection` or `CornerSection`, which abort as mixed |
| `crates/topo/src/replace_face.rs:4996` | `Escalated`, `RadiusEscalated` | `SectionVerdict::Closed` | as above |
| `crates/topo/src/boolean/join.rs:4106` `section_refusal` | `Escalated`, `Carrier(Escalated)` | `FrameError::Desync`, except `DegenerateTorus` | `RadiusEscalated` and `Spiric(Escalated)` read as a desync |

`crates/topo/src/boolean/section_cert.rs:1829-1841` reads `Carrier(_)`, `Spiric(_)`, `RadiusEscalated` and `ConeAtTheBand` as "no arm" (R-reach). That is a certificate vocabulary rather than a ladder standing, so it is weaker, but it is the same hand list.

## Also here

- **The torus convention's decided `Zero` aborts** (review F2). `TorusRefusal::refusal` (`crates/geom-brep/src/torus_convention.rs:130`) maps every `TorusVerdict::Refused` to `Aborts`, `Zero` and `Negative` alike. By `ConeAtTheBand`'s reasoning, and by `recourse::Refused::Zero`'s own doc ("a smaller tolerance may decide"), a torus decided `Zero` on its convention is in band. `SectionError::DegenerateTorus` delegates to it, so two conventions give two standings for one decided `Zero`.
- **`MinorNotPositive` mixes `Zero` with `Negative`** (review F5). The ellipse and spiric constructors raise it for both (`crates/geom/src/curves.rs:1346`, `:1392`), and `refusal()` aborts it as a constructor disagreeing with its arm. A `Zero` there may be the band's edge read at another lever than the arm's pose: in band, like `DegenerateOperand`'s mix.
- **The standing census is hand-written** (review S1). `each_section_refusal_stands_by_its_cause` (`intersect.rs:3407`) lists 25 rows by hand, so a new variant fails the build at `refusal()` but never has to gain a pinned row. `chart_region`'s sibling iterates `ChartRegionErrorKind`; this one should iterate a `SectionErrorKind` (strum) the same way.
- **`route_pose`'s errors are stated narrower than they are** (review S6). `route_pose` (`intersect.rs:618-628`) can return `NoMarch` or `Unaccounted`. `replace_face.rs:2954-2958` marks both unreachable, saying `route_pose` "returns only an escalation, a torus off its ring convention, or a dispatch naming the wrong arm or seat".

## Done when

- Every site above reads `SectionError::refusal()`. Each maps the standing to its own error, and none lists the variants by hand.
- A gate or a type goes red on the next hand classifier of a section refusal.
- The torus convention's decided `Zero` and `MinorNotPositive`'s `Zero` each stand as their cause decides.
- The census iterates a kind.
- `route_pose`'s readers handle what it returns.
- Measure which outcomes move.
