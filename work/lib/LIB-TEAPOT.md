---
id: LIB-TEAPOT
kind: unit
title: the tour's teapot through the document
status: review
opened: 2026-09-08
branch: lib/teapot
refs: [teapot-scene-through-node-shell]
pr: 2206
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
  fillet refuses `Naming(Duplicate)` before any geometry is doubted —
  ATTEMPTED live in `per_rim_answers` and pinned there, so the finding
  is a probe and not a sentence. Filed as
  `blend-slit-name-collides-when-two-rims-share-a-meridian`, whose
  first cut over-generalised to "any two ADJACENT rims" and is
  corrected to the true invariant (two bands slitting ONE seam
  meridian; `{2,3}` and `{3,4}` are adjacent and compose).
  `demos/tour/tests/teapot_document.rs` is the table, beside the
  equality the split owes: the two requests build the kernel's
  one-request body — same census, the same three bands bit for bit,
  the same mass — and differ only in face ORDER.
- **Wall 3's payload is the same face, operand and kinds — not the
  same bits.** The base raised `other_face: FaceKey(1v1)` and the head
  raises `2v1`: the same physical face (the mouth-rim annulus at
  `y = 1/8`), whose arena key moved with the document's numbering.
  Neither wall's probe ever pinned a key.
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
- **The tess-budget baseline was re-cut, and the gate did not ask for
  it.** `tools/tess-lint` is CLEAN against the PREVIOUS baseline and
  clean against the re-cut one; both reviewers ran it both ways. So
  nothing forced the file to move, and §8's third clause — a re-cut is
  ordinary "only when the scene's geometry legitimately changed" — is
  contradicted by exactly this: no geometry moved and the file moved
  anyway. The move is 30 of 1353 rows, 27 gaining the durable face
  NAME `SceneBody::named` now hands the sweep and 3 `teapotlid` rows
  permuting their triangle counts among themselves (`{16200, 2048,
  42560}`, same multiset, same per-scene total) because the
  two-request roll mints the lid's faces in a different order. It was
  taken because a committed baseline that no longer describes what the
  sweep produces is the failure mode `docs/TESS-BUDGET.md` fears most,
  and because CLAUDE.md's standing rule is to re-baseline and say what
  moved. The reverse is SCHEDULED rather than remembered:
  `per_rim_answers`' live pin says, on the day the naming
  discriminator lands, to go back to one request and re-cut back.
- **The uv render cell that moved is a §9 finding by the spec's
  letter** — §9 admits only §5's reason for a moved cell, and §5's did
  not fire. Its cause is the same face order: the uv sheet labels its
  cells by face INDEX. Four cells moved in the sheet;
  "charts intact and re-slotted" covers three of them, and the fourth
  (`f005`) also changes where its trim loop STARTS and which way it is
  traversed, which is the same permutation one level down.
- **The lid's STL is the same triangle set either way**, measured: the
  base and head `teapotlid` exports carry the same 63,204 canonical
  triangles over equal vertex sets, and differ in EMISSION ORDER.
  That is what "the geometry did not move" means here, stated as a
  measurement rather than as an inference from a clean gate.
- **Two duplications are disclosed rather than shared**, and both are
  filed: `band`/`band_pi`/`band_rim`/`meridian_vertex` are twins of
  `crates/editor-core/tests/corpus/vessel.rs`'s and are hand-spelled
  in five places across this repo, and the vessel's whole constant
  block is carried by both files. Neither can be shared — the tour is
  a detached workspace and the kernel must never depend on demo
  tooling — so what is filed is the missing FAÇADE door
  (`no-facade-door-mints-a-revolves-role-names`).
- **A recipe cannot hold a narration or probe body without it becoming
  a product root**, which `gallery_document`'s three deletions and the
  two probe documents absorb. Filed
  (`a-recipe-cannot-hold-a-narration-body-without-it-becoming-a-root`)
  and gap-commented at the door that pays it.
- **The panel note renders the germ PAIR rather than `Debug`.** The
  base dumped the refusal struct, which puts Rust field names and
  arena keys into prose a person reads — the thing `pncad-py`'s own
  `errors.rs` refuses to do. The wall-7 evidence is the PAIR, and it
  survives verbatim; the console diagnostics still print the whole
  payload, where a key is a diagnostic rather than prose.
