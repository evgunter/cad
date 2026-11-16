#![allow(clippy::expect_used)]

/// Which of this module's four postures a mutation door holds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Posture {
    /// Clears and re-mints before returning — over the whole body
    /// or over exactly the faces it wrote. Read out of the source
    /// (a `mint_pcurves` or `mint_pcurves_of` call in the door's
    /// own body); an entry declares it only when the re-mint is one
    /// delegation away, which a source read cannot see, or is an
    /// Euler operator's site mint ([`super::site_rows`]).
    ///
    /// **The site mint is not the subset pass, and differs from it
    /// in two places.** What it shares with the pass is that no row
    /// the operator could have staled survives: on a face whose rows
    /// were complete, what the operator creates on the loops it
    /// rewires — new images, new joints' elements — and any image
    /// or element it finds missing there is derived exactly as the
    /// pass derives it, and every other row stood the surgery
    /// untouched; on a face whose only gaps a null edge holds open,
    /// or that the operator takes the last null edge off, so is
    /// what is missing on the loops it rewires and leaves running
    /// through no null edge, and a loop still held open keeps what
    /// it had.
    /// Where the pass would MINT — a face storing no row, or one
    /// half-minted any other way — an operator's site mint leaves
    /// the face as found: the pass owns it. Where the pass would
    /// REFUSE — a loop that does not close, a branch that meets no
    /// neighbour, a certification that refuses — the site mint
    /// clears the face and the operator returns `Ok`: it runs
    /// mid-surgery, on states a later door finishes describing, and
    /// the producer's final pass is where a finished state is
    /// refused.
    Maintains,
    /// Disposes of every row whose KEY or MEANING the door
    /// changed, rather than leaving one behind: moves it onto the
    /// key that now carries what it says — the transplanted
    /// half-edge's fresh key, or the two keys a parameter split
    /// leaves where one edge was — and drops what no key can
    /// carry, including a row whose key never moved but whose
    /// chart did (the loop-re-parenting doors, the run `mef`'s
    /// chord surgery or `kef`'s unsplice moves between two faces'
    /// loops, and the surface setter, under which a face's whole
    /// row set changes chart at once). What a door in this bucket
    /// never does is return with a row that says something the
    /// body no longer holds. Where a door in this bucket moves a
    /// loop or run onto a face that was complete and the moved rows
    /// do not stand there, it either refuses, keys-only, or — its
    /// `_minting` twin — re-mints that face through the site mint
    /// ([`crate::Body::plan_moved_rows`]), so a complete
    /// destination on an analytic chart leaves complete, or storing
    /// nothing; on a spline chart the drop stands.
    /// Whether a half-edge a door mints gets a row at the mint site
    /// is the minting posture ([`super::site_rows`]), and a door in
    /// this bucket that mints half-edges says so in its note.
    Transfers,
    /// Leaves every image as it found it — a primitive, or a write
    /// the map is not keyed on — and writes on each joint it makes
    /// the element it sums from the joints it bridges (a kill) or
    /// carries across a null edge's point (`mev_null`), keys-only
    /// and exact (module docs). What this bucket rests on is
    /// the tier-3 pcurve pass, and only as far as that pass looks
    /// (module docs): it reports a face missing any of its rows,
    /// and a stored row that no longer certifies; it is silent about a complete
    /// face whose rows were stated in another chart and certify
    /// against this one, and about any face on a chart
    /// [`super::chart_mints`] refuses.
    Neither,
    /// Re-mints through the site mint ([`super::site_rows`]) every
    /// face its description moves the rows of, and keeps the rows it
    /// finds on every other face, as `Neither` does, resting on the
    /// same tier-3 pass for what its write stales in them. Those
    /// faces are the ones a null edge's halves are on where the door
    /// installs its first carrier, which no door before it could
    /// give a row; the ones a certified edge's halves are on where
    /// the door replaces its carrier, since the rows kept there
    /// state the carrier and interval the edge leaves; and, for a kill that re-describes the
    /// members it merges, every face a listed member's halves are
    /// on, null or certified, since the kill moves a certified
    /// member's end with its carrier. On such a face the site mint
    /// selects ([`super::StoredRows::remints`]) it re-mints every
    /// loop no other null edge holds open — every loop of it,
    /// whatever it missed, once no null edge is left on it. Those
    /// loops leave complete, or the face rowless where the
    /// closed-form lane cannot mint it. A certified edge's face on a
    /// spline chart is left as found, for the tier-3 pass.
    Completes,
}

/// `(door, posture, note)` — the doors that do NOT re-mint the
/// map in their own body. The reason a posture is SAFE lives once,
/// on the [`Posture`] variant; a note here says only what is
/// particular to this door.
///
/// Module-scoped rather than local to the guard so that
/// [`crate::review_m1_pr5_internal::the_two_door_tables_cover_the_same_surface`]
/// can read it. That row is the only other reader; this table
/// stays this guard's.
pub(crate) const DECLARED: &[(&str, Posture, &str)] = {
    use Posture::{Completes, Maintains, Neither, Transfers};
    &[
        // ---- Maintains, one delegation away from the re-mint. ----
        // ---- Maintains: the doors that re-mint their own staged
        // result. They carry prose here rather than in
        // `review_m1_pr5_internal::ALLOWED` because they assert
        // tier 1 through a surgery scope (`crate::surgery`) and so
        // are no longer allowlisted there; the two tables still
        // have to cover this one population between them. ----
        (
            "merge_coplanar_faces",
            Maintains,
            "calls `merge_coplanar_faces_staged`, which re-mints the staged result \
                 before it is adopted, whenever the operand carried rows, and ends with \
                 `join_edges`, which re-mints again where a join moved rows",
        ),
        (
            "replace_faces_offset",
            Maintains,
            "re-mints the clone whole-body before adopting it",
        ),
        (
            "offset_planes_together",
            Maintains,
            "re-mints the moved faces' rows on the clone (`mint_pcurves_of`) before \
                 adopting it",
        ),
        (
            "offset_charts_together",
            Maintains,
            "the axial spelling of `offset_planes_together`, with the same re-mint",
        ),
        (
            "offset_surfaces_together",
            Maintains,
            "runs `replace_faces_offset`'s body over every moving chart, which re-mints \
                 the clone whole-body before adopting it; with nothing moving it adopts a \
                 joined clone, whose join re-mints where it moved rows",
        ),
        (
            "replace_face_offset",
            Maintains,
            "the one-face spelling of `replace_faces_offset`, which re-mints the clone \
                 before adopting it",
        ),
        (
            "join_edges",
            Maintains,
            "re-mints the staged body whole where it carried rows, before the sweeping \
                 close adopts it",
        ),
        (
            "join_edges_within",
            Maintains,
            "re-mints the faces its kills' kept edges bound (`mint_pcurves_of`), after \
                 dropping the dead cells' rows, before the sweeping close",
        ),
        // ---- The pass itself. ----
        (
            "mint_pcurves",
            Maintains,
            "IS the pass: clears the map, then re-mints every row of the body it is given",
        ),
        (
            "mint_pcurves_of",
            Maintains,
            "IS the pass restricted to a face subset: clears exactly those faces' rows, \
             then re-mints exactly those faces. `Maintains` FOR A CALLER THAT KILLS NO \
             HALF-EDGE, which is the whole of its production population (the two \
             simultaneous offset doors); it cannot discharge the whole-body claim, and its \
             own docs carry why",
        ),
        // ---- Transfers: the graft's remap-and-drop, and the
        // split's restriction onto the two keys it leaves. ----
        (
            "graft_disjoint",
            Transfers,
            "the graft, through `boolean::combine` — see `graft_disjoint_all_keyed`",
        ),
        (
            "graft_disjoint_all",
            Transfers,
            "the graft — see `graft_disjoint_all_keyed`",
        ),
        (
            "graft_disjoint_all_keyed",
            Transfers,
            "remaps each row onto the transplanted half-edge's fresh key and DROPS any \
             row the graft walk did not reach, which is the staleness test itself",
        ),
        (
            "insert_void",
            Transfers,
            "the void-insertion door — see `insert_voids`, which it calls with the one \
             destination as a slice",
        ),
        (
            "insert_voids",
            Transfers,
            "the void-insertion door: reverts the cavity (`Body::revert` carries every \
             row key for key, the plane faces' rows mirrored with their frames — the \
             module docs' producer position) and grafts through `boolean::combine`, \
             which remaps the transplanted rows onto fresh keys; every producer's final \
             mint pass — the boolean's, the revolve's and `shell`'s — re-derives every \
             row of the merged body",
        ),
        (
            "split_edge",
            Transfers,
            "restricts each parent half-edge's row to the two children's sub-intervals \
             and re-certifies both before it mutates (`split_cache`), so no row it could \
             have staled survives; a `Fitted`/`General` row is the one lane it leaves as \
             found",
        ),
        // ---- Neither: the primitives. Their stale rows are what
        // the tier-3 pcurve pass exists to catch. ----
        (
            "cyl_wall_sheet_keyed",
            Neither,
            "a fixture builder composed of `Neither` operators, minting no row of its \
             own; `cyl_wall_sheet` is the door that runs the pass over what it grew",
        ),
        (
            "cyl_wall_sheet_tilted",
            Neither,
            "`cyl_wall_sheet_keyed` with its bottom rim tilted: the same `Neither` \
             operators, minting no row of its own",
        ),
        (
            "restate_surface",
            Neither,
            "a test-support door that overwrites one surface datum and leaves every row \
                 as found; a row the new datum stales is the tier-3 pass's to report",
        ),
        (
            "rotate_shell_faces",
            Neither,
            "a test-support door that rotates each shell's stored face list; no face, \
                 loop or carrier moves, so every row stands as found",
        ),
        ("mvfs", Neither, "Euler operator"),
        (
            "merge_unjoined",
            Maintains,
            "test support: calls `merge_coplanar_faces_unjoined`, which re-mints the \
                 staged result before it is adopted",
        ),
        (
            "mev_null",
            Neither,
            "Euler operator minting a NULL edge: scaffolding with no carrier, so no image \
             to derive, and the loop it joins is held open, missing the edge's two images; \
             its halves carry the identity element and the joint out of each keeps its \
             successor's, read across the edge's point. \
             The door that releases the loop mints it whole, except on a spline chart: an \
             operator rewiring it out from under the edge (the boolean's and the \
             splitting lane's joins), the edge's first description (`set_edge_curve`, or \
             a `kev_describing` that lists it), or the edge's band kill \
             (`kev_describing`, `kef_minting`, `kemr_minting`); its keys-only kill \
             refuses `KeysOnly` where the loop would miss a row; \
             tier 2 refuses a null edge at rest",
        ),
        (
            "kemr",
            Neither,
            "Euler operator: each side's closing joint takes the sum of the two elements \
             it bridges; where it kills a null edge and the loop it releases would miss a \
             row, it refuses `KeysOnly` before mutating",
        ),
        (
            "kev",
            Neither,
            "kill op: the joint it makes takes the sum of the elements it bridges; where \
             it kills a null edge and the loop it releases would miss a row, it refuses \
             `KeysOnly` before mutating",
        ),
        ("kvfs", Neither, "kill op"),
        // ---- The kills' band twins: each mints what a loop its
        // killed null edge releases misses. ----
        (
            "kev_describing",
            Completes,
            "kill op, completing every face a listed member's halves are on: a NULL \
             member's description is its first, and a CERTIFIED member's end moves with its \
             carrier, so each such face is re-minted through `set_edge_curve`'s planner as \
             the kill leaves it (the killed halves gone), every listed member's halves under \
             the curve the kill installs, a certified member's face on a spline chart left \
             as found; and `kev` with a band, minting what a loop a killed null edge \
             releases misses on every other face it is on (`Body::plan_released_rows`)",
        ),
        (
            "kemr_minting",
            Maintains,
            "`kemr` with a band: where `kemr` refuses `KeysOnly`, the two sides it leaves \
             are walked and what they miss is minted (`Body::plan_released_rows`)",
        ),
        // ---- Maintains: the half-edge-minting Euler operators,
        // whose re-mint is the site mint, not a pass call. ----
        (
            "mev",
            Maintains,
            "Euler operator: on every face that its two new halves join whose rows were \
             complete, or whose only gaps a null edge holds open, re-mints the loops it \
             rewires that run through no null edge before it mutates and keeps the rest \
             (`pcurves::site_rows`), so no row it could have staled survives; the face \
             leaves as it was found — complete, or complete but for the loops a null edge \
             holds open — or rowless where the closed-form lane cannot mint it; where the \
             pass would refuse, this clears and returns `Ok`. An unminted face, or one \
             half-minted any other way, which the pass would mint, is left as found; a \
             spline chart refuses typed on a complete face and leaves a held-open one as \
             found",
        ),
        ("mev_line", Maintains, "Euler operator (sugar over `mev`)"),
        (
            "mekr",
            Maintains,
            "Euler operator: `mev`'s site mint over the one face whose ring it merges",
        ),
        (
            "mekr_chord",
            Maintains,
            "Euler operator (sugar over `mekr`)",
        ),
        // ---- Transfers: the loop-re-parenting doors, which carry
        // a moved loop's rows onto the target face and drop them
        // when that face is on another CHART. Each is two doors:
        // the keys-only one refuses where the target was complete
        // and the moved rows do not stand on it
        // (`SiteRowRefusal::KeysOnly`, before mutating); its
        // `_minting` twin takes a band and re-mints the target
        // through the site mint there (`Body::plan_moved_rows`). ----
        (
            "kfmrh",
            Transfers,
            "Euler operator, and a loop re-parenting: `f2`'s demoted outer loop keeps its \
             rows where `f1` is on the same chart \
             (`Body::same_chart`) and the same side of it (`Body::sides_agree`) and loses them where \
             it is not (`Body::drop_rows_on_chart_change`); where they do not stand and \
             `f1`'s rows were complete on an analytic chart it refuses `KeysOnly` before \
             mutating, and a spline chart keeps the drop",
        ),
        (
            "kfmrh_minting",
            Transfers,
            "`kfmrh` with a band: where `kfmrh` refuses `KeysOnly`, `f1` is re-minted with \
             the demoted loop walked in its chart (`Body::plan_moved_rows`)",
        ),
        (
            "kfmrh_describing",
            Transfers,
            "`kfmrh_minting` with the demoted loop's re-descriptions: a re-description \
             keeps its edge's carrier and interval, so the rows `kfmrh_minting` leaves stand",
        ),
        (
            "mfkrh",
            Transfers,
            "Euler operator, and a loop re-parenting: the promoted ring keeps its rows \
             where the spec lands on the demoting face's chart \
             (`Body::same_chart`) and the same side of it (`Body::sides_agree`) and \
             loses them where it does not; where they do not stand and the demoting face's \
             rows were complete on an analytic chart it refuses `KeysOnly` before mutating, \
             and a spline chart keeps the drop",
        ),
        (
            "mfkrh_minting",
            Transfers,
            "`mfkrh` with a band: where `mfkrh` refuses `KeysOnly`, the new face is minted \
             with the ring walked in its chart (`Body::plan_moved_rows`)",
        ),
        (
            "mfkrh_plug",
            Transfers,
            "`mfkrh` with a PLACEHOLDER surface — see `mfkrh`; a placeholder is not a \
             described surface, so it is not the chart any row was stated in and the \
             promoted ring's rows always go, with no refusal: it is a spline chart. \
             Decided by kind, not by the fresh key the sugar happens to mint",
        ),
        (
            "ring_move",
            Transfers,
            "ring surgery: re-parents a ring, mints no half-edge, carries or drops the \
             ring's rows by whether the two faces are on one chart, and refuses `KeysOnly` \
             where they do not stand on a complete target — see `kfmrh`",
        ),
        (
            "ring_move_minting",
            Transfers,
            "`ring_move` with a band: where `ring_move` refuses `KeysOnly`, the target is \
             re-minted with the ring walked in its chart — see `kfmrh_minting`",
        ),
        // ---- Transfers: the two doors that move a RUN of
        // half-edges between two faces' loops, and dispose of the
        // run's rows the way the loop doors dispose of a loop's. ----
        (
            "mef",
            Transfers,
            "Euler operator, and a run re-parenting: the run `[he1 .. he2)` it moves onto \
             the new face keeps its rows where that face is on the old face's chart \
 \
             (`Body::same_chart`) and the same side of it (`Body::sides_agree`) and loses them where it is not (`Body::drop_rows`). The \
             two halves it mints get their rows at the site, as `mev`'s do: the old face \
             is re-minted when the site mint selects it, and so is the new face, its run \
             walked in the new face's chart; a spline chart other than the old face's \
             keeps the drop",
        ),
        ("mef_chord", Transfers, "Euler operator (sugar over `mef`)"),
        (
            "kef",
            Transfers,
            "kill op, and a run re-parenting: the dying loop's remnant keeps its rows \
             where the surviving face is on the dying face's chart \
             (`Body::same_chart`) and the same side of it (`Body::sides_agree`) and \
             loses them where it is not (`Body::drop_rows`); where they do not stand and the \
             surviving face's rows were complete on an analytic chart it refuses `KeysOnly` \
             before mutating, and a spline chart keeps the drop; where they stand, each \
             joint it makes takes the sum of the two elements it bridges, and where it \
             kills a null edge and the loop it releases would miss a row it refuses \
             `KeysOnly` too; the two killed halves' rows outlive their keys as every kill \
             op's do",
        ),
        (
            "kef_minting",
            Transfers,
            "`kef` with a band: where `kef` refuses `KeysOnly`, the surviving loop is \
             re-minted in the surviving face's chart (`Body::plan_moved_rows`), or, where \
             the remnant's rows stand, what the loop a killed null edge releases misses is \
             minted (`Body::plan_released_rows`)",
        ),
        (
            "kef_describing",
            Transfers,
            "`kef_minting` with the moved edges' re-descriptions: a re-description keeps \
             its edge's carrier and interval, so the rows `kef_minting` leaves stand",
        ),
        (
            "movefac",
            Neither,
            "re-parents faces between shells; no half-edge key changes meaning",
        ),
        (
            "move_shells_to_new_solid",
            Neither,
            "re-parents shells between solids; no half-edge key changes meaning",
        ),
        // ---- Neither: the caller's own row-level control of the
        // map, and writes the map is not keyed on. ----
        (
            "attach_pcurve",
            Neither,
            "writes ONE row the caller chose; every other row is untouched, and \
             certifying this one is the caller's",
        ),
        ("detach_pcurve", Neither, "drops ONE row the caller chose"),
        (
            "attach_joint",
            Neither,
            "writes ONE joint element the caller chose; tier 3 re-decides it",
        ),
        (
            "detach_joint",
            Neither,
            "drops ONE joint element the caller chose",
        ),
        (
            "set_face_surface",
            Transfers,
            "re-charts a face in place, which changes what every row the face stores is \
             ABOUT while changing no key: the rows are kept across a swap onto the same \
             chart (`Body::same_chart`) and dropped on any other (`Body::drop_rows`, over the loops \
             `Body::half_edges_proven_whole` proves). \
             Content staleness alone would be \
             the tier-3 pass's, but only where the NEW surface mints — a swap onto a plane \
             or a placeholder left a COMPLETE row set stated in the chart the face left, \
             which that pass skips entirely",
        ),
        (
            "set_face_surface_unvouched_for_tests",
            Transfers,
            "the failure-injection twin of `set_face_surface`, whose rows it keeps and \
             drops on the same terms",
        ),
        (
            "lifting_rechart_refusals_for_tests",
            Neither,
            "a failure-injection scope that writes nothing itself: every door its \
             closure calls holds its own posture",
        ),
        (
            "set_face_surfaces_describing",
            Transfers,
            "`set_face_surface`'s swap per face, on its terms: a face's rows are kept \
             across a move onto the same chart and dropped on any other. The edges it \
             re-describes are the listed certified ones, whose rows it keeps on a face that \
             keeps its chart, for the tier-3 pass to re-certify against the new carrier \
             (`work/topo/set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart`)",
        ),
        (
            "one_record_across",
            Transfers,
            "`set_face_surfaces_describing`'s move of a smooth join's moved faces onto the \
             kept record (`one_record`), on its terms: the moved faces' rows are dropped where \
             the two charts differ, for the caller's re-mint",
        ),
        (
            "set_edge_curve",
            Completes,
            "a carrier swap is NOT the surface setter's case: neither the row's key nor \
             its chart moves, and pass 2 re-derives each row's agreement from the edge's \
             current carrier. Every description of a CERTIFIED edge re-mints the faces its \
             halves are on, since its rows are stated over the carrier and interval it \
             replaces; one that restates both re-derives the rows it found. A NULL edge's first description is where its halves' rows can first be \
             derived. On a face the halves are on whose only gaps a null edge holds open, \
             every loop no other null edge holds open is re-minted before the door mutates, \
             and on one no null edge is left on, every loop is. A HALF-MINTED face, and a \
             certified edge's face on a spline chart, keep the rows they store, for the \
             pass to re-certify",
        ),
        (
            "describe_at_rest",
            Completes,
            "`set_edge_curve` with the edge's own carrier and interval put back \
                 verbatim, which re-mints the faces the edge's halves are on as any \
                 certified description does",
        ),
        (
            "restate_stored_edge",
            Completes,
            "`describe_at_rest`'s write: `set_edge_curve` with the edge's own carrier and \
                 interval put back verbatim",
        ),
        (
            "set_face_sense",
            Transfers,
            "writes the sense bit; where the flip changes the side of a chart with a \
                 singular set (`Body::sides_agree`) the face's rows state its turns on the \
                 other side, and are dropped for a band door to mint again; on any other \
                 chart they stand",
        ),
        (
            "begin_surgery",
            Neither,
            "opens a debug-only surgery scope: no arena key, no pcurve row",
        ),
        ("set_null_face_pair", Neither, "null-face annotation"),
        ("clear_null_face_pair", Neither, "removes that annotation"),
        // ---- Neither: the test-support fixture builders. Why
        // they are in this walk's population is stated once, on
        // [`crate::source_walk::mutation_doors`]. ----
        (
            "prism_ops",
            Neither,
            "grows a prism with `mvfs`/`mev`/`mef` and `set_face_surface`, every one of \
                 them already sorted above; it attaches no pcurve of its own",
        ),
        (
            "describe_as_intersections",
            Completes,
            "`set_edge_curve` per transverse edge, on that entry's terms",
        ),
        (
            "cube_into",
            Neither,
            "`prism_ops` at the unit square then `describe_as_intersections`",
        ),
        (
            "plant_ring_face",
            Neither,
            "`mev_line`, `kemr` and `mef_chord`, every one of them already sorted above",
        ),
        (
            "plant_disc_face",
            Neither,
            "`mev_line`, `kemr` and `mef`, every one of them already sorted above",
        ),
        (
            "drill_hole",
            Neither,
            "`plant_ring_face`, then `mev_line`, `mef_chord` and `kfmrh`, every one of \
                 them already sorted above",
        ),
        (
            "plane_every_face",
            Neither,
            "`set_face_surface` per face, on that entry's terms",
        ),
    ]
};

/// **The convention at the top of this module, checked rather than
/// surveyed.** Every public mutation path into a [`crate::Body`] —
/// `pub fn` taking `&mut self`, plus the free functions taking
/// `&mut Body<T>` — either re-mints the map in its own body, which
/// this walk reads directly, or is declared below with its posture
/// and a note on that door.
///
/// **Why a test and not a list in prose.** A prose index has no way
/// to notice a door being added, and the previous one did not. This
/// goes red the day one lands unsorted, which is the rot the prose
/// could only describe.
///
/// **What it checks, exactly.** Three failures, all mechanical: a
/// door that neither calls the pass — in either spelling,
/// `mint_pcurves` or `mint_pcurves_of` — nor appears below; a door
/// whose entry says anything but `Maintains` while its body calls
/// it; and an entry naming a door that no longer exists.
///
/// **Where the door set comes from, and what it cannot see:**
/// [`crate::source_walk::mutation_doors`], shared with the tier-1
/// postcondition guard in [`crate::review_m1_pr5_internal`], which
/// walks the same population to ask a different question. That
/// function's docs carry the reason the two tables do not merge
/// and the whole inherited blind-spot list; this guard does not
/// restate either.
///
/// **"Calls `mint_pcurves`" is a read of code, not of prose.** The
/// body arrives with comments and literals blanked. This guard
/// used a raw `body.contains`, and a planted door whose body only
/// *mentioned* `mint_pcurves(` in a comment was counted as
/// re-minting, in both this guard and the tier-1 one, both green.
///
/// **What it does not check.** That a `Maintains` entry is TRUE. A
/// re-mint reached through a delegate, and an Euler operator's
/// site mint, are invisible to a source read, so every `Maintains`
/// entry in `DECLARED` is taken at its word — the guard
/// establishes that every door is sorted and that no door has
/// silently started minting, not that each sort is correct.
///
/// **Nor which SPELLING of the pass a door calls, nor what it
/// passes.** A `mint_pcurves_of(` call reads as `Maintains` here
/// whatever face list it is handed — including an empty one — and
/// the subset pass's guarantee is strictly weaker than the
/// whole-body pass's: it cannot reach a row whose half-edge is
/// dead ([`mint_pcurves_of`]'s docs). So a door that KILLS a
/// half-edge and closes with the subset pass is classified
/// `Maintains` by this walk and is not. Nothing in this crate does
/// today, and a source read has no way to tell; what covers it is
/// the entry's own text and the SHELL-10 probe rows in `sweep`. The
/// module docs' *"what the guard does NOT establish"* list carries
/// this and the rest of the blind spot: delegation, and everything
/// outside `topo/src`'s `&mut Body` surface. The full inherited
/// list is on [`crate::source_walk::mutation_doors`].
/// **The crate's own ring-promotion doors**, which the public walk
/// does not reach ([`crate::source_walk::mutation_doors`] reads the
/// `Construct` surface): `(door, posture, note)` for every function
/// of `euler_kill.rs` that enters `mfkrh_with` beside the public
/// `mfkrh`, `mfkrh_minting` and `mfkrh_plug`.
const PIECE_DOORS: [(&str, Posture, &str); 2] = [
    (
        "mfkrh_piece",
        Posture::Transfers,
        "a ring promoted as its face's own region, keys only: same chart and same \
             side (`Body::sides_agree`), so the ring's rows stand and nothing is decided",
    ),
    (
        "mfkrh_piece_minting",
        Posture::Transfers,
        "`mfkrh_piece` with a band: a ring that stores no rows yet is minted in its \
             chart (`Body::plan_moved_rows`)",
    ),
];

/// Every function that enters the ring promotion is the public
/// `mfkrh` pair or a declared piece door: a new entry declares its
/// posture here.
#[test]
fn every_ring_promotion_door_declares_its_posture() {
    let source = include_str!("../euler_kill.rs");
    let mut current = "";
    let mut entering: Vec<&str> = Vec::new();
    for line in source.lines() {
        let code = line.trim_start();
        if code.starts_with("//") {
            continue;
        }
        if let Some(rest) = code
            .strip_prefix("pub(crate) fn ")
            .or_else(|| code.strip_prefix("fn "))
        {
            current = rest.split(['(', '<']).next().unwrap_or(rest);
        }
        if (code.contains(".mfkrh_with(") || code.contains(".mfkrh_piece_with("))
            && !entering.contains(&current)
        {
            entering.push(current);
        }
    }
    let mut declared: Vec<&str> = vec!["mfkrh", "mfkrh_minting", "mfkrh_plug"];
    declared.extend(PIECE_DOORS.iter().map(|(name, ..)| *name));
    declared.push("mfkrh_piece_with");
    entering.sort_unstable();
    declared.sort_unstable();
    assert_eq!(
        entering, declared,
        "the functions of `euler_kill.rs` that enter the ring promotion are not the \
             public pair and the declared piece doors: declare the new door's posture in \
             `PIECE_DOORS`, or strike the one that went"
    );
}

#[test]
fn every_mutation_door_declares_its_pcurve_posture() {
    use Posture::Maintains;
    let mut minting: Vec<String> = Vec::new();
    let mut declared: Vec<&str> = Vec::new();
    let mut undeclared: Vec<String> = Vec::new();
    let mut mislabelled: Vec<String> = Vec::new();

    for door in crate::source_walk::mutation_doors() {
        let entry = DECLARED.iter().find(|(n, _, _)| *n == door.name);
        if door.names("mint_pcurves") || door.names("mint_pcurves_of") {
            if let Some((_, posture, _)) = entry.filter(|(_, p, _)| *p != Maintains) {
                mislabelled.push(format!("{} declared {posture:?}", door.name));
            }
            minting.push(door.name);
        } else if let Some((n, _, _)) = entry {
            declared.push(n);
        } else {
            undeclared.push(door.site());
        }
    }

    assert!(
        undeclared.is_empty(),
        "public mutation path(s) that neither re-mint the pcurve map nor declare a \
             posture in this test: {undeclared:?}. Either re-mint before returning, or add \
             the door above with the posture it holds.",
    );
    assert!(
        mislabelled.is_empty(),
        "door(s) whose body now calls `mint_pcurves` but whose entry says otherwise: \
             {mislabelled:?}. This is the rot the prose index suffered — move the entry to \
             `Maintains`, or drop it and let the walk classify the door.",
    );
    // The entries rot in the other direction too.
    for (name, _, _) in DECLARED {
        assert!(
            declared.contains(name) || minting.iter().any(|n| n == name),
            "this test declares a posture for `{name}`, which is no longer a public \
                 mutation path — it was renamed or deleted. Drop the entry.",
        );
    }
    // **Over-stripping is SILENT here**, which is why this pin is
    // by name rather than a count. A door that stops reading as
    // calling `mint_pcurves` does not red — it falls into the
    // `else if let Some(entry)` arm and is accepted as declared.
    // Only a door with no entry at all reds, and today exactly one
    // door is classified by its body, so a lexing gap that erased
    // the needle everywhere would leave this guard green over a
    // surface it had stopped reading. The walk's own floor is
    // upstream on `mutation_doors`; this is the needle's.
    //
    // **It is exactly one door wide, and that is the whole of it.**
    // It does not cover the `DECLARED` doors: those land in the
    // same `else if` arm whether or not their needle survives, so a
    // declared non-`Maintains` door that STARTS minting while its
    // call is over-stripped would not reach `mislabelled`. Closing
    // that needs a second oracle for "does this body call it",
    // which a source read does not have.
    assert!(
        minting.iter().any(|n| n == "join_edges"),
        "`join_edges` no longer reads as calling `mint_pcurves`. Either the door \
             stopped re-minting the rows its kills moved — a finding, and its entry \
             belongs above — or the source read lost the call.",
    );
    println!(
        "[pcurve posture] {} door(s): {} re-mint, {} declared",
        declared.len() + minting.len(),
        minting.len(),
        declared.len(),
    );
}
