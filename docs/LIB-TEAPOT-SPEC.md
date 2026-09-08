# LIB-TEAPOT — the tour's teapot through the document

The binding spec for unit `LIB-TEAPOT` (`work/lib/teapot-scene-through-node-shell.md`), the LIB program's next full-protocol unit. Every door it uses is ratified and bound; what this spec decides is the CONVERSION — which of the tour's four kernel-direct bodies becomes a document node, how each is said, what may move on the render lanes and why, and which audit rows flip on which evidence. Nothing here is a design conversation: `docs/RECIPE-DOORS-DESIGN.md` D5 (the shell door), the revolve, fillet, transform and tube nodes and the Python doors that bind them are all on main. The spec is deleted at the unit's merge and recorded in `docs/DOC-LEDGER.md`, per the rule there.

## 1. The unit in one paragraph

`demos/tour/src/teapot.rs` builds four solids kernel-direct — a revolved pot hollowed by `shell_open` with its mouth found by a numeric plane scan, a revolved lid whose three latitude rims are rolled by `fillet_edges` on keys found by a numeric rim scan, a revolved spout placed by `transform_rigid`, and a `tube_along_arc` handle — and pins two walls (the two unions refuse `CurvedPairUnsupported`). The unit converts the scene to ONE `Doc` whose four root nodes evaluate to the same four bodies, with the mouth and the rims named by ROLE rather than found by scan, the same two walls pinned as document refusals, the same five findings recorded, the render lanes and the tess-budget baseline holding or re-baselined with their reasons, and a Python row in `crates/pncad-py/tests/test_north_star.py` that authors the scene through the bound doors and holds it to the scene's own closed forms — which is what flips audit rows 27 and 44 from NO to YES under that page's discipline (a YES is a scene the Python suite executes). Row 45 stays NO on its named secondary (`Body::merge_coplanar_faces`, unbound) and is not this unit's.

## 2. The document

One `Doc<ProfileProgram>` (`Doc::empty_derived("teapot", tol)`, the heatsink's shape), four roots. The tour's `Stop` is built from the evaluation (`SceneBody::named(&ev, node)`, the shape `heatsink.rs` and `diefillet.rs` already use), so the scene's bodies are the document's values and nothing is built beside the document. `gallery_document` exposes the document for the gallery, as the heatsink does. Every constant in `teapot.rs` stays a `const` with its dyadic argument; the meridians become `LoopProgram`s over `Expr::literal` exactly as `crates/editor-core/tests/corpus/vessel.rs::meridian` spells the vessel's (the lane copies that spelling rather than re-deriving it).

The sketch frame and the revolve axis are the vessel corpus's: a `Datum::Frame` with u = +X and v = +Z (the meridian's own axis is its +v through the origin, so the pot stands on +Z in world) — **note that the tour's pot stands on +Y today** (`revolved` revolves about the sketch's +y with `SketchPlane::xy()`). The unit keeps the WORLD placement the frames were rendered from: choose the datum frame that puts the axis where the tour's is (u = +X, v = +Y, the `xy` plane), so the bodies land where they landed. A moved frame from a changed world orientation is a defect, not a re-baseline.

## 3. The vessel (rows 27 and 44's body)

`vessel.rs`'s spelling, verbatim at the document layer: `Node::Profile` over the meridian program, `Node::Revolve { profile, axis, angle: TAU }`, `Node::shell(pot, WALL, [band(pot, SEG_MOUTH), band_pi(pot, SEG_MOUTH)])`. The mouth is the two half-faces of the meridian's mouth-disc segment, NAMED — `plane_chart_at` is deleted with the scan it existed for. `SEG_MOUTH` is the segment index of the mouth disc in program order, and the lane asserts it against the program rather than transcribing the corpus's constant.

Pins carried over unchanged from the tour: the cup's per-shell classification, genus 0, one annular rim, the closed forms (`pot_volume`, `pot_area` and the cup's props against the stack integrals), `volume_pad == 0.0`, the mesh's triangle count > 0, the STEP round trip. Each is asserted against the DOCUMENT's value.

## 4. The lid

`Node::Revolve` of the lid meridian (annular: the vent bore is what makes its latitude rims closed edges — finding 3 stays, verbatim), then `Node::Fillet { target: lid, distance: ROLL, selection }` where `selection` is the three rims BY NAME. A full revolve splits each latitude circle at the seam, so a rim is two edges — `RoleSeg::BandRim(pv)` and `RoleSeg::BandRimPi(pv)` at the profile vertex `pv` between two meridian segments (`names/emit_sweep.rs`). The three rims (flange/dome foot, dome/knob wall, knob top) are therefore SIX names; the lane reads the emitter and asserts the name set it selects resolves to exactly the six edges `rim_at` found, before deleting `rim_at`. If a rim the tour rolls has no name (an emitter gap), that is a stop clause (§8), not a scan kept.

Pins carried over: three band faces, their surfaces (torus/torus/torus or as the tour asserts), the two tangency lines per band, the mass delta of the roll against the sharp lid, and finding 3's negative half cited not re-asserted.

## 5. The spout

`Node::Revolve` of the spout meridian, then `Node::Transform`. The document's placement vocabulary is axis-angle (`Node.transform(input, translation, rotation_axis, rotation_angle)`): the 3-4-5 turn the tour writes as an exact matrix is the rotation about +z by `θ = atan2(0.8, 0.6)`, which is NOT a binary-exact angle, so the placed spout may differ from the tour's by rounding at the 1e-16 level. The unit states which it is: if the rendered spout frame and the tess-budget rows are byte-identical, nothing moves; if the frame moves, it is re-baselined WITH THIS REASON (the document's placement is axis-angle, the tour's was a matrix), and the spout's mass pins are held to the same tolerance the tour used. The lane measures `|placed − exact|` on the root disc's centre and reports it.

## 6. The handle

`Node::tube` on a `Node::datum_axis` at `HANDLE_C` with the spine's normal, `u_ref` chosen so the window's angles match `tube_along_arc`'s call in the tour (the lane reads that call's `TubeWindow` and reproduces it, including `HANDLE_OVER` past the semicircle at both ends). `HANDLE_OVER`'s leak note stays verbatim: nothing asserted depends on the overshoot and the union refuses before any intersection.

## 7. The two walls, as document refusals

The tour runs `union(&cup, &handle)` and `union(&cup, &spout)` once each and carries their `CurvedPairUnsupported` refusals into the panel's note. The document says the same two requests as `Node::Boolean { op: Union, a: vessel, b: handle }` and `{ ..., b: spout }`, which lower to the same kernel `union`; each refuses at `evaluate` under `NodeErrorKind::Boolean` with the kernel's payload, and the panel note quotes the document refusal (the `describe` of the node's error) instead. Finding 4's whole text stays — including the wall-7 lesson that the pair the refusal NAMES is not the pair the spout pierces — and the unit adds one sentence: the refusal now arrives through the document, under the same payload. `curved_pair_unsupported` is asserted by tag in the Python row.

## 8. Stop clauses

- A rim, band or mouth face the emitter does not name: STOP, file the emitter gap on LIB's slate citing the `RoleSeg` vocabulary, keep that ONE selection kernel-direct with the scan it needs, disclose it as the reason the corresponding audit row stays where it is, and continue with the rest.
- A body whose document value differs from the tour's beyond the tour's own pin tolerance (mass, face count, genus, shell class): STOP on that body — do not loosen a pin; report the difference with the lowering step that introduced it.
- A tess-budget verdict the gate refuses (`tools/tess-lint` against the committed baseline): report, do not re-cut the baseline to make it pass; a re-cut is an ordinary commit under `docs/TESS-BUDGET.md`'s "Re-cutting the baseline" only when the scene's geometry legitimately changed, which §2–§6 say it must not.

## 9. Rendering, the tess budget and the audit

- The render lanes REPORT: a cell that "matches this render" is the expected outcome for the pot, the lid and the handle; a moved cell is re-baselined by the lane with its reason in the commit (never restored), and the only reason this spec admits is §5's. Any other moved cell is a finding.
- The tess-budget baseline (`docs/tess-budget-data/tess-budget-baseline.csv`) holds unless §5 moves the spout; the lane runs the sweep locally (`demo-tour tess-budget`) and `tools/tess-lint` against the baseline and reports the verdict.
- `docs/guide/north-star-audit.md`: row 27 flips to YES on the Python row (§10) with the sentence about the plane scan retired; row 44 flips to YES on the same row (the vessel is the scene's own body); row 45 stays NO with its secondary named; the G17 cell's "Rows 27, 44 and 45 stay NO as JOBS" sentence is rewritten to say which flipped and on what.

## 10. Python

`crates/pncad-py/tests/test_north_star.py::TestTeapot`: the four bodies authored through `Node.sketch_frame` / `Node.profile` (the meridians as `Open`-lattice chains, `arc_to` about a centre on the axis) / `Node.revolve` / `Node.shell` (the mouth named through `Evaluation.select` on the pot's revolve, `SegPat` by segment and side, never by composed text) / `Node.fillet` (the six rim names the same way) / `Node.transform` / `Node.datum_axis` + `Node.tube` / `Node.boolean` for the two walls; held to the scene's closed forms (the pot's and cup's volumes and areas from the same stack integrals, restated in Python from the constants — never transcribed decimals — the lid's roll delta, the spout's frustum forms, the handle's torus forms) at the tour's own tolerances; the two unions raise `EvaluationError` with `kind == "boolean"` and the inner refusal's text naming `CurvedPairUnsupported` (or its per-arm tag if `[ev]` PR 2196's ruling (A) has landed by then — say which). `test_shell.py`'s cup rows are untouched.

## 11. Not this unit

Binding `Body::merge_coplanar_faces` (row 45's secondary; a repair door, its own question); any kernel change (the operand gate's curved arms, the sweep's U-turn, a variable-section sweep — findings 4 and 5 stay findings); composing the unions; thinning the handle; the spout's real shape; the lily and every other kernel-direct tour scene.

## 12. A/B protocol fields

Written knowing the slot's arm (block LIB-13 was drawn 2026-09-06; slot 2 is OPUS by the block's arithmetic), disclosed here. Difficulty **M** (a scene conversion over ratified doors: one document, four lowerings the kernel already runs, a naming read of the emitter for the rims, a render-lane and tess-budget verification, a Python row; no new numeric decision and no kernel edit). Task class **STRUCTURAL**. Dual review under v6 at the frozen head; the ordinal is claimed at review dispatch from the LIB band (300–399; claimed through 303).
