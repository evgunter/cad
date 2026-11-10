---
id: line-holders-the-line-placement-sweep-missed
kind: issue
title: Two line holders the line-placement sweep missed: link.rs's forward decision and sweep revolve's AxisFrame (narrowed)
status: open
opened: 2026-11-09
priority: P2
cost: M
---


Found by the FULL review of PR 6665 (`onecopy/line-placement`, head `b7b5f34c`). The sweep's pair shapes (`(origin|apex|…): Point3<T>, (axis|dir|…): Vec3<T>`) did not see these. Each holds a line as loose parts, or decides "ahead along a ray" outside `geom::Ray::ahead`.

**1. `topo/src/link.rs`: a second forward decision under `LINK_AHEAD`.**
- `ahead` / `ahead_settle` / `ahead_distance` (`link.rs:1306-1335`) decide whether `x` lies ahead of `p` along the ray through `r`. They decide it as `metric::Distance::of(x, p, (r − p).normalize())` through `metric::settle` (`link.rs:1319`; also `:1624`).
- The closure `past(from, dir)` at `link.rs:2880` decides `Distance::of(apex, from, dir)` under the same row with `metric::sign`.

It is the same question `Ray::ahead` answers: a point's advance along a half-line, decided against zero. It is spelled through link's own metric door, though:
- the direction is normalized at the site, so the advance is a length (`Distance`, `link.rs:167`), where `Ray::ahead` takes a parameter in units of `|dir|`;
- `settle` reads a `Reading` (the `own` / `within` clamp to the wedge's reach, `link.rs:1328-1333`), which `Ray::ahead` has no counterpart for.

The port is a `Ray` door taking a point, its advance read as a length over a unit direction, with link's clamp kept at the caller. Whether that is one question or two (a metered point reading against a parameter reading) is the first thing to settle. Either way `LINK_AHEAD` keeps its row name.

**2. `sweep/src/revolve/axis.rs:33` `AxisFrame`.** It holds the placed axis as `o3` / `a3`, with `axis: UnitVec3` and `u3` beside them: a full frame. Three sites build `geom::Line { origin: frame.o3, dir: frame.a3 }` by hand: `revolve/full.rs:957`, `revolve/surfaces.rs:117` and `revolve/turn.rs:76`. The cheap step is an `AxisFrame::axis_line()` read at all three. The frame itself is `analytic-surfaces-spell-their-frame-field-by-field`'s to type.

**3. `sweep/src/blend/arms.rs:852` `Meridian`.** It holds `origin` and the unit `axis` (a line) beside `rim`, and builds `geom::Line { origin: self.origin, dir: self.axis }` at `arms.rs:979`. The port is a `Line` field (or `Meridian::axis_line()`).

**4. `geom/src/cover.rs` `Core` (arrived with main, 2026-11-09).** BOXES' redesign of the depth readings made `geom::cover::Core { center, axis, radius }` pub, read through `Around::Circle`. It holds the circle's axis as a loose `center` / `axis` pair, which a private `axis_line(center, axis)` in `cover.rs` rebuilds into a `Line` twice. This replaces PR 6665's private `Core { axis: Line, radius }` and `topo` `Depth::Core { axis: Line, major }`: the merge took main's side, because the redesign belongs to a program in flight. Porting `Core` to hold a `Line` is this row's.

## Narrowed (2026-11-10, `onecopy/placement-holders`, PR 6713)

Items 3 and 4 are done:
- **3.** `arms::Meridian` holds its axis as a `geom::Line`, and the hand `Line { origin, dir }` build retires.
- **4.** `cover::Core` holds `axis: Line` (its origin the circle's centre), and so do `Slab` and `Frustum`, so `cover.rs`'s private `axis_line(center, axis)` retires.

What remains is items 1 (`link.rs`'s forward decision) and 2 (`AxisFrame::axis_line()` at its three hand builds).
