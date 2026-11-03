//! **A cylinder or cone face along its own ruling, read by its chord**
//! ([`FaceSector::chord`], [`chords_clear`]), the edge lying in a planar
//! face of its own solid and judged against it ([`rest::judge`]): the
//! arc's side read where its tangent leaves the face's plane and, where
//! the tangent lies on it, by its bend; the cone as the cylinder; an
//! arc crossing the plane short of its chord, a tangent in band, an arc
//! in the plane, a sheet in the lens and an edge off the ruling refused.
//! Near the origin and a kilometre off it.

#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use core::f64::consts::{FRAC_PI_2, FRAC_PI_3, PI};

use geom_core::Tol;

use super::*;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the witness tolerance is a band")
}

fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

const Z: Vec3<f64> = Vec3::new(0.0, 0.0, 1.0);

/// Near the origin, and a kilometre off it.
const AT: [Point3<f64>; 2] = [
    Point3::new(0.0, 0.0, 0.0),
    Point3::new(1000.0, -1000.0, 1000.0),
];

/// The unit direction at angle `a` about `z` from `+x`.
fn dir(a: f64) -> Vec3<f64> {
    let (s, c) = a.sin_cos();
    v(c, s, 0.0)
}

fn plane(n: Vec3<f64>, p: Point3<f64>) -> geom::Surface<f64> {
    geom::Surface::Plane {
        origin: p,
        normal: n,
        u_ref: n.cross(Z).normalize(),
    }
}

/// A planar sheet through `p`: the half-plane off the line along `a`
/// to its left about `outward`, or the whole plane.
fn sheet(p: Point3<f64>, outward: Vec3<f64>, a: Vec3<f64>, half: bool) -> FaceSector<f64> {
    FaceSector {
        sector: Sector {
            outward,
            apex: p,
            wedges: fan(p, outward, a, 2.0, half),
            closed: !half,
        },
        face: LinkFace {
            surface: plane(outward, p),
            sense: true,
            clear: None,
            tangent: None,
        },
    }
}

/// The carrier a ruled face lies on, its ruling through `p` along `z`,
/// bending toward `bend` at `p` with radius `rho` across it.
#[derive(Clone, Copy)]
enum Carrier {
    Cylinder,
    /// A cone of this half-angle, its apex `3` below `p` on the ruling.
    Cone(f64),
}

impl Carrier {
    fn surface(self, p: Point3<f64>, bend: Vec3<f64>, rho: f64) -> geom::Surface<f64> {
        match self {
            Self::Cylinder => geom::Surface::Cylinder {
                origin: p + bend * rho,
                axis: Z,
                radius: rho,
                u_ref: -bend,
            },
            Self::Cone(alpha) => {
                let axis = Z * alpha.cos() + bend * alpha.sin();
                geom::Surface::Cone {
                    apex: p - Z * 3.0,
                    axis,
                    half_angle: alpha,
                    u_ref: axis.cross(bend.cross(axis)).normalize() * -1.0,
                }
            }
        }
    }
}

/// **One edge along `z` through `p` lying in the face `y = 0` there**,
/// outward `floor`. The edge's wedge runs counterclockwise about `z`
/// from its ruled face's tangent `dir(a)` to its planar face's ray
/// `dir(b)`; the ruled face (on `carrier`, radius `rho` across the edge)
/// bends into the wedge where `into`, out of it otherwise; its other
/// edges clear `p` by `clear`, and the edge runs between `ends`.
#[allow(clippy::too_many_arguments)] // the edge's geometry, its whole
fn judged(
    p: Point3<f64>,
    (a, b): (f64, f64),
    carrier: Carrier,
    into: bool,
    rho: f64,
    floor: Vec3<f64>,
    clear: f64,
    ends: [Point3<f64>; 2],
) -> TouchVerdict {
    let h = dir(a);
    // The wedge lies counterclockwise of `h`, behind the outward normal.
    let outward = -(Z.cross(h));
    let bend = if into { -outward } else { outward };
    let ruled = Ruled {
        surface: carrier.surface(p, bend, rho),
        sense: into,
        outward,
        along: -Z,
        ends,
        clear,
    };
    let d = dir(b);
    let read = || -> Result<TouchVerdict, TouchUnread> {
        let chord = FaceSector::chord(&ruled, p, floor, band())?;
        chords_clear(p, core::slice::from_ref(&chord), &[d], band())?;
        let edge = Link::of(p, vec![sheet(p, Z.cross(d), Z, true), chord.sector])?;
        let face = Link::of(p, vec![sheet(p, floor, v(1.0, 0.0, 0.0), false)])?;
        Ok(rest::judge(&edge, &face, band()))
    };
    read().unwrap_or_else(TouchVerdict::Unread)
}

/// [`judged`] with the radius `1` across the edge, its other edges `1`
/// off, from `2` below `p` to `2` above.
fn read(p: Point3<f64>, ab: (f64, f64), carrier: Carrier, into: bool, up: bool) -> TouchVerdict {
    let floor = v(0.0, if up { 1.0 } else { -1.0 }, 0.0);
    judged(p, ab, carrier, into, 1.0, floor, 1.0, [p - Z * 2.0, p + Z * 2.0])
}

fn rests(t: &TouchVerdict) -> bool {
    matches!(t, TouchVerdict::Rest(_))
}

const CARRIERS: [(&str, Carrier); 2] = [
    ("a cylinder", Carrier::Cylinder),
    ("a cone", Carrier::Cone(0.4)),
];

/// **A ruled face leaving the face across it reads by its chord.** The
/// edge's wedge from `60°` to `120°` about it, its ruled face bending
/// either way, on a cylinder or a cone: above the face `y = 0` whose
/// material is below, a rest; with the face's material above, a
/// crossing; the reflex wedge from `120°` round to `60°`, a crossing.
/// Red if the ruled face does not read (its sector refuses), or its
/// chord stands on the wrong side.
#[test]
fn a_ruled_face_leaving_the_face_across_it_reads_by_its_chord() {
    for p in AT {
        for (what, carrier) in CARRIERS {
            for into in [true, false] {
                let at = format!("{what} bending into the wedge {into}, at {p:?}");
                let up = (FRAC_PI_3, 2.0 * FRAC_PI_3);
                assert!(rests(&read(p, up, carrier, into, true)), "{at}: rests");
                assert_eq!(
                    read(p, up, carrier, into, false),
                    TouchVerdict::Crossing,
                    "{at}: the face's material in the wedge"
                );
                assert_eq!(
                    read(p, (2.0 * FRAC_PI_3, FRAC_PI_3), carrier, into, true),
                    TouchVerdict::Crossing,
                    "{at}: the face's sheet in the wedge"
                );
            }
        }
    }
}

/// **A ruled face tangent to the face reads by its bend.** The tangent
/// along `+x`, in the face `y = 0` whose material is below, the wedge
/// to `90°`: bending up into the wedge, off the face, a rest; bending
/// down into the face's material, a crossing. The keel: the tangent
/// along `−x`, the wedge from it round through `−y` to `60°`, the face's
/// material above and the ruled face bending up into it, a crossing.
/// On a cylinder and a cone. Red if a tangent reading Zero is not read
/// by the point the arc bends to (refused, or read as its tangent's
/// half-plane, which lies on the face).
#[test]
fn a_ruled_face_tangent_to_the_face_reads_by_its_bend() {
    for p in AT {
        for (what, carrier) in CARRIERS {
            let at = format!("{what} at {p:?}");
            assert!(
                rests(&read(p, (0.0, FRAC_PI_2), carrier, true, true)),
                "{at}: bending up off the face rests"
            );
            assert_eq!(
                read(p, (0.0, FRAC_PI_2), carrier, false, true),
                TouchVerdict::Crossing,
                "{at}: bending down into the face's material"
            );
            assert_eq!(
                read(p, (PI, FRAC_PI_3), carrier, false, false),
                TouchVerdict::Crossing,
                "{at}: the keel bending into the face's material"
            );
        }
    }
}

/// **A tangent tilted under the band reads by its bend; one in the band
/// refuses.** The ruled face's tangent tilted `θ` below the face `y = 0`,
/// bending up off it into a wedge to `90°`: at a tilt whose rise at the
/// lever (`½`) is half the band's zero, the tangent reads Zero and the
/// bend decides, a rest (the arc dips under the face by `ρ·θ²`, far
/// inside the band, before it crosses back); at the band's geometric
/// middle, in band. A tangent decided below the face with the arc's
/// point decided above (`θ = 0.05`), the arc crossing the face short of
/// its chord, refuses in band too. Red if the tangent is read decided
/// past the band, or a decided pair that disagrees is read.
#[test]
fn a_tangent_tilted_under_the_band_reads_by_its_bend() {
    let band = band();
    let lever = 0.5;
    let under = (0.5 * band.zero() / lever).asin();
    let inside = ((band.zero() * band.escalate()).sqrt() / lever).asin();
    for p in AT {
        for (what, carrier) in CARRIERS {
            let at = format!("{what} at {p:?}");
            assert!(
                rests(&read(p, (-under, FRAC_PI_2), carrier, true, true)),
                "{at}: tilted {under:e}, under the band"
            );
            assert_eq!(
                read(p, (-inside, FRAC_PI_2), carrier, true, true),
                TouchVerdict::Unread(TouchUnread::InBand),
                "{at}: tilted {inside:e}, in band"
            );
            assert_eq!(
                read(p, (-0.05, FRAC_PI_2), carrier, true, true),
                TouchVerdict::Unread(TouchUnread::InBand),
                "{at}: the arc crosses the face short of its chord"
            );
        }
    }
}

/// **An arc lying in the face to its chord refuses as two sheets on
/// each other.** A cylinder tangent to the face `y = 0`, its radius so
/// large that the arc's point at the lever rises half the band's zero
/// off the face: `Degenerate`. Red if a Zero pair is read.
#[test]
fn an_arc_in_the_face_to_its_chord_refuses_degenerate() {
    let lever = 0.5;
    let rho = lever * lever / band().zero();
    for p in AT {
        assert_eq!(
            judged(
                p,
                (0.0, FRAC_PI_2),
                Carrier::Cylinder,
                true,
                rho,
                v(0.0, 1.0, 0.0),
                1.0,
                [p - Z * 2.0, p + Z * 2.0]
            ),
            TouchVerdict::Unread(TouchUnread::Degenerate),
            "at {p:?}"
        );
    }
}

/// **A sheet of the edge inside the lens refuses; one past it reads.**
/// The ruled face tangent along `+x`, bending up toward the edge's planar
/// face at `b`: the lens between the arc and its chord spans
/// `asin(¼) ≈ 0.25` about the edge at the lever `½` of radius `1`, so
/// the planar face at `0.05` lies inside it and refuses, and at `0.4`
/// lies past it and rests. Red if a sheet inside the lens is not held
/// out of it, or one past it is.
#[test]
fn a_sheet_inside_the_lens_refuses() {
    for p in AT {
        for (what, carrier) in CARRIERS {
            let at = format!("{what} at {p:?}");
            assert_eq!(
                read(p, (0.0, 0.05), carrier, true, true),
                TouchVerdict::Unread(TouchUnread::Unreadable),
                "{at}: the planar face inside the lens"
            );
            assert!(
                rests(&read(p, (0.0, 0.4), carrier, true, true)),
                "{at}: the planar face past the lens"
            );
        }
    }
}

/// **An edge off its face's ruling refuses.** The edge's far end `1e-3`
/// off the ruling through `p`, along the face's tangent or along its
/// outward normal: `Unreadable`. Red if the edge is taken for the ruling
/// unread.
#[test]
fn an_edge_off_the_ruling_refuses() {
    for p in AT {
        for (what, carrier) in CARRIERS {
            // Along the face's tangent at `60°`, and along its outward
            // normal there.
            for off in [dir(FRAC_PI_3) * 1e-3, dir(FRAC_PI_3 - FRAC_PI_2) * 1e-3] {
                assert_eq!(
                    judged(
                        p,
                        (FRAC_PI_3, 2.0 * FRAC_PI_3),
                        carrier,
                        true,
                        1.0,
                        v(0.0, 1.0, 0.0),
                        1.0,
                        [p - Z * 2.0, p + Z * 2.0 + off]
                    ),
                    TouchVerdict::Unread(TouchUnread::Unreadable),
                    "{what} at {p:?}, the end {off:?} off"
                );
            }
        }
    }
}

/// **The chord is the half-plane from the edge through the arc's real
/// point, oriented as the face.** A cylinder of radius `1` tangent along
/// `+x` at the origin, bending up: its point at the lever `½` lies on
/// the carrier and on the chord's plane, the chord's fan turns from the
/// edge toward it, its outward normal within a quarter turn of the
/// face's, and its ball is the lever. Red if the point leaves the
/// carrier, the chord turns off it, or its sense flips.
#[test]
fn the_chord_is_the_half_plane_through_the_arcs_point() {
    let p = Point3::origin();
    let outward = v(0.0, -1.0, 0.0);
    let ruled = Ruled {
        surface: Carrier::Cylinder.surface(p, -outward, 1.0),
        sense: false,
        outward,
        along: -Z,
        ends: [p - Z * 2.0, p + Z * 2.0],
        clear: 1.0,
    };
    let chord = FaceSector::chord(&ruled, p, v(0.0, 1.0, 0.0), band()).expect("a chord");
    let s = &chord.sector.sector;
    let q = p + chord.bounds[1] * 0.5;
    assert!(
        ((q - Point3::new(0.0, 1.0, 0.0)).norm() - 1.0).abs() < 1e-12,
        "the arc's point on the carrier: {q:?}"
    );
    assert!(s.outward.dot(q - p).abs() < 1e-12, "on the chord's plane");
    assert!(s.outward.dot(outward) > 0.9, "oriented as the face");
    let square = (s.wedges[1].1 - p).normalize();
    assert!(
        square.dot(chord.bounds[1]) > 1.0 - 1e-12,
        "the fan's quarter turn from the edge is toward it: {square:?}"
    );
    assert!((chord.sector.face.clear.unwrap() - 0.5).abs() < 1e-12, "the ball");
}

/// **The lever stays inside the section's circle.** A cylinder of radius
/// `0.1` whose other edges stand `10` off: the arc's point is read at
/// half the radius, not half the clearance, which no circle of that
/// radius reaches; the wedge from `60°` to `120°` rests. Red if the
/// lever outruns the circle (its point unread, the edge in band).
#[test]
fn the_lever_stays_inside_the_sections_circle() {
    for p in AT {
        assert!(
            rests(&judged(
                p,
                (FRAC_PI_3, 2.0 * FRAC_PI_3),
                Carrier::Cylinder,
                true,
                0.1,
                v(0.0, 1.0, 0.0),
                10.0,
                [p - Z * 20.0, p + Z * 20.0]
            )),
            "at {p:?}"
        );
    }
}

/// **An edge tilted off the ruling inside the face's tangent plane
/// refuses.** The edge through `p` along `z` turned `0.01` toward the
/// face's tangent at `60°`, both ends and its direction on that line: it
/// lies in the tangent plane, so only the carrier's own ruling parts it
/// from the edge. `Unreadable`. Red if the ruling is read off the edge's
/// direction rather than the carrier.
#[test]
fn an_edge_tilted_in_the_tangent_plane_refuses() {
    for p in AT {
        for (what, carrier) in CARRIERS {
            let h = dir(FRAC_PI_3);
            let outward = -(Z.cross(h));
            let edge = (Z + h * 0.01).normalize();
            let ruled = Ruled {
                surface: carrier.surface(p, -outward, 1.0),
                sense: true,
                outward,
                along: -edge,
                ends: [p - edge * 2.0, p + edge * 2.0],
                clear: 1.0,
            };
            assert!(
                matches!(
                    FaceSector::chord(&ruled, p, v(0.0, 1.0, 0.0), band()),
                    Err(TouchUnread::Unreadable)
                ),
                "{what} at {p:?}"
            );
        }
    }
}
