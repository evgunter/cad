---
id: apex-links-the-nappe-reading-still-refuses
kind: issue
title: Apex links the nappe reading still refuses: three lobes, a horn beside, a cut half-plane, a tangent pair
status: open
opened: 2026-11-12
priority: P3
cost: M
---



Found by the valid-bodies lane (branch
`restread/self-touch-valid-bodies`), which reads several nappes at one
apex. Priority P3: no measured body reaches any of these, and each
refuses, which is sound.

## What

`rest::at_apex` and `rest::whole_nappes` (`crates/topo/src/link.rs`)
read a link holding cone apexes as whole nappes, apart or nested. These
links still refuse, the first four as `Curved(NoTangentPlane)` and the
last `InBand`:

- **A nappe of three or more lobes.** `whole_nappes` pairs a lobe with
  one other of its carrier and sense, each bounded by the other's two
  edges. A cone face cut along three rulings at its apex is three lobes
  round one nappe, and no two of them close it.
- **A horn beside a nappe.** `at_apex` refuses any link with a horn: the
  horn's ray may lie inside a nappe's cone, which no region of the rest
  holds.
- **A half-plane cutting a nappe.** A sector that is no whole plane
  whose plane the nappe straddles: its wedges may pass the nappe by, and
  the reading does not place the rulings against a wedge.
- **A needle inside no other nappe with no sector round it**, alone or
  beside thorns. Beside a thorn the windings are three, a crossing the
  reading does not report.
- **Two nappes tangent along a ruling** (`rest::rulings_against` reads
  the near or far ruling Zero). A valid body can hold this: a cone less
  a narrower one tangent to it inside along one ruling. The germ is a
  line contact, which the nappe reading does not read.

## The fix

Each is its own reading. Lobes round one nappe can be chained by their
edges as `second_order::thorns` chains horns. A half-plane needs the
rulings placed against its wedges (`Sector::place`), not only its plane.
The tangent pair is the edge-in-face reading's case along the shared
ruling.
