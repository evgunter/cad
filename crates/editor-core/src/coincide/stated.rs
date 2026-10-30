//! **What the document's mates state** (`[coincidence.structural]`: "a
//! mate is that rung's statement", and "being one construction is the
//! equivalence the document's constructions and mates generate").
//!
//! A pose's construction ([`Key`]) is the construction of what it is
//! read off, by its kind: a face's plane is that face's construction
//! ([`super::construction`]); a carrier's axis is the axis of its
//! cell, except that a band a revolve mints is about the revolve's own
//! `axis` output, which is that rung's "projection of one
//! construction" ([`axis_of`]; the revolve's caps are not); a revolve's
//! `axis` output is itself; any other pose is the pose, through the
//! role it is read through. Each mate of a pinning bundle equates its
//! two sides' keys, each read in every copy that holds it where the
//! mate reads it ([`crate::mate::reads_in`]), with the sense its flips
//! state ([`crate::mate::poses_equated`]); [`Stated`] closes those
//! statements transitively. Rung one and the replay read this one
//! class function.

use super::{Construction, Map, Origin, construction};
use crate::names::RoleSeg;
use crate::node::{Node, RecipeNodeId};

/// **A pose's construction, of one kind**: two keys of one kind in one
/// class ([`Stated::sense`]) are one pose read twice.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Key {
    /// The pose kind.
    kind: crate::VarKind,
    /// What it is read off.
    at: Construction,
}

impl Key {
    /// The plane of a face whose construction is `at`.
    pub(crate) fn plane(at: &Construction) -> Self {
        Self {
            kind: crate::VarKind::Plane,
            at: at.clone(),
        }
    }

    /// The axis of a cell whose construction is `at` ([`axis_of`]).
    pub(crate) fn axis<P>(doc: &crate::doc::Doc<P>, at: &Construction) -> Self {
        Self {
            kind: crate::VarKind::Axis,
            at: axis_of(doc, at),
        }
    }
}

/// **The equivalence the mates generate** over pose constructions:
/// each key's parent and its sense against that parent (`true` where
/// the two are opposite), the root its own parent.
#[derive(Debug, Default)]
pub(crate) struct Stated {
    /// Every key a mate side reaches.
    keys: Vec<Key>,
    /// Each key's parent, and whether it is opposite that parent.
    parent: Vec<(usize, bool)>,
}

impl Stated {
    /// **What `doc`'s mates state**: every mate of a bundle the doors
    /// admit ([`crate::mate::solve::admits`]) equates each copy of its
    /// `on` side with each of its `to` side. A bundle they refuse
    /// (overconstrained, unpinned, or a mate refused on its own, as a
    /// load or a rebind can leave one) places no copy, so states
    /// nothing.
    ///
    /// # Panics
    ///
    /// Where two admitted statements equate one pose with itself and
    /// with its opposite ([`Self::union`]).
    pub(crate) fn of<P: crate::ProfilePayload>(
        doc: &crate::doc::Doc<P>,
        tol: geom_core::Tol,
    ) -> Self {
        use crate::mate::MateSide;
        let mut stated = Self::default();
        for placement in doc.placements() {
            let Some(bundle) = doc.node(placement).and_then(crate::mate::solve::bundle_of) else {
                continue;
            };
            if !crate::mate::solve::admits(doc, placement, bundle) {
                continue;
            }
            for mate in bundle.iter().filter_map(crate::mate::Constraint::mate) {
                let Some(kind) = doc.var(mate.on).map(crate::Var::kind) else {
                    continue;
                };
                let (on, to, flipped) = crate::mate::poses_equated(doc, mate);
                let on = side(doc, placement, MateSide::On, on, tol);
                let to = side(doc, placement, MateSide::To, to, tol);
                for a in &on {
                    for b in &to {
                        stated.union(
                            Key {
                                kind,
                                at: a.clone(),
                            },
                            Key {
                                kind,
                                at: b.clone(),
                            },
                            flipped,
                        );
                    }
                }
            }
        }
        stated
    }

    /// **Whether `a` and `b` are one pose read twice**, and their sense
    /// (`Some(true)` where opposite): one key, or two in one class, or
    /// two reads through one role of two frames (or planes, or axes) in
    /// one class with no flip between them. `None` where the mates
    /// state no such thing.
    pub(crate) fn sense(&self, a: &Key, b: &Key) -> Option<bool> {
        if a == b {
            return Some(false);
        }
        if let (Some((ra, pa)), Some((rb, pb))) = (self.find(a), self.find(b))
            && ra == rb
        {
            return Some(pa != pb);
        }
        // A role read of two poses one class equates, unflipped.
        let base = |k: &Key| match &k.at.origin {
            Origin::Pose(read, Some(role)) => Some((
                *role,
                Key {
                    kind: role.of(),
                    at: Construction {
                        origin: Origin::Pose(*read, None),
                        maps: k.at.maps.clone(),
                    },
                },
            )),
            _ => None,
        };
        match (base(a), base(b)) {
            (Some((ra, ba)), Some((rb, bb))) if ra == rb && a.kind == b.kind => {
                (self.sense(&ba, &bb) == Some(false)).then_some(false)
            }
            _ => None,
        }
    }

    /// **Every key in `key`'s class**, with its sense against `key`:
    /// `key` alone where no mate reaches it.
    pub(crate) fn class<'s>(&'s self, key: &'s Key) -> Vec<(&'s Construction, bool)> {
        let Some((root, parity)) = self.find(key) else {
            return vec![(&key.at, false)];
        };
        (0..self.keys.len())
            .filter_map(|i| {
                let (r, p) = self.root(i);
                (r == root).then(|| (&self.keys[i].at, p != parity))
            })
            .collect()
    }

    /// **The pose `key`'s class reaches**: a pose variable read plainly
    /// where it stands (a revolve's `axis` output, say) in it, else one
    /// read in a copy. `None` where no mate reaches the key, or its
    /// class holds no pose: the key is read off its cell alone.
    pub(crate) fn reached(&self, key: &Key) -> Option<crate::VarId> {
        self.find(key)?;
        let poses: Vec<(crate::VarId, bool)> = self
            .class(key)
            .into_iter()
            .filter_map(|(at, _)| match at.origin {
                Origin::Pose(read, None) => Some((read, at.maps.is_empty())),
                _ => None,
            })
            .collect();
        poses
            .iter()
            .find(|(_, here)| *here)
            .or(poses.first())
            .map(|(read, _)| *read)
    }

    /// The index of `key`, where a mate reaches it.
    fn index(&self, key: &Key) -> Option<usize> {
        self.keys.iter().position(|k| k == key)
    }

    /// `key`'s root and its sense against it.
    fn find(&self, key: &Key) -> Option<(usize, bool)> {
        self.index(key).map(|i| self.root(i))
    }

    /// The root of key `i` and its sense against it.
    fn root(&self, mut i: usize) -> (usize, bool) {
        let mut parity = false;
        loop {
            let (up, opposite) = self.parent[i];
            if up == i {
                return (i, parity);
            }
            parity ^= opposite;
            i = up;
        }
    }

    /// The index of `key`, added as its own class where it is new.
    fn insert(&mut self, key: Key) -> usize {
        self.index(&key).unwrap_or_else(|| {
            self.keys.push(key);
            self.parent.push((self.keys.len() - 1, false));
            self.keys.len() - 1
        })
    }

    /// States `a` and `b` one pose, opposite where `opposite`.
    ///
    /// # Panics
    ///
    /// Where the classes already hold `a` and `b` one pose of the other
    /// sense. Every `on` key is read in its own placement's copy, so
    /// two keys are joined twice only through `to` keys one mate reads
    /// alike, with one sense; an admitted bundle states no pose twice.
    fn union(&mut self, a: Key, b: Key, opposite: bool) {
        let (ia, ib) = (self.insert(a), self.insert(b));
        let ((ra, pa), (rb, pb)) = (self.root(ia), self.root(ib));
        if ra != rb {
            self.parent[rb] = (ra, pa ^ pb ^ opposite);
        } else if pa ^ pb != opposite {
            unreachable!(
                "the mates state {:?} and {:?} one pose in both senses, though every on side \
                 is read in its own copy and an admitted bundle states no pose twice",
                self.keys[ia], self.keys[ib]
            );
        }
    }
}

/// **The axis of the cell whose construction is `cell`**: a band a
/// revolve of this document mints is about that revolve's `axis`
/// output, read through the cell's maps (the minter's own statement,
/// beside [`super::lies_on`]'s "a rim lies on its cap", checked against
/// the symbolic evaluation by `coincidence_door`'s pin of every corpus
/// revolve). Any other cell's axis is its own. A part's entity is named
/// in the part's ids, which this document does not hold, so a band
/// the walk stopped at an instance for is its own wall's: a curved
/// wall's faces are one carrier ([`carrier`]).
pub(crate) fn axis_of<P>(doc: &crate::doc::Doc<P>, cell: &Construction) -> Construction {
    let cell = carrier(cell);
    let Origin::Minted(name) = &cell.origin else {
        return cell;
    };
    if !matches!(name.path.as_slice(), [RoleSeg::Band(_)]) || in_a_part(doc, &cell) {
        return cell;
    }
    match axis_output(doc, name.node) {
        Some(axis) => Construction {
            origin: Origin::Pose(axis, None),
            maps: cell.maps,
        },
        None => cell,
    }
}

/// **The carrier a swept face is on, named once**: a revolve's band's
/// two halves ([`RoleSeg::BandPi`] beside [`RoleSeg::Band`]) are one
/// carrier, and so are the faces an extrude or a revolve sweeps over the
/// pieces of one carrier-form loop (a circle's, every one an arc of one
/// carrier), named by the loop's first piece. Any other cell is its own.
pub(crate) fn carrier(cell: &Construction) -> Construction {
    use crate::names::ProfileEdgeRef;
    use profile::PieceRole;
    let Origin::Minted(name) = &cell.origin else {
        return cell.clone();
    };
    // A carrier form's piece is its loop's first.
    let first = |run: &crate::names::PieceRun| {
        crate::names::PieceRun::new(
            run.pieces()
                .iter()
                .map(|piece| match piece {
                    ProfileEdgeRef::Piece {
                        step,
                        role: PieceRole::Piece(_),
                    } => ProfileEdgeRef::Piece {
                        step: *step,
                        role: PieceRole::Piece(0),
                    },
                    other => *other,
                })
                .collect(),
        )
        .unwrap_or_else(|| run.clone())
    };
    let path = match name.path.as_slice() {
        [RoleSeg::Band(run) | RoleSeg::BandPi(run)] => vec![RoleSeg::Band(first(run))],
        [RoleSeg::Lateral(run)] => vec![RoleSeg::Lateral(first(run))],
        _ => return cell.clone(),
    };
    Construction {
        origin: Origin::Minted(crate::names::StableName {
            path,
            ..name.clone()
        }),
        maps: cell.maps.clone(),
    }
}

/// **Whether the walk to `cell` stopped at a part's instance**, so its
/// name is in the part's ids, which this document does not hold.
pub(crate) fn in_a_part<P>(doc: &crate::doc::Doc<P>, cell: &Construction) -> bool {
    matches!(
        cell.maps.last(),
        Some(Map::World(at)) if matches!(doc.node(*at), Some(Node::InstantiatePart { .. }))
    )
}

/// **The `axis` output of the node `node`**: a revolve's.
fn axis_output<P>(doc: &crate::doc::Doc<P>, node: RecipeNodeId) -> Option<crate::VarId> {
    let at = doc.node(node)?;
    if !matches!(at, Node::Revolve { .. }) {
        return None;
    }
    let port = at.outputs().iter().position(|p| p.name == "axis")?;
    doc.output(node, u8::try_from(port).ok()?)
}

/// **A pose read where it stands, and the body it is read off**: a
/// face's plane or a carrier's centre is its cell's construction, a
/// carrier's axis its cell's axis ([`axis_of`]), read off the
/// selection's body; a revolve's `axis` output is itself, read off the
/// revolve's body; any other pose is itself through its role, read off
/// no body. `None` where the selection is not one entity or the walk
/// stops.
fn pose_at<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    read: crate::pose::RoleRead<crate::VarId>,
    tol: geom_core::Tol,
) -> Option<(Construction, Option<crate::VarId>)> {
    use crate::pose::PoseDef;
    let pose = |role| Construction {
        origin: Origin::Pose(read.read, role),
        maps: Vec::new(),
    };
    if read.role.is_some() {
        return Some((pose(read.role), None));
    }
    let entity = |of: crate::VarId| {
        let select = doc.selection(of)?;
        let [name] = select.names.as_slice() else {
            return None;
        };
        let at = construction(doc, select.body, name, tol).ok()?;
        Some((at, select.body))
    };
    match doc.var(read.read).map(crate::Var::def)? {
        crate::var::VarDef::Pose(PoseDef::Plane { face }) => {
            entity(*face).map(|(at, body)| (at, Some(body)))
        }
        crate::var::VarDef::Pose(PoseDef::Point { of }) => {
            entity(*of).map(|(at, body)| (at, Some(body)))
        }
        crate::var::VarDef::Pose(PoseDef::Axis { of }) => {
            entity(*of).map(|(at, body)| (axis_of(doc, &at), Some(body)))
        }
        crate::var::VarDef::Output { node, .. } if axis_output(doc, *node) == Some(read.read) => {
            Some((pose(None), doc.output(*node, 0)))
        }
        _ => Some((pose(None), None)),
    }
}

/// **The constructions of the copies a mate side is read in**: the
/// side's pose where it stands ([`pose_at`]), in each copy of its body
/// that holds it where a mate of `placement` reads it
/// ([`crate::mate::reads_in`], the rule the door reads the mate by):
/// the body itself where it is that copy's output, else under the
/// `Place` that copies it. A body no placement outputs stands where it
/// was built, which is where its `to` side reads it too; a pose read
/// off no body is read in `placement`'s copy on its `on` side and where
/// it stands on its `to` side. Empty where the walk stops.
fn side<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    placement: RecipeNodeId,
    side: crate::mate::MateSide,
    read: crate::pose::RoleRead<crate::VarId>,
    tol: geom_core::Tol,
) -> Vec<Construction> {
    use crate::mate::MateSide;
    let Some((at, body)) = pose_at(doc, read, tol) else {
        return Vec::new();
    };
    let copied = |p: RecipeNodeId| Construction {
        origin: at.origin.clone(),
        maps: core::iter::once(Map::World(p))
            .chain(at.maps.iter().cloned())
            .collect(),
    };
    let Some(body) = body else {
        return vec![match side {
            MateSide::On => copied(placement),
            MateSide::To => at,
        }];
    };
    let copies = crate::mate::copies_of(doc, body);
    let built = !copies
        .iter()
        .any(|&(_, how)| how == crate::mate::Copying::Own);
    let mut read: Vec<Construction> = copies
        .into_iter()
        .filter(|&copy| crate::mate::reads_in(doc, placement, side, body, copy))
        .map(|(p, how)| match how {
            crate::mate::Copying::Own => at.clone(),
            crate::mate::Copying::Whole | crate::mate::Copying::Member => copied(p),
        })
        .collect();
    if side == MateSide::To && built {
        read.push(at);
    }
    read
}

/// **The axis a frame's `z` is along, by construction**: the axis a
/// [`crate::pose::PoseDef::Through`] frame is read through, kept by a
/// slide (which moves no direction), a flip (which reverses it), and a
/// spin about an axis one class with it (`stated`). Its direction alone:
/// a slide across it moves the line, never the heading. `None` where
/// the frame is read any other way, or a spin turns its `z`.
pub(crate) fn frame_z<P: crate::ProfilePayload>(
    doc: &crate::doc::Doc<P>,
    stated: &Stated,
    frame: crate::VarId,
    tol: geom_core::Tol,
) -> Option<Key> {
    use crate::pose::{PoseDef, PoseRole, RoleRead};
    // An axis read: a frame's `z`, or an axis read where it stands.
    let axis = |read: &RoleRead<crate::VarId>| match read.role {
        Some(PoseRole::Z) => frame_z(doc, stated, read.read, tol),
        Some(_) => None,
        None => (doc.var(read.read).map(crate::Var::kind) == Some(crate::VarKind::Axis))
            .then(|| pose_at(doc, *read, tol))
            .flatten()
            .map(|(at, _)| Key {
                kind: crate::VarKind::Axis,
                at,
            }),
    };
    let crate::var::VarDef::Pose(def) = doc.var(frame).map(crate::Var::def)? else {
        return None;
    };
    match def {
        PoseDef::Through { axis: z, .. } => axis(z),
        PoseDef::Slide { pose, .. } | PoseDef::Flip { pose } if pose.role.is_none() => {
            frame_z(doc, stated, pose.read, tol)
        }
        PoseDef::Spin { pose, about, .. } if pose.role.is_none() => {
            let z = frame_z(doc, stated, pose.read, tol)?;
            let about = axis(about)?;
            stated.sense(&z, &about).is_some().then_some(z)
        }
        _ => None,
    }
}
