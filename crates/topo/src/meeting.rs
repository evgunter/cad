//! **What the Boolean's join meets**: the face pairs its germ-pair join
//! joins and the edge kinds every lane walks, each stated once, with
//! the recourses that name them. The join's lane dispatch reads the
//! same table ([`plane_joins`]), so a recourse names a pair exactly
//! when the join takes it.

use geom::SurfaceKind;

/// The wall kinds a plane face joins against, in the order
/// [`meeting_recourse`] names them. A plane joins a plane too.
pub(crate) const PLANE_WALLS: [SurfaceKind; 4] = [
    SurfaceKind::Cylinder,
    SurfaceKind::Sphere,
    SurfaceKind::Cone,
    SurfaceKind::Torus,
];

/// Whether a plane face's germ pair against a `wall` face is joined.
pub(crate) fn plane_joins(wall: SurfaceKind) -> bool {
    PLANE_WALLS.contains(&wall)
}

/// A wall kind as [`meeting_recourse`] names it: a torus joins only
/// where the plane holds its axis or stands square to it.
fn wall_clause(wall: SurfaceKind) -> String {
    match wall {
        SurfaceKind::Torus => format!("{} whose axis it holds or is square to", wall.word()),
        wall => wall.word().to_owned(),
    }
}

/// The recourse every "these faces cannot meet yet" refusal ends on,
/// naming the pairs [`plane_joins`] admits. The operand gate's box test
/// is conservative (a box overlap is a MAY), so moving the face clear
/// of the other solid is a real recourse even where the faces never
/// touch in exact geometry.
#[must_use]
pub(crate) fn meeting_recourse(kind: &str) -> String {
    let (last, rest) = PLANE_WALLS
        .split_last()
        .map(|(last, rest)| (wall_clause(*last), rest))
        .unwrap_or_default();
    let rest: Vec<String> = rest.iter().map(|w| wall_clause(*w)).collect();
    format!(
        "Recourse: reshape the parts so every meeting is a plane against a plane, {}, or \
         {last}, or move the {kind} face clear of the other solid",
        rest.join(", ")
    )
}

/// The edge kinds every lane walks: the line and the two conics.
macro_rules! walked_edge_kinds {
    () => {
        "lines, circles or ellipses"
    };
}

/// The recourse of every refusal of a spiric or spline edge of a solid.
pub(crate) const CONIC_EDGES_RECOURSE: &str =
    concat!("rebuild that solid so its edges are ", walked_edge_kinds!());

/// The recourse of every refusal of a spiric or spline edge on a face's
/// outline.
pub(crate) const CONIC_OUTLINE_RECOURSE: &str =
    concat!("model the outline with ", walked_edge_kinds!());

#[cfg(test)]
mod tests {
    use strum::VariantArray as _;

    use super::*;

    /// **The recourse names a wall kind exactly when the join takes
    /// it.** Red where a kind joins [`PLANE_WALLS`] without reaching the
    /// sentence, or the sentence names one the table lacks.
    #[test]
    fn the_recourse_names_the_pairs_the_join_takes() {
        let said = meeting_recourse("spline");
        let list = said
            .split_once("a plane against a plane, ")
            .and_then(|(_, tail)| tail.split_once(", or move the "))
            .map(|(list, _)| list)
            .unwrap_or_default();
        for &kind in SurfaceKind::VARIANTS {
            if kind == SurfaceKind::Plane {
                continue;
            }
            assert_eq!(
                list.contains(kind.word()),
                plane_joins(kind),
                "{kind:?} in {list:?}"
            );
        }
        assert_eq!(
            said,
            "Recourse: reshape the parts so every meeting is a plane against a plane, \
             cylinder, sphere, cone, or torus whose axis it holds or is square to, or move \
             the spline face clear of the other solid"
        );
    }
}
