---
id: LIB-TEAPOT
kind: unit
title: the tour's teapot through the document
status: review
opened: 2026-09-08
branch: lib/teapot
refs: [teapot-scene-through-node-shell]
---


The tour's teapot converted to one recipe document under
`docs/LIB-TEAPOT-SPEC.md`, with the mouth and the lid's rims named by
role, the two unions said as document requests, and the audit rows the
conversion flips.

## Delivered

Every deviation from the spec, with the home the reasoning lives in.
The PR body carries the measurements.

- **§4's SIX rim names are THREE.** The spec derives them from a full
  revolve splitting each latitude circle at the seam. The LID's
  profile is ANNULAR, so the revolve takes the emitter's lamina branch
  instead — one whole wall per segment, one CLOSED rim per meridian
  vertex, no `BandPi` or `BandRimPi` on that body at all — and the
  pot's axis-touching profile is what splits. Home:
  `demos/tour/src/teapot.rs` (`LID_RIMS`, `band_rim`, the note's own
  sentence). The tour's `rim_at` already asserted each rim was one
  closed edge, which is the same fact.
- **§4's ONE `Node::Fillet` call is TWO.** The one-request document
  fillet refuses `Naming(Duplicate)` before any geometry is doubted.
  Filed as `blend-slit-name-collides-when-two-rims-share-a-meridian`;
  recorded as the scene's sixth finding and gap-commented at the site.
- **§4's cross-check against `rim_at` is in the branch, not in the
  shipped scene.** The assertion ran in the commit that converted the
  scene and the scan went in the next one; what ships is the live
  form — each rim's own circle read back through its NAME
  (`edge_frame`'s origin and the meridian vertex it passes through),
  against the station and radius the meridian authored.
- **§2's "four roots" is four in the GALLERY document.** The scene's
  own document has seven sinks: the four bodies, the sealed hollow
  whose census, capacity and Void classification §3's pins read, and
  the two refusals. `gallery_document` deletes the last three, which
  is what makes the file draw the teapot; `demos/tour/src/gallery.rs`
  carries the row and its separation `why`.
- **Two PROBES have documents of their own**: wall 1's re-planted
  torus-bellied pot, and the three per-rim fillet questions the scene
  prints. Both are bodies the scene measures rather than models, and
  the gallery opens the scene's recipe. Home: `wall_one_pot` and
  `per_rim_answers`.
- **§10's refusal text.** `pncad-py` renders a kernel refusal's own
  `Display` prose and never a `Debug` dump, so the variant name
  `CurvedPairUnsupported` is not in the message. `[ev]` PR 2196's
  ruling (A) has NOT landed — `crates/pncad-py/src/tags.rs` maps every
  `NodeErrorKind::Boolean` to the one tag `"boolean"` — so the Python
  row pins that tag plus the germ PAIR the gate named, which is the
  fact the Rust wall matches on.
- **§10 is enumerated for the teapot; row 44 needed its own row.**
  `torusvessel` is a different scene with a different meridian, and
  the audit's discipline is that a YES is a scene the Python suite
  executes, so `TestTorusvessel` authors that vessel's sealed hollow
  against its own closed form. Without it the §9 flip would have been
  a sentence rather than a row.
- **The tess-budget baseline was re-cut.** The gate is CLEAN against
  the committed file, and §9 predicted it would hold — but three
  `teapotlid` rows permute their triangle counts (same multiset, same
  per-scene total) because the two-request roll mints the lid's faces
  in a different order, and thirty teapot rows gain the durable face
  NAMES `SceneBody::named` now hands the sweep. The prediction's
  premise failed rather than the gate, so the file was re-cut and what
  moved is stated in the commit.
