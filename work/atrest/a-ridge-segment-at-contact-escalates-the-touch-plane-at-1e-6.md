---
id: a-ridge-segment-at-contact-escalates-the-touch-plane-at-1e-6
kind: issue
title: link's segments_at_contact_read_zero fuzzer finds a ridge segment at contact that escalates census_touch_plane at ε 1e-6
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by `atrest/aligned-arc-or-curved-faces-that-abut-are-flush`
(PR 5884) running the topo `link` rows at three ε. Not that branch's: it
does not touch `crates/topo/src/link*`. `topo::link` is ATREST's
(PR 5773), so the ATREST orchestrator moved it here from `work/issues/`.

## What

`crates/topo/src/link/oracle_rows.rs` `segments_at_contact_read_zero`
(a seeded fuzzer) fails at ε = 1e-6, about one run in twenty:

```
CAD_FUZZ_SEED=0x08bf80a8152a8eb1 CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p topo --lib segments_at_contact_read_zero
link_contact: 1 wrong
ridge (Point3 { x: -0.5381169046338388, y: -0.2791028865813151, z: 0.3960765877674144 },
       Point3 { x: 0.2334274997934839, y: 0.12102897547099672, z: -0.17181241996512442 }):
Err(Undecided(Indeterminate { margin: -8.792433570270486e-6, band: { zero: 1e-6,
escalate: 1e-5 }, predicate: Some("census_touch_plane") }))
```

The segment passes through the ridge's line, so its lowest point is a
contact and should read zero; `census_touch_plane` reads a margin of
−8.8e-6, inside the escalation gap, and refuses. The seed replays
deterministically.

## What closing it takes

Find why the touch plane's margin at a point on the ridge's line reads
nearly the escalation width at 1e-6 (a lever on the segment's length or
direction that the margin does not discount), fix it, and pin the seed
as a deterministic row beside the fuzzer.

## Seen again (2026-11-01, PR 6210 gate)

The PR gate's 1e-6 row of `test (kernel)` failed this fuzzer on PR 6210
(`touchrec/kept-side-reads-the-glue`, head 7d18c80cf), which does not
touch `crates/topo/src/link*`:

```
CAD_FUZZ_SEED=0xd28c76886fdb791c CAD_FUZZ_EFFORT=1 CAD_TOLERANCE_EPS=1e-6
ridge (Point3 { x: -0.3407114228332363, y: -0.48885945078971893, z: 0.15291002337969203 },
       Point3 { x: 0.675925503203203, y: 0.9697684951281023, z: -0.30335285984327015 }):
Err(Undecided(Indeterminate { margin: -5.988201234069512e-6, band: { zero: 1e-6,
escalate: 1e-5 }, ... }))
```

A second seed, same shape: the margin sits inside the escalation gap.
Because it fails about one run in twenty, any PR touching `topo` can
pick up this red.
