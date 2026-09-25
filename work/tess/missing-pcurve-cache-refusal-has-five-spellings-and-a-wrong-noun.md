---
id: missing-pcurve-cache-refusal-has-five-spellings-and-a-wrong-noun
kind: issue
title: the missing-pcurve-cache refusal is spelled five times, calls an Approx face a NURBS face, lists four inconsistent producers and names no recourse
status: open
opened: 2026-09-22
priority: P3
cost: E
---


From the Approx-face survey (`tess/approx-face-survey`, `d423d1b46`),
filed by the TESS orchestrator, 2026-09-22.

- **Wrong noun.** `mesh::chords::nurbs_tighten`'s `UnsupportedCurve`
  note and `topo::props`' `QuadratureUnsupported` (`nurbs_face` and
  `trimmed_face`, byte-identical) say "NURBS face"; the face wears
  `Surface::Approx` and only its chart is a spline. A reader grepping
  for a NURBS face on that body finds none.
- **Producer lists, four, mutually inconsistent, none naming the
  door that fixes it**: `chords.rs` "caches mint at loft/sweep assembly
  and STEP adoption" (STEP adoption never mints an `Approx`);
  `trimmed.rs` "the split/boolean pipelines"; `props.rs` ×2 "the loft
  assembly mints them; a body that lost its caches must re-mint". The
  recourse — `topo::mint_pcurves` — is named at no site. Contrast
  `chart_region.rs`'s `MissingCache { half_edge }`: half-edge, recourse,
  no producer list.
- **Payload**: `chords.rs` names the `edge`; the cache is per
  half-edge. `props`' is a `&'static str` shared by two raising sites,
  so the refusal cannot say which raised it. `trimmed.rs`'s
  missing-cache arm is unreachable on the NURBS/`Approx` lane (chords
  refuses first) and reachable only on the cylinder lane.

One spelling, one payload (half-edge and face), the recourse named,
no producer inventory. `chords.rs` is TESS/CHORD shared ground;
`props.rs` is TOPO's — a crossing to announce, or file the props half
there when the unit is cut.
