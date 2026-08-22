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
//! in deterministic slot order (D9), same verbatim provenance, same
//! `GeomSource` and pcurve-cache carry, same description surface-key
//! remap. Two differences, both forced by what a DISJOINT graft is:
//! the destination is an empty solid minted here instead of one
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
//! one face, one edge, or one edge–face pair, and the one whole-body
//! check (the +V signed volume) SUMS flux, so overlapping positive
//! volumes only reinforce it. Two grafted solids share no edge, so no
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
//! curved OR planar partner, F5) and one instance's extents nested
//! inside another's REFUSE as `CensusUndecidable` — the conservative
//! loudness backstop for the classes no arm can examine yet (the
//! C9-ring conformal-rest / partial-embedding class; C6's
//! interference class, representable only through recorded
//! gate-skips that do not exist yet). A pair of PLANAR faces is left
//! to the sweeps only when both are bounded entirely by line edges,
//! which is what puts a whole boundary in front of them — so an
//! arc-bounded planar face (a cylinder's cap) is backstopped like a
//! curved one, and nothing in the inter-instance touching/overlap
//! space validates silently; a caller assembling instances at rest
//! runs THAT gate with its declaration records.

use crate::body::Body;
use crate::boolean::BooleanError;
use crate::entity::{Solid, SolidKey};
use geom_core::Tol;

/// Grafts `src`'s single solid into `dst` as a NEW solid, returning its
/// key (module docs).
///
/// `src` must be a single-solid body — a source holding N solids is a
/// caller error at THIS door, whose one returned key could only name
/// one of them; [`graft_disjoint_all`] is the N-solid door. Its shells
/// arrive whole, in source order, under the minted solid. The minted
/// solid's provenance
/// is `src`'s own solid provenance, transplanted verbatim like every
/// other record the graft carries (a graft is not a re-birth).
///
/// # Errors
///
/// [`BooleanError`] — the transplant's own refusals: `JoinDesync` when
/// `src` is not a well-formed single-solid body, `GraftRecertify` when
/// a transplanted edge description does not re-certify against the
/// destination's surfaces.
pub fn graft_disjoint<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
    tol: Tol,
) -> Result<SolidKey, BooleanError> {
    if src.solids().count() != 1 {
        return Err(BooleanError::JoinDesync {
            what: "graft source is not a well-formed single-solid body",
        });
    }
    let mut keys = graft_disjoint_all(dst, src, tol)?;
    keys.pop().ok_or(BooleanError::JoinDesync {
        what: "graft source is not a well-formed single-solid body",
    })
}

/// Grafts EVERY solid of `src` into `dst`, each as a new solid,
/// returning their keys in the source's solid order (module docs).
///
/// The N-solid door. A source holding N solids arrives as N solids of
/// `dst`, each carrying its own source solid's provenance and its own
/// shells in source order — entity for entity, and key for key, what N
/// sequential [`graft_disjoint`] calls over the source's solids in slot
/// order (D9) would have built. Which source solid a grafted face came
/// from stays derivable exactly as it was before the graft: from the
/// solid it now sits under, and from the `GeomSource`/provenance
/// records the transplant carries verbatim.
///
/// Sharing is impossible here for the same reason it is at the single
/// door: every transplanted entity is re-created under a FRESH key, so
/// two grafts of one source produce two disjoint key ranges. Validity
/// remains the caller's to establish — this is a raw transplant, and a
/// multi-solid source's solids are gated by the same at-rest validator
/// as any other body's (the step-import loop's per-solid-then-aggregate
/// shape).
///
/// # Errors
///
/// [`BooleanError::JoinDesync`] — when `src` holds no solid, when a
/// solid has no provenance, or when a source-internal reference does
/// not resolve during the remap. **`GraftRecertify` is not among them
/// at this door**, and the distinction is load-bearing rather than
/// pedantic: this door bridges with `combine::Bridge::RemapKeys`, whose
/// arm carries each certificate verbatim and never reaches the
/// re-certification that is the only site raising that variant. Only
/// the in-crate `Bridge::Recertify` path (the booleans') can.
///
/// The failure STATE is unchanged and is what a caller must plan for:
/// the destination solids are minted before the transplant, and the
/// remap writes as it goes, so a `JoinDesync` raised mid-transplant
/// leaves `dst` partially written — a failed graft's destination is
/// spent, never resumable.
pub fn graft_disjoint_all<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
    tol: Tol,
) -> Result<Vec<SolidKey>, BooleanError> {
    Ok(graft_disjoint_all_keyed(dst, src, tol)?.solids)
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
/// Exactly [`graft_disjoint_all`]'s, including its spent-destination
/// failure state.
pub fn graft_disjoint_all_keyed<T: geom_core::Decide>(
    dst: &mut Body<T>,
    src: &Body<T>,
    tol: Tol,
) -> Result<GraftKeys, BooleanError> {
    let desync = || BooleanError::JoinDesync {
        what: "graft source is not a well-formed body: a solid without provenance",
    };
    let provenances = src
        .solids()
        .map(|(k, _)| src.solid_provenance.get(k).cloned().ok_or_else(desync))
        .collect::<Result<Vec<_>, _>>()?;
    if provenances.is_empty() {
        return Err(BooleanError::JoinDesync {
            what: "graft source holds no solid to graft",
        });
    }
    // Mint the destinations first, in source order, so the graft's
    // per-solid attachment is positional (nothing is written before
    // the source is known to be graftable at all).
    let targets: Vec<SolidKey> = provenances
        .into_iter()
        .map(|p| dst.add_solid(Solid { shells: Vec::new() }, p))
        .collect();
    let map = crate::boolean::combine::graft_solids_with(
        dst,
        &targets,
        src,
        crate::boolean::combine::Bridge::RemapKeys,
        tol,
    )?;
    Ok(GraftKeys {
        solids: targets,
        map,
    })
}

/// Grafts every solid of `src` onto ALREADY-EXISTING destination
/// solids, one target per source solid, so the source's shells join
/// solids that are already there instead of minting new ones.
///
/// The same transplant as [`graft_disjoint_all_keyed`], with the one
/// difference the caller's semantics ask for. Repeating a key in
/// `targets` is legal and is the point: N placed copies of a
/// single-solid prototype grafted onto the SAME target become one solid
/// of N shells.
///
/// **Why this shape exists.** It is what a UNION of separated bodies
/// already means in this kernel: the boolean pipeline's `combine` door
/// transplants a disjoint operand's shells into the destination's
/// EXISTING solid, so a chain of pairwise `Union`s over N separated
/// bodies produces one solid of N shells — and the seamed boolean path
/// accepts that result as an operand, while an N-SOLID body is refused
/// (`setopfinish`'s single-solid gate). A group union that wants to
/// feed a later boolean must therefore produce the representation the
/// chain it replaces produced, entity for entity.
///
/// Validity remains the caller's, exactly as at the sibling doors: the
/// shells must genuinely be disjoint, and nothing here checks it
/// ([`crate::Separation`] is the door that certifies it). Touching or
/// overlapping shells build a body the at-rest validator cannot catch
/// (module docs, #382).
///
/// The returned bridge's `solids()` is `targets`, echoed — the same
/// positional contract, so a caller re-keying per source solid needs no
/// special case.
///
/// # Errors
///
/// Exactly [`graft_disjoint_all_keyed`]'s, plus `JoinDesync` when a
/// target is not a live solid of `dst` or the arity does not match the
/// source's solid count.
pub fn graft_disjoint_all_onto_keyed<T: geom_core::Decide>(
    dst: &mut Body<T>,
    targets: &[SolidKey],
    src: &Body<T>,
    tol: Tol,
) -> Result<GraftKeys, BooleanError> {
    if targets.iter().any(|&k| dst.get_solid(k).is_none()) {
        return Err(BooleanError::JoinDesync {
            what: "graft destination solid is not live",
        });
    }
    if src.solids().next().is_none() {
        return Err(BooleanError::JoinDesync {
            what: "graft source holds no solid to graft",
        });
    }
    let map = crate::boolean::combine::graft_solids_with(
        dst,
        targets,
        src,
        crate::boolean::combine::Bridge::RemapKeys,
        tol,
    )?;
    Ok(GraftKeys {
        solids: targets.to_vec(),
        map,
    })
}

/// Direct rows for this door (R1 MINOR-2): the integration coverage
/// lives in `step-import`, but the contract this module states —
/// "nothing is fused, nothing is shared, nothing but the keys may
/// differ" — is a kernel claim and is checked here.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use crate::body::Body;
    use crate::fixtures::ops_cube;
    use crate::instance::graft_disjoint;

    fn cube() -> Body<f64> {
        ops_cube(Tol::witness()).body
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
        let key = graft_disjoint(&mut dst, &src, Tol::witness()).expect("a single-solid graft");
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
            dst.get_solid(key).unwrap().shells.len(),
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
        let key = graft_disjoint(&mut dst, &src, Tol::witness()).expect("a graft");

        let grafted: Vec<_> = dst
            .get_solid(key)
            .unwrap()
            .shells
            .iter()
            .flat_map(|&sh| dst.get_shell(sh).unwrap().faces.clone())
            .collect();
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

    /// **A planted collision refuses LOUD.** The door's precondition is
    /// a single-solid source; a body that is not one is `JoinDesync`,
    /// never a partial transplant.
    #[test]
    fn a_source_that_is_not_a_single_solid_refuses_typed() {
        // Empty: no solid at all.
        let mut dst = cube();
        let err = graft_disjoint(&mut dst, &Body::<f64>::new(), Tol::witness())
            .expect_err("no solid to graft");
        assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
        assert_eq!(dst.solids().count(), 1, "and nothing was written");

        // Two solids: the graft transplants ONE, so a two-solid source
        // is a caller error, not a thing to guess at.
        let mut two = cube();
        graft_disjoint(&mut two, &cube(), Tol::witness()).expect("build a two-solid body");
        let mut dst = cube();
        let err =
            graft_disjoint(&mut dst, &two, Tol::witness()).expect_err("two solids in the source");
        assert!(format!("{err:?}").contains("JoinDesync"), "{err:?}");
    }

    /// The minted solid's provenance is the SOURCE's, verbatim — a
    /// graft is not a re-birth (module docs).
    #[test]
    fn the_minted_solid_carries_the_source_solids_provenance() {
        let src = cube();
        let want = {
            let (k, _) = src.solids().next().unwrap();
            format!("{:?}", src.solid_provenance.get(k).unwrap())
        };
        let mut dst = cube();
        let key = graft_disjoint(&mut dst, &src, Tol::witness()).expect("a graft");
        assert_eq!(
            format!("{:?}", dst.solid_provenance.get(key).unwrap()),
            want
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
