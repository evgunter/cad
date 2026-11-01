---
id: material-and-convexity-are-read-three-ways
kind: issue
title: A cell's material and an edge's convexity are read three ways with three levers
status: closed
closed: 2026-10-26
branch: atrest/topo-link-one-material-reader
pr: 5773
priority: P1
cost: H
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells, gap-reads-no-curved-cell-below-a-face]
---

Three readers decide which side of a vertex's or an edge's faces its material lies on, and each has its own lever and its own decision:

- **`Gap`'s cell arms** (`crates/editor-core/src/eval/measure.rs`: `Wedge::turn`, `Fold::of_corner`, `Fold::offset`) decide an edge's turn as the sign of `dir·(plus × minus)`. That sign is decided under `gap_edge_convexity`, levered by the faces' reach (`topo::edge_wedge_reach`). A vertex's material is then the intersection or the union of its faces' half-spaces, and a saddle refuses.
- **The census's touch analysis** (`crates/topo/src/census.rs`, the section before `TOUCH_WEDGE`, and `rest::judge`) reads each solid's whole link at a touch point as polygonal cones. Every sign it decides is a point's signed distance from a plane, through one door (`metric::Distance`). Directions only propose planes, and one named `pairing` decision is allowed (`census::tests::the_touch_analysis_decides_only_through_its_doors` holds that). `Gap` decides a sign of normals directly, which that discipline rules out, and it reads a saddle not at all where the census reads one sector by sector.
- **`geom_brep::dihedral`** (`classify_dihedral`, `classify_material_pairing`, `material_kappa_rel`) classifies the wedge by the tangent planes' angle, levered by `folded_lever_arm` (curvature arms and the faces' reach). It then signs the wedge by the faces' material pairing and the jet's κ_rel. This is the one reader that handles curved faces.

`crates/editor-core/src/checks/at_rest.rs` `opposed` reads a curved face's sense off `Face::sense` directly, beside a `Carrier` that now carries the same fact as `convex`. That is a fourth spelling, one hop from the measure's. It is D′3's file and was left alone.

**What a fix looks like (not decided).** One reader of a cell's local material, at the census's one-door discipline: signs are point-to-plane distances, and directions only propose planes. `Gap`, the census and the dihedral classifier each read it at their own point. The curved-face cases of `gap-reads-no-curved-cell-below-a-face` and the saddle then come with it. `opposed` reads `Carrier`'s `convex`.

Also see `sector_shape.rs` and `census/curved`, which read sectors the same way the census does.

## Decided (2026-10-26, ATREST orchestrator, on a converged designer pair)

Designers A and B were dispatched concurrently with one brief; the record is on `analysis/design-fork/atrest-material-reader`. Both corrected the premise the same way. These are not three readers of one concept. They are three questions about one object, and the object is what is duplicated:
- `Face::sense` already fixes a face's side, so no reader decides it.
- `geom_brep::dihedral` classifies a wedge's shape (transverse, seam, cusp, slit) and is not a material reader. It stays as it is.
- The duplicated object is the cone of material at a point. The census holds it as `Link`/`Sector`, read by parity through the one `metric::Distance` door. `Gap` holds it as a Boolean of half-spaces (`Wedge`, `Corner`, `Fold`), which covers only the convex and co-convex special case. That is why `Gap` has to decide an angle (`gap_edge_convexity`), borrow a lever, and refuse saddles and near-flat edges.

**The final state.**
1. **`topo::link`**: the census's `Link`/`Sector` and its metric door, lifted into a public module.
   - It has a builder off a live `Body` at a vertex, inside an edge and inside a face (`Link::at_point` / `on_edge` / `in_face`), beside the census's `Geo` snapshot builder.
   - It answers `reads(q)` (In / Out / On, by parity), `judge(a, b)`, and a new `offset(q)`: the signed distance from `q` to the cone's boundary, negative where `reads` says In, as a closed-form minimum over the sectors.
   - `census::tests::the_touch_analysis_decides_only_through_its_doors` moves with the module and covers every caller.
2. **`Gap`'s vertex and edge arms read `offset`**, with the outer cell's link reading the inner cell. An inner edge takes the lowest value along the segment, over a finite closed-form candidate set.
   - `Wedge`, `Corner`, `Turn`, `Fold`, `gap_edge_convexity` and `gap_wedge_crossing` are deleted, along with the saddle and near-flat-edge refusals.
   - Off contact the magnitude becomes the true distance to the local cone, which retires `gap-reads-a-cells-carrier-beyond-the-cell` for cells.
   - Zero and the sign are unchanged, so nothing `[assertions.quiet]` reads moves.
3. **Face–face `Gap` is unchanged.** It is C5's carrier-relative signed offset, ratified text that predates this (`crates/topo/README.md` C5's list). One designer would have made the face arms the one-sector case of the link. That would change what ratified C5 decides, and nothing here needs it, so it is not taken.
4. **`topo/README.md` C5's cell sentence** ("read as the half-spaces of the face carriers the reading cell lies on") becomes "read as the outer cell's link, the directions entering its solid at the point: the signed distance to it, negative inside". It is agent text that #5746 wrote today (`git log -S`: ba032f255e) and nobody ratified, so it re-words with the build and needs no Ev ruling. The one-door discipline stays a source test; no new README clause is written for it, because that would bind future work.
5. **`opposed`** calls the one pairing door (`geom_brep::classify_material_pairing_as`), levered by the faces' reach. This is `at-rest-opposed-decides-a-unitless-dot-as-a-length`, which rides stage 5 C.
6. **Curved faces** are one arm in the metric door: point-to-carrier distance for the sphere and the cylinder, which closes the census's curved-face touch refusal and `Gap`'s together. That is `gap-reads-no-curved-cell-below-a-face`, re-cut and parked on this row. Cone, torus, NURBS and a curved inner edge stay refused there.

**Taken as rows, not in this build:**
- `blend-reads-its-chain-convexity-off-the-link`: blend's `fillet3_convexity_sign` is the same triple-product predicate.
- `the-booleans-cone-reader-is-a-second-link`: `boolean::sectors` `cone_side`/`wedge_classes` is kept in step with the census's reader by `cone_fuzz`. That is the boolean's ground.

**The over-refusal to pin while building:** `Link::reads` refuses a point in band of a sector's plane even where the point lies decidedly outside that sector. Add a row, and sharpen it only if a `Gap` row needs it.

Build order: unit 1–2–4 as one PR (this row, cost H); then the curved arm; blend and the boolean on their owners' slates.

## Closed

Items 1, 2 and 4 of the decision, as one PR on `atrest/topo-link-one-material-reader`. Item 3 holds: the face arms are unchanged.

- **`topo::link`** (`crates/topo/src/link.rs`) holds the census's `Link`, `Sector`, `face_piece`, the `metric` door, `rest::judge` and `TouchUnread`, moved without a change of arithmetic. The census keeps its snapshot builders (`Link::touch_at_point`, `touch_on_edge`, `touch_in_face`) and gathers each face for the shared `Sector::of_face`. The census rows and the `perf12_census_*` goldens are unchanged at ε 1e-6, 1e-9 and 1e-12.
- **Builders off a live body**: `Link::at_point`, `Link::on_edge` and `Link::in_face`.
- **`Link::offset`** is a point's signed distance from the cone's boundary, negative where `Link::reads` says In. It refuses `InBand` only where the parity reading is in band and the point stands off every sector. **`Link::lowest_on`** is its lowest along a segment, over a closed candidate set (`link::tests`).
- **`Gap`'s vertex and edge arms** read the link: an edge reads a vertex in either seat, and of two edges or two vertices the outer reads the inner. `Wedge`, `Corner`, `CornerOf`, `WedgeOf`, `Turn`, `Fold`, `gap_edge_convexity`, `gap_wedge_crossing` and the saddle and near-flat-edge refusals are gone.
- **The doors row** is `link::tests::the_link_decides_only_through_its_doors`. It reads the whole module and the census's touch section.
- **C5's cell sentence** (`crates/topo/README.md`) reads the reading cell's link.

The over-refusal the decision asked to pin sank most edge-pair `Gap` readings at random poses (the dual review's probes), so `Link::offset` reads past it; the census's reading keeps it, pinned by `link::tests::a_point_in_a_planes_sliver_outside_its_sector_reads_its_offset` and filed as `the-census-reads-its-link-without-the-wide-parity`. `gap-reads-a-cells-carrier-beyond-the-cell` is narrowed to the face arms. `gap-reads-no-curved-cell-below-a-face` is un-parked with the curved arm left on it. Items 5 and 6 stay on `at-rest-opposed-decides-a-unitless-dot-as-a-length` and `gap-reads-no-curved-cell-below-a-face`. The blend and boolean rows are on BAND's and CLEAVE's slates.
