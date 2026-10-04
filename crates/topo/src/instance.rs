//! **The disjoint-graft door** — bringing another body's solids into
//! this body as SEPARATE solids: one solid ([`graft_disjoint`]) or all
//! N of them ([`graft_disjoint_all`], the same transplant run once per
//! source solid — a multi-solid source is an assembly's instantiated
//! part, not a caller mistake).
//!
//! The boolean pipeline's [`combine`](crate::boolean) door transplants
//! a source body's solid into an EXISTING destination solid, so the two
//! operands' shells end up bounding one volume: that is fusion, and it
//! is the only cross-body operation the Euler layer's `CrossSolid`
//! refusal sanctions because a fused result needs the seam zip
//! afterwards to be a body at all.
//!
//! This door is the other half of the same primitive and asserts
//! strictly less: the source's shells arrive under a **fresh solid of
//! their own**, so nothing is fused, no seam is implied, and the result
//! is the disjoint union of two bodies' contents in one arena. The
//! transplant itself is `combine`'s, called verbatim — same fresh keys
//! in deterministic slot order (D9), same forwarded provenance, same
//! `GeomSource` and pcurve-cache carry, same description surface-key
//! remap. Two differences, both forced by what a DISJOINT graft is:
//! the destination is an empty solid the graft mints instead of one
//! already holding shells, and the description bridge carries the
//! source's certificate with the handles rewritten rather than
//! re-running the schedule (`combine::Bridge::RemapKeys`). Nothing was
//! operated on between the two bodies, so there is nothing new to
//! certify — and re-running would REFUSE descriptions the lanes cannot
//! express at all, such as a rational NURBS wall's, which a body that
//! imported cleanly may hold.
//!
//! **Who needs it.** A STEP assembly states N placed INSTANCES of M
//! component representations. Each instance's frame is its own rigid
//! map, and [`transform_rigid`](crate::transform_rigid) — the kernel's
//! placement door, which re-checks rigidity with decided predicates and
//! re-certifies every carrier — maps a WHOLE body. So an instance is
//! materialized by building its component alone, mapping that lone body
//! through the kernel door, and grafting the placed result in. The body
//! that ships is then, entity for entity, the union of the bodies that
//! were individually certified and gated — not a re-derivation of them.
//!
//! Validity is the caller's to establish, exactly as with `combine`:
//! this is a raw transplant, and the at-rest validator
//! ([`validate_geometric`](crate::validate_geometric)) is what says
//! whether the result is a body. Know what that gate proves: the
//! structural tiers, then tier 3's LOCAL battery — every check reads
//! one face, one edge, or one edge–face pair, and the +V signed
//! volume is read per solid, on that solid's own faces. Two grafted
//! solids share no edge, so no
//! tier-3 check ever compares one against the other: solids that
//! OVERLAP or TOUCH pass `validate_geometric` undetected. The gate
//! with cross-solid reach is the tier-3′ form
//! ([`validate_pseudomanifold`](crate::validate_pseudomanifold)) —
//! the census growth issue #382 planned, landed in M9-2 — and its
//! reach is stated exactly (the census module docs carry the full
//! class-by-class envelope): an overlap or touch that leaves
//! vertex/line/planar boundary evidence surfaces as the
//! undeclared-contact hard error naming the guilty pair (a proper
//! pierce is categorically undeclarable) and certifies where
//! declared; cross-solid proximity with a curved side (against a
//! curved OR planar partner, F5) REFUSES as `CensusUndecidable` — the
//! conservative loudness backstop for the class no arm can examine yet
//! (the C9-ring conformal-rest / partial-embedding class); one
//! instance's extents nested inside another's go to the material
//! test, which clears a part sitting in a concavity and refuses an
//! embedded one typed (`InstanceInterference` — recorded gate-skips,
//! the declaration that would admit it, do not exist yet). A pair of
//! PLANAR faces is left
//! to the sweeps only when both are bounded entirely by line edges,
//! which is what puts a whole boundary in front of them — so an
//! arc-bounded planar face (a cylinder's cap) is backstopped like a
//! curved one, and nothing in the inter-instance touching/overlap
//! space validates silently; a caller assembling instances at rest
//! runs THAT gate with its declaration records.

use crate::body::Body;
use crate::boolean::BooleanError;
use crate::entity::SolidKey;

/// Grafts `src`'s single solid into `dst` as a NEW solid, returning its
/// key (module docs).
///
/// `src` must be a single-solid body — a source holding N solids is a
/// caller error at THIS door, whose one returned key could only name
/// one of them; [`graft_disjoint_all`] is the N-solid door. Its shells
/// arrive whole, in source order, under the minted solid. The minted
/// solid's provenance
/// is `src`'s own solid provenance, forwarded into `dst`'s keys like
/// every other record the graft carries (a graft is not a re-birth).
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] when `src` does not hold exactly one
/// solid; [`graft_disjoint_all`]'s refusals otherwise, with `dst`
/// deep-unchanged.
///
/// # Panics
///
/// As [`graft_disjoint_all`].
pub fn graft_disjoint<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
) -> Result<SolidKey, BooleanError> {
    if src.solids().count() != 1 {
        return Err(BooleanError::JoinDesync {
            what: "graft source does not hold exactly one solid",
        });
    }
    let keys = graft_disjoint_all(dst, src)?;
    let [key] = keys[..] else {
        unreachable!(
            "a committed graft mints one solid per source solid, and the source was \
             checked to hold exactly one; it minted {} (kernel bug)",
            keys.len()
        )
    };
    Ok(key)
}

/// Grafts EVERY solid of `src` into `dst`, each as a new solid,
/// returning their keys in the source's solid order (module docs).
///
/// The N-solid door. A source holding N solids arrives as N solids of
/// `dst`, each carrying its own source solid's provenance and its own
/// shells in source order — entity for entity what N sequential
/// [`graft_disjoint`] calls over the source's solids in slot order (D9)
/// would have built. Entity for entity, not key for key: the keys
/// differ in slot version, because each graft mints its own
/// dead-on-arrival keys for the dead ancestors its records name
/// (`combine`'s module docs), and N calls mint where this one call
/// shares. Which source solid a grafted face came
/// from stays derivable exactly as it was before the graft: from the
/// solid it now sits under, and from the `GeomSource`/provenance
/// records the transplant carries.
///
/// Sharing is impossible here for the same reason it is at the single
/// door: every transplanted entity is re-created under a FRESH key, so
/// two grafts of one source produce two disjoint key ranges. Validity
/// remains the caller's to establish — this is a raw transplant, and a
/// multi-solid source's solids are gated by the same at-rest validator
/// as any other body's.
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] when `src` holds no solid. **That is
/// the only refusal at this door**: it bridges with
/// `combine::Bridge::RemapKeys`, whose arm carries each certificate
/// verbatim and never reaches the re-certification that is the only
/// site raising `GraftRecertify`. Only the in-crate `Bridge::Recertify`
/// path (the booleans') can.
///
/// # Panics
///
/// Where a record of `src` names something `src` does not hold, or a
/// solid of it carries no provenance, naming it (D2 row 4): `src` is a
/// body every public door keeps tier-1-valid.
///
/// **Every refusal and every such panic leaves `dst` deep-unchanged.**
/// The transplant runs into a fresh staging body and is committed into
/// `dst` only once it has succeeded in full, so a caller keeps the
/// destination it had and no refusal leaves a body tier-1-invalid.
pub fn graft_disjoint_all<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
) -> Result<Vec<SolidKey>, BooleanError> {
    Ok(graft_disjoint_all_keyed(dst, src)?.solids)
}

/// **Whether an aggregate of `aggregate_solids` solids owes its parts
/// the at-rest gate one by one** (F8/D7) — the one statement of that
/// policy. A caller that gates each part BEFORE the aggregate asks here
/// rather than spelling the threshold itself; a caller that gates only
/// the aggregate and re-gates parts to attribute a refusal
/// (`editor_core::product_recorded`) does not ask, because it owes no
/// part a gate on the success path.
///
/// A caller that asks gates for two subjects: each part on its own
/// body, so a refusal names the part it is about and arrives before the
/// part is grafted, and then the aggregate. `docs/DESIGN.md`
/// import step 4 states why each part is asked: whole-body sums letting
/// an inside-out part cancel against its neighbour. Whether that still
/// holds now that check 7 reads each solid's sign on its own faces
/// ([`crate::validate_geometric`]) is an open question
/// (`work/exch/the-per-instance-tier-3-gate-reads-every-assembly-face-twice.md`).
/// This function decides only WHEN the parts are asked.
///
/// **With one solid the part and the aggregate are the same body**, so
/// the per-part call would re-run the aggregate call on identical
/// geometry. It is skipped as an IDENTITY, never as an exemption: the
/// aggregate gate still runs, on that same solid.
///
/// **The count is over the aggregate's SOLIDS.** Its one caller,
/// `step_import::import_step`, gates each placed instance, which is one
/// solid, with tier 3; its aggregate gate is tier 3′, the
/// declared-contact census, which is where the cross-part structure is
/// checked. Its instance count IS its solid count, and it says why at
/// the call. A caller that counts something else owes the reason its
/// count IS the solid count, at the call.
///
/// That caller is held to consulting this function by a source-reading
/// guard (`step-import`'s `tests/per_part_gate_policy.rs`); a new caller
/// is held to it by convention only.
#[must_use]
pub const fn per_part_gate_owed(aggregate_solids: usize) -> bool {
    aggregate_solids > 1
}

/// The source → destination key correspondence a graft established
/// (ASM-2A D-4): which entity of `dst` each entity of `src` became.
///
/// A graft re-creates every transplanted entity under a FRESH key, so
/// a caller holding per-entity data keyed by the SOURCE's arena — a
/// name table above all — has no way to re-key it without this bridge.
/// Solid keys ride in source solid order; the per-entity maps are total
/// over the source's live faces, edges and vertices.
///
/// The fields stay private: the internal bridge is a slotmap
/// `SecondaryMap`, and this door's contract is the LOOKUP, not the
/// container.
#[derive(Debug)]
pub struct GraftKeys {
    solids: Vec<SolidKey>,
    map: crate::boolean::combine::GraftMap,
}

impl GraftKeys {
    /// The grafted solids' destination keys, in the source's solid
    /// order.
    pub fn solids(&self) -> &[SolidKey] {
        &self.solids
    }

    /// The destination face a source face became.
    pub fn face(&self, src: crate::entity::FaceKey) -> Option<crate::entity::FaceKey> {
        self.map.faces.get(src).copied()
    }

    /// The destination edge a source edge became.
    pub fn edge(&self, src: crate::entity::EdgeKey) -> Option<crate::entity::EdgeKey> {
        self.map.edges.get(src).copied()
    }

    /// The destination vertex a source vertex became.
    pub fn vertex(&self, src: crate::entity::VertexKey) -> Option<crate::entity::VertexKey> {
        self.map.vertices.get(src).copied()
    }
}

/// [`graft_disjoint_all`] plus the source → destination key bridge
/// (ASM-2A D-4): identical transplant, and the correspondence a caller
/// needs to carry per-entity data (stable names) across the graft.
///
/// # Errors
///
/// Exactly [`graft_disjoint_all`]'s, with `dst` deep-unchanged on
/// every one. The returned keys name entities of `dst` as committed.
pub fn graft_disjoint_all_keyed<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
) -> Result<GraftKeys, BooleanError> {
    if src.solids().next().is_none() {
        return Err(BooleanError::JoinDesync {
            what: "graft source holds no solid to graft",
        });
    }
    #[cfg(debug_assertions)]
    let before = dst.arena_counts();
    let (map, targets) = crate::boolean::combine::graft_solids_minted(
        dst,
        src,
        crate::boolean::combine::Bridge::RemapKeys,
    )?;
    #[cfg(debug_assertions)]
    dst.assert_euler_postcondition(before, graft_delta(src), "graft_disjoint_all_keyed");
    Ok(GraftKeys {
        solids: targets,
        map,
    })
}

/// A graft's arena shift: one fresh entity per live source entity, a
/// minted solid per source solid included (the dead-on-arrival keys it
/// mints for dead record payloads are removed at once).
#[cfg(debug_assertions)]
fn graft_delta<T: geom_core::Decide>(src: &Body<T>) -> crate::euler::ArenaDelta {
    let c = src.arena_counts();
    let n = |x: usize| isize::try_from(x).unwrap_or(isize::MAX);
    crate::euler::ArenaDelta {
        solids: n(c.solids),
        shells: n(c.shells),
        faces: n(c.faces),
        loops: n(c.loops),
        half_edges: n(c.half_edges),
        edges: n(c.edges),
        vertices: n(c.vertices),
    }
}

/// Direct rows for this door (R1 MINOR-2): the integration coverage
/// lives in `step-import`, but the contract this module states —
/// "nothing is fused, nothing is shared, nothing but the keys may
/// differ" — is a kernel claim and is checked here.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_brep::EdgeCurveSpec;
    use geom_core::Point3;
    use geom_core::Tol;

    use crate::body::Body;
    use crate::entity::EdgeKey;
    use crate::fixtures::deep_snapshot;
    use crate::instance::{graft_disjoint, graft_disjoint_all_keyed};
    use crate::test_support_fixtures::declined_cube;

    fn cube() -> Body<f64> {
        declined_cube::<f64>(Tol::witness()).body
    }

    /// **Disjointness.** The graft adds a SECOND solid; every arena
    /// grows by exactly the source's contents, every shell points at
    /// its own solid, and the two solids share not one face.
    #[test]
    fn a_graft_adds_a_whole_second_solid_and_shares_nothing() {
        let src = cube();
        let mut dst = cube();
        let before = (
            dst.solids().count(),
            dst.shells().count(),
            dst.faces().count(),
            dst.edges().count(),
            dst.vertices().count(),
            dst.points().count(),
            dst.surfaces().count(),
        );
        let key = graft_disjoint(&mut dst, &src).expect("a single-solid graft");
        assert_eq!(dst.solids().count(), before.0 + 1, "one more solid");
        assert_eq!(dst.shells().count(), before.1 + src.shells().count());
        assert_eq!(dst.faces().count(), before.2 + src.faces().count());
        assert_eq!(dst.edges().count(), before.3 + src.edges().count());
        assert_eq!(dst.vertices().count(), before.4 + src.vertices().count());
        assert_eq!(dst.points().count(), before.5 + src.points().count());
        assert_eq!(dst.surfaces().count(), before.6 + src.surfaces().count());

        // Every shell's back-pointer names the solid that lists it, and
        // no face is claimed by two solids.
        let mut seen = std::collections::BTreeSet::new();
        for (sk, solid) in dst.solids() {
            for &sh in &solid.shells {
                assert_eq!(dst.get_shell(sh).unwrap().solid, sk, "shell back-pointer");
                for &f in &dst.get_shell(sh).unwrap().faces {
                    assert!(seen.insert(f), "a face in two solids is fusion");
                }
            }
        }
        // The grafted solid is the one the call named, and it holds
        // exactly the source's shells — arrived whole, no surgery.
        assert_eq!(
            dst.shells_of_solid(key).unwrap().len(),
            src.solids().next().unwrap().1.shells.len()
        );
        // And the union is a body: the disjoint pair validates.
        assert_eq!(crate::validate(&dst), Ok(()), "tier 1 on the union");
        assert_eq!(crate::validate_closed(&dst), Ok(()), "tier 2 on the union");
    }

    /// **Key remapping is a bijection onto FRESH keys.** No key of the
    /// destination's original solid is reused by the graft, and the
    /// grafted copy's geometry has the same VALUES under different
    /// handles — which is the whole content of "body-lineage-scoped".
    #[test]
    fn the_graft_mints_fresh_keys_for_every_transplanted_entity() {
        let src = cube();
        let mut dst = cube();
        let original: std::collections::BTreeSet<_> = dst.faces().map(|(k, _)| k).collect();
        let key = graft_disjoint(&mut dst, &src).expect("a graft");

        let grafted = dst.faces_of_solid(key).expect("the grafted solid");
        assert_eq!(grafted.len(), src.faces().count(), "every face arrived");
        for f in &grafted {
            assert!(!original.contains(f), "a transplanted face reused a key");
        }
        // Distinct surface keys, equal surface values: the copy is a
        // copy, not a share.
        let src_surfaces: Vec<_> = src.surfaces().map(|(_, s)| format!("{s:?}")).collect();
        let new_surfaces: Vec<_> = grafted
            .iter()
            .map(|&f| {
                let k = dst.get_face(f).unwrap().surface;
                (k, format!("{:?}", dst.get_surface(k).unwrap()))
            })
            .collect();
        for (k, s) in &new_surfaces {
            assert!(src_surfaces.contains(s), "the surface value travelled");
            assert!(
                dst.faces()
                    .filter(|(f, _)| !grafted.contains(f))
                    .all(|(_, face)| face.surface != *k),
                "a transplanted surface key collided with the destination's"
            );
        }
    }

    /// Sources torn so the transplant meets the tear only after it has
    /// begun writing its stage: a half-edge whose `next` dangles, met in
    /// the cross-reference pass after every arena has been copied; a
    /// solid listing a dead shell, met at the shell attachment, the
    /// transplant's last step, after the minted solids exist. A public
    /// consumer cannot build either (every public door keeps tier 1, and
    /// tier 1 vouches for both references), so the tear takes
    /// `pub(crate)` reach: these stand for whatever a kernel bug
    /// elsewhere leaves. Each comes with the record its panic names.
    fn torn_sources() -> [(&'static str, Body<f64>, String); 2] {
        use crate::entity::{EntityId, HalfEdgeKey, ShellKey};
        let mut next = cube();
        let he = next.half_edges.iter().next().unwrap().0;
        next.get_half_edge_mut(he).unwrap().next = HalfEdgeKey::default();
        let mut shell = cube();
        let solid = shell.solids().next().unwrap().0;
        shell
            .get_solid_mut(solid)
            .unwrap()
            .shells
            .push(ShellKey::default());
        [
            (
                "a dangling `next` (cross-reference pass)",
                next,
                format!(
                    "{}'s next names {}",
                    EntityId::HalfEdge(he),
                    EntityId::HalfEdge(HalfEdgeKey::default())
                ),
            ),
            (
                "a dead shell in the solid's list (attachment)",
                shell,
                format!(
                    "{}'s shells names {}",
                    EntityId::Solid(solid),
                    EntityId::Shell(ShellKey::default())
                ),
            ),
        ]
    }

    /// **A torn source panics naming the record, at every graft door,
    /// and the destination is deep-unchanged** — every row, every
    /// provenance record, every arena's next key — and so still tier-1
    /// valid. The panic fires inside the stage, after the transplant has
    /// begun writing it; a destination written before it would differ
    /// from its snapshot and fail tier 1 (an empty minted solid is
    /// `SolidWithoutShells`).
    #[test]
    fn a_torn_source_panics_with_the_destination_deep_unchanged() {
        type Door = fn(&mut Body<f64>, &Body<f64>) -> Result<(), crate::boolean::BooleanError>;
        let doors: [(&str, Door); 3] = [
            ("graft_disjoint", |d, s| graft_disjoint(d, s).map(drop)),
            ("graft_disjoint_all", |d, s| {
                crate::instance::graft_disjoint_all(d, s).map(drop)
            }),
            ("graft_disjoint_all_keyed", |d, s| {
                graft_disjoint_all_keyed(d, s).map(drop)
            }),
        ];
        for (door, graft) in doors {
            for (tear, src, names) in torn_sources() {
                let mut dst = cube();
                let before = deep_snapshot(&dst);
                let report =
                    crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
                        let _ = graft(&mut dst, &src);
                    }));
                for fragment in [names.as_str(), crate::live::NAMES_ONLY_LIVE] {
                    assert!(
                        report.contains(fragment),
                        "{door}, {tear}: want {fragment:?} in: {report}"
                    );
                }
                assert_eq!(
                    deep_snapshot(&dst),
                    before,
                    "{door}, {tear}: the panicking graft wrote the destination (tier 1 now: {:?})",
                    crate::validate(&dst),
                );
                assert_eq!(crate::validate(&dst), Ok(()), "{door}, {tear}");
            }
        }
    }

    /// **A planted collision refuses LOUD.** The door's precondition is
    /// a single-solid source; a body that is not one is `JoinDesync`,
    /// never a partial transplant.
    #[test]
    fn a_source_that_is_not_a_single_solid_refuses_typed() {
        // Empty: no solid at all.
        let mut dst = cube();
        let before = deep_snapshot(&dst);
        let err = graft_disjoint(&mut dst, &Body::<f64>::new()).expect_err("no solid to graft");
        assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
        assert_eq!(
            deep_snapshot(&dst),
            before,
            "an empty source writes nothing"
        );

        // Two solids: the graft transplants ONE, so a two-solid source
        // is a caller error, not a thing to guess at.
        let mut two = cube();
        graft_disjoint(&mut two, &cube()).expect("build a two-solid body");
        let mut dst = cube();
        let before = deep_snapshot(&dst);
        let err = graft_disjoint(&mut dst, &two).expect_err("two solids in the source");
        assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
        assert_eq!(
            deep_snapshot(&dst),
            before,
            "a two-solid source writes nothing"
        );
    }

    /// The minted solid's provenance is the SOURCE's — a graft is not
    /// a re-birth (module docs).
    #[test]
    fn the_minted_solid_carries_the_source_solids_provenance() {
        let src = cube();
        let want = {
            let (k, _) = src.solids().next().unwrap();
            format!("{:?}", src.solid_provenance.get(k).unwrap())
        };
        let mut dst = cube();
        let key = graft_disjoint(&mut dst, &src).expect("a graft");
        assert_eq!(
            format!("{:?}", dst.solid_provenance.get(key).unwrap()),
            want
        );
    }

    /// Splits `e0` three times (its first child `e1` twice) and kills
    /// the kept-key first child, so the three new pieces name a DEAD
    /// parent (`e0`'s key) or a live one: returns them.
    fn split_thrice_and_kill_the_parent(src: &mut Body<f64>, e0: EdgeKey) -> [EdgeKey; 3] {
        let tol = Tol::witness();
        let param = |b: &Body<f64>, e, f: f64| {
            let c = b.get_edge(e).unwrap().curve;
            let (t0, t1) = b.get_curve_geom(c).unwrap().certified().unwrap().params();
            t0 + f * (t1 - t0)
        };
        let e1 = src
            .split_edge(e0, param(src, e0, 0.5), tol)
            .unwrap()
            .new_edge;
        let e2 = src
            .split_edge(e1, param(src, e1, 0.5), tol)
            .unwrap()
            .new_edge;
        let e3 = src
            .split_edge(e0, param(src, e0, 0.5), tol)
            .unwrap()
            .new_edge;
        // The kill merges `e3` back over the dead child's span, so it
        // takes the describing door, with `e3` as the chord it spans.
        let he0 = src.get_edge(e0).unwrap().he_plus;
        let chords: Vec<_> = src
            .kev_merged_members(he0)
            .unwrap()
            .iter()
            .map(|m| (m.edge, EdgeCurveSpec::line_between(m.start, m.end)))
            .collect();
        assert_eq!(
            chords.iter().map(|c| c.0).collect::<Vec<_>>(),
            [e3],
            "the merge re-bases `e3` alone"
        );
        src.kev_describing(he0, &chords, tol)
            .expect("the first child dies");
        [e1, e2, e3]
    }

    /// **Staging moves no key.** A staged graft leaves `dst` exactly as
    /// a transplant straight into it would, next keys included, and
    /// returns the same bridge, onto minted and onto existing solids —
    /// on a source whose records name dead
    /// edges (so the dead-on-arrival keys are minted twice over) and a
    /// destination with freed slots for the commit to reuse.
    #[test]
    fn a_staged_graft_is_key_for_key_the_unstaged_one() {
        let mut src = cube();
        let e0 = src.edges().next().unwrap().0;
        split_thrice_and_kill_the_parent(&mut src, e0);
        let mut dst = cube();
        let d0 = dst.edges().nth(3).unwrap().0;
        split_thrice_and_kill_the_parent(&mut dst, d0);
        let mut unstaged = dst.clone();
        let staged = graft_disjoint_all_keyed(&mut dst, &src).expect("the staged graft");
        let (map, solids) = crate::boolean::combine::graft_unstaged(&mut unstaged, &[], &src)
            .expect("the unstaged graft");
        assert!(!map.dead_edges.is_empty(), "the source names a dead edge");
        assert_eq!(deep_snapshot(&dst), deep_snapshot(&unstaged), "minted");
        assert_eq!(staged.solids, solids);
        assert_eq!(format!("{:?}", staged.map), format!("{map:?}"));

        // Onto an existing solid (the void door's and the booleans' shape).
        let target = dst.solids().next().unwrap().0;
        let mut unstaged = dst.clone();
        let staged = crate::boolean::combine::graft_solids_with(
            &mut dst,
            &[target],
            &src,
            crate::boolean::combine::Bridge::RemapKeys,
        )
        .expect("the staged graft");
        let (map, _) = crate::boolean::combine::graft_unstaged(&mut unstaged, &[target], &src)
            .expect("the unstaged graft");
        assert_eq!(deep_snapshot(&dst), deep_snapshot(&unstaged), "existing");
        assert_eq!(format!("{staged:?}"), format!("{map:?}"));
    }

    /// **A grafted split lineage chases inside the destination, to the
    /// image of the root it reached in the source.** Three splits of
    /// one cube edge, then the kept-key first child killed: two children
    /// name a DEAD parent, one names a live one. Grafted into a
    /// destination that already holds a cube (so no source key can
    /// coincide with its image), every edge's root in `dst` is the image
    /// of its root in `src` — the one dead root a key that resolves
    /// nowhere in `dst`, shared by every piece that reached it.
    #[test]
    fn a_grafted_split_lineage_chases_inside_the_destination() {
        let mut src = cube();
        let e0 = src.edges().next().unwrap().0;
        let [e1, e2, e3] = split_thrice_and_kill_the_parent(&mut src, e0);
        let mut dst = cube();
        let keys = graft_disjoint_all_keyed(&mut dst, &src).expect("a graft");
        let dead_root = *keys
            .map
            .dead_edges
            .get(&e0)
            .expect("the dead parent has a row");
        assert!(
            dst.get_edge(dead_root).is_none(),
            "the dead root resolves nowhere"
        );
        for e in [e1, e2, e3] {
            assert_eq!(
                dst.split_root(keys.edge(e).unwrap(), |_| false),
                Ok(dead_root),
                "{e:?} chases to the dead parent's key in dst"
            );
        }
        for (e, _) in src.edges() {
            let root = src.split_root(e, |_| false).unwrap();
            let want = keys.edge(root).unwrap_or(dead_root);
            assert_eq!(
                dst.split_root(keys.edge(e).unwrap(), |_| false),
                Ok(want),
                "{e:?}'s root in dst is the image of its root in src"
            );
        }
    }

    /// **Distinct dead ancestors keep distinct dead keys, per lineage
    /// and per copy.** Two vertex-disjoint cube edges each split and
    /// their parent killed: two lineages rooted at two different dead
    /// keys. Grafted twice into one destination, the four lineages
    /// (two parents, two copies) root at four distinct keys, none of
    /// which resolves, each shared by exactly its own three pieces.
    #[test]
    fn distinct_dead_parents_keep_distinct_dead_roots_per_copy() {
        let mut src = cube();
        let ends = |b: &Body<f64>, e: EdgeKey| {
            let he = b.get_edge(e).unwrap().he_plus;
            [
                b.get_half_edge(he).unwrap().start,
                b.half_edge_end(he).unwrap(),
            ]
        };
        let a0 = src.edges().next().unwrap().0;
        let a_ends = ends(&src, a0);
        let b0 = src
            .edges()
            .map(|(e, _)| e)
            .find(|&e| ends(&src, e).iter().all(|v| !a_ends.contains(v)))
            .expect("a cube edge sharing no vertex with the first");
        let lineages = [
            split_thrice_and_kill_the_parent(&mut src, a0),
            split_thrice_and_kill_the_parent(&mut src, b0),
        ];
        let mut dst = cube();
        let copies = [
            graft_disjoint_all_keyed(&mut dst, &src).expect("a first graft"),
            graft_disjoint_all_keyed(&mut dst, &src).expect("a second graft"),
        ];
        let mut roots = Vec::new();
        for (c, keys) in copies.iter().enumerate() {
            for (l, pieces) in lineages.iter().enumerate() {
                let root = dst
                    .split_root(keys.edge(pieces[0]).unwrap(), |_| false)
                    .unwrap();
                for &e in pieces {
                    assert_eq!(
                        dst.split_root(keys.edge(e).unwrap(), |_| false),
                        Ok(root),
                        "copy {c}, lineage {l}: {e:?} shares its lineage's root"
                    );
                }
                assert!(
                    dst.get_edge(root).is_none(),
                    "copy {c}, lineage {l}: the root is dead on arrival"
                );
                roots.push(root);
            }
        }
        let distinct: std::collections::BTreeSet<_> = roots.iter().collect();
        assert_eq!(
            distinct.len(),
            4,
            "one dead root per lineage per copy: {roots:?}"
        );
    }

    /// **A minted solid's record names destination solids.** A source
    /// whose second solid was moved out of its first carries
    /// `MoveShells { solid: first }`; grafted into a non-empty
    /// destination, that record names the first solid's target.
    #[test]
    fn a_minted_solids_record_names_the_destination_solid() {
        let mut src = cube();
        let first = src.solids().next().unwrap().0;
        graft_disjoint_all_keyed(&mut src, &cube()).expect("a second cube");
        src.merge_all_solids().unwrap();
        let moved = src.shells_of_solid(first).unwrap()[1];
        src.move_shells_to_new_solid(&[moved])
            .expect("a second solid");
        let mut dst = cube();
        let keys = graft_disjoint_all_keyed(&mut dst, &src).expect("a graft");
        assert_eq!(keys.solids.len(), 2);
        assert_eq!(
            dst.solid_provenance.get(keys.solids[1]),
            Some(&crate::Provenance::MoveShells {
                solid: keys.solids[0]
            })
        );
    }

    /// A cheap guard that the fixture is what these rows think it is.
    #[test]
    fn the_fixture_is_one_closed_cube() {
        let b = cube();
        assert_eq!(b.solids().count(), 1);
        assert_eq!(b.faces().count(), 6);
        assert_eq!(b.edges().count(), 12);
        assert_eq!(b.vertices().count(), 8);
        let origin = Point3::new(0.0, 0.0, 0.0);
        let first = *b.points().next().unwrap().1;
        assert!((first.x - origin.x).abs() < 1e-12 && (first.y - origin.y).abs() < 1e-12);
    }
}

#[cfg(test)]
mod per_part_gate_rows {
    use super::per_part_gate_owed;

    /// The threshold itself, at the policy's home: no solid and one
    /// solid owe nothing beyond the aggregate gate; two and more owe
    /// each part its own. A consumer that argues from one side of it
    /// pins that side where it argues (step-import's
    /// `the_per_part_policy_still_skips_a_lone_solid`).
    #[test]
    fn the_per_part_gate_is_owed_from_two_solids_up() {
        let owed: Vec<bool> = (0..=3).map(per_part_gate_owed).collect();
        assert_eq!(owed, [false, false, true, true]);
    }
}
