//! **A tangent link one order down, at a ray where its first order
//! ties** ([`super::rest::one_material`]): a cusp corner's face, which
//! side of the shared tangent its two edges bend to ([`cusp`]); a ray
//! where every sheet is a horn, the thorn they bound ([`thorns`]); and a
//! ray where sheets' bounds or planes meet without an edge they share,
//! whether the sheets part across it ([`parts`]).
//!
//! **The picture one order down.** Along a ray `u` from the point `p`,
//! cut the faces by the plane square to `u` at distance `s` and scale
//! the cut by `2/s²`. An edge leaving `p` along `u` with curvature vector
//! `κ` meets the cut at `κ`, to that order; a face whose tangent plane
//! holds `u` meets it in a straight line, through its edges' points, of
//! the face's outward normal `m` (its normal section's bend shifts the
//! line along `m`, and the line's direction is `m × u`); a face whose
//! sector is bounded by an edge along `u` meets it in the half of that
//! line on the sector's side of the edge's point; a horn, in the segment
//! between its two edges' points. Every reading here is a sign of that
//! picture, read at the arm `l` as the Taylor term `κ·l²/2` in metres
//! ([`Distance::departure`], `Parted`).
//!
//! **The band charged.** The rays the picture is drawn along tie only
//! to the band: two edges' tangents, or a ray and a face's plane, may
//! part at first order by a residue `δ` with `δ·l` decided zero. Each
//! sign is taken with that residue at the same arm charged against it
//! ([`Reading::charged`]), so it clears the band with the first order's
//! part moved against it. Then at every scale `s` from the arm down to
//! `s₀ = 2δ/|Δκ|` the second-order term outweighs the first-order one
//! and the picture is the one read; below `s₀` the faces stand within
//! `δ·s₀ ≤ 2(δl)²/(|Δκ|l²)` of it, which, the bend's term at `l` being
//! past the escalation threshold `K·zero` and `δl` within the zero band,
//! is within `4·zero/K` of the picture: inside the band. The verdict is
//! the faces' at every scale the band resolves.

use super::metric::{self, Departure, Distance, Reading};
use super::{
    Band, Bound, Decide, LINK_PLANE, LINK_SECOND_ORDER, Link, Material, Place, Point3, Sign,
    TouchUnread, Vec3, ahead, wedge_side,
};
use crate::entity::EdgeKey;

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
pub(super) fn cusp<T: Decide>(
    apex: Point3<T>,
    m: Vec3<T>,
    (d0, d1): (Vec3<T>, Vec3<T>),
    [k0, k1]: [Vec3<T>; 2],
    band: Band,
) -> Option<Cusp> {
    let l = d0.norm().min(d1.norm());
    let (t0, t1) = (d0.normalize(), d1.normalize());
    let w = m.cross(t0).normalize();
    let residue = Distance::of(apex + t1 * l, apex + t0 * l, w);
    let bend = Distance::departure(Departure::Parted(k1 - k0), w, l);
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
        let l = cycle
            .iter()
            .flat_map(|&i| horns[i].ends.map(|(d, _)| d.norm()))
            .fold(horns[start].ends[0].0.norm(), T::min);
        let vertex = |i: usize| horns[cycle[i]].ends[0];
        let mut side = None;
        for i in 0..cycle.len() {
            let face = &horns[cycle[i]];
            let (ta, a) = vertex(i);
            for j in (0..cycle.len()).filter(|&j| j != i && j != (i + 1) % cycle.len()) {
                let (tc, c) = vertex(j);
                let s = beside(p, (ta, a), (tc, c), face.outward, l, rd).ok_or(unread)?;
                if s == Sign::Zero || side.is_some_and(|x| x != s) {
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
                    metric::sign(LINK_PLANE, Distance::of(*other, p, horns[*f].outward), rd.band),
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
    l: T,
    rd: Reading<T>,
) -> Option<Sign> {
    let residue = Distance::of(p + tc.normalize() * l, p + ta.normalize() * l, m);
    if metric::sign(LINK_PLANE, residue, rd.band) != Some(Sign::Zero) {
        return None;
    }
    metric::settle(
        LINK_SECOND_ORDER,
        Distance::departure(Departure::Parted(c.bend - a.bend), m, l),
        rd.charged(residue),
    )
    .ok()
}

/// One sheet of a tangent link along a ray, one order down ([`parts`]):
/// where it meets the cut, the point `anchor` of the edge it starts at
/// (its curvature vector; nothing for a plane through the ray), that
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
    Line(Vec3<T>),
    Half(Vec3<T>),
}

/// **Whether the sheets along the ray through `r` part one order
/// down**, where first order leaves a bound of one lying on another's
/// plane, inside its sector or on its bound
/// ([`super::rest::one_material`]'s first-order hold). Every sector of
/// the link whose plane holds the ray is a sheet along it: a planar one
/// holding it inside, the whole line of its plane through the point; one
/// bounded along it, the half-line from its edge's point into the
/// sector, the bound's own side. Every two must decidedly not meet, but
/// at the point of an edge they share (an edge's two faces, which must
/// not fold onto each other): one stays strictly on one side of the
/// other's line (its anchor off it, and not turning toward it), or
/// touches the line only at its own anchor, which lies decidedly behind
/// the other's, or runs along it away from it. Then the sheets along the
/// ray are curves no two of which meet off an edge, each bounding the
/// material on its outward side, so the cone, which sees them meet on
/// the ray, reads the faces' winding about it as the cut does.
///
/// A horn along the ray is no sheet here: [`thorns`] refuses a horn
/// whose ray a sector of the cone reaches. A curved sector holding the
/// ray inside is false.
pub(super) fn parts<T: Decide>(link: &Link<T>, r: Point3<T>, rd: Reading<T>) -> bool {
    let p = link.cone.p;
    let unit = (r - p).normalize();
    let mut arm = (r - p).norm();
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
                if !tangent.is_some_and(|g| g.plane_exact) {
                    return false;
                }
                sheets.push(Sheet {
                    anchor: Vec3::zero(),
                    at: unit,
                    normal: t.outward,
                    run: Run::Line(t.outward.cross(unit).normalize()),
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
                    arm = arm.min((q - p).norm());
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
        && sheets.iter().enumerate().all(|(i, a)| {
            sheets[i + 1..]
                .iter()
                .all(|b| apart(p, a, b, arm, rd))
        })
}

/// Whether two sheets along a ray decidedly do not meet one order down
/// but at an edge's point they share ([`parts`]).
fn apart<T: Decide>(p: Point3<T>, a: &Sheet<T>, b: &Sheet<T>, l: T, rd: Reading<T>) -> bool {
    let toward = |v: Vec3<T>, n: Vec3<T>| metric::sign_over(LINK_PLANE, Distance::of(p + v * l, p, n), rd);
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
    misses(p, a, b, l, rd) || misses(p, b, a, l, rd)
}

/// Whether the sheet `b` decidedly does not meet the sheet `a`, read
/// against `a`'s line in the cut ([`parts`]).
fn misses<T: Decide>(p: Point3<T>, a: &Sheet<T>, b: &Sheet<T>, l: T, rd: Reading<T>) -> bool {
    let toward = |v: Vec3<T>, n: Vec3<T>| metric::sign_over(LINK_PLANE, Distance::of(p + v * l, p, n), rd);
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
            Distance::departure(Departure::Parted(b.anchor - a.anchor), n, l),
            rd,
        )
        .ok()
    };
    // `b`'s anchor decidedly behind `a`'s along `a`'s half-line.
    let behind = || match a.run {
        Run::Half(wa) => off(wa, None) == Some(Sign::Negative),
        Run::Line(_) => false,
    };
    let wb = match b.run {
        Run::Line(w) | Run::Half(w) => w,
    };
    // A sheet run along `a`'s line at first order may still lean toward
    // it by what that zero leaves: charged, out to the arm, where the
    // anchor's offset is read against it.
    let tilt = Distance::of(p + wb * l, p, a.normal);
    let side = off(a.normal, None);
    let clear = off(a.normal, Some(tilt));
    match b.run {
        Run::Line(wb) => {
            matches!(clear, Some(Sign::Positive | Sign::Negative))
                && toward(wb, a.normal) == Some(Sign::Zero)
        }
        Run::Half(wb) => match (side, toward(wb, a.normal)) {
            (Some(s @ (Sign::Positive | Sign::Negative)), Some(Sign::Zero)) => clear == Some(s),
            (Some(s @ (Sign::Positive | Sign::Negative)), Some(g)) => g == s,
            (Some(Sign::Zero), Some(Sign::Positive | Sign::Negative)) => behind(),
            (Some(Sign::Zero), Some(Sign::Zero)) => match a.run {
                Run::Half(wa) => toward(wb, wa) == Some(Sign::Negative) && behind(),
                Run::Line(_) => false,
            },
            _ => false,
        },
    }
}
