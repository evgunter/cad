//! **A tangent link one order down, at a ray where its first order
//! ties** ([`super::rest::one_material`]): a cusp corner's face, which
//! side of the shared tangent its two edges bend to ([`cusp`]); a ray
//! where every sheet is a horn, the thorn they bound ([`thorns`]); and a
//! ray where sheets' bounds or planes meet without an edge they share,
//! whether the sheets part across it ([`parts`]); and two sheets on one
//! wedge, bounded by the same two edges, which side of the other each
//! bends to ([`lenses`]).
//!
//! The picture these read, the cut square to the tied ray scaled by
//! `2/s²`, and why a sign read at the arm with the first order's residue
//! charged ([`Reading::charged`]) is the germ's, are argued once, in
//! `crates/topo/README.md` beside the first-order link claim. Every
//! reading here is a sign of that picture at the arm `l`, the Taylor
//! term `Δκ·l²/2` in metres ([`Distance::departure`], `Parted`), under
//! the K row [`LINK_SECOND_ORDER`].

use super::metric::{self, Departure, Distance, Reading};
use super::{
    Band, Bound, CurvedUnread, Decide, FaceSector, LINK_PLANE, LINK_SECOND_ORDER, Link, LinkFace,
    Material, Place, Point3, Sector, Segment, Sign, Tangent, TouchUnread, Vec3, ahead, wedge_side,
};
use crate::entity::EdgeKey;
use geom_core::Lever;
use geom_core::ladder::{Climb, Rung, climb_agreeing};

/// The arm of a departure `d` from `apex`: the point it reaches, read
/// from `apex` (the link's own idiom, [`Link::of`]).
fn reach<T: Decide>(apex: Point3<T>, d: Vec3<T>) -> Lever<T> {
    geom_brep::ExtentBall::point(apex + d).lever_from(apex)
}

/// The side of the plane through `p` of normal `n` that the direction
/// `v` reaches at the arm `l`.
fn toward<T: Decide>(p: Point3<T>, l: T, v: Vec3<T>, n: Vec3<T>, rd: Reading<T>) -> Option<Sign> {
    metric::sign_over(LINK_PLANE, Distance::of(p + v * l, p, n), rd)
}

/// Which way a cusp corner's face lies about its ray ([`cusp`]).
pub(super) enum Cusp {
    /// The whole plane but a horn about the ray: the corner's fan is the
    /// whole turn.
    Whole,
    /// A horn about the ray ([`super::Horn`]).
    Horn,
}

/// **A cusp corner, one order down**: the face's corner at `apex`, in
/// the plane of outward unit `m`, from the departure `d0` out of the
/// vertex counterclockwise to `d1` back into it, the two on one ray,
/// the edges' curvature vectors `bends` there. With `ŵ = m × d̂0` the
/// direction left of the ray, each edge stands `κ·ŵ` across it one
/// order down, the face between them as the loop turns: a horn where
/// the edge back in stands left of the edge out (`(κ₁ − κ₀)·ŵ > 0`),
/// the whole turn but a horn where it stands right. Read at the
/// shorter departure, the departures' first-order parting across the
/// ray charged ([`Reading::charged`]); `None` where it does not part
/// them.
///
/// Not `geom_brep::tied_germ_turn`, which reads the same sign: it decides
/// through `dihedral::decide`, a levered reading the link's doors keep
/// out (S4), and charges no first-order residue.
pub(super) fn cusp<T: Decide>(
    apex: Point3<T>,
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    [k0, k1]: [Vec3<T>; 2],
    band: Band,
) -> Option<Cusp> {
    let arm = reach(apex, d0).min(reach(apex, d1));
    let l = arm.length();
    let (t0, t1) = (d0.normalize(), d1.normalize());
    let w = m.cross(t0).normalize();
    let residue = Distance::of(apex + t1 * l, apex + t0 * l, w);
    let bend = Distance::departure(Departure::Parted(k1 - k0), w, arm);
    match metric::settle_as(
        LINK_SECOND_ORDER,
        bend,
        Reading::plain(band).charged(residue),
    ) {
        Ok(Sign::Positive) => Some(Cusp::Horn),
        Ok(Sign::Negative) => Some(Cusp::Whole),
        Ok(Sign::Zero) | Err(_) => None,
    }
}

/// **The link's horns, read as the thorns they bound**: every horn is
/// held to a cycle of horns along one ray, each meeting the next at the
/// edge back into it. One order down a cycle is
/// a polygon, its vertices the edges' points and its sides the horns'
/// segments. It is read as one where every vertex off a side stands
/// strictly on one side of that side's face, the same side for every
/// side and vertex ([`LINK_SECOND_ORDER`], the band charged): then each
/// side's line has every other vertex strictly on one side of it, so the
/// polygon is convex and simple, and every face's outward normal points
/// out of it (a thorn of material) or every one into it (a needle of
/// void). The cone of the link's sectors must read the ray decidedly
/// outside its material for a thorn, inside it for a needle
/// ([`super::Cone::reads_over`]; a cone of no sector is nowhere): so no
/// sheet of the cone comes near the ray, and the region round the
/// polygon winds one less than inside it for a thorn, one more for a
/// needle. Two cycles must lie on rays decidedly apart. Anything else
/// is [`crate::CurvedUnread::Unreadable`]: a horn beside a sector along its ray,
/// a ring of two horns (no vertex off a side), a polygon the band does
/// not part.
pub(super) fn thorns<T: Decide>(link: &Link<T>, rd: Reading<T>) -> Result<(), TouchUnread> {
    let (p, horns) = (link.cone.p, &link.horns);
    let unread = TouchUnread::Curved(CurvedUnread::Unreadable);
    let along = |d: Vec3<T>, l: T| p + d.normalize() * l;
    // Each horn's next: the horn whose edge out is this one's edge back
    // in, along the same ray.
    let next = |i: usize| -> Result<usize, TouchUnread> {
        let (d1, b1) = horns[i].ends[1];
        let l = d1.norm();
        let mut found = (0..horns.len()).filter(|&j| {
            let (d0, b0) = horns[j].ends[0];
            b0.edge == b1.edge && ahead(p, along(d1, l), along(d0, l), rd) == Some(Sign::Positive)
        });
        match (found.next(), found.next()) {
            (Some(j), None) => Ok(j),
            _ => Err(unread),
        }
    };
    let mut seen = vec![false; horns.len()];
    let mut rays: Vec<(Point3<T>, Vec<usize>)> = Vec::new();
    for start in 0..horns.len() {
        if seen[start] {
            continue;
        }
        let mut cycle = vec![start];
        seen[start] = true;
        let mut at = next(start)?;
        while at != start {
            if seen[at] {
                return Err(unread);
            }
            seen[at] = true;
            cycle.push(at);
            at = next(at)?;
        }
        let arm = cycle
            .iter()
            .flat_map(|&i| horns[i].ends.map(|(d, _)| reach(p, d)))
            .fold(reach(p, horns[start].ends[0].0), Lever::min);
        let l = arm.length();
        let vertex = |i: usize| horns[cycle[i]].ends[0];
        let n = cycle.len();
        let pairs = (0..n).flat_map(|i| {
            (0..n)
                .filter(move |&j| j != i && j != (i + 1) % n)
                .map(move |j| (i, j))
        });
        // Every vertex off a side on one side of that side's face
        // ([`climb_agreeing`]): two that differ, or one that does not
        // read, leave the polygon unread. A pair that does not read aborts
        // rather than being kept: the polygon's convexity needs every
        // pair, so an agreement read past an unread pair would claim a
        // convexity no reading showed.
        let side = match climb_agreeing(
            pairs,
            |(i, j)| {
                let (ta, a) = vertex(i);
                match beside(p, (ta, a), vertex(j), horns[cycle[i]].outward, arm, rd) {
                    Some(s) => Rung::Answers(Some(s)),
                    None => Rung::Aborts(None),
                }
            },
            |x: &Option<Sign>, y: &Option<Sign>| x == y,
        ) {
            Ok(Climb::Answered {
                answer: Some(s), ..
            }) => Some(s),
            Ok(Climb::Passed) => None,
            Ok(Climb::Answered { answer: None, .. } | Climb::Undecided(())) | Err(_) => {
                return Err(unread);
            }
        };
        let ray = along(horns[start].ends[0].0, l);
        let around = if link.cone.sectors.is_empty() {
            Material::Out
        } else {
            link.cone.reads_over(ray, rd).ok_or(unread)?
        };
        match (side, around) {
            (Some(Sign::Negative), Material::Out) | (Some(Sign::Positive), Material::In) => {}
            _ => return Err(unread),
        }
        rays.push((ray, cycle));
    }
    for (k, (ray, cycle)) in rays.iter().enumerate() {
        for (other, _) in &rays[k + 1..] {
            let off = |f: &usize| {
                matches!(
                    metric::sign_as(
                        LINK_PLANE,
                        Distance::of(*other, p, horns[*f].outward),
                        rd.band
                    ),
                    Some(Sign::Positive | Sign::Negative)
                )
            };
            if ahead(p, *ray, *other, rd) != Some(Sign::Negative) && !cycle.iter().any(off) {
                return Err(unread);
            }
        }
    }
    Ok(())
}

/// **The edge `c`'s point against the face through the edge `a`'s, one
/// order down**: the sign of `(κ_c − κ_a)·m · l²/2`, `m` the face's
/// outward unit normal and `ta`, `tc` the edges' departures, their
/// first-order parting off the face's plane charged; `None` where that
/// parting is not decided zero (the edge leaves the face's plane, so it
/// is no point of the picture) or the sign does not read.
fn beside<T: Decide>(
    p: Point3<T>,
    (ta, a): (Vec3<T>, Bound<T>),
    (tc, c): (Vec3<T>, Bound<T>),
    m: Vec3<T>,
    arm: Lever<T>,
    rd: Reading<T>,
) -> Option<Sign> {
    let l = arm.length();
    let residue = Distance::of(p + tc.normalize() * l, p + ta.normalize() * l, m);
    if metric::sign_as(LINK_PLANE, residue, rd.band) != Some(Sign::Zero) {
        return None;
    }
    metric::settle_as(
        LINK_SECOND_ORDER,
        Distance::departure(Departure::Parted(c.bend - a.bend), m, arm),
        rd.charged(residue),
    )
    .ok()
}

/// One sheet of a tangent link along a ray, one order down ([`parts`]):
/// where it meets the cut, the point `anchor` of the edge it starts at
/// (its curvature vector), or of a face through the ray (its normal
/// section's curvature vector along it, nothing for a plane), that
/// edge's first-order direction `at`, its outward unit `normal`, and
/// how it runs from the anchor.
#[derive(Clone, Copy)]
struct Sheet<T: Decide> {
    anchor: Vec3<T>,
    at: Vec3<T>,
    normal: Vec3<T>,
    run: Run<T>,
    edge: Option<EdgeKey>,
}

/// How a [`Sheet`] runs from its anchor in the cut: both ways, or one,
/// along a unit direction square to the ray and its normal.
#[derive(Clone, Copy)]
enum Run<T: Decide> {
    Both(Vec3<T>),
    Half(Vec3<T>),
}

/// **Whether the sheets along the ray through `r` part one order
/// down**, where first order leaves a bound of one lying on another's
/// plane ([`super::rest::one_material`]'s first-order hold). The sheets
/// are the sectors whose planes hold the ray: a planar one holding it
/// inside, the whole line through the point; a curved one holding it
/// inside, the line through its normal section's curvature vector along
/// the ray ([`crate::bound_side::FaceBend::section`]), square to the ray
/// in its plane; one bounded along it, the half-line from its edge's
/// point into the sector. Every two must
/// decidedly not meet but at an edge's point they share, without
/// folding there: one strictly on one side of the other's line, or
/// touching it only at its own anchor, decidedly behind the other's, or
/// running along it away from it. A horn is no sheet ([`thorns`] refuses
/// one a sector reaches); a curved face with no certified bend is false.
pub(super) fn parts<T: Decide>(link: &Link<T>, r: Point3<T>, rd: Reading<T>) -> bool {
    let p = link.cone.p;
    let unit = (r - p).normalize();
    let mut arm = reach(p, r - p);
    let mut sheets: Vec<Sheet<T>> = Vec::new();
    for (j, t) in link.cone.sectors.iter().enumerate() {
        match link.cone.side(t, r, rd) {
            Some(Sign::Zero) => {}
            Some(_) => continue,
            None => return false,
        }
        let tangent = link.faces[j].tangent;
        match t.place(p, r, rd) {
            Ok(Place::Outside) => {}
            Ok(Place::Inside) => {
                // The ray as the plane holds it: what the ray leans off
                // the plane is a residue the readings charge.
                let at = (unit - t.outward * t.outward.dot(unit)).normalize();
                let anchor = match tangent {
                    Some(g) if g.plane_exact => Vec3::zero(),
                    Some(_) => {
                        let face = &link.faces[j];
                        let bend =
                            crate::bound_side::FaceBend::at(&face.surface, face.sense, t.apex);
                        match bend.section(at, t.outward) {
                            Some(h) => t.outward * h,
                            None => return false,
                        }
                    }
                    None => return false,
                };
                sheets.push(Sheet {
                    anchor,
                    at,
                    normal: t.outward,
                    run: Run::Both(t.outward.cross(unit).normalize()),
                    edge: None,
                });
            }
            Ok(Place::Bound) => {
                let (Some(bounds), Some(rays)) = (tangent.and_then(|g| g.bounds), t.bounds())
                else {
                    return false;
                };
                let before = sheets.len();
                for (i, (b, q)) in bounds.into_iter().zip(rays).enumerate() {
                    if wedge_side(p, t.outward, q, r, rd) != Some(Sign::Zero)
                        || ahead(p, q, r, rd) != Some(Sign::Positive)
                    {
                        continue;
                    }
                    let at = (q - p).normalize();
                    let into = if i == 0 {
                        t.outward.cross(at)
                    } else {
                        at.cross(t.outward)
                    };
                    arm = arm.min(reach(p, q - p));
                    sheets.push(Sheet {
                        anchor: b.bend,
                        at,
                        normal: t.outward,
                        run: Run::Half(into.normalize()),
                        edge: Some(b.edge),
                    });
                }
                if sheets.len() == before {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
    sheets.len() >= 2
        && sheets
            .iter()
            .enumerate()
            .all(|(i, a)| sheets[i + 1..].iter().all(|b| apart(p, a, b, arm, rd)))
}

/// Whether two sheets along a ray decidedly do not meet one order down
/// but at an edge's point they share ([`parts`]).
fn apart<T: Decide>(
    p: Point3<T>,
    a: &Sheet<T>,
    b: &Sheet<T>,
    arm: Lever<T>,
    rd: Reading<T>,
) -> bool {
    let l = arm.length();
    let toward = |v, n| toward(p, l, v, n, rd);
    if a.edge.is_some() && a.edge == b.edge {
        // One edge's two faces, from its one point: they part unless
        // they run on along one line one way.
        return match (a.run, b.run) {
            (Run::Half(wa), Run::Half(wb)) => match toward(wb, a.normal) {
                Some(Sign::Positive | Sign::Negative) => true,
                Some(Sign::Zero) => toward(wb, wa) == Some(Sign::Negative),
                None => false,
            },
            _ => false,
        };
    }
    misses(p, a, b, arm, rd) || misses(p, b, a, arm, rd)
}

/// Whether the sheet `b` decidedly does not meet the sheet `a`, read
/// against `a`'s line in the cut ([`parts`]).
fn misses<T: Decide>(
    p: Point3<T>,
    a: &Sheet<T>,
    b: &Sheet<T>,
    arm: Lever<T>,
    rd: Reading<T>,
) -> bool {
    let l = arm.length();
    let toward = |v, n| toward(p, l, v, n, rd);
    // `b`'s anchor against a line through `a`'s, of normal `n`, with
    // `also` charged beside the anchors' first-order parting.
    let off = |n: Vec3<T>, also: Option<Distance<T>>| {
        let residue = Distance::of(p + b.at * l, p + a.at * l, n);
        if metric::sign_as(LINK_PLANE, residue, rd.band) != Some(Sign::Zero) {
            return None;
        }
        let rd = also.map_or(rd, |d| rd.charged(d)).charged(residue);
        metric::settle_as(
            LINK_SECOND_ORDER,
            Distance::departure(Departure::Parted(b.anchor - a.anchor), n, arm),
            rd,
        )
        .ok()
    };
    // `b`'s anchor decidedly behind `a`'s along `a`'s half-line.
    let behind = || match a.run {
        Run::Half(wa) => off(wa, None) == Some(Sign::Negative),
        Run::Both(_) => false,
    };
    let wb = match b.run {
        Run::Both(w) | Run::Half(w) => w,
    };
    // A sheet run along `a`'s line at first order may still lean toward
    // it by what that zero leaves: charged, out to the arm, where the
    // anchor's offset is read against it.
    let tilt = Distance::of(p + wb * l, p, a.normal);
    let side = off(a.normal, None);
    let clear = off(a.normal, Some(tilt));
    match b.run {
        Run::Both(wb) => {
            matches!(clear, Some(Sign::Positive | Sign::Negative))
                && toward(wb, a.normal) == Some(Sign::Zero)
        }
        Run::Half(wb) => match (side, toward(wb, a.normal)) {
            (Some(s @ (Sign::Positive | Sign::Negative)), Some(Sign::Zero)) => clear == Some(s),
            (Some(s @ (Sign::Positive | Sign::Negative)), Some(g)) => g == s,
            (Some(Sign::Zero), Some(Sign::Positive | Sign::Negative)) => behind(),
            (Some(Sign::Zero), Some(Sign::Zero)) => match a.run {
                Run::Half(wa) => toward(wb, wa) == Some(Sign::Negative) && behind(),
                Run::Both(_) => false,
            },
            _ => false,
        },
    }
}

/// What a lens holds between its two sheets ([`lenses`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Lens {
    /// Void: each sheet stands on the other's outward side.
    Void,
    /// A sliver of material: each stands behind the other.
    Sliver,
}

/// **The link's lenses, one order down**: pairs of sectors bounded at
/// the point by the same two edges, each one's first edge the other's
/// last, on each other to first order ([`on_each_other`]) and not both
/// planes (two planes on each other are a slit,
/// [`super::rest::one_material`]'s to refuse). Each pair's kind is the
/// sign of its sheets' second-order gap at the wedge's middle
/// ([`lens`]). A candidate pair that does not read, or a sector in two
/// pairs, is [`crate::CurvedUnread::Unreadable`]. The argument:
/// `crates/topo/README.md`, beside the first-order one.
///
/// Then the **lunes**: a pair `(i, j)` on one wedge `W`, `j`'s one wedge
/// and `i`'s first or last, not both planes and on each other to first
/// order, whose ends ([`Close`]) are not both pinches, read by
/// [`lune`]. A lune whose sector is in another pair is
/// [`crate::CurvedUnread::Unreadable`]; a pair whose ends do not close
/// is no lune, and its sheets stay on each other.
pub(super) fn lenses<T: Decide>(
    link: &Link<T>,
    rd: Reading<T>,
) -> Result<Vec<OneWedge<T>>, TouchUnread> {
    let n = link.cone.sectors.len();
    let tangent = |k: usize| link.faces[k].tangent;
    let unread = TouchUnread::Curved(CurvedUnread::Unreadable);
    let mut out: Vec<OneWedge<T>> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let (Some(ti), Some(tj)) = (tangent(i), tangent(j)) else {
                continue;
            };
            let (Some([i0, i1]), Some([j0, j1])) = (ti.bounds, tj.bounds) else {
                continue;
            };
            if i0.edge != j1.edge || i1.edge != j0.edge || i0.edge == i1.edge {
                continue;
            }
            if (super::rest::planar(link, i) && super::rest::planar(link, j))
                || !on_each_other(link, [i, j], rd)
            {
                continue;
            }
            if out
                .iter()
                .any(|w| w.pair.contains(&i) || w.pair.contains(&j))
            {
                return Err(unread);
            }
            let Some(&wedge) = link.cone.sectors[i].wedges.first() else {
                return Err(unread);
            };
            out.push(OneWedge {
                pair: [i, j],
                wedge,
                kind: lens(link, [i, j], rd).ok_or(unread)?,
                caps: Vec::new(),
                rest: None,
            });
        }
    }
    let lenses = out.len();
    for j in 0..n {
        for i in (0..n).filter(|&i| i != j) {
            if out[..lenses].iter().any(|w| w.pair.contains(&i) && w.pair.contains(&j)) {
                continue;
            }
            let Some(found) = lune(link, [i, j], rd) else {
                continue;
            };
            // Two sectors of one wedge each are one pair either way round.
            if out[lenses..]
                .iter()
                .any(|w| w.pair == [j, i] && link.cone.sectors[i].wedges.len() == 1)
            {
                continue;
            }
            if out.iter().any(|w| w.pair.contains(&i) || w.pair.contains(&j)) {
                return Err(unread);
            }
            out.push(found.ok_or(unread)?);
        }
    }
    Ok(out)
}

/// **Two sectors on one wedge, read one order down** ([`lenses`]): the
/// pair, sector `j`'s one wedge `wedge` (the first's sector, whose one
/// wedge it is, for a lens), what lies between their sheets, the horns
/// that close it (`(horn, a point of its ray)`), and, where the first
/// runs on past the wedge, that sector less it, bounded at the mouth by
/// the second's edge there.
pub(super) struct OneWedge<T: Decide> {
    pub(super) pair: [usize; 2],
    pub(super) wedge: Segment<T>,
    pub(super) kind: Lens,
    pub(super) caps: Vec<(usize, Point3<T>)>,
    pub(super) rest: Option<FaceSector<T>>,
}

/// How a lune's sheets meet at one end of its wedge ([`lune`]).
#[derive(Clone, Copy)]
enum Close {
    /// Both sectors bounded there by one edge, or by two lines: both
    /// faces hold the curve, and their sheets meet along it.
    Pinch,
    /// Both bounded there by the two edges of the horn with this index,
    /// which spans the gap between the sheets.
    Cap(usize),
    /// The first sector runs on past the end, which is a ray of its fan.
    Mouth,
}

/// Whether `x` lies on the ray through `r`, both on the plane through `p`
/// of normal `m`.
fn same_ray<T: Decide>(p: Point3<T>, m: Vec3<T>, r: Point3<T>, x: Point3<T>, rd: Reading<T>) -> bool {
    wedge_side(p, m, r, x, rd) == Some(Sign::Zero) && ahead(p, r, x, rd) == Some(Sign::Positive)
}

/// **The pair `(i, j)` as a lune** ([`lenses`]): `None` where it is no
/// candidate — `j` not one wedge, `W`, `i` not holding `W` reversed as its
/// first or last wedge, both planes, not on each other, both ends
/// pinches (a lens, or two lines at each end), or an end that does not
/// close ([`Close`]); `Some(None)` where it is one and does not read.
///
/// Along `W` from `u₀` to `u₁`, the second sheet past the first along the
/// first's outward `m` is the quadratic form
/// `q((1 − t)u₀ + t·u₁) = b₀(1 − t)² + 2b₁t(1 − t) + b₂t²`, `b₀ = q(u₀)`,
/// `b₂ = q(u₁)`, `b₁` its polar value. A pinch reads its `b` Zero; a cap or
/// a mouth reads its `b` decided, one sign `σ` for all of them, with the
/// second plane's parting along that ray charged; `b₁` reads Zero or `σ`,
/// with the parting at the middle charged. `σ` positive is a void, each
/// sheet on the other's outward side, and a cap's horn must face into
/// `W` for a void and out of it for a sliver. The argument:
/// `crates/topo/README.md`, beside the lens's.
fn lune<T: Decide>(
    link: &Link<T>,
    [i, j]: [usize; 2],
    rd: Reading<T>,
) -> Option<Option<OneWedge<T>>> {
    let p = link.cone.p;
    let (s, t) = (&link.cone.sectors[i], &link.cone.sectors[j]);
    let (Some(ti), Some(tj)) = (link.faces[i].tangent, link.faces[j].tangent) else {
        return None;
    };
    let (Some([i0, i1]), Some([j0, j1]), &[(c, d)]) = (ti.bounds, tj.bounds, t.wedges.as_slice())
    else {
        return None;
    };
    if (super::rest::planar(link, i) && super::rest::planar(link, j))
        || !on_each_other(link, [i, j], rd)
    {
        return None;
    }
    let along = |(a, b): Segment<T>| {
        same_ray(p, s.outward, a, d, rd) && same_ray(p, s.outward, b, c, rd)
    };
    let (&first, &last) = (s.wedges.first()?, s.wedges.last()?);
    let at_first = along(first);
    if !at_first && !along(last) {
        return None;
    }
    // `W` as `i` holds it, from its ray at `d` to its ray at `c`.
    let (x, y) = if at_first { first } else { last };
    let only = s.wedges.len() == 1;
    let close = |ei: Option<Bound<T>>, ej: Bound<T>, ray: Point3<T>| match ei {
        None => Some(Close::Mouth),
        Some(ei) if ei.edge == ej.edge || (ei.line && ej.line) => Some(Close::Pinch),
        Some(ei) => link
            .horns
            .iter()
            .position(|h| {
                let [(e0, b0), (e1, b1)] = h.ends;
                ((b0.edge, b1.edge) == (ei.edge, ej.edge) || (b0.edge, b1.edge) == (ej.edge, ei.edge))
                    && same_ray(p, s.outward, ray, h.apex + e0, rd)
                    && same_ray(p, s.outward, ray, h.apex + e1, rd)
            })
            .map(Close::Cap),
    };
    let at_d = close((at_first).then_some(i0), j1, x)?;
    let at_y = close((!at_first || only).then_some(i1), j0, y)?;
    if matches!((at_d, at_y), (Close::Pinch, Close::Pinch)) {
        return None;
    }
    let ends = [(y, at_y), (x, at_d)];
    let rest = match ends.iter().find(|(_, e)| matches!(e, Close::Mouth)) {
        None => None,
        Some(_) => {
            let n = s.wedges.len();
            let (wedges, bounds) = if at_first {
                (s.wedges[1..].to_vec(), [j0, i1])
            } else {
                (s.wedges[..n - 1].to_vec(), [i0, j1])
            };
            Some(FaceSector {
                sector: Sector {
                    outward: s.outward,
                    apex: s.apex,
                    wedges,
                    closed: false,
                },
                face: LinkFace {
                    tangent: Some(Tangent {
                        bounds: Some(bounds),
                        ..ti
                    }),
                    ..link.faces[i].clone()
                },
            })
        }
    };
    let caps = ends
        .iter()
        .filter_map(|&(r, e)| match e {
            Close::Cap(h) => Some((h, r)),
            Close::Pinch | Close::Mouth => None,
        })
        .collect();
    Some(lune_kind(link, [i, j], ends, rd).map(|kind| OneWedge {
        pair: [i, j],
        wedge: (c, d),
        kind,
        caps,
        rest,
    }))
}

/// **A lune's kind** ([`lune`]): the sign `σ` its open ends read, each
/// pinch read Zero and the polar value `b₁` Zero or `σ`, each cap's horn
/// facing as `σ` says; `None` where any of these does not hold.
fn lune_kind<T: Decide>(
    link: &Link<T>,
    [i, j]: [usize; 2],
    ends: [(Point3<T>, Close); 2],
    rd: Reading<T>,
) -> Option<Lens> {
    let p = link.cone.p;
    let (s, t) = (&link.cone.sectors[i], &link.cone.sectors[j]);
    let m = s.outward;
    let arm = ends
        .map(|(q, _)| q)
        .into_iter()
        .chain(t.wedges.iter().flat_map(|&(c, d)| [c, d]))
        .map(|q| reach(p, q - p))
        .fold(reach(p, ends[0].0 - p), Lever::min);
    let l = arm.length();
    let bend = |k: usize| {
        let face = &link.faces[k];
        crate::bound_side::FaceBend::at(&face.surface, face.sense, link.cone.sectors[k].apex)
    };
    let (first, second) = (bend(i), bend(j));
    // The second sheet past the first along `m`, per arc length squared,
    // along `u` (a quadratic form: `u` need not be a unit).
    let rise = |u: Vec3<T>| -> Option<T> { Some(second.section(u, m)? - first.section(u, m)?) };
    let gap = |r: T| Distance::departure(Departure::Parted(m * r), m, arm);
    // The two planes' parting along the unit `u`: its distance off each,
    // whichever sector's rays it was read from.
    let parting = |rd: Reading<T>, u: Vec3<T>| {
        rd.charged(Distance::of(p + u * l, p, s.outward))
            .charged(Distance::of(p + u * l, p, t.outward))
    };
    let [u0, u1] = ends.map(|(q, _)| (q - p).normalize());
    let mut sigma: Option<Sign> = None;
    for (u, close) in [(u0, ends[0].1), (u1, ends[1].1)] {
        let b = gap(rise(u)?);
        match close {
            Close::Pinch => {
                if metric::sign_as(LINK_SECOND_ORDER, b, rd.band) != Some(Sign::Zero) {
                    return None;
                }
            }
            Close::Cap(_) | Close::Mouth => {
                match metric::settle_as(LINK_SECOND_ORDER, b, parting(rd, u)) {
                    Ok(g @ (Sign::Positive | Sign::Negative)) if sigma.is_none_or(|x| x == g) => {
                        sigma = Some(g);
                    }
                    _ => return None,
                }
            }
        }
    }
    let sigma = sigma?;
    let mid = (u0 + u1).normalize();
    let polar = (rise(u0 + u1)? - rise(u0)? - rise(u1)?) * T::from_f64(0.5);
    match metric::settle_as(LINK_SECOND_ORDER, gap(polar), parting(rd, mid)) {
        Ok(Sign::Zero) => {}
        Ok(g) if g == sigma => {}
        _ => return None,
    }
    for (_, close) in ends {
        if let Close::Cap(h) = close
            && toward(p, l, mid, link.horns[h].outward, rd) != Some(sigma)
        {
            return None;
        }
    }
    Some(if sigma == Sign::Positive {
        Lens::Void
    } else {
        Lens::Sliver
    })
}

/// Whether sectors `i` and `j` are one wedge to first order: their
/// outward normals decidedly opposed, and `i` in `j`'s plane
/// ([`super::rest::in_plane_of`]; two bounds on one line leave the
/// planes free to differ).
fn on_each_other<T: Decide>(link: &Link<T>, [i, j]: [usize; 2], rd: Reading<T>) -> bool {
    let p = link.cone.p;
    let (s, t) = (&link.cone.sectors[i], &link.cone.sectors[j]);
    let Some(&(a, _)) = s.wedges.first() else {
        return false;
    };
    toward(p, reach(p, a - p).length(), t.outward, s.outward, rd) == Some(Sign::Negative)
        && super::rest::in_plane_of(link, i, j, rd)
}

/// **One pair's lens** ([`lenses`]): the second sheet's height past the
/// first's along the first's outward normal, one order down
/// ([`crate::bound_side::FaceBend::section`]), Zero at both bounds and
/// decided at the wedge's middle at the arm with the second plane's
/// parting there charged ([`Reading::charged`]): positive a void,
/// negative a sliver. `None` where a sector is more than one wedge, a
/// face has no certified bend, a bound does not read Zero, or the middle
/// does not decide.
fn lens<T: Decide>(link: &Link<T>, [i, j]: [usize; 2], rd: Reading<T>) -> Option<Lens> {
    let p = link.cone.p;
    let (s, t) = (&link.cone.sectors[i], &link.cone.sectors[j]);
    let (&[(a, b)], &[(c, d)]) = (s.wedges.as_slice(), t.wedges.as_slice()) else {
        return None;
    };
    let m = s.outward;
    let arm = [a, b, c, d]
        .into_iter()
        .map(|q| reach(p, q - p))
        .fold(reach(p, a - p), Lever::min);
    let l = arm.length();
    let bend = |k: usize| {
        let face = &link.faces[k];
        crate::bound_side::FaceBend::at(&face.surface, face.sense, link.cone.sectors[k].apex)
    };
    let (first, second) = (bend(i), bend(j));
    // The second sheet past the first along `m` at the arm, one order
    // down along the unit `u`.
    let gap = |u: Vec3<T>| -> Option<Distance<T>> {
        let rise = second.section(u, m)? - first.section(u, m)?;
        Some(Distance::departure(Departure::Parted(m * rise), m, arm))
    };
    let (u0, u1) = ((a - p).normalize(), (b - p).normalize());
    for u in [u0, u1] {
        if metric::sign_as(LINK_SECOND_ORDER, gap(u)?, rd.band) != Some(Sign::Zero) {
            return None;
        }
    }
    let mid = (u0 + u1).normalize();
    let residue = Distance::of(p + mid * l, p, t.outward);
    match metric::settle_as(LINK_SECOND_ORDER, gap(mid)?, rd.charged(residue)) {
        Ok(Sign::Positive) => Some(Lens::Void),
        Ok(Sign::Negative) => Some(Lens::Sliver),
        Ok(Sign::Zero) | Err(_) => None,
    }
}
