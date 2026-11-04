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
    Convexity, Dihedral, JetStation, MaterialPairing, MixedStations, SecondOrderWalk, Station,
    StationHook, Tangency, WedgeEnd, WedgeEscalation, WedgeRung, classify_dihedral, decide_nonzero,
    interior_stations, material_kappa_rel, second_order_walk,
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

/// [`edge_wedge_over`]'s verdict: **D1's tier-3 table, in one enum.**
///
/// The first-order class compares tangent planes and is unsigned: wedge
/// 0, π and 2π all read tangent. The verdict signs it by the two faces'
/// outward normals (each face's chart normal selected by its
/// `Face::sense` bit):
///
/// | verdict | wedge | legality |
/// |---|---|---|
/// | [`Crease`](Self::Crease) | ∈ (0, 2π) at the θ = ε/r margin | legal |
/// | [`Seam`](Self::Seam) | π | legal |
/// | [`End`](Self::End) (`Cusp`) | 0 | legal iff jet-determinate |
/// | [`End`](Self::End) (`Slit`) | 2π | legal iff jet-determinate |
/// | [`Lamina`](Self::Lamina) | 0 or 2π, osculating | refused |
///
/// The fold hands out an end only over a jet-determinate tangency, so an
/// end it returns is legal. In-band readings escalate
/// ([`InBand`](Self::InBand)); stations that disagree are
/// [`Mixed`](Self::Mixed).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WedgeVerdict {
    /// A corner at every station, of one sign.
    Crease(Convexity),
    /// Tangent at every station with the outward normals aligned: the
    /// wedge is π, the legal seam, whatever the bend.
    ///
    /// **Not `topo::ContactMark::Seam`**, which is a statement about an
    /// edge's DESCRIPTION (a chart seam, exempt by kind) rather than a
    /// material verdict; each name is load-bearing in its own vocabulary.
    Seam {
        /// Whether the surfaces determine the locus along the edge.
        bend: Bend,
    },
    /// Tangent at every station with the outward normals opposed and the
    /// jets separating at every station: one end of the wedge range,
    /// jet-determinate.
    End(WedgeEnd),
    /// Tangent with the outward normals opposed and the jets osculating
    /// at a station: conformal contact along the locus.
    Lamina,
    /// Neighbouring stations disagree about the class, the sign, the
    /// pairing or the end.
    Mixed(MixedStations),
    /// A decision at a station escalated.
    InBand(WedgeEscalation),
}

/// **The faces and stations of one wedge read, built together**, so the
/// face order and the walk cannot be paired by hand ([`Convexity`]'s
/// convention). Non-empty by construction: each door yields at least one
/// station.
#[derive(Clone, Debug)]
pub struct WedgeWalk<'a, T: Real> {
    plus: crate::SideFace<'a, T>,
    minus: crate::SideFace<'a, T>,
    first: (u32, Station<T>),
    rest: Vec<(u32, Station<T>)>,
}

impl<'a, T: Decide> WedgeWalk<'a, T> {
    /// **An edge's walk**: `plus` is the face of the edge's `he_plus`,
    /// `minus` the face of its `he_minus`, and the stations are the
    /// certification schedule's interior over `[t0, t1]` on `carrier`,
    /// the edge's own curve, whose increasing parameter is the way
    /// `he_plus` walks the edge. `topo`'s one caller builds it from an
    /// edge key (`topo::validate`'s `edge_wedge_walk`).
    #[must_use]
    pub fn along_edge(
        plus: crate::SideFace<'a, T>,
        minus: crate::SideFace<'a, T>,
        carrier: &geom::Curve3<T>,
        t0: T,
        t1: T,
    ) -> Self {
        let first = (1, Station::on(carrier, crate::sample_param(t0, t1, 1)));
        let rest = interior_stations(carrier, t0, t1)
            .zip(1_u32..)
            .skip(1)
            .map(|(station, i)| (i, station))
            .collect();
        Self {
            plus,
            minus,
            first,
            rest,
        }
    }

    /// **A closed rim's walk**: the circle `circle` at `n` uniform phases
    /// from its `u_ref`, numbered `0..n`, with `plus` read first. A rim
    /// is no loop's walk, so the sign a crease reads here is the
    /// circle's, not either face's; what a rim reads of it is only
    /// whether it holds all the way round, which no reversal of the
    /// circle changes.
    #[must_use]
    pub fn around_circle(
        plus: crate::SideFace<'a, T>,
        minus: crate::SideFace<'a, T>,
        circle: &geom::Curve3<T>,
        n: core::num::NonZeroU32,
    ) -> Self {
        let phase = |i: u32| {
            let phase = core::f64::consts::TAU * (f64::from(i) / f64::from(n.get()));
            Station::on(circle, T::from_f64(phase))
        };
        Self {
            plus,
            minus,
            first: (0, phase(0)),
            rest: (1..n.get()).map(|i| (i, phase(i))).collect(),
        }
    }

    /// The stations in schedule order, each with its sample.
    fn stations(&self) -> impl Iterator<Item = (u32, Station<T>)> + '_ {
        core::iter::once(self.first).chain(self.rest.iter().copied())
    }
}

/// One station's first-order read: a signed corner, or a tangency with
/// the station it was read at.
#[derive(Clone, Copy)]
enum Read<T: Real> {
    Corner(Convexity),
    Tangent(Station<T>, Tangency<T>),
}

/// **The whole-edge wedge fold** (module docs): the wedge `walk`'s faces
/// make along its stations, the class metered over `reach`, the faces'
/// extent (`topo::wedge_reach`).
///
/// Every station is classified, and every corner signed, before any is
/// metered second-order: an escalation anywhere is the verdict, else a
/// disagreement anywhere, and only an edge tangent at every station
/// descends into the second-order walk, where the first station not
/// positive decides.
#[must_use]
pub fn edge_wedge_over<T: Decide>(
    walk: &WedgeWalk<'_, T>,
    reach: geom_core::Lever<T>,
    band: Band,
) -> WedgeVerdict {
    let (plus, minus) = (walk.plus, walk.minus);
    let senses = [plus.sense, minus.sense];
    let read = |(i, station): (u32, Station<T>)| match classify_dihedral(
        plus.surface,
        minus.surface,
        station.point(),
        reach,
        band,
    )? {
        Dihedral::Corner(c) => Ok((i, Read::Corner(c.convexity(&station, senses, band)?))),
        Dihedral::Tangent(t) => Ok((i, Read::Tangent(station, t))),
    };
    let reads: Result<Vec<_>, WedgeEscalation> = walk.stations().map(read).collect();
    let reads = match reads {
        Ok(reads) => reads,
        Err(escalation) => return WedgeVerdict::InBand(escalation),
    };
    let class: Vec<_> = reads
        .iter()
        .map(|&(i, r)| (i, matches!(r, Read::Corner(_))))
        .collect();
    if let Some(mixed) = MixedStations::class(&class) {
        return WedgeVerdict::Mixed(mixed);
    }
    let corners: Vec<_> = reads
        .iter()
        .filter_map(|&(i, r)| match r {
            Read::Corner(c) => Some((i, c)),
            Read::Tangent(..) => None,
        })
        .collect();
    if let Some(&(_, first)) = corners.first() {
        return MixedStations::sign(&corners)
            .map_or(WedgeVerdict::Crease(first), WedgeVerdict::Mixed);
    }
    let tangents: Vec<_> = reads
        .iter()
        .filter_map(|&(i, r)| match r {
            Read::Tangent(station, t) => Some((station, (i, t))),
            Read::Corner(_) => None,
        })
        .collect();
    let mut reads = MaterialReads::new(plus.sense, senses, band);
    let bend = match second_order_walk(
        plus.surface,
        minus.surface,
        tangents,
        reach,
        band,
        &mut reads,
    ) {
        SecondOrderWalk::Determinate => Bend::Determinate,
        SecondOrderWalk::UnderDetermined => Bend::UnderDetermined,
        SecondOrderWalk::InBand(cause) => {
            return WedgeVerdict::InBand(WedgeEscalation::at(WedgeRung::Bend, cause));
        }
        SecondOrderWalk::Stopped(escalation) => return WedgeVerdict::InBand(escalation),
    };
    material_outcome(&reads.pairings, bend, &reads.ends)
}

/// **The material reads at each tangent station**, taken inside the
/// second-order walk, each station handed its own tangency beside it:
/// the pairing before the station's second-order decision, and the end
/// after a positive one. A read that escalates stops the walk with its
/// rung.
struct MaterialReads {
    sense_plus: bool,
    senses: [bool; 2],
    band: Band,
    pairings: Vec<(u32, MaterialPairing)>,
    ends: Vec<(u32, WedgeEnd)>,
}

impl MaterialReads {
    /// No station read yet.
    const fn new(sense_plus: bool, senses: [bool; 2], band: Band) -> Self {
        Self {
            sense_plus,
            senses,
            band,
            pairings: Vec::new(),
            ends: Vec::new(),
        }
    }
}

impl<T: Decide> StationHook<T> for MaterialReads {
    type Break = WedgeEscalation;
    type Item = (u32, Tangency<T>);

    fn before_decision(
        &mut self,
        _station: &JetStation<T>,
        &(i, tangency): &(u32, Tangency<T>),
    ) -> ControlFlow<WedgeEscalation> {
        match tangency.pairing(self.senses, self.band) {
            Ok(pairing) => self.pairings.push((i, pairing)),
            Err(escalation) => return ControlFlow::Break(escalation),
        }
        ControlFlow::Continue(())
    }

    /// Which end, decided only where it is asked: the SIGN, in the
    /// material frame, of the quantity whose magnitude just classified
    /// definitely positive — so neither `Zero` nor an escalation is
    /// reachable through a margin the run can read. Both are announced
    /// anyway, through the gate that refuses a `Zero` with its margin.
    fn after_positive(
        &mut self,
        station: &JetStation<T>,
        &(i, _): &(u32, Tangency<T>),
    ) -> ControlFlow<WedgeEscalation> {
        let signed = material_kappa_rel(station.jet.kappa_rel, self.sense_plus);
        let end = match decide_nonzero(
            "material_cusp_side",
            Margin::sagitta(signed, station.arm.length()),
            self.band,
        ) {
            Ok(NonzeroSign::Positive) => WedgeEnd::Cusp,
            Ok(NonzeroSign::Negative) => WedgeEnd::Slit,
            Err(cause) => {
                return ControlFlow::Break(WedgeEscalation::at(WedgeRung::End, cause));
            }
        };
        self.ends.push((i, end));
        ControlFlow::Continue(())
    }
}

/// **The material fold**: the readings one contact's tangent stations
/// left, in schedule order, resolved into the one verdict that contact
/// earns. Total and pure.
///
/// Pairings that differ, or ends that differ, are [`WedgeVerdict::Mixed`]
/// naming the stations, never the wedge seen first. Opposed faces
/// whose bend is not determinate are the lamina, and so are opposed
/// faces with no end read: an end is read after every positive bend, so
/// none read means no station's bend decided positive.
pub(crate) fn material_outcome(
    pairings: &[(u32, MaterialPairing)],
    bend: Bend,
    ends: &[(u32, WedgeEnd)],
) -> WedgeVerdict {
    if let Some(mixed) = MixedStations::pairing(pairings) {
        return WedgeVerdict::Mixed(mixed);
    }
    match pairings.first() {
        Some((_, MaterialPairing::Aligned)) => WedgeVerdict::Seam { bend },
        _ if bend == Bend::UnderDetermined => WedgeVerdict::Lamina,
        _ => match (MixedStations::end(ends), ends.first()) {
            (Some(mixed), _) => WedgeVerdict::Mixed(mixed),
            (None, Some(&(_, end))) => WedgeVerdict::End(end),
            (None, None) => WedgeVerdict::Lamina,
        },
    }
}

/// **The end read's stop on a zero κ_rel**, raised by the fold's own hook
/// after a positive bend: the contradiction no margin the walk reads can
/// reach, for a reader's row that must see the stop the fold raises.
///
/// # Panics
///
/// Never: a plane meets itself smoothly, and a zero end always stops.
#[cfg(any(test, feature = "test-support"))]
#[must_use]
#[allow(clippy::panic)]
pub fn zero_end_stop(band: Band) -> WedgeEscalation {
    let floor = geom::Surface::Plane {
        origin: geom_core::Point3::new(0.0, 0.0, 0.0),
        normal: geom_core::Vec3::unit_z(),
        u_ref: geom_core::Vec3::unit_x(),
    };
    let arm = geom_core::test_support::fixture_lever(1.0);
    let Ok(Dihedral::Tangent(tangency)) = classify_dihedral(
        &floor,
        &floor,
        geom_core::Point3::new(0.0, 0.0, 0.0),
        arm,
        band,
    ) else {
        panic!("a plane meets itself smoothly");
    };
    let station = JetStation {
        jet: crate::TangentJet {
            sin_theta: 0.0,
            kappa_rel: 0.0,
        },
        arm,
    };
    match MaterialReads::new(true, [true, true], band).after_positive(&station, &(1, tangency)) {
        ControlFlow::Break(stop) => stop,
        ControlFlow::Continue(()) => panic!("a zero end stops the walk"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom::Surface;
    use geom_core::test_support::fixture_lever;
    use geom_core::{MarginDiag, Point3, Sign, Vec3};

    /// **The material fold, state by state**, including the states no
    /// certified geometry is known to reach: pairings or ends that differ
    /// name their stations, never the wedge seen first.
    #[test]
    fn the_material_fold_names_the_stations_that_disagree() {
        use MaterialPairing::{Aligned, Opposed};
        use WedgeEnd::{Cusp, Slit};
        use WedgeVerdict as V;
        let (det, under) = (Bend::Determinate, Bend::UnderDetermined);
        type Row<'a> = (&'a [(u32, MaterialPairing)], Bend, &'a [(u32, WedgeEnd)], V);
        let rows: [Row<'_>; 9] = [
            (
                &[(1, Aligned), (2, Aligned)],
                det,
                &[(1, Cusp)],
                V::Seam { bend: det },
            ),
            (&[(1, Aligned)], under, &[], V::Seam { bend: under }),
            (
                &[(1, Opposed), (2, Opposed)],
                det,
                &[(1, Cusp), (2, Cusp)],
                V::End(Cusp),
            ),
            (&[(1, Opposed)], det, &[(1, Slit)], V::End(Slit)),
            (&[(1, Opposed)], under, &[], V::Lamina),
            (
                &[(1, Opposed), (2, Opposed)],
                under,
                &[(1, Cusp)],
                V::Lamina,
            ),
            (
                &[(1, Opposed), (2, Opposed)],
                det,
                &[(1, Slit), (2, Cusp)],
                V::Mixed(MixedStations::End { cusp: 2, slit: 1 }),
            ),
            (&[(1, Opposed)], det, &[], V::Lamina),
            (
                &[(1, Opposed), (2, Aligned)],
                det,
                &[(1, Cusp)],
                V::Mixed(MixedStations::Pairing {
                    aligned: 2,
                    opposed: 1,
                }),
            ),
        ];
        for (pairings, bend, ends, want) in rows {
            assert_eq!(
                material_outcome(pairings, bend, ends),
                want,
                "{pairings:?} {bend:?} {ends:?}"
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

    fn face(surface: &Surface<f64>, sense: bool) -> crate::SideFace<'_, f64> {
        crate::SideFace { surface, sense }
    }

    /// The unit stretch of the x axis from `from`, walked along `dir`.
    fn line(from: f64, dir: f64) -> geom::Curve3<f64> {
        geom::Curve3::line(Point3::new(from, 0.0, 0.0), Vec3::new(dir, 0.0, 0.0))
    }

    /// The fold over `plus` then `minus`, walked along `carrier`.
    fn read(
        plus: crate::SideFace<'_, f64>,
        minus: crate::SideFace<'_, f64>,
        carrier: &geom::Curve3<f64>,
    ) -> WedgeVerdict {
        let walk = WedgeWalk::along_edge(plus, minus, carrier, 0.0, 1.0);
        edge_wedge_over(&walk, fixture_lever(1.0), band())
    }

    /// **The sign convention, pinned** ([`Convexity`]): a floor whose
    /// outward normal is `+z` (the plus face) and a wall whose outward
    /// normal is `+y`, the floor walking the edge along `−x`, is a block's
    /// top front edge, convex; walked along `+x` the same pair is the
    /// inside corner, concave. A flipped convention reds both rows.
    #[test]
    fn the_top_front_edge_of_a_block_reads_convex() {
        let (floor, wall) = (plane(Vec3::unit_z()), plane(Vec3::unit_y()));
        assert_eq!(
            read(face(&floor, true), face(&wall, true), &line(1.0, -1.0)),
            WedgeVerdict::Crease(Convexity::Convex)
        );
        assert_eq!(
            read(face(&floor, true), face(&wall, true), &line(0.0, 1.0)),
            WedgeVerdict::Crease(Convexity::Concave)
        );
    }

    /// **A corner is signed by its faces' senses, the face order and the
    /// walk**: flipping one face's material flips the sign, swapping the
    /// faces against the same walk flips it (the reader's order matters,
    /// which is why the walk builds both), and swapping them while
    /// reversing the walk — the edge read through its other half —
    /// keeps it.
    #[test]
    fn a_crease_reads_its_sign_off_the_senses_the_order_and_the_walk() {
        let (floor, wall) = (plane(Vec3::unit_z()), plane(Vec3::unit_y()));
        let (back, forth) = (line(1.0, -1.0), line(0.0, 1.0));
        let convex = WedgeVerdict::Crease(Convexity::Convex);
        let concave = WedgeVerdict::Crease(Convexity::Concave);
        assert_eq!(read(face(&floor, true), face(&wall, true), &back), convex);
        assert_eq!(read(face(&floor, true), face(&wall, false), &back), concave);
        assert_eq!(read(face(&wall, true), face(&floor, true), &back), concave);
        assert_eq!(read(face(&wall, true), face(&floor, true), &forth), convex);
    }

    /// **A tangency reads its material through the walk**: coplanar faces
    /// with one material side are the seam, under-determined (the planes'
    /// κ_rel is zero); with opposed sides and no bend, the lamina.
    #[test]
    fn coplanar_faces_read_the_seam_or_the_lamina_by_their_senses() {
        let floor = plane(Vec3::unit_z());
        let axis = line(0.0, 1.0);
        assert_eq!(
            read(face(&floor, true), face(&floor, true), &axis),
            WedgeVerdict::Seam {
                bend: Bend::UnderDetermined
            }
        );
        assert_eq!(
            read(face(&floor, true), face(&floor, false), &axis),
            WedgeVerdict::Lamina
        );
    }

    /// **An edge's walk numbers its stations as the schedule does**
    /// (`1..CERT_SAMPLES-1`), and a rim's from phase 0, so a bracket
    /// names the samples its own schedule read.
    #[test]
    fn the_walks_number_their_stations_by_their_schedules() {
        let floor = plane(Vec3::unit_z());
        let edge = WedgeWalk::along_edge(
            face(&floor, true),
            face(&floor, true),
            &line(0.0, 1.0),
            0.0,
            1.0,
        );
        let samples: Vec<u32> = edge.stations().map(|(i, _)| i).collect();
        assert_eq!(samples, (1..crate::CERT_SAMPLES - 1).collect::<Vec<_>>());
        let circle = geom::Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_z(),
            1.0,
            Vec3::unit_x(),
        );
        let n = core::num::NonZeroU32::new(5).unwrap();
        let rim = WedgeWalk::around_circle(face(&floor, true), face(&floor, true), &circle, n);
        let phases: Vec<u32> = rim.stations().map(|(i, _)| i).collect();
        assert_eq!(phases, vec![0, 1, 2, 3, 4]);
    }

    /// **A bracket names the first disagreeing neighbours** by their
    /// samples, each in its role, whichever side it lies on.
    #[test]
    fn the_brackets_name_the_first_disagreeing_neighbours() {
        use Convexity::{Concave, Convex};
        assert_eq!(
            MixedStations::sign(&[(1, Convex), (2, Convex), (3, Concave), (4, Concave)]),
            Some(MixedStations::Sign {
                convex: 2,
                concave: 3
            })
        );
        assert_eq!(
            MixedStations::sign(&[(0, Concave), (1, Convex)]),
            Some(MixedStations::Sign {
                convex: 1,
                concave: 0
            })
        );
        assert_eq!(MixedStations::sign(&[(1, Convex), (2, Convex)]), None);
    }

    /// **The end read stops with its gate's decided Zero**, not a
    /// hand-minted poison: a jet whose κ_rel is zero after a positive
    /// second-order decision is a contradiction the walk cannot reach
    /// through a margin it reads, so the hook is called directly.
    #[test]
    fn a_zero_end_stops_with_its_decided_margin() {
        let stop = zero_end_stop(band());
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
