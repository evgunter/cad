//! **The cuts at a vertex and their germs** (`star`'s module docs, "The
//! cut"): where a verb's sides of a datum change round the star, and the
//! direction the datum's section leaves the vertex along inside each such
//! sector.

use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Real, Sign, Vec3};

use super::Star;

/// K name: a cut's section line, `|n_s × n_d|` levered at the sector's
/// farther reach (metres): the sector's tilt against the datum. Zero is a
/// sector lying in the datum, which has no line.
pub(crate) const GERM_LINE: &str = "germ_line";

/// Which bound of a cut's sector lies on the datum's positive side (the
/// split's Above, the Boolean's Out): its CCW-first bound (`Start`, bound
/// `k + 1` of sector `k`) or its CCW-last (`End`, bound `k`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CutEnd {
    /// The sector's CCW-first bound.
    Start,
    /// The sector's CCW-last bound.
    End,
}

/// **A cut**: a sector whose two bounds a verb reads on opposite sides of
/// its datum, and which of them lies on the datum's positive side. Its
/// germ is the section line's ray [`germ_line`] names by that end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Cut {
    /// The sector, between bound `sector` (its end) and bound
    /// `sector + 1` (its start).
    pub sector: usize,
    /// Its bound on the datum's positive side.
    pub above: CutEnd,
}

/// **The sides → cuts reading, the one place**: of `n` bounds, each
/// sector whose two bounds `above` reads on opposite sides, in sector
/// order. `above(k)` is bound `k`'s side after the verb's folds (rule (a)
/// and (b), the lumps, the On resolution): a verb whose runs move the
/// negative side passes the positive side all the same, so a cut names the
/// datum's side, never the verb's.
pub(crate) fn cuts(n: usize, above: impl Fn(usize) -> bool) -> Vec<Cut> {
    (0..n)
        .filter_map(|k| {
            let (end, start) = (above(k), above((k + 1) % n));
            apart(Some(end), Some(start)).then_some(Cut {
                sector: k,
                above: if start { CutEnd::Start } else { CutEnd::End },
            })
        })
        .collect()
}

/// **Whether a cut lies between two bounds read so**: `Some(above)` a
/// strict side of the datum, `None` in it. A cut lies between two
/// bounds read strictly apart; a bound in the datum bounds none (the
/// datum runs along it). [`cuts`] reads folded sides, where every bound is
/// strict, and is this test round the star.
pub(crate) fn apart(a: Option<bool>, b: Option<bool>) -> bool {
    matches!((a, b), (Some(x), Some(y)) if x != y)
}

impl<T: Real> Star<T> {
    /// [`cuts`] over the star's bounds.
    pub(crate) fn cuts(&self, above: impl Fn(usize) -> bool) -> Vec<Cut> {
        cuts(self.bounds.len(), above)
    }

    /// The farther of sector `k`'s two bounds' reaches: how far from the
    /// vertex the sector's own geometry is known, where [`germ_line`]
    /// levers its line.
    pub(crate) fn span(&self, k: usize) -> Lever<T> {
        self.bounds[k].reach.lever().max(self.start(k).reach.lever())
    }
}

/// **A unit direction and its lean** `(0, 1]`: a germ read as two faces'
/// line carries `|n_a × n_b|`, how steeply a tilt of either face turns it,
/// and a reading of its side is levered by its lean
/// (`boolean::sectors::leaned_lever`'s doctrine); a direction read
/// off one carrier (a bound, a ruling) leans 1.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Leaned<T: Real> {
    /// The unit direction.
    pub dir: Vec3<T>,
    /// Its lean, in `(0, 1]`.
    pub lean: T,
}

impl<T: Real> Leaned<T> {
    /// A direction read off one carrier: lean 1.
    pub(crate) fn upright(dir: Vec3<T>) -> Self {
        Self {
            dir,
            lean: T::from_f64(1.0),
        }
    }

    /// Two faces' line from its un-normalized cross `m = n_a × n_b`.
    pub(crate) fn of_line(m: Vec3<T>) -> Self {
        Self {
            dir: m.normalize(),
            lean: m.norm(),
        }
    }

    /// The direction scaled by its lean: two faces' line un-normalized.
    pub(crate) fn line(self) -> Vec3<T> {
        self.dir * self.lean
    }

    /// The opposite ray, its lean unchanged.
    pub(crate) fn reversed(self) -> Self {
        Self {
            dir: -self.dir,
            lean: self.lean,
        }
    }
}

/// Why a cut's sector has no section line ([`germ_line`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum GermLineFault {
    /// The sector's tangent plane is the datum's, decided so: no line.
    Coplanar,
    /// The tilt reads in the band.
    Undecided(Indeterminate),
}

/// **A cut's germ in a tangent-plane sector, the one rule**: the section
/// line `n_s × n_d` of the sector's tangent plane (outward normal `n_s`)
/// and the datum's (`n_d`), signed by the cut's codes ([`CutEnd`]), with
/// its lean.
///
/// **The sign is the transition's, not a second reading.** The sector
/// sweeps CCW about `n_s` from its start to its end, and its plane's
/// directions on the datum's positive side are the half-plane about `u`,
/// `n_d`'s component in that plane. A quarter turn CCW carries `u` to
/// `n_s × u = n_s × n_d`, so a sweep starting above the section line
/// leaves that half-plane along `n_s × n_d`, and one starting below enters
/// it along `−n_s × n_d`; a sweep under a half-turn (every tangent-plane
/// sector of a star) crosses the line once. A start above gives
/// `n_s × n_d`, an end above its negation. Where a bound the verb read in
/// the datum folded to one side, the line runs along that bound to within
/// the band and, the sector being convex, the ray this picks is the one
/// along it: the germ the fold put there.
///
/// The line is levered at the sector's farther reach `span`
/// ([`Star::span`]): a cut sector has a bound read definitely off the
/// datum, beyond `K·zero`, at its reach `L`, which is at most
/// `L·|n_s × n_d|` plus the departures of the vertices it is taken
/// between from the sector's own plane, each up to `zero`. So this gate
/// reads at least `(K − 2)·zero`, in the band only where the sector's
/// tilt against the datum is undecided.
///
/// # Errors
///
/// [`GermLineFault`].
pub(crate) fn germ_line<T: Decide>(
    face: Vec3<T>,
    datum: Vec3<T>,
    above: CutEnd,
    span: Lever<T>,
    band: Band,
) -> Result<Leaned<T>, GermLineFault> {
    let int = face.cross(datum);
    match crate::validate::decide(GERM_LINE, Margin::levered(int.norm(), span), band) {
        Ok(Sign::Positive) => Ok(Leaned::of_line(match above {
            CutEnd::Start => int,
            CutEnd::End => -int,
        })),
        Ok(_) => Err(GermLineFault::Coplanar),
        Err(diag) => Err(GermLineFault::Undecided(diag)),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// **Sides → cuts names the datum's side of each changing sector**:
    /// sector `k` runs from bound `k + 1` (start) to bound `k` (end).
    #[test]
    fn a_cut_names_the_bound_on_the_datums_positive_side() {
        let sides = [true, true, false, false, true];
        let cuts = cuts(sides.len(), |k| sides[k]);
        assert_eq!(
            cuts,
            vec![
                Cut {
                    sector: 1,
                    above: CutEnd::End
                },
                Cut {
                    sector: 3,
                    above: CutEnd::Start
                },
            ],
            "sector 1 ends above and starts below; sector 3 starts above; sector 4 \
             (bound 4 to bound 0) is above throughout"
        );
        assert!(cuts_of_one_side_are_none(), "a star on one side has no cut");
    }

    fn cuts_of_one_side_are_none() -> bool {
        cuts(4, |_| false).is_empty() && cuts(4, |_| true).is_empty()
    }

    /// **The germ is the ray inside the sector**, for every sector under a
    /// half-turn crossing the datum, either way round: the rule's sign
    /// (module docs) checked against membership by angle.
    #[test]
    fn the_germ_line_is_the_ray_inside_the_cut_sector() {
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let n_s = Vec3::new(0.0, 0.0, 1.0_f64);
        let span = Lever::unsourced(1.0);
        let mut checked = 0;
        for tilt in [0.2_f64, 0.9, 1.4] {
            let datum = Vec3::new(0.0, tilt.cos(), tilt.sin());
            for a in 0..24 {
                for w in 1..12 {
                    let (t0, t1) = (
                        f64::from(a) * std::f64::consts::TAU / 24.0,
                        f64::from(a + w) * std::f64::consts::TAU / 24.0,
                    );
                    let dir = |t: f64| Vec3::new(t.cos(), t.sin(), 0.0);
                    let (start, end) = (dir(t0), dir(t1));
                    let (s, e) = (start.dot(datum), end.dot(datum));
                    if s.abs() < 1e-9 || e.abs() < 1e-9 || (s > 0.0) == (e > 0.0) {
                        continue;
                    }
                    let above = if s > 0.0 { CutEnd::Start } else { CutEnd::End };
                    let g = germ_line(n_s, datum, above, span, band).unwrap().dir;
                    let t = g.y.atan2(g.x).rem_euclid(std::f64::consts::TAU);
                    let inside = (t - t0).rem_euclid(std::f64::consts::TAU) <= t1 - t0 + 1e-12;
                    assert!(
                        inside,
                        "tilt {tilt}: the germ at {t} lies outside the sector {t0}..{t1} \
                         ({above:?} above)"
                    );
                    checked += 1;
                }
            }
        }
        assert!(checked > 100, "the sweep reached {checked} cut sectors");
    }
}
