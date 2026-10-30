//! **A face's chart image**: its loops with every spur taken out, an
//! edge walked out and straight back, which bounds no area. Seams stay:
//! a Green form written in a lifted chart coordinate (the torus's tube
//! angle) reads a seam's two traversals a period apart, and that term is
//! the face's area. Every integrator and chart reader reads this view.
//!
//! Which traversals are one edge walked back is structure, read by the
//! caller from its own identities: half-edge keys and their mates in a
//! body (`topo::surface_boundary::chart_image`), `EDGE_CURVE` ids and
//! orientations in a STEP file (`step-import`'s normalization). This
//! home holds the two rules both read:
//!
//! - **the reduction** ([`without_reversals`]): a pair of consecutive
//!   traversals the caller reads as one walked back is taken out, again
//!   until none is left, the pairs that close across the cycle's start
//!   included, so a spur whose edge was split comes out inner reversal
//!   first;
//! - **the loops that bound** ([`bounds_area`]): a loop whose image is
//!   empty, all spurs, bounds nothing and is left out of every reading.
//!   A face none of whose loops bound is the whole of its surface where
//!   that surface is closed (`crate::props::whole_surface`).

use crate::props::LoopEdge;
use geom_core::Real;

/// **A cycle with its reversals taken out** (module docs): every pair of
/// consecutive members, in what is left, of which `back(first, second)`
/// says the second walks the first straight back, removed until none is
/// left, those that close across the cycle's start included. Empty where
/// the whole cycle is reversals.
pub fn without_reversals<E: Clone>(cycle: &[E], back: impl Fn(&E, &E) -> bool) -> Vec<E> {
    let mut kept: Vec<E> = Vec::with_capacity(cycle.len());
    for e in cycle {
        if kept.last().is_some_and(|k| back(k, e)) {
            kept.pop();
        } else {
            kept.push(e.clone());
        }
    }
    while kept.len() >= 2 && back(&kept[kept.len() - 1], &kept[0]) {
        kept.pop();
        kept.remove(0);
    }
    kept
}

/// **Whether a loop's chart image bounds area** (module docs): it is not
/// empty. A reader leaves every other loop out.
#[must_use]
pub fn bounds_area<T: Real>(image: &[LoopEdge<T>]) -> bool {
    !image.is_empty()
}

#[cfg(test)]
mod tests {
    use super::without_reversals;

    /// `(edge, forward)`: the second walks the first back where it is
    /// the same edge the other way.
    fn back(a: &(u8, bool), b: &(u8, bool)) -> bool {
        a.0 == b.0 && a.1 != b.1
    }

    /// A spur out of a loop of two edges, read from every member: the
    /// reduction answers the two edges in their cyclic order however the
    /// cycle is rotated, including where it starts on the spur's return
    /// and ends on its way out, which only the pass across the start
    /// takes out.
    #[test]
    fn a_spur_comes_out_from_every_starting_member() {
        let cycle = [(9, true), (9, false), (1, true), (2, true)];
        for k in 0..cycle.len() {
            let mut rotated = cycle.to_vec();
            rotated.rotate_left(k);
            let got = without_reversals(&rotated, back);
            assert_eq!(got.len(), 2, "from {k}: {got:?}");
            assert!(got.iter().all(|e| e.0 != 9), "from {k}: {got:?}");
        }
        let across = [(9, false), (1, true), (2, true), (9, true)];
        assert_eq!(
            without_reversals(&across, back),
            vec![(1, true), (2, true)],
            "a cycle that starts on the return half"
        );
    }

    /// A split spur (`a` out, `b` out, `b` back, `a` back) comes out
    /// inner reversal first, and a seam (`s` out, a rim, `s` back, a
    /// rim) is no consecutive reversal and stays.
    #[test]
    fn a_split_spur_goes_and_a_seam_stays() {
        let split = [(1, true), (2, true), (2, false), (1, false)];
        assert!(without_reversals(&split, back).is_empty());
        let seam = [(1, true), (5, true), (1, false), (6, true)];
        assert_eq!(without_reversals(&seam, back), seam.to_vec());
    }

    /// **An empty chart image, read by every door**: no loop, or only
    /// loops that bound nothing. On a sphere and a torus the closed form
    /// is the whole surface, its flux signed by the sense bit, and the
    /// side reading answers that the boundary encodes none; on a plane,
    /// a cylinder or a cone both refuse the face as degenerate.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn an_empty_image_is_the_whole_closed_surface_signed_by_its_sense() {
        use crate::props::{
            LoopEdge, MaterialSign, PropsError, boundary_material_sign_loops, curved_face_loops,
        };
        use core::f64::consts::PI;
        use geom::Surface;
        use geom_core::{Band, Point3, Tol, Vec3};
        let band = Band::linear(Tol::witness()).unwrap();
        let (o, z, x) = (
            Point3::new(1.0, 2.0, 3.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let (big, small, r) = (3.0, 0.5, 2.0);
        let sphere = Surface::Sphere {
            center: o,
            radius: r,
            axis: z,
            u_ref: x,
        };
        let torus = Surface::Torus {
            center: o,
            axis: z,
            major_radius: big,
            minor_radius: small,
            u_ref: x,
        };
        let empty: &[LoopEdge<f64>] = &[];
        for (what, s, flux, area) in [
            ("sphere", &sphere, 4.0 * PI * r.powi(3), 4.0 * PI * r * r),
            (
                "torus",
                &torus,
                6.0 * PI * PI * big * small * small,
                4.0 * PI * PI * big * small,
            ),
        ] {
            for loops in [&[][..], &[empty][..], &[empty, empty][..]] {
                for sense in [true, false] {
                    let got = curved_face_loops(s, loops, sense, band).unwrap();
                    let want = if sense { flux } else { -flux };
                    assert!(
                        (got.flux - want).abs() <= 1e-12 * flux,
                        "{what}, sense {sense}, {} loops: flux {} against {want}",
                        loops.len(),
                        got.flux
                    );
                    assert!((got.area - area).abs() <= 1e-12 * area, "{what}: area");
                }
                assert_eq!(
                    boundary_material_sign_loops(s, loops, band).unwrap(),
                    MaterialSign::Unencoded,
                    "{what}: an empty image encodes no side"
                );
            }
        }
        let open = [
            Surface::Plane {
                origin: o,
                normal: z,
                u_ref: x,
            },
            Surface::Cylinder {
                origin: o,
                axis: z,
                radius: r,
                u_ref: x,
            },
            Surface::Cone {
                apex: o,
                axis: z,
                half_angle: 0.5,
                u_ref: x,
            },
        ];
        for s in &open {
            assert!(
                matches!(
                    curved_face_loops(s, &[empty], true, band),
                    Err(PropsError::DegenerateFace)
                ),
                "{:?}: an empty image on an open surface bounds nothing",
                s.kind()
            );
            assert!(
                matches!(
                    boundary_material_sign_loops(s, &[], band),
                    Err(PropsError::DegenerateFace)
                ),
                "{:?}: and encodes no side of anything",
                s.kind()
            );
        }
    }
}
