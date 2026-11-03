//! **The whole-edge wedge fold**: what an edge's two faces make of the
//! wedge between them along the edge, read at the edge's stations and
//! folded into one verdict ([`edge_wedge_over`]).
//!
//! At each station the class is read first ([`crate::classify_dihedral`]),
//! unsigned and sense-free. A corner station then reads its sign
//! ([`crate::Corner::convexity`]), so every corner is signed where it is
//! read. The signed dihedral is continuous along the edge, so two
//! neighbouring stations that disagree — a corner beside a tangency, or a
//! convex corner beside a concave one — bracket a point in the band that
//! no station read, and the edge is [`WedgeVerdict::Mixed`] (D2).
//!
//! An edge tangent at every station descends one order, through the one
//! [`crate::dihedral::second_order_walk`]. Before each station's
//! second-order decision the walk reads its tangency's material pairing
//! ([`crate::Tangency::pairing`]); after a positive one, which end of the
//! wedge the faces make, the jet's κ_rel signed into the plus face's
//! outward frame ([`crate::material_kappa_rel`]). The flags those reads
//! leave are folded into the material verdict (D1's tier-3 table): the
//! π seam, the cusp and the slit where the tangency is jet-determinate,
//! the lamina where the opposed faces' jets osculate, and a split where
//! the stations disagree about the pairing or the end.
//!
//! Every escalation along the way stops the fold and is its verdict
//! ([`WedgeVerdict::InBand`]), tagged with the rung that raised it
//! ([`crate::WedgeRung`]); no verdict is read past one.

use core::ops::ControlFlow;

use geom_core::k_stats::NonzeroSign;
use geom_core::{Band, Decide, Margin, Real};

use crate::dihedral::{
    Convexity, Dihedral, JetStation, MaterialWedge, MixedStations, SecondOrderWalk, Station,
    StationHook, Tangency, WedgeEscalation, WedgeRung, classify_dihedral, decide_nonzero,
    material_kappa_rel, second_order_walk,
};

/// Whether two faces that meet smoothly along an edge separate at second
/// order at every station: the surfaces determine the locus.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bend {
    /// Every station's second-order separation read definitely positive.
    Determinate,
    /// A station's read definitely zero or negative: a G2 join, a
    /// same-surface split, coplanar planes.
    UnderDetermined,
}

/// [`edge_wedge_over`]'s verdict.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WedgeVerdict {
    /// A corner at every station, of one sign.
    Crease(Convexity),
    /// Tangent at every station with the outward normals aligned: the
    /// wedge is π, the legal seam, whatever the bend.
    Seam {
        /// Whether the surfaces determine the locus along the edge.
        bend: Bend,
    },
    /// Tangent with the outward normals opposed and the material in the
    /// vanishing crescent between the faces: wedge 0, over a
    /// jet-determinate tangency.
    Cusp,
    /// The cusp's `revert` image: wedge 2π, over a jet-determinate
    /// tangency.
    Slit,
    /// Tangent with the outward normals opposed and the jets osculating
    /// at a station: conformal contact along the locus.
    Lamina,
    /// The stations did not deliver one material verdict: their pairings
    /// disagreed ([`WedgeRung::Pairing`]) or, over opposed faces, their
    /// ends did ([`WedgeRung::End`]).
    Split {
        /// The decision whose readings disagreed.
        rung: WedgeRung,
    },
    /// Neighbouring stations disagree about the class or the sign.
    Mixed(MixedStations),
    /// A decision at a station escalated.
    InBand(WedgeEscalation),
}

/// **The whole-edge wedge fold** (module docs): the wedge `faces` make
/// along the edge whose `stations` are given in schedule order, the class
/// metered over `reach`, the faces' extent (`topo::wedge_reach`).
///
/// The stations are the edge's own ([`crate::interior_stations`] of its
/// carrier, whose direction is the plus face's walk) or, for a contact
/// with no edge yet, its carrier's. `faces` are in that order: the plus
/// face first, each with its `Face::sense` bit.
///
/// Every station is classified, and every corner signed, before any is
/// metered second-order: an escalation anywhere is the verdict, else a
/// disagreement anywhere, and only an edge tangent at every station
/// descends into the second-order walk, where the first station not
/// positive decides.
#[must_use]
pub fn edge_wedge_over<T: Decide>(
    faces: [crate::SideFace<'_, T>; 2],
    stations: &[Station<T>],
    reach: geom_core::Lever<T>,
    band: Band,
) -> WedgeVerdict {
    let [plus, minus] = faces;
    let senses = [plus.sense, minus.sense];
    let mut corner = Vec::with_capacity(stations.len());
    let mut convex = Vec::with_capacity(stations.len());
    let mut tangents = Vec::with_capacity(stations.len());
    for station in stations {
        match classify_dihedral(plus.surface, minus.surface, station.point(), reach, band) {
            Ok(Dihedral::Corner(c)) => match c.convexity(station, senses, band) {
                Ok(sign) => {
                    corner.push(true);
                    convex.push(sign == Convexity::Convex);
                }
                Err(escalation) => return WedgeVerdict::InBand(escalation),
            },
            Ok(Dihedral::Tangent(t)) => {
                corner.push(false);
                tangents.push(t);
            }
            Err(escalation) => return WedgeVerdict::InBand(escalation),
        }
    }
    if let Some(mixed) = MixedStations::bracketing(&corner) {
        return WedgeVerdict::Mixed(mixed);
    }
    if let Some(&first) = convex.first() {
        return MixedStations::sign_flip(&convex).map_or_else(
            || {
                WedgeVerdict::Crease(if first {
                    Convexity::Convex
                } else {
                    Convexity::Concave
                })
            },
            WedgeVerdict::Mixed,
        );
    }
    let mut reads = MaterialReads::new(&tangents, plus.sense, senses, band);
    let determinate = match second_order_walk(
        plus.surface,
        minus.surface,
        stations.iter().copied(),
        reach,
        band,
        &mut reads,
    ) {
        SecondOrderWalk::Determinate => true,
        SecondOrderWalk::UnderDetermined => false,
        SecondOrderWalk::InBand(cause) => {
            return WedgeVerdict::InBand(WedgeEscalation::at(WedgeRung::Bend, cause));
        }
        SecondOrderWalk::Stopped(escalation) => return WedgeVerdict::InBand(escalation),
    };
    material_outcome(
        reads.aligned,
        reads.opposed,
        determinate,
        reads.side,
        reads.side_mixed,
    )
}

/// **The material reads at each tangent station**, taken inside the
/// second-order walk: the pairing before the station's second-order
/// decision, and the end after a positive one. A read that escalates
/// stops the walk with its rung.
struct MaterialReads<'a, T: Real> {
    tangents: &'a [Tangency<T>],
    next: usize,
    sense_plus: bool,
    senses: [bool; 2],
    band: Band,
    opposed: bool,
    aligned: bool,
    side: Option<MaterialWedge>,
    side_mixed: bool,
}

impl<'a, T: Real> MaterialReads<'a, T> {
    /// No station read yet: both pairings still possible, no end seen.
    fn new(tangents: &'a [Tangency<T>], sense_plus: bool, senses: [bool; 2], band: Band) -> Self {
        Self {
            tangents,
            next: 0,
            sense_plus,
            senses,
            band,
            opposed: true,
            aligned: true,
            side: None,
            side_mixed: false,
        }
    }
}

impl<T: Decide> StationHook<T> for MaterialReads<'_, T> {
    type Break = WedgeEscalation;

    fn before_decision(&mut self, _station: &JetStation<T>) -> ControlFlow<WedgeEscalation> {
        let tangency = self.tangents.get(self.next).unwrap_or_else(|| {
            unreachable!("the walk reads only the stations the fold read tangent")
        });
        self.next += 1;
        match tangency.pairing(self.senses, self.band) {
            Ok(crate::MaterialPairing::Aligned) => self.opposed = false,
            Ok(crate::MaterialPairing::Opposed) => self.aligned = false,
            Err(escalation) => return ControlFlow::Break(escalation),
        }
        ControlFlow::Continue(())
    }

    /// Which end, decided only where it is asked: the SIGN, in the
    /// material frame, of the quantity whose magnitude just classified
    /// definitely positive — so neither `Zero` nor an escalation is
    /// reachable through a margin the run can read. Both are announced
    /// anyway, through the gate that refuses a `Zero` with its margin.
    fn after_positive(&mut self, station: &JetStation<T>) -> ControlFlow<WedgeEscalation> {
        let signed = material_kappa_rel(station.jet.kappa_rel, self.sense_plus);
        let this = match decide_nonzero(
            "material_cusp_side",
            Margin::sagitta(signed, station.arm.length()),
            self.band,
        ) {
            Ok(NonzeroSign::Positive) => MaterialWedge::Cusp,
            Ok(NonzeroSign::Negative) => MaterialWedge::Slit,
            Err(cause) => {
                return ControlFlow::Break(WedgeEscalation::at(WedgeRung::End, cause));
            }
        };
        match self.side {
            Some(seen) if seen != this => self.side_mixed = true,
            _ => self.side = Some(this),
        }
        ControlFlow::Continue(())
    }
}

/// **The material fold**: the flags the reads leave over one contact's
/// tangent stations, resolved into the one verdict that contact earns.
/// Schedule-blind: it reads flags, not samples.
///
/// Total and pure. Two of its input states — a pairing that split
/// (`aligned == opposed`) and an end that split (`side_mixed`) — are
/// not known to be reachable through certified geometry, and both
/// resolve to [`WedgeVerdict::Split`] rather than to a wedge: a legal
/// verdict the stations never established.
pub(crate) fn material_outcome(
    aligned: bool,
    opposed: bool,
    determinate: bool,
    side: Option<MaterialWedge>,
    side_mixed: bool,
) -> WedgeVerdict {
    match (aligned, opposed) {
        (true, false) => WedgeVerdict::Seam {
            bend: if determinate {
                Bend::Determinate
            } else {
                Bend::UnderDetermined
            },
        },
        (false, true) if !determinate => WedgeVerdict::Lamina,
        (false, true) => match side {
            Some(MaterialWedge::Cusp) if !side_mixed => WedgeVerdict::Cusp,
            Some(MaterialWedge::Slit) if !side_mixed => WedgeVerdict::Slit,
            _ => WedgeVerdict::Split {
                rung: WedgeRung::End,
            },
        },
        _ => WedgeVerdict::Split {
            rung: WedgeRung::Pairing,
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::Surface;
    use geom_core::test_support::fixture_lever;
    use geom_core::{MarginDiag, Point3, Sign, Vec3};

    /// **The material fold, state by state**, including the two split
    /// states no certified geometry is known to reach: each resolves to a
    /// split naming its decision, never to the wedge it saw first.
    #[test]
    fn the_material_fold_splits_where_the_stations_disagree() {
        use MaterialWedge::{Cusp, Slit};
        use WedgeVerdict as V;
        let seam = |bend| V::Seam { bend };
        let split = |rung| V::Split { rung };
        let rows = [
            ((true, false, true, None, false), seam(Bend::Determinate)),
            (
                (true, false, false, None, false),
                seam(Bend::UnderDetermined),
            ),
            ((false, true, true, Some(Cusp), false), V::Cusp),
            ((false, true, true, Some(Slit), false), V::Slit),
            ((false, true, false, None, false), V::Lamina),
            ((false, true, false, Some(Cusp), false), V::Lamina),
            ((false, true, true, Some(Cusp), true), split(WedgeRung::End)),
            ((false, true, true, None, false), split(WedgeRung::End)),
            (
                (false, false, true, Some(Cusp), false),
                split(WedgeRung::Pairing),
            ),
            (
                (true, true, true, Some(Cusp), false),
                split(WedgeRung::Pairing),
            ),
        ];
        for ((aligned, opposed, determinate, side, mixed), want) in rows {
            assert_eq!(
                material_outcome(aligned, opposed, determinate, side, mixed),
                want,
                "{aligned} {opposed} {determinate} {side:?} {mixed}"
            );
        }
    }

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).unwrap()
    }

    fn plane(normal: Vec3<f64>) -> Surface<f64> {
        Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal,
            u_ref: Vec3::unit_y().cross(normal).normalize(),
        }
    }

    /// The stations of a carrier along the x axis.
    fn along_x() -> Vec<Station<f64>> {
        let axis = geom::Curve3::line(Point3::new(0.0, 0.0, 0.0), Vec3::unit_x());
        crate::interior_stations(&axis, 0.0, 1.0).collect()
    }

    fn face(surface: &Surface<f64>, sense: bool) -> crate::SideFace<'_, f64> {
        crate::SideFace { surface, sense }
    }

    /// **A corner is signed by its faces' senses and the walk**: a floor
    /// and a wall meeting along x read convex one way round and concave
    /// with the wall's material flipped, and a walk reversed by swapping
    /// the faces keeps the sign.
    #[test]
    fn a_crease_reads_its_sign_off_the_senses_and_the_walk() {
        let (floor, wall) = (plane(Vec3::unit_z()), plane(Vec3::unit_y()));
        let stations = along_x();
        let read = |a, b| edge_wedge_over([a, b], &stations, fixture_lever(1.0), band());
        let convex = read(face(&floor, true), face(&wall, true));
        let concave = read(face(&floor, true), face(&wall, false));
        assert!(
            matches!(convex, WedgeVerdict::Crease(_)) && matches!(concave, WedgeVerdict::Crease(_)),
            "both read creases: {convex:?} {concave:?}"
        );
        assert_ne!(
            convex, concave,
            "flipping one face's material flips the sign"
        );
        let reversed: Vec<_> = {
            let back = geom::Curve3::line(Point3::new(1.0, 0.0, 0.0), -Vec3::unit_x());
            crate::interior_stations(&back, 0.0, 1.0).collect()
        };
        assert_eq!(
            edge_wedge_over(
                [face(&wall, true), face(&floor, true)],
                &reversed,
                fixture_lever(1.0),
                band()
            ),
            convex,
            "swapping the faces reverses the walk and keeps the sign"
        );
    }

    /// **A tangency reads its material through the walk**: coplanar faces
    /// with one material side are the seam, under-determined (the planes'
    /// κ_rel is zero); with opposed sides and no bend, the lamina.
    #[test]
    fn coplanar_faces_read_the_seam_or_the_lamina_by_their_senses() {
        let floor = plane(Vec3::unit_z());
        let stations = along_x();
        let read = |a, b| edge_wedge_over([a, b], &stations, fixture_lever(1.0), band());
        assert_eq!(
            read(face(&floor, true), face(&floor, true)),
            WedgeVerdict::Seam {
                bend: Bend::UnderDetermined
            }
        );
        assert_eq!(
            read(face(&floor, true), face(&floor, false)),
            WedgeVerdict::Lamina
        );
    }

    /// **A sign flip names the first disagreeing neighbours** as the
    /// class bracket does, convex first, whichever side it lies on.
    #[test]
    fn the_brackets_name_the_first_disagreeing_neighbours() {
        assert_eq!(
            MixedStations::sign_flip(&[true, true, false, false]),
            Some(MixedStations::Sign {
                convex: 2,
                concave: 3
            })
        );
        assert_eq!(
            MixedStations::sign_flip(&[false, true]),
            Some(MixedStations::Sign {
                convex: 2,
                concave: 1
            })
        );
        assert_eq!(MixedStations::sign_flip(&[true, true]), None);
    }

    /// **The end read stops with its gate's decided Zero**, not a
    /// hand-minted poison: a jet whose κ_rel is zero after a positive
    /// second-order decision is a contradiction the walk cannot reach
    /// through a margin it reads, so the hook is called directly.
    #[test]
    fn a_zero_end_stops_with_its_decided_margin() {
        let mut reads = MaterialReads::<f64>::new(&[], true, [true, true], band());
        let station = JetStation {
            jet: crate::TangentJet {
                sin_theta: 0.0,
                kappa_rel: 0.0,
            },
            arm: fixture_lever(1.0),
        };
        let ControlFlow::Break(stop) = reads.after_positive(&station) else {
            panic!("a zero end stops the walk");
        };
        assert_eq!(
            (
                stop.rung(),
                stop.diag().predicate,
                stop.diag().margin.rejected_sign(),
                stop.diag().margin == MarginDiag::INVALID
            ),
            (
                WedgeRung::End,
                Some("material_cusp_side"),
                Some(Sign::Zero),
                false
            )
        );
    }
}
