//! **A role path's canonical form**: which positions in a path are in
//! NAME ORDER.
//!
//! Some positions in a [`RolePath`](super::RolePath) have no order of
//! their own. Their canonical order is the order of the names they
//! hold, not the order an emitter found them in:
//!
//! - a [`RoleSeg::Merged`] constituent set and a [`RoleSeg::BandFace`]
//!   edge set: sorted, and deduplicated, because the SET is the name
//!   (N3);
//! - a [`Qualifier::Borders`] or [`Qualifier::Keeps`] set: sorted, and
//!   deduplicated, because the set is the qualifier (N2);
//! - a [`Qualifier::Ends`] pair: sorted and NOT deduplicated, because a
//!   piece whose two ends share a name is still a piece with two ends;
//! - a seam JUNCTION, the vertex where k ≥ 2 seam lines meet, named by
//!   the run of those lines' [`RoleSeg::Seam`] segments and nothing
//!   else: the run is sorted and NOT deduplicated. The lines are
//!   distinct by construction — the mint gathers them as a set, and a
//!   rewrite that makes two of them equal is not one-to-one, which the
//!   collapse refuses — so a repeat is a fault to refuse, never a
//!   duplicate to drop;
//! - a [`RoleSeg::EdgeCrossing`]'s run, each line with its sense:
//!   sorted, in every name;
//! - a [`RoleSeg::Seam`]'s two sides, in every name: the pair spells
//!   a set, and which input a side came from is in the side's own name,
//!   the face's name in the minting node's own table (N3).
//!
//! - a [`Sense::Through`]'s things, wherever a sense is carried (a
//!   `Crossing`, a `CrossingVertex`, each line of an `EdgeCrossing`):
//!   sorted and deduplicated, because the subset is the sense.
//!
//! Any rewrite that changes the names in those positions has to put
//! them back in order, or it publishes a name the emitter would not
//! mint for the same entity in the rewritten space. This module is the
//! one list of those positions, and [`form`] the one door every path is
//! built through: the pair emitter's mint (`emit_topo`, `emit_blend`),
//! the union's collapse (`emit_union`), and every rewrite of a
//! published name ([`StableName::rewrite_path`]: the anchor, the
//! whole-program edit and the split re-map).
//!
//! Putting a position in order changes no VALUE: nothing in a name is
//! read along an order of names. A seam's sense names the sides it
//! enters along, and a flush line's the edges, so a rewrite that
//! reorders a seam's pair re-sorts the pair and nothing else.
//!
//! [`StableName::rewrite_path`]: super::role::StableName

use super::role::{EntityKind, Qualifier, RoleSeg, RunLine, Sense, StableName, name_free_seg};

/// **A name in canonical form**: every name-ordered position in order
/// (module docs). Only the name's OWN positions are ordered: a name it
/// embeds reaches here already in canonical form, since the collapse
/// and every rewriter canonicalize each embedded name on the way down.
pub(crate) fn form(mut name: StableName) -> StableName {
    name.path = core::mem::take(&mut name.path)
        .into_iter()
        .map(minted_segment)
        .collect();
    if is_junction(&name) {
        name.path.sort();
    }
    one_constituent(name)
}

/// **A `Merged` whose set has one element, minted by the node that minted
/// the set, is that element** (N3): a face glued to its twin, the same
/// read spelled twice (REFERENCES DM5), is named as the face, with
/// whatever follows the set kept after it. A set of one name minted
/// upstream stays a set: hoisting that name would publish another node's
/// row in this node's table.
fn one_constituent(name: StableName) -> StableName {
    match name.path.split_first() {
        Some((RoleSeg::Merged(set), rest)) if set.len() == 1 && set[0].node == name.node => {
            let one = &set[0];
            let mut path = one.path.clone();
            path.extend(rest.iter().cloned());
            StableName {
                kind: name.kind,
                node: one.node,
                path,
            }
        }
        _ => name,
    }
}

/// Whether `name` is a seam JUNCTION: a vertex named by a run of two
/// or more `Seam` lines and nothing else.
pub(crate) fn is_junction(name: &StableName) -> bool {
    name.kind == EntityKind::Vertex
        && name.path.len() >= 2
        && name
            .path
            .iter()
            .all(|seg| matches!(seg, RoleSeg::Seam { .. }))
}

/// **One segment's own name-ordered positions**, in order: [`form`]'s
/// ordering one segment at a time, for an emitter that mints a segment
/// rather than a whole path (`emit_blend`'s band face) as well.
///
/// EXHAUSTIVE, with no wildcard (the `walk_names` rule): a variant
/// added to [`RoleSeg`] says here whether it carries a name-ordered
/// position, or stops the build.
#[allow(clippy::too_many_lines)] // one arm per name-carrying RoleSeg variant
pub(crate) fn minted_segment(seg: RoleSeg) -> RoleSeg {
    match seg {
        // The name-ordered positions.
        RoleSeg::Seam { a, b } if a > b => RoleSeg::Seam { a: b, b: a },
        RoleSeg::Crossing { edge, face, sense } => RoleSeg::Crossing {
            edge,
            face,
            sense: sorted_sense(sense),
        },
        RoleSeg::CrossingVertex { edge, sense } => RoleSeg::CrossingVertex {
            edge,
            sense: sorted_sense(sense),
        },
        RoleSeg::EdgeCrossing(run) => {
            let mut run: Vec<RunLine> = run
                .into_iter()
                .map(|l| RunLine {
                    line: l.line,
                    sense: sorted_sense(l.sense),
                })
                .collect();
            run.sort();
            RoleSeg::EdgeCrossing(run)
        }
        RoleSeg::Merged(set) => RoleSeg::Merged(sorted_set(set)),
        RoleSeg::BandFace(set) => RoleSeg::BandFace(sorted_set(set)),
        RoleSeg::BandCross { edge, band } => RoleSeg::BandCross {
            edge,
            band: sorted_set(band),
        },
        RoleSeg::BandSlit { edge, band } => RoleSeg::BandSlit {
            edge,
            band: sorted_set(band),
        },
        RoleSeg::Fragment(Qualifier::Borders(walls)) => {
            RoleSeg::Fragment(Qualifier::Borders(sorted_set(walls)))
        }
        RoleSeg::Fragment(Qualifier::Keeps(edges)) => {
            RoleSeg::Fragment(Qualifier::Keeps(sorted_set(edges)))
        }
        RoleSeg::Fragment(Qualifier::Ends(mut ends)) => {
            ends.sort();
            RoleSeg::Fragment(Qualifier::Ends(ends))
        }
        // Everything else carries its names, or none, in an order of
        // its own.
        RoleSeg::Seam { .. }
        | RoleSeg::From { .. }
        | RoleSeg::SectionEdge { .. }
        | RoleSeg::SplitFragment { .. }
        | RoleSeg::OnToolVertex { .. }
        | RoleSeg::BlendFace(_)
        | RoleSeg::CornerFace(_)
        | RoleSeg::TrimEdge { .. }
        | RoleSeg::FootVertex { .. }
        | RoleSeg::EndArc { .. }
        | RoleSeg::Mitre { .. }
        | RoleSeg::TurnFoot { .. }
        | RoleSeg::BandTrim { .. }
        | RoleSeg::BandFoot(_)
        | RoleSeg::BandCut(_)
        | RoleSeg::Inner(_)
        | RoleSeg::Rim(_)
        | RoleSeg::HoleRim { .. }
        | RoleSeg::Member { .. }
        | name_free_seg!() => seg,
    }
}

/// A sense in its canonical order: a `Through`'s things sorted, each
/// once.
fn sorted_sense(sense: Sense) -> Sense {
    match sense {
        Sense::Through { mut entering } => {
            entering.sort();
            entering.dedup();
            Sense::Through { entering }
        }
        plain @ (Sense::Enters | Sense::Leaves | Sense::Outside | Sense::Inside) => plain,
    }
}

/// A set of names in its canonical order: sorted, each name once.
fn sorted_set(mut set: Vec<StableName>) -> Vec<StableName> {
    set.sort();
    set.dedup();
    set
}

#[cfg(test)]
mod tests {
    //! Per segment kind: the canonical form is idempotent, and it does
    //! not depend on the order the positions were written in.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, NameRef};
    use crate::node::RecipeNodeId;

    fn seam(a: StableName, b: StableName) -> RoleSeg {
        RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        }
    }

    fn face(node: u64, member: u64) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, node),
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, member),
                of: NameRef::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId::new(0, member),
                    path: vec![RoleSeg::Cap(CapEnd::Start)],
                }),
            }],
        }
    }

    fn edge(node: u64, member: u64) -> StableName {
        let mut edge = face(node, member);
        edge.kind = EntityKind::Edge;
        edge
    }

    fn name(kind: EntityKind, node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind,
            node: RecipeNodeId::new(0, node),
            path,
        }
    }

    /// `written` in canonical form.
    fn canon(written: StableName) -> StableName {
        form(written)
    }

    /// Every permutation of `xs` (at most a handful of items).
    fn permutations<T: Clone>(xs: &[T]) -> Vec<Vec<T>> {
        if xs.len() <= 1 {
            return vec![xs.to_vec()];
        }
        let mut out = Vec::new();
        for i in 0..xs.len() {
            let mut rest = xs.to_vec();
            let x = rest.remove(i);
            for mut p in permutations(&rest) {
                p.insert(0, x.clone());
                out.push(p);
            }
        }
        out
    }

    /// `build` over every order of `items`, in canonical form: one form
    /// for all of them, and that form is a fixed point of [`form`].
    fn one_form<T: Clone>(items: &[T], build: impl Fn(Vec<T>) -> StableName) -> StableName {
        let forms: Vec<StableName> = permutations(items)
            .into_iter()
            .map(|p| canon(build(p)))
            .collect();
        for f in &forms {
            assert_eq!(
                f, &forms[0],
                "the canonical form depends on the written order"
            );
        }
        let f = forms[0].clone();
        assert_eq!(canon(f.clone()), f, "not a fixed point");
        f
    }

    #[test]
    fn a_merged_set_is_one_form_in_every_order() {
        let cs = [face(9, 3), face(9, 1), face(9, 2), face(9, 1)];
        let out = one_form(&cs, |p| name(EntityKind::Face, 9, vec![RoleSeg::Merged(p)]));
        assert_eq!(
            out.path,
            vec![RoleSeg::Merged(vec![face(9, 1), face(9, 2), face(9, 3)])],
            "sorted, each constituent once"
        );
    }

    #[test]
    fn a_band_face_set_is_one_form_in_every_order() {
        let es = [edge(4, 2), edge(4, 0), edge(4, 1), edge(4, 0)];
        let out = one_form(&es, |p| {
            name(EntityKind::Face, 5, vec![RoleSeg::BandFace(p)])
        });
        assert_eq!(
            out.path,
            vec![RoleSeg::BandFace(vec![edge(4, 0), edge(4, 1), edge(4, 2)])]
        );
        assert_eq!(
            minted_segment(RoleSeg::BandFace(es.to_vec())),
            out.path[0],
            "the one-segment door is the same ordering"
        );
    }

    #[test]
    fn a_borders_set_is_one_form_in_every_order() {
        let ws = [face(9, 2), face(9, 1), face(9, 3), face(9, 1)];
        let out = one_form(&ws, |w| {
            name(
                EntityKind::Face,
                9,
                vec![
                    RoleSeg::Cap(CapEnd::Start),
                    RoleSeg::Fragment(Qualifier::Borders(w)),
                ],
            )
        });
        assert_eq!(
            out.path.last(),
            Some(&RoleSeg::Fragment(Qualifier::Borders(vec![
                face(9, 1),
                face(9, 2),
                face(9, 3)
            ]))),
            "the walls are sorted and a repeat is one wall"
        );
    }

    #[test]
    fn a_junction_run_is_one_form_in_every_order() {
        let lines = [
            (face(9, 2), face(9, 3)),
            (face(9, 1), face(9, 2)),
            (face(9, 1), face(9, 3)),
        ];
        let build = |p: Vec<(StableName, StableName)>| {
            name(
                EntityKind::Vertex,
                9,
                p.into_iter().map(|(a, b)| seam(a, b)).collect(),
            )
        };
        // Each line's sides AND the run are in name order, so a line
        // written the other way round is the same line.
        let flipped: Vec<(StableName, StableName)> =
            lines.iter().map(|(a, b)| (b.clone(), a.clone())).collect();
        let by_name = one_form(&lines, build);
        assert_eq!(by_name, one_form(&flipped, build));
        assert_eq!(
            by_name.path,
            vec![
                seam(face(9, 1), face(9, 2)),
                seam(face(9, 1), face(9, 3)),
                seam(face(9, 2), face(9, 3)),
            ]
        );
    }

    fn run_line(line: StableName, sense: Sense) -> RunLine {
        RunLine {
            line: NameRef::new(line),
            sense,
        }
    }

    #[test]
    fn an_edge_crossing_run_is_one_form_in_every_order_each_sense_with_its_line() {
        let lines = [
            run_line(edge(9, 3), Sense::Leaves),
            run_line(edge(9, 1), Sense::Outside),
            run_line(
                name(EntityKind::Edge, 9, vec![seam(face(9, 1), face(9, 2))]),
                Sense::Through {
                    entering: vec![NameRef::new(face(9, 2))],
                },
            ),
        ];
        let out = one_form(&lines, |p| {
            name(EntityKind::Vertex, 9, vec![RoleSeg::EdgeCrossing(p)])
        });
        let [RoleSeg::EdgeCrossing(run)] = out.path.as_slice() else {
            panic!("one run: {out:?}")
        };
        assert!(run.windows(2).all(|w| w[0].line < w[1].line), "{run:?}");
        for l in &lines {
            assert!(run.contains(l), "{l:?} kept its sense: {run:?}");
        }
    }

    fn vertex(node: u64, member: u64) -> StableName {
        let mut vertex = face(node, member);
        vertex.kind = EntityKind::Vertex;
        vertex
    }

    fn ends(v0: StableName, v1: StableName) -> RoleSeg {
        RoleSeg::Fragment(Qualifier::Ends(vec![v0, v1]))
    }

    #[test]
    fn a_seam_edge_piece_is_one_form_from_either_side_and_whatever_order_its_ends_are_in() {
        let (x, y) = (face(9, 1), face(9, 2));
        let (v, w) = (vertex(9, 3), vertex(9, 4));
        let want = vec![seam(x.clone(), y.clone()), ends(v.clone(), w.clone())];
        for (a, b) in [(&x, &y), (&y, &x)] {
            for (v0, v1) in [(&v, &w), (&w, &v)] {
                let written = name(
                    EntityKind::Edge,
                    9,
                    vec![seam(a.clone(), b.clone()), ends(v0.clone(), v1.clone())],
                );
                let out = canon(written);
                assert_eq!(out.path, want, "the pair and the ends in name order");
                assert_eq!(canon(out.clone()), out, "not a fixed point");
            }
        }
        // A repeated end is kept: a piece has two ends.
        let closed = name(
            EntityKind::Edge,
            9,
            vec![seam(x.clone(), y.clone()), ends(v.clone(), v.clone())],
        );
        assert_eq!(canon(closed.clone()), closed);
    }

    /// **A crossing of a seam names the side it enters along, so the
    /// canonical form re-sorts nothing in it and changes no sense,
    /// whichever order the seam's pair came in.** A pair boolean (node
    /// 12) crossed a union's (node 9) seam edge entering along side `y`,
    /// the edge written by its rewriter in name order either way round.
    #[test]
    fn a_crossings_sense_on_a_seam_is_one_form_whichever_order_its_pair_is_in() {
        let cutter = face(11, 1);
        let crossing = |a: &StableName, b: &StableName, entering: &StableName| {
            name(
                EntityKind::Vertex,
                12,
                vec![RoleSeg::Crossing {
                    edge: NameRef::new(canon(name(
                        EntityKind::Edge,
                        9,
                        vec![seam(a.clone(), b.clone())],
                    ))),
                    face: NameRef::new(cutter.clone()),
                    sense: Sense::Through {
                        entering: vec![NameRef::new(entering.clone())],
                    },
                }],
            )
        };
        for (x, y) in [(face(9, 1), face(9, 2)), (face(9, 5), face(9, 4))] {
            let written = crossing(&x, &y, &y);
            assert_eq!(canon(written.clone()), written, "already canonical");
            assert_eq!(
                crossing(&y, &x, &y),
                written,
                "the pair written either way is one edge, entered along y"
            );
        }
    }

    /// **A `Through`'s things are one form in every order**, each once.
    #[test]
    fn a_through_senses_things_are_one_form_in_every_order() {
        let things = [edge(9, 3), edge(9, 1), edge(9, 2), edge(9, 1)];
        let out = one_form(&things, |p| {
            name(
                EntityKind::Vertex,
                9,
                vec![RoleSeg::CrossingVertex {
                    edge: NameRef::new(edge(9, 7)),
                    sense: Sense::Through {
                        entering: p.into_iter().map(NameRef::new).collect(),
                    },
                }],
            )
        });
        let [RoleSeg::CrossingVertex { sense, .. }] = out.path.as_slice() else {
            panic!("one crossing: {out:?}")
        };
        assert_eq!(
            *sense,
            Sense::Through {
                entering: [1, 2, 3]
                    .into_iter()
                    .map(|m| NameRef::new(edge(9, m)))
                    .collect()
            },
            "sorted, each thing once"
        );
    }

    #[test]
    fn a_seams_sides_are_in_name_order_whichever_node_minted_them() {
        // A pair seam beside a face the pair merged cites that face's
        // `Merged` parent and the other face's name in the pair's own
        // table (N3), both minted by the pair; a union's sides are its
        // own too. Either way the two are put in name order.
        let parent = name(
            EntityKind::Face,
            9,
            vec![RoleSeg::Merged(vec![face(9, 1), face(9, 3)])],
        );
        let beside = face(9, 2);
        let want = name(
            EntityKind::Edge,
            9,
            vec![seam(beside.clone(), parent.clone())],
        );
        assert!(
            parent > beside,
            "the premise: the merged parent names after"
        );
        for written in [
            name(
                EntityKind::Edge,
                9,
                vec![seam(parent.clone(), beside.clone())],
            ),
            want.clone(),
        ] {
            assert_eq!(canon(written.clone()), want, "{written:?}");
        }
    }
}
