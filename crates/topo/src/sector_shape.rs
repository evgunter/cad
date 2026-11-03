//! The vertex-neighborhood **corner readings** over the sector-shape
//! rungs ([`geom_brep::sector_shape`], which owns the three rungs, their
//! K names and their contract): a corner's shape where the structure
//! says more than its two bounds — at a slit's tip ([`SlitTips`]) the
//! face wraps round the two coincident edges, and [`corner_shape`] reads
//! the full turn and subdivides it at its three quarters ([`FullTurn`]);
//! at a cone's apex there is no tangent plane, and [`apex_corner_shape`]
//! reads the fan of rulings between the bounds against a plane through
//! the apex. Both take the turn by one rule ([`reads_slit_tip_turn`]).
//! A wrap edge's tip is the one-edge form of the same structure: its
//! face's loop walks the edge out and straight back, the face wraps the
//! full turn round it, and [`SlitTips`] holds it beside the slits' tips
//! (the name covers both), so its corner reads the full turn by the same
//! rule. Any other one-edge orbit (a strut vertex) is the legitimate
//! full-circle sector and takes the 90° device.

use geom_brep::OutwardNormal;
use geom_brep::sector_shape::{
    SectorFault, SectorRung, SectorShape, metering_arm, sector_arm, sector_shape,
};
use geom_core::{Band, Decide, Margin, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, VertexKey};
use crate::live::proven;
use crate::validate::{decide, decide_positive};

/// What the structure says about a corner's two bounds, which
/// [`corner_shape`] reads a θ ≈ 0 / ≈ 2π reading by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CornerBounds {
    /// Two distinct edges, not a slit's tip: such a reading is a spike.
    Distinct,
    /// One orbit half-edge, a strut vertex ([`sector_shape`]'s
    /// `full_circle`).
    OneEdge,
    /// A slit's two coincident edges at its tip, or a wrap edge's tip
    /// walked out and back ([`SlitTips`]): the face wraps a full turn
    /// round them.
    SlitTip,
}

/// What [`corner_shape`] read: the sector-shape rungs' answer, or a
/// slit tip's full turn.
#[derive(Clone, Copy, Debug)]
pub(crate) enum CornerShape<T: Real> {
    /// The rungs decided the corner ([`sector_shape`]).
    Sector(SectorShape<T>),
    /// The face's full turn round a slit's tip or a wrap edge's.
    FullTurn(FullTurn<T>),
}

/// A face's full turn round a slit's tip or a wrap edge's ([`SlitTips`],
/// whose name covers both), from the next orbit chord
/// counter-clockwise (about the face's outward normal) round to the
/// corner's own, the two along one ray.
#[derive(Clone, Copy, Debug)]
pub(crate) struct FullTurn<T: Real> {
    /// The metering arm, as [`SectorShape::arm`].
    pub arm: T,
    /// Unit direction of the next orbit chord, the turn's start.
    pub unit_next: Vec3<T>,
    /// Unit direction of the corner's own orbit chord, its end.
    pub unit_own: Vec3<T>,
    /// `unit_next` turned a quarter about the outward normal.
    quarter: Vec3<T>,
}

/// The order a lane reads a [`FullTurn`]'s quarters in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TurnOrder {
    /// Counter-clockwise from the next chord: the sector bounds' order.
    FromNext,
    /// From the corner's own chord round to the next: the orbit's order,
    /// in which the split's walk lists a sector's duplicate entries.
    FromOwn,
}

impl TurnOrder {
    /// **The quarter cuts of a whole turn, as fractions of it from the
    /// next bound**, listed in this order: a quarter, a half and three
    /// quarters from [`TurnOrder::FromNext`], the reverse from
    /// [`TurnOrder::FromOwn`]. The one home of the cut both whole turns
    /// take: a slit tip's ([`FullTurn::quarters`]) and a cone apex
    /// corner's in the vertex–vertex lane (`boolean::sectors`).
    pub(crate) const fn quarter_fractions(self) -> [f64; 3] {
        match self {
            Self::FromNext => [0.25, 0.5, 0.75],
            Self::FromOwn => [0.75, 0.5, 0.25],
        }
    }
}

impl<T: Real> FullTurn<T> {
    /// The three directions that cut the turn into convex quarters:
    /// `unit_next` turned [`TurnOrder::quarter_fractions`] of the turn,
    /// listed in `order`.
    pub(crate) fn quarters(&self, order: TurnOrder) -> [Vec3<T>; 3] {
        let (q, back) = (self.quarter, -self.unit_next);
        match order {
            TurnOrder::FromNext => [q, back, -q],
            TurnOrder::FromOwn => [-q, back, q],
        }
    }
}

/// [`sector_shape`], with a reading that the bounds leave along one ray
/// read by `bounds`: at a slit's tip the face's full turn, otherwise a
/// spike as the rungs refuse it. The one reading of a slit tip's corner
/// for both lanes.
///
/// # Errors
///
/// As [`sector_shape`], but for the full turn at a slit's tip.
pub(crate) fn corner_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    normal: OutwardNormal<T>,
    bounds: CornerBounds,
    band: Band,
) -> Result<CornerShape<T>, SectorFault> {
    match sector_shape(
        dir_own,
        dir_next,
        normal,
        bounds == CornerBounds::OneEdge,
        band,
    ) {
        Ok(shape) => Ok(CornerShape::Sector(shape)),
        Err(fault) if reads_slit_tip_turn(bounds, &fault, Sign::Positive) => {
            let unit_next = dir_next.normalize();
            Ok(CornerShape::FullTurn(FullTurn {
                arm: dir_own.norm().min(dir_next.norm()),
                unit_next,
                unit_own: dir_own.normalize(),
                quarter: normal.vec().cross(unit_next),
            }))
        }
        Err(fault) => Err(fault),
    }
}

/// **The tips of an operand's slits and zero-area rings**, read from
/// the operand at rest: each `(vertex, face)` where two distinct line
/// edges on one segment by structure ([`crate::census::shared_segments`],
/// the same two point keys) both end at `vertex` and both bound `face`.
/// That is tier 3's two coincident distinct edges (`crates/topo/README.md`),
/// and `face` carries them as a slit of one of its loops or as a ring:
/// it wraps round the tip, so its corner there between the two edges is
/// a full turn (the Boolean's `sectors::build_sectors`, the split's
/// [`crate::splitting::wide_entries`]). A wrap edge's tip
/// (`validate::wrap_tips`, DESIGN `[topology]`'s seam clause) is the
/// one-edge form: its face's loop walks the edge out and straight back,
/// so the face wraps the same full turn round the one edge, at a pole or
/// a cone's apex. The Boolean's reduction may split the two edges apart
/// from the tip, each at its own vertex, so each lane takes the reading
/// before its reduction.
///
/// **The name covers both, and the two are kept apart inside it.**
/// Every corner reading ([`Self::holds`], [`Self::bounds`]) reads a tip
/// of either kind alike, as the full turn they share. A reader of a
/// slit's line, its two edges ([`Self::face_at`], for `recl::slit_line`
/// and `join::slit_tip_on`), reads the two-edge tips alone: a wrap tip
/// has one edge, and no line of two.
#[derive(Clone, Debug, Default)]
pub(crate) struct SlitTips {
    /// The two-edge tips, of slits and rings.
    slits: std::collections::BTreeSet<(VertexKey, FaceKey)>,
    /// The one-edge tips, of wrap edges.
    wraps: std::collections::BTreeSet<(VertexKey, FaceKey)>,
}

impl SlitTips {
    /// The tips of `body` at rest.
    pub(crate) fn of<T: Real>(body: &Body<T>) -> Self {
        let mut tips = std::collections::BTreeSet::new();
        for pair in crate::census::shared_segments(body) {
            let [a, b] = pair.edges;
            let (ea, eb) = (
                crate::query::edge_ends(body, a),
                crate::query::edge_ends(body, b),
            );
            let faces = |e: EdgeKey| -> Vec<FaceKey> {
                let plus = proven(&body.edges, e, EntityId::Edge).he_plus;
                [Some(plus), body.mate(plus)]
                    .into_iter()
                    .flatten()
                    .filter_map(|h| body.face_of_half_edge(h))
                    .collect()
            };
            let (fa, fb) = (faces(a), faces(b));
            for v in [ea.0, ea.1] {
                if v != eb.0 && v != eb.1 {
                    continue;
                }
                for &f in fa.iter().filter(|f| fb.contains(f)) {
                    tips.insert((v, f));
                }
            }
        }
        let wraps = crate::validate::wrap_tips(body)
            .into_iter()
            .map(|(v, he)| {
                let face = body.face_of_half_edge(he).unwrap_or_else(|| {
                    unreachable!(
                        "{v:?}'s tip half {he:?} has no face: on a tier-1-valid body every \
                         half-edge lies in a loop of a face"
                    )
                });
                (v, face)
            })
            .collect();
        Self { slits: tips, wraps }
    }

    /// Whether `face`'s corner at `vertex` lies between a slit's two
    /// edges, or round a wrap edge's tip.
    pub(crate) fn holds(&self, vertex: VertexKey, face: FaceKey) -> bool {
        self.slits.contains(&(vertex, face)) || self.wraps.contains(&(vertex, face))
    }

    /// Both operands' tips at rest, A's first: the order the Boolean's
    /// reduction keeps them in (`boolean::BooleanReduction::slit_tips`).
    pub(crate) fn of_operands<T: Real>(a: &Body<T>, b: &Body<T>) -> [Self; 2] {
        [Self::of(a), Self::of(b)]
    }

    /// What the structure says about `face`'s corner at `vertex`, whose
    /// bounds are one orbit half-edge where `one_edge`: a tip's, a wrap
    /// tip's one edge among them, before a strut's.
    pub(crate) fn bounds(&self, vertex: VertexKey, face: FaceKey, one_edge: bool) -> CornerBounds {
        if self.holds(vertex, face) {
            CornerBounds::SlitTip
        } else if one_edge {
            CornerBounds::OneEdge
        } else {
            CornerBounds::Distinct
        }
    }

    /// Tips named outright, for a unit row that has no body to read.
    #[cfg(test)]
    pub(crate) fn of_pairs(pairs: impl IntoIterator<Item = (VertexKey, FaceKey)>) -> Self {
        Self {
            slits: pairs.into_iter().collect(),
            wraps: std::collections::BTreeSet::new(),
        }
    }

    /// The face carrying a slit `vertex` is the tip of, if it is one: a
    /// two-edge tip, whose edges the reduction may part or glue. A wrap
    /// edge's tip is not one.
    pub(crate) fn face_at(&self, vertex: VertexKey) -> Option<FaceKey> {
        self.slits
            .iter()
            .find(|(v, _)| *v == vertex)
            .map(|&(_, f)| f)
    }
}

/// K name: how far a plane's side varies round the rulings of a cone's
/// apex, `sin α·‖n_r‖` levered at the sector's arm (metres), `n_r` the
/// plane normal's part square to the axis. Zero reads every ruling on
/// one side as every other.
const SECTOR_APEX_TILT: &str = "sector_apex_tilt";

/// K name: how far a ruling of a cone's apex stands from a bound of the
/// apex sector it is read in, the chord between their points a lever
/// from the apex (metres): the window's width between distinct edges,
/// and an extremal ruling's distance in from each bound.
const SECTOR_APEX_WINDOW: &str = "sector_apex_window";

/// What [`apex_shape`] decided about a sector at a cone's apex.
#[derive(Clone, Debug)]
pub(crate) struct ApexShape<T: Real> {
    /// The metering arm, as [`SectorShape::arm`].
    pub arm: T,
    /// The azimuth the face's sweep turns through from the next bound to
    /// its own ([`crate::sector_face::ConeApex::azimuth_from`]), as the
    /// window was decided: the whole turn `2π` where the corner is one
    /// (a strut's one edge, a wrap edge's tip), whatever the two bounds'
    /// rounding reads. Its one reader is the Boolean's apex corner
    /// (`boolean::sectors::push_apex_corner`), which cuts the window by
    /// it.
    pub window: T,
    /// Whether the window is the whole turn, decided by the corner's
    /// bounds ([`Self::window`]).
    pub whole_turn: bool,
    /// The unit rulings the sector is subdivided at, in orbit order (from
    /// its own bound toward the next): none, one or two.
    pub rulings: Vec<Vec3<T>>,
}

/// **A sector at a cone's apex, read against a plane through the apex.**
///
/// The sector's interior is not the positive cone of its bounds: its
/// directions are the cone's rulings between them, round the axis, at
/// every width. Along the ruling at azimuth `φ` the plane (unit normal
/// `across`) reads `s(φ) = n·d(φ) = σ cos α (n·a) + sin α (n_r·r̂(φ))`,
/// which is monotone between its two extrema, the rulings along `±n_r`:
/// the apex lane's own ([`geom_brep::plane_cone_apex_extrema`]), so the
/// section rulings the lane serves lie between them.
/// So the bounds decide the interior's sides only where no extremum lies
/// strictly inside the window, and the sector is subdivided at each one
/// that does: none, one, or (in a window over a half-turn) two. Each
/// subdivision ruling is an interior direction like a bisector, and the
/// side of the plane it reads is the caller's.
///
/// The window is read in the face's own sweep: azimuth zero along
/// `dir_next`, increasing into the sector (the interior leaves a bound
/// along `n × b̂`, `n` the outward normal along it), to `dir_own`, or the
/// whole turn where `full_circle`. An extremum within the band of a
/// bound is that bound, which reads its side already.
///
/// # Errors
///
/// As [`sector_shape`]'s arm rungs; a window between distinct edges
/// whose width is not decided positive, or a reading in the band, as
/// [`SectorRung::Straight`] (whether the corner folds back on itself).
fn apex_shape<T: Decide>(
    dir_own: Vec3<T>,
    dir_next: Vec3<T>,
    apex: &crate::sector_face::ConeApex<T>,
    full_circle: bool,
    across: Vec3<T>,
    band: Band,
) -> Result<ApexShape<T>, SectorFault> {
    let arm = sector_arm(dir_own, dir_next, band)?;
    let rung = |diag| SectorFault::Rung {
        rung: SectorRung::Straight { full_circle },
        diag,
    };
    let axis = apex.axis;
    let (unit_own, unit_next) = (dir_own.normalize(), dir_next.normalize());
    let sin_a = apex.half_angle.sin();
    let tau = T::tau();
    let azimuth = |d: Vec3<T>| apex.azimuth_from(unit_next, d);
    // The chord between the points a lever out along two rulings
    // `angle` apart.
    let chord = |angle: T| {
        Margin::levered(
            (angle * T::from_f64(0.5)).sin() * sin_a * T::from_f64(2.0),
            metering_arm(arm),
        )
    };
    let window = if full_circle {
        tau
    } else {
        let w = azimuth(unit_own);
        decide_positive(SECTOR_APEX_WINDOW, chord(w), band).map_err(rung)?;
        w
    };
    let n_r = across - axis * across.dot(axis);
    if decide(
        SECTOR_APEX_TILT,
        Margin::levered(n_r.norm() * sin_a, metering_arm(arm)),
        band,
    )
    .map_err(rung)?
        != Sign::Positive
    {
        return Ok(ApexShape {
            arm,
            window,
            whole_turn: full_circle,
            rulings: Vec::new(),
        });
    }
    // The apex lane's own extremal rulings, each turned onto the face's
    // nappe: the line along a ruling of the other nappe holds this
    // nappe's ruling opposite it in azimuth, the other extremum.
    let [toward, away] = geom_brep::plane_cone_apex_extrema(&apex.surface(), across)
        .unwrap_or_else(|| unreachable!("an apex reading's surface is a cone"))
        .map(|g| apex.onto_nappe_of(g, unit_next));
    let (psi_toward, psi_away) = (azimuth(toward), azimuth(away));
    // In orbit order: from the own bound, the larger azimuth, first.
    let later_first = psi_away - psi_toward;
    let mut rulings = Vec::with_capacity(2);
    for (g, psi) in [
        (
            Vec3::select(later_first, toward, away),
            psi_toward.max(psi_away),
        ),
        (
            Vec3::select(later_first, away, toward),
            psi_toward.min(psi_away),
        ),
    ] {
        let from_next = decide(SECTOR_APEX_WINDOW, chord(psi), band).map_err(rung)?;
        let to_own = decide(SECTOR_APEX_WINDOW, chord(window - psi), band).map_err(rung)?;
        if from_next == Sign::Positive && to_own == Sign::Positive {
            rulings.push(g);
        }
    }
    Ok(ApexShape {
        arm,
        window,
        whole_turn: full_circle,
        rulings,
    })
}

/// [`apex_shape`], with a window that reads no width read by `bounds`
/// ([`reads_slit_tip_turn`]): at a slit's tip the face's whole turn round
/// the apex, otherwise the refusal.
///
/// # Errors
///
/// As [`apex_shape`], but for the whole turn at a slit's tip.
pub(crate) fn apex_corner_shape<T: Decide>(
    (dir_own, dir_next): (Vec3<T>, Vec3<T>),
    apex: &crate::sector_face::ConeApex<T>,
    bounds: CornerBounds,
    across: Vec3<T>,
    band: Band,
) -> Result<ApexShape<T>, SectorFault> {
    let shape = |full_circle| apex_shape(dir_own, dir_next, apex, full_circle, across, band);
    match shape(bounds == CornerBounds::OneEdge) {
        Err(fault) if reads_slit_tip_turn(bounds, &fault, Sign::Zero) => shape(true),
        read => read,
    }
}

/// **Whether a refused corner is a slit tip's whole turn**: `bounds`
/// is a slit's tip, and `fault` is the straight rung's rejection of the
/// reading `along`, which says the corner's bounds leave along one ray.
/// `along` is `Positive` for a tangent plane's `cos θ = +1`
/// ([`corner_shape`]) and `Zero` for an apex window's chord
/// ([`apex_corner_shape`]). Between distinct edges the same rejection is
/// a spike, and it stays refused.
fn reads_slit_tip_turn(bounds: CornerBounds, fault: &SectorFault, along: Sign) -> bool {
    bounds == CornerBounds::SlitTip
        && matches!(
            fault,
            SectorFault::Rung {
                rung: SectorRung::Straight { .. },
                diag,
            } if diag.margin.rejected_sign() == Some(along)
        )
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Point3, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn shape(
        own: Vec3<f64>,
        next: Vec3<f64>,
        full_circle: bool,
    ) -> Result<SectorShape<f64>, SectorFault> {
        let n = OutwardNormal::from_chart(Vec3::new(0.0, 0.0, 1.0), true);
        sector_shape(own, next, n, full_circle, band())
    }

    /// `Vec3` carries no `PartialEq`; these rows want BIT equality, not
    /// a tolerance.
    fn bits_eq(a: Vec3<f64>, b: Vec3<f64>) -> bool {
        a.x.to_bits() == b.x.to_bits()
            && a.y.to_bits() == b.y.to_bits()
            && a.z.to_bits() == b.z.to_bits()
    }

    /// At a cone's apex the window between a slit's two coincident
    /// rulings reads no width: between distinct edges it refuses, at a
    /// slit's tip it is the whole turn, subdivided at both extremal
    /// rulings. A corner between two rulings apart reads the same
    /// whichever bounds it is given, so a tip's reading reaches no other
    /// corner of its face.
    #[test]
    fn an_apex_window_of_no_width_is_a_slit_tips_whole_turn() {
        let apex = crate::sector_face::ConeApex {
            apex: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            half_angle: core::f64::consts::FRAC_PI_4,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
            sense: true,
        };
        let across = Vec3::new(0.3, 1.0, -0.2);
        let read =
            |own, next, bounds| apex_corner_shape((own, next), &apex, bounds, across, band());
        let ruling = Vec3::new(0.0, 1.0, 1.0);
        let refused = read(ruling, ruling, CornerBounds::Distinct)
            .expect_err("a window of no width between distinct edges");
        assert!(
            matches!(
                refused,
                SectorFault::Rung {
                    rung: SectorRung::Straight { full_circle: false },
                    ..
                }
            ),
            "the window's own rung refuses: {refused:?}"
        );
        let turn = read(ruling, ruling, CornerBounds::SlitTip).expect("the slit tip's whole turn");
        assert_eq!(
            turn.rulings.len(),
            2,
            "both extremal rulings lie inside the turn: {turn:?}"
        );
        let apart = Vec3::new(1.0, 0.0, 1.0);
        for (own, next) in [(apart, ruling), (ruling, apart)] {
            let (distinct, tip) = (
                read(own, next, CornerBounds::Distinct).unwrap(),
                read(own, next, CornerBounds::SlitTip).unwrap(),
            );
            assert!(
                distinct.arm == tip.arm
                    && distinct.rulings.len() == tip.rulings.len()
                    && distinct
                        .rulings
                        .iter()
                        .zip(&tip.rulings)
                        .all(|(&a, &b)| bits_eq(a, b)),
                "a corner between rulings apart reads alike under either bounds: {distinct:?} \
                 against {tip:?}"
            );
            assert_eq!(
                distinct.rulings.len(),
                1,
                "one extremal ruling lies inside it"
            );
        }
    }

    /// **A rung's refusal of a definite verdict is on the frame's
    /// escalation log**, beside that verdict: the funnel mints it, so a
    /// bracket around the call holds exactly the escalation the caller
    /// receives. One row per gate — a collapsed chord at the arm rung,
    /// a spike between distinct edges at the straightness rung.
    /// **The straightness margin's sign is the recourse's polarity.** A
    /// corner whose bounds are too short to read decides `sector_straight`
    /// in band; the Boolean's ending offers a tighter tolerance exactly
    /// when a tighter one would decide the corner PASSING — a negative
    /// `cos θ`, a straight corner — and nothing on the spike side, where
    /// no tolerance makes it pass. Flipping the metered sign here reds
    /// this row while the ending table still reads `Negative`.
    #[test]
    fn a_straight_corner_offers_to_tighten_and_a_spike_does_not() {
        // Arm just past the escalation band, so `sin θ` and `cos θ` at
        // 45° off either axis both land in band.
        let arm = 1.2 * band().escalate();
        let next = Vec3::new(arm, 0.0, 0.0);
        for (theta_deg, offers) in [(135.0_f64, true), (45.0, false)] {
            let theta = theta_deg.to_radians();
            let own = Vec3::new(arm * theta.cos(), arm * theta.sin(), 0.0);
            let Err(SectorFault::Rung { rung, diag }) = shape(own, next, false) else {
                panic!("{theta_deg}°: an in-band corner is a rung refusal");
            };
            assert_eq!(
                rung,
                SectorRung::Straight { full_circle: false },
                "{theta_deg}°"
            );
            let text = crate::boolean::refusal_routes::BooleanDecision::Corner(rung)
                .written(&diag, Some(geom_brep::recourse::Reading::Build.into()))
                .to_string();
            assert_eq!(
                text.contains("tighten the tolerance"),
                offers,
                "{theta_deg}°: {text}"
            );
        }
    }

    /// **The anti-re-fork row.** The three sector-shape K names are
    /// decided in `geom_brep::sector_shape` and nowhere else in that
    /// crate or this one, and the six per-lane names they replaced
    /// (#652) are decided NOWHERE at all: no file under `geom-brep/src`
    /// or `topo/src` but the home may spell the first set as a string
    /// literal, and none may spell the second. Re-forking the rungs means re-introducing a
    /// `decide("…_sector_arm", …)` somewhere, and that is exactly what
    /// this reads for.
    ///
    /// Both rosters are BUILT from the rung words rather than written
    /// out, so the check covers this file like any other — a
    /// literal roster would be its own first counter-example, which is
    /// exactly how this row went red on its own module while being
    /// written.
    ///
    /// **What the derivation does NOT buy, said plainly because the
    /// obvious reading is the reverse of the truth.** It does not keep
    /// the two rosters in step across a future rename; it is what
    /// breaks them. Rename a rung and `retired` becomes
    /// `"bool_<newname>"` / `"split_<newname>"`, strings that have
    /// never existed anywhere, so the six genuinely retired spellings
    /// stop being guarded — silently, with nothing going red. #652 is
    /// itself the proof that renames happen. **Nothing in the tree
    /// protects the retired set across one.** Those six spellings
    /// survive only in prose (`docs/K-REPORT.md`'s census note) and in
    /// the committed CSVs they date. Whoever renames a rung next has to carry the old
    /// spellings forward by hand — and has to assemble them from parts
    /// the way this does, or this file becomes the counter-example
    /// again.
    ///
    /// It walks the whole of both crates' `src/` at RUNTIME rather than
    /// `include_str!`ing the two lane files it happens to know about,
    /// so a re-fork that grows in a third file — a new module under
    /// `boolean/` or `splitting/`, or a fresh crate-root sibling — is
    /// caught too, and the guard does not need editing when a lane is
    /// split across more files.
    ///
    /// **What replaced the second guard.** Before #652 this row had a
    /// sibling, `both_lanes_decide_the_same_shape`, which ran every
    /// input shape under both lanes' name sets and demanded
    /// bit-identical results — the guard against a re-fork from INSIDE
    /// the shared body (`names.arm == "…"`, a lane flag, a per-lane
    /// band). Pooling deleted its subject: [`sector_shape`] no longer
    /// takes a lane parameter, so there is nothing for the body to
    /// branch on and nothing for that row to compare. It is gone
    /// rather than trivially green.
    ///
    /// **The residue #647 shipped is closed.** Its third uncovered
    /// case — a lane re-implementing the rungs while IMPORTING the
    /// names, `decide(BOOL_SECTOR_PREDICATES.arm, …)`, which no string
    /// search can see — cannot happen: the three consts are private to
    /// this module, so no other file can name them at all.
    ///
    /// **What this still does not cover** (stated plainly rather than
    /// as one careful caveat that implies the rest is covered — four
    /// shapes, two of them found in review of #661):
    ///
    /// 1. **A re-fork under FRESH names.** Out of reach by
    ///    construction; that one surfaces as new rows in the
    ///    `docs/K-REPORT.md` census, which is the mechanism that
    ///    already exists for it.
    /// 2. **These names spelled outside `geom-brep/src` and
    ///    `topo/src`.** The walk is scoped to the two crates, and any
    ///    crate can call `geom_core::k_stats::decide` directly.
    /// 3. **A lane that CALLS [`sector_shape`] and then re-derives the
    ///    verdict from what comes back.** `arm`, `unit_own` and
    ///    `unit_next` are `pub` fields of [`SectorShape`], so a
    ///    lane can recompute `sin θ` or its own bisector and act on
    ///    that instead of on `bisector`. The rungs still fire exactly
    ///    once each, from here, so the K stream stays byte-identical,
    ///    the census shows nothing new, and no string search can see
    ///    it. It is the closed residue's shape one level in: the names
    ///    are no longer importable, but the rungs' INPUTS are.
    /// 4. **A re-fork that decides with NO K name at all** — a raw
    ///    comparison where a `decide` call belongs. Residue 1's
    ///    mitigation cannot reach this one: new census rows are how a
    ///    freshly-named re-fork surfaces, and this emits no row. What
    ///    catches it is review, or a hole noticed later in the census.
    ///
    /// **A permanent constraint on prose, not just on code.** The check
    /// is textual and matches the QUOTED forms, so it is not only a
    /// `decide` call that reds it: any doc comment anywhere under
    /// `geom-brep/src` or `topo/src` that spells a retired name inside real quote marks —
    /// documenting what was retired, say — fails this row. Backticks
    /// are fine, and are what the module docs above use throughout.
    /// That constraint is the price of the check being able to cover
    /// its own home file, and it does not expire.
    #[test]
    fn the_rungs_are_decided_in_one_place() {
        let pooled: Vec<String> = ["arm", "reflex", "straight"]
            .iter()
            .map(|rung| format!("sector_{rung}"))
            .collect();
        // The retired six, each WITH its surrounding quotes — assembled
        // rather than spelled, so this file is subject to the check like
        // any other.
        let retired: Vec<String> = ["bool", "split"]
            .iter()
            .flat_map(|lane| pooled.iter().map(move |rung| format!("\"{lane}_{rung}\"")))
            .collect();
        let geom_brep = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../geom-brep/src");
        let home = geom_brep.join("sector_shape.rs");
        let mut files = crate::source_walk::crate_sources();
        let mut dirs = vec![geom_brep];
        while let Some(dir) = dirs.pop() {
            for entry in std::fs::read_dir(&dir).expect("a readable source directory") {
                let path = entry.expect("a readable directory entry").path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    files.push(path);
                }
            }
        }
        assert!(
            files.contains(&home),
            "the walk found {} file(s) but not the home module — it is not reading geom-brep/src",
            files.len()
        );
        for path in &files {
            let text = std::fs::read_to_string(path).expect("a readable source file");
            for name in &retired {
                assert!(
                    !text.contains(name.as_str()),
                    "{} decides the RETIRED per-lane sector predicate {name} — #652 \
                     pooled the six lane names into three. Call `sector_shape`, which \
                     spells the pooled name itself.",
                    path.display()
                );
            }
            if path == &home {
                continue;
            }
            for name in &pooled {
                assert!(
                    !text.contains(&format!("\"{name}\"")),
                    "{} names the sector-shape predicate `{name}` again — the rungs \
                     have been re-forked out of geom_brep::sector_shape, which must hold \
                     the only implementation. Call `sector_shape` instead.",
                    path.display()
                );
            }
        }
        // The guard earns its line only if the strings it looks for are
        // reachable at all: they must all be spelled in the home.
        let here = std::fs::read_to_string(&home).expect("the home module is readable");
        for name in &pooled {
            assert!(
                here.contains(&format!("\"{name}\"")),
                "`{name}` is not spelled in geom_brep::sector_shape"
            );
        }
    }
}
