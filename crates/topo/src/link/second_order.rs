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
    Band, Bound, Decide, LINK_PLANE, LINK_SECOND_ORDER, Link, Material, Place, Point3, Sign,
    TouchUnread, Vec3, ahead, wedge_side,
};
use crate::entity::EdgeKey;
use geom_core::Lever;

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
    match metric::settle(
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
/// is [`TouchUnread::Unreadable`]: a horn beside a sector along its ray,
/// a ring of two horns (no vertex off a side), a polygon the band does
/// not part.
pub(super) fn thorns<T: Decide>(link: &Link<T>, rd: Reading<T>) -> Result<(), TouchUnread> {
    let (p, horns) = (link.cone.p, &link.horns);
    let unread = TouchUnread::Unreadable;
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
        let mut side = None;
        for i in 0..cycle.len() {
            let face = &horns[cycle[i]];
            let (ta, a) = vertex(i);
            for j in (0..cycle.len()).filter(|&j| j != i && j != (i + 1) % cycle.len()) {
                let (tc, c) = vertex(j);
                let s = beside(p, (ta, a), (tc, c), face.outward, arm, rd).ok_or(unread)?;
                if side.is_some_and(|x| x != s) {
                    return Err(unread);
                }
                side = Some(s);
            }
        }
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
                    metric::sign(
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
    if metric::sign(LINK_PLANE, residue, rd.band) != Some(Sign::Zero) {
        return None;
    }
    metric::settle(
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
        if metric::sign(LINK_PLANE, residue, rd.band) != Some(Sign::Zero) {
            return None;
        }
        let rd = also.map_or(rd, |d| rd.charged(d)).charged(residue);
        metric::settle(
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

/// **The link's lenses, one order down**: two sectors of opposed
/// outward normals bounded at the point by the same two edges, each
/// sector's first edge the other's last, not both planes (two planes on
/// each other are a slit, [`super::rest::one_material`]'s to refuse).
/// To first order the two are one wedge; one order down each is its
/// face's graph over it, `h(u)·s²/2` along the first's outward unit `m`
/// at distance `s` along the unit `u`, `h` its normal section's
/// acceleration ([`crate::bound_side::FaceBend::section`]). Both faces
/// hold both edges, whose curvature vectors are each face's normal
/// section along them (Meusnier), so the difference `q(u) = h₂(u) −
/// h₁(u)`, a quadratic form on the plane, vanishes along both bounds:
/// it is `c·(u × u₀)·(u × u₁)` along the normal, one sign across the
/// open wedge. That sign, read at the wedge's middle at the arm with the
/// second plane's parting from the first there charged
/// ([`Reading::charged`]), is the lens's: positive, each sheet on the
/// other's outward side, a void between them; negative, a sliver of
/// material. A pair is one whose normals read decidedly opposed and
/// whose first sector's rays all read on the second's plane; each bound
/// must then read `q` Zero, and each sector be one wedge (under half a
/// turn: a wider one holds a bound's opposite, where `q` vanishes
/// again). A face with no certified bend, a middle the charge leaves in
/// band, a sector in two pairs, or anything else is
/// [`TouchUnread::Unreadable`].
pub(super) fn lenses<T: Decide>(
    link: &Link<T>,
    rd: Reading<T>,
) -> Result<Vec<([usize; 2], Lens)>, TouchUnread> {
    let n = link.cone.sectors.len();
    let tangent = |k: usize| link.faces[k].tangent;
    let mut out: Vec<([usize; 2], Lens)> = Vec::new();
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
            if (ti.plane_exact && tj.plane_exact) || !on_each_other(link, [i, j], rd) {
                continue;
            }
            if out
                .iter()
                .any(|(pair, _)| pair.contains(&i) || pair.contains(&j))
            {
                return Err(TouchUnread::Unreadable);
            }
            out.push((
                [i, j],
                lens(link, [i, j], rd).ok_or(TouchUnread::Unreadable)?,
            ));
        }
    }
    Ok(out)
}

/// Whether sectors `i` and `j` are one wedge to first order: their
/// outward normals decidedly opposed, and every ray of `i` on `j`'s plane.
fn on_each_other<T: Decide>(link: &Link<T>, [i, j]: [usize; 2], rd: Reading<T>) -> bool {
    let p = link.cone.p;
    let (s, t) = (&link.cone.sectors[i], &link.cone.sectors[j]);
    let Some(&(a, _)) = s.wedges.first() else {
        return false;
    };
    toward(p, reach(p, a - p).length(), t.outward, s.outward, rd) == Some(Sign::Negative)
        && s.wedges.iter().all(|&(a, b)| {
            [a, b]
                .into_iter()
                .all(|q| link.cone.side(t, q, rd) == Some(Sign::Zero))
        })
}

/// One pair's lens ([`lenses`]), `None` where it does not read.
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
        if metric::sign(LINK_SECOND_ORDER, gap(u)?, rd.band) != Some(Sign::Zero) {
            return None;
        }
    }
    let mid = (u0 + u1).normalize();
    let residue = Distance::of(p + mid * l, p, t.outward);
    match metric::settle(LINK_SECOND_ORDER, gap(mid)?, rd.charged(residue)) {
        Ok(Sign::Positive) => Some(Lens::Void),
        Ok(Sign::Negative) => Some(Lens::Sliver),
        Ok(Sign::Zero) | Err(_) => None,
    }
}
