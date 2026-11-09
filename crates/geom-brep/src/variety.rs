//! **An analytic piece against a carrier's variety**: the one statement
//! of the polynomial-identity certificate a boolean's coincidence readings
//! rest on.
//!
//! **The argument.** Along a piece whose parameterisation is real-analytic
//! on a connected parameter set, an analytic carrier's implicit equation
//! is a real-analytic function of the piece's parameters. Zero over any
//! open set of the piece makes it zero over the whole piece, so the piece
//! then lies on the carrier's whole real variety. One point of the piece
//! decided off that variety therefore certifies the piece lies on the
//! carrier along no part of it. The pieces read here, in two forms:
//! - **a polynomial or trigonometric piece**, where the equation along it
//!   is itself a polynomial (or a trigonometric one): a line, a conic, one
//!   Bézier piece of a spline. Across a piece's boundary the polynomial
//!   changes, so each piece is read on its own, and one read off says
//!   nothing of another;
//! - **a spiric oval**, an irreducible quartic whose parameterisation is
//!   real-analytic in its parameter over the whole oval, so the whole
//!   oval is one piece.
//!
//! Readers cite this statement rather than restating it.
//!
//! The decided residual ([`crate::implicit::implicit_residual`]) is read as
//! "off the variety", so the certificate holds only for a carrier whose
//! residual vanishes on its whole real variety ([`Variety::of`]): a
//! plane, a cylinder, a sphere, a cone (whose residual reads both nappes,
//! `|h|`) and a ring torus (`R > r`, decided: the torus's convention, not
//! its check). A spindle's residual
//! vanishes on one of its torus's two sheets while the quartic vanishes on
//! both: a piece on the read sheet over an open set and running on into
//! the other reads off there, so the spindle is out (a [`Surface::Spindle`],
//! or a [`Surface::Torus`] not decided a ring), as are a spline sheet and
//! an approximation, whose residuals are no variety's.
//!
//! The certificate reads the OFF side only. Where every point reads on,
//! the points say nothing of the piece between them ([`Piece::Sampled`]);
//! a reader that certifies a piece ON bounds it between its samples itself.
//! The sample schedule is the caller's: which points of a piece it reads,
//! and how many.

use geom::Surface;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign};

use crate::ladder::{IdentityLadder, read_identity};

/// What a piece's points read against a carrier ([`Variety::read_piece`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Piece {
    /// A point decided off the variety: the piece lies on the carrier
    /// along no part of it (module docs).
    Off,
    /// Every point decided on: nothing is certified between them.
    Sampled,
}

/// **A carrier the certificate holds for** (module docs): made only by
/// [`Self::of`], which decides the carrier-kind gate, so a piece is read
/// against a variety only once that gate has passed.
#[derive(Clone, Copy, Debug)]
pub struct Variety<'s, T: geom_core::Real> {
    carrier: &'s Surface<T>,
}

impl<'s, T: Decide> Variety<'s, T> {
    /// **`carrier`, where its decided residual vanishes on its whole real
    /// variety** (module docs): a plane, a cylinder, a sphere, a cone, and
    /// a torus whose `R − r` is decided positive under `ring`. `Ok(None)`
    /// for every other kind, and for a torus decided `R ≤ r`. The one list
    /// of the carriers the certificate holds for, matched exhaustively so a
    /// new kind decides here.
    ///
    /// # Errors
    ///
    /// The ring decision's escalation, where a torus's `R − r` is in band:
    /// a reader escalates there rather than reading the torus as no member.
    pub fn of(
        carrier: &'s Surface<T>,
        ring: &'static str,
        band: Band,
    ) -> Result<Option<Self>, Indeterminate> {
        let whole = match *carrier {
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Sphere { .. }
            | Surface::Cone { .. } => true,
            Surface::Torus {
                major_radius,
                minor_radius,
                ..
            } => decide(ring, Margin::of(major_radius - minor_radius), band)? == Sign::Positive,
            Surface::Spindle { .. } | Surface::Nurbs(_) | Surface::Approx(_) => false,
        };
        Ok(whole.then_some(Self { carrier }))
    }

    /// **A piece's `points` read against the variety** (module docs):
    /// [`Piece::Off`] where one point's residual decides off under `site`,
    /// whatever the others read; [`Piece::Sampled`] where every one decides
    /// on.
    ///
    /// # Errors
    ///
    /// The first undecided reading, where no point decides off and one
    /// does not decide.
    pub fn read_piece(
        &self,
        points: &[Point3<T>],
        site: &'static str,
        band: Band,
    ) -> Result<Piece, Indeterminate> {
        let residuals = points.iter().map(|&q| {
            (
                site,
                Margin::of(crate::implicit::implicit_residual(self.carrier, q)),
            )
        });
        match read_identity(residuals, band) {
            IdentityLadder::Apart { .. } => Ok(Piece::Off),
            IdentityLadder::Zero(_) => Ok(Piece::Sampled),
            IdentityLadder::Undecided(cause) => Err(cause),
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod rows {
    use geom::Surface;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{Piece, Variety};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn torus(major_radius: f64, minor_radius: f64) -> Surface<f64> {
        Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius,
            minor_radius,
            u_ref: Vec3::unit_x(),
        }
    }

    /// **The gate holds the carriers whose residual is their whole
    /// variety, and no other**: the four quadrics and a ring torus pass; a
    /// torus with `R ≤ r` and a spindle sheet do not. Red where a kind
    /// slips in or out of the list.
    #[test]
    fn the_gate_admits_the_whole_variety_kinds_only() {
        let o = Point3::origin();
        let (x, z) = (Vec3::unit_x(), Vec3::unit_z());
        let admitted = [
            Surface::Plane {
                origin: o,
                normal: z,
                u_ref: x,
            },
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: 1.0,
                u_ref: x,
            },
            Surface::Sphere {
                center: o,
                radius: 1.0,
                u_ref: x,
                axis: z,
            },
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.5,
                u_ref: x,
            },
            torus(2.0, 1.0),
        ];
        for s in &admitted {
            assert!(
                matches!(Variety::of(s, "variety_row_ring", band()), Ok(Some(_))),
                "{s:?}"
            );
        }
        let refused = [
            torus(1.0, 1.0),
            torus(0.5, 1.0),
            Surface::Spindle {
                center: o,
                axis: z,
                major_radius: 0.5,
                minor_radius: 1.0,
                u_ref: x,
            },
        ];
        for s in &refused {
            assert!(
                matches!(Variety::of(s, "variety_row_ring", band()), Ok(None)),
                "{s:?}"
            );
        }
    }

    /// **A torus whose `R − r` is in band is neither a member nor not**:
    /// the ring decision's escalation, which a reader escalates on.
    #[test]
    fn a_torus_whose_ring_is_in_band_escalates() {
        let b = band();
        let gap = 0.5 * (b.zero() + b.escalate());
        assert!(Variety::of(&torus(1.0 + gap, 1.0), "variety_row_ring", b).is_err());
    }

    /// **One point off reads the piece off, whatever the others read; all
    /// on reads only the samples**: a point on the cylinder beside one off
    /// it is `Off`, and points all on it are `Sampled`, never `Off`.
    #[test]
    fn one_point_off_reads_the_piece_off() {
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let variety = Variety::of(&cyl, "variety_row_ring", band())
            .unwrap()
            .unwrap();
        let (on, off) = (Point3::new(1.0, 0.0, 0.5), Point3::new(2.0, 0.0, 0.5));
        assert_eq!(
            variety.read_piece(&[on, off], "variety_row_off", band()),
            Ok(Piece::Off)
        );
        assert_eq!(
            variety.read_piece(
                &[on, Point3::new(0.0, 1.0, -3.0)],
                "variety_row_off",
                band()
            ),
            Ok(Piece::Sampled)
        );
    }
}
