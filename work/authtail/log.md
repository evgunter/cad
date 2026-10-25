# AUTHTAIL — log

## 2026-10-02 — opened at AUTHOR's exit

This is the unblocked half of AUTHOR's cut: 9 rows, 19.5 budget points
against 30. The blocked half is DOORS. One row was priced at the move:
`fmt-re-minted-a-continued-literal-into-the-assertion-message`, which
carried no band or cost, is P4 E. One row is new:
`drawing-on-a-picked-face-is-a-two-form-trip` (P2 M), filed by the
exit sweep because two code comments named a closed AUTHOR row as its
carrier and no live row held it.

AUTHOR's narrative is recoverable from
`docs/doc-ledger/author-leaves-the-tracker.md`.
- 2026-10-23 — Re-read every open row against main at `e4045c3132`. Closed two that are already fixed: `drawing-on-a-picked-face-is-a-two-form-trip` (`e58fec1fb5`) and `chrome-calls-one-node-two-names` (`295c2e786e`). Re-pointed `fmt-re-minted-a-continued-literal-into-the-assertion-message` to the three literals that now carry its defect. The rest stand, with these notes:
  - **Citations to re-point:** `a-held-face-stays-marked-while-its-form-is-collapsed` holds on the successor of its cited sites, the add-profile on-face latch (`Drafts::held_face`, `drafts.rs:938`, set by `sketch_face_rows` inside the collapsible "Add feature" section). `the-mate-and-blend-commit-rows-restate-tool-commit-row`'s `kind.says` is now `kind.refuses`.
  - **Needs a new statement:** `committed-nodes-do-not-light-the-faces-they-reference` names `Node::Declare` (deleted `37ebc5bafc`), `Datum::FaceFrame` (deleted in 5506) and mate nodes, which are gone. A measure and a placement's mates still reference faces, so the question survives, but it needs re-stating before anyone takes it.
- 2026-10-25 — Picked up by a fresh orchestrator (status `active`). Order is set against INTENT's in-flight work (D′3 PR 5636, then G `patterns-are-index-variables` and E), so that nothing lands that INTENT is about to rewrite:
  - **First, P0 `the-viewer-cannot-author-a-mirror`.** `Node::Mirror` is already built in its D10 shape (PR 5607), and G's §8 (c) says its Mirror "waits on nothing", so G will not move it. The gesture picks a planar face and writes `PoseDef::Plane { face }`. It does not type numbers through the split form: those numbers go through `session::frame_off`/`moves_of`, which INTENT's `the-viewers-numeric-gestures-read-a-frame-back-off-geometry` names as the retired heuristics. A face-read plane is also the plane the mirror-seam ruling (`a-symmetric-parts-seam-is-not-walked-as-one-construction`, Decided) makes a projection of its face's construction. INTENT's `a-mirrors-body-reads-as-in-no-space` (P2 E) bites any union whose first member is the image; the lane reports whether the gesture reaches it.
  - **Then, in parallel, the rows INTENT does not touch:** the two P4 E rows, on the orchestrator's read. P3 `a-last-leg-no-close-can-follow-is-dropped` is sketch-only, but half of it waits on PROFILETAIL's open `a-close-from-a-tip-on-its-start-is-refused-as-a-tangent-seam`, and the other half is a design question about what the preview is. It goes after the held-mark design, not before.
  - **Design first:** P2 `a-seated-tools-held-node-is-drawn-nowhere`, and P3 `a-held-face-stays-marked-while-its-form-is-collapsed` (`drafts.rs` is INTENT's ground too, and D′3 edits it).
  - **Held until D′3 lands:** `committed-nodes-do-not-light-the-faces-they-reference`, which needs re-stating against D′3's placement (mates now read only geometry), and `the-mate-and-blend-commit-rows-restate-tool-commit-row`, which D′3 reshapes (`matetool.rs` is in PR 5636's diff). Without the hold, either would be built against a mate tool that is about to change.
- 2026-10-25 — Closed the two P4 E rows on the orchestrator's read (`authtail/p4-hygiene`): `a-dimensionless-computed-value-reads-with-a-trailing-space` (and `no_reading`'s double space beside it) and `fmt-re-minted-a-continued-literal-into-the-assertion-message` (three viewer literals, plus two drive-bys the tree-wide grep found in sweep and topo tests). Dispatched the P0 `the-viewer-cannot-author-a-mirror` lane to `authtail/viewer-mirror`, under single review (style plus correctness): its failure mode is a confident wrong picture, an image across the wrong plane.
