IN PROGRESS

# JOIN-2 review, lane r1 (PR 3880 @ 17c254c99d)

First finding under test: `rect a=3 r=1` (crates/sweep/tests/join2_r1_probes.rs,
`join2_r1_like_far_ends`) — a declared union whose join refuses on the tangent
faces now refuses `RestZipUnsupported { ChordBetweenIsolatedPierces }` in the zip
(rest.rs `realize_seam`'s ring-first order picks a segment with both ends
isolated). Differential against main pending.
