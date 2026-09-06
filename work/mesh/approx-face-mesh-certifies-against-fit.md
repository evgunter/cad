---
id: approx-face-mesh-certifies-against-fit
kind: issue
title: mesh + props - widen an Approx face's tolerance by its certificate's bound, so the mesh certifies against the DESCRIPTION
status: open
opened: 2026-08-26
github: 1018
refs: [1012]
---

## From GitHub issue 1018

Opened 2026-08-26; 0 comments.

`mesh::tessellate` routes a `Surface::Approx` face through the trimmed spline lane on its **fit** (`crates/mesh/src/tessellate.rs`, `trimmed.rs`), and the triangles it produces certify against that fit. They do **not** certify against the face's *description*.

The gap is exactly the certificate's `hull_sup`: the fit is within ε_precision of `S + d·n`, so a mesh certified to δ against the fit is certified to `δ + hull_sup` against the offset the modeller asked for. Nothing in the pipeline says so today — the delegation is plain, deliberately, per VERBS-OFF-C's spec ("delegate plainly, note the widening as a scheduled follow-on").

**What to build.** Fold the re-derived `hull_sup` into the chord/UV-step budget for `Approx` faces so the emitted mesh's certificate is a statement about the description. The same omission is stated at the second delegating site, `topo::props`' quadrature lane (`quad_lane::cut_face`), whose flux and area are likewise the fit's.

**Where the comments already point at this**: `crates/mesh/src/tessellate.rs` (the `Surface::Nurbs(_) | Surface::Approx(_)` arm) and `crates/topo/src/props.rs` (`cut_face`'s spline-lane arm).

Filed from VERBS-OFF-C (#1012), MINOR-3.

## Home

The primary site is `crates/mesh/src/tessellate.rs`, S-MESH's territory, and honesty of the emitted certificate is its charter; the `props` quadrature half needs S-CERT's coordination per S-MESH's keep_out.
