---
id: a-filleted-one-segment-hole-rims-band-exceeds-its-mesh-certificate
kind: issue
title: A plate hole whose one-segment rim is filleted refuses tessellation CertificateExceeded at some (radius, delta) where its two-arc twin meshes: r = 0.005 at delta 1e-3 is 7.7x over
status: open
opened: 2026-11-21
priority: P1
cost: M
---



Found by PROFILE's `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment, so a hole's rim is one closed edge at one vertex and its fillet band is one ring face with one slit where the two-arc rim's band was two.

## What

The plate `[−1, 1]² × [0, 1]` with a hole `circle(0, 0, 0.3)`, its bottom rim (or both rims) filleted through `Node::Fillet`, meshed by `mesh::tessellate` (`crates/editor-core/tests/edit_bore_two_rims.rs`, the lane's probe), against the same plate whose hole is `circle_split(2)`:

| fillet r | δ | one segment | two arcs |
|---|---|---|---|
| 0.05 | 1e-2 … 3e-5 | meshes | meshes |
| 0.005 | 1e-3 | `CertificateExceeded { face: 9v1, bound: 7.72e-3, requested: 1e-3 }` (bottom rim; `10v1` with both) | meshes, 452 triangles |
| 0.005 | 1e-2, 3e-4, 1e-4, 3e-5 | meshes | meshes |
| 0.0005 | 1e-2 … 3e-5 | meshes | meshes |

The viewer meets it on its startup document: `viewer`'s `a_held_face_on_a_copy_a_feature_re_points_stays_held` fillets the hole's bottom rim at `r = 0.0005`, and the re-pointed copy is undrawn, `Tessellate(CertificateExceeded { face: 9v1, bound: 2.24e-4, requested: 1e-4 })`.

So it is a lottery over `(r, δ)`, not a floor: one δ fails where coarser and finer both pass. Which face `9v1` is (the band, by the census, is the likeliest) and which bound the certificate exceeds are not measured.

## Why it matters

A filleted round hole is everyday geometry, and the refusal is the picture's and the export's: the body builds and validates, and the mesher refuses it at a display δ.

## What closing it takes

Read the refused face's certificate: which lane meshes it (a one-ring torus band with one slit, its chart window a whole turn), and why its bound comes out several times the requested δ at that δ alone. The two witnesses above are pinned red at their stops.
