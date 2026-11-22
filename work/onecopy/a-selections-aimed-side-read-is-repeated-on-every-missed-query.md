---
id: a-selections-aimed-side-read-is-repeated-on-every-missed-query
kind: issue
title: A selection's aimed side read is repeated on every query whose ray misses, though two comments say it is read once
status: open
opened: 2026-11-16
priority: P3
refs: [6592]
---

Found while answering Ev on PR 6592 (simplicity and speed), measured on main `0be17a13e`.

Two comments say a selection's side at infinity is read once: `crates/topo/src/census.rs` near line 10653 ("One selection per container … its side at infinity is read once") and `crates/topo/src/solid_contain.rs` near line 1800, on `point_in_solid`. That is true only of the volume fallback, which is cached on the selection (`VolumeReads`). The aimed rays are cast again on every query whose ray misses, because `point_in_faces` makes a fresh `MissedRead` per call. On n-gon prisms, 34 of 40 outside queries repeated the read. Forty queries cost 2.0, 30, 131 and 821 ms at 6, 66, 258 and 1,026 faces, against 1.0, 16, 72 and 468 ms when the side is carried.

**Fix:** PR 6592's part 2, where a selection carries its side from its constructor, retires the re-read entirely (`work/onecopy/side-at-infinity-aimed-rays-disagree.md`). If that stalls, a selection-level cache of the aimed read fixes this alone. That cache is also one of the two ways part 2 can serve the census's instance containment, which runs before the solid's roles are known.

**Done when:** a query count over a selection shows one aimed read however many queries miss, and the two comments are true as written.
