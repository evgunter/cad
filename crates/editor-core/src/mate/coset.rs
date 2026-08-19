//! **The coset algebra** — the binding class×class intersection table
//! (ASM-R2a D-4; ASM-R2-SPEC-DRAFT "R2-a coset table"; A11 rule 1).
//!
//! Each mate primitive pins a pair's relative pose to a COSET: a
//! representative pose times a residual SE(3) subgroup. Folding a
//! pair's mates is exact coset intersection, and because intersection
//! is order-independent the table must cover the CLOSURE of the
//! primitive set. That closure is seven entries —
//! [`Subgroup::Se3`], [`Subgroup::Planar`], [`Subgroup::Cylindrical`],
//! [`Subgroup::Prismatic`], [`Subgroup::Revolute`],
//! [`Subgroup::Trivial`], [`Subgroup::Empty`] — and its closedness is
//! a PROOF OBLIGATION, executed as the enumeration test rather than
//! asserted in prose. No screw subgroup arises: the primitives'
//! parallel-cylinder intersection is pure translation.
//!
//! # Why the intersection is two stages, not a table of formulas
//!
//! A coset `G·R` intersects `G′·R′` in either the empty set or a coset
//! of `G ∩ G′` — elementary, and it splits the work exactly:
//!
//! 1. **The subgroup** ([`intersect_subgroups`]) is the table's case
//!    splits — every one a decided predicate through the `k_stats`
//!    funnel with a NAMED predicate, Indeterminate escalating typed.
//! 2. **The representative** is then constructed to satisfy the HELD
//!    side exactly and checked against the added side, so a failure's
//!    measured clash is the added mate's own violation rather than an
//!    averaged residue — which is what a CONTRADICTORY refusal must
//!    quote.
//!
//! Membership is itself a decided predicate per subgroup, so step 2's
//! check is the same funnel: "an unsatisfiable candidate is empty,
//! named", the table's own closing sentence.

use geom_core::k_stats::decide;
use geom_core::linalg::{Affine3, Mat3, Point3, Vec3};
use geom_core::predicate::{Band, Indeterminate, Margin, Sign};

/// A residual SE(3) subgroup — the closure set the table is closed
/// over. Directions are UNIT by construction of every constructor
/// here; a subgroup's parameters are exactly what an UNDER refusal
/// must name.
#[derive(Debug, Clone, Copy)]
pub enum Subgroup {
    /// Unconstrained: the fold's identity element (dim 6).
    Se3,
    /// The planar group of a plane with unit normal `normal`: the two
    /// in-plane translations plus rotation about the normal (dim 3).
    /// Point-free — rotations about ANY axis parallel to the normal
    /// are in it, so no base point distinguishes one plane from a
    /// parallel one.
    Planar {
        /// The plane's unit normal.
        normal: Vec3<f64>,
    },
    /// The cylindrical group of a line: rotation about it plus
    /// translation along it (dim 2).
    Cylindrical {
        /// A point on the line.
        point: Point3<f64>,
        /// The line's unit direction.
        direction: Vec3<f64>,
    },
    /// Translation along a unit direction (dim 1). Point-free.
    Prismatic {
        /// The translation's unit direction.
        direction: Vec3<f64>,
    },
    /// Rotation about a line (dim 1).
    Revolute {
        /// A point on the axis.
        point: Point3<f64>,
        /// The axis's unit direction.
        direction: Vec3<f64>,
    },
    /// The identity alone: DETERMINED (dim 0).
    Trivial,
    /// No pose satisfies the fold: CONTRADICTORY. Carried as a closure
    /// element rather than an error type so `X ∩ empty = empty` is a
    /// table entry that can be enumerated like every other.
    Empty,
}

/// Coordinate-wise equality of the linear types, which carry no
/// `PartialEq` of their own (the kernel keeps float comparison at the
/// decided-predicate door, deliberately). This is STRUCTURAL equality
/// for tests and diagnostics — never a geometric one; two subgroups
/// that are equal as SETS may differ here (a line stated at a
/// different base point), which is exactly why the algebra decides
/// coincidence through predicates instead of comparing values.
fn vec_eq(a: Vec3<f64>, b: Vec3<f64>) -> bool {
    a.x == b.x && a.y == b.y && a.z == b.z
}

fn point_eq(a: Point3<f64>, b: Point3<f64>) -> bool {
    a.x == b.x && a.y == b.y && a.z == b.z
}

impl PartialEq for Subgroup {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Se3, Self::Se3)
            | (Self::Trivial, Self::Trivial)
            | (Self::Empty, Self::Empty) => true,
            (Self::Planar { normal: a }, Self::Planar { normal: b })
            | (Self::Prismatic { direction: a }, Self::Prismatic { direction: b }) => {
                vec_eq(*a, *b)
            }
            (
                Self::Cylindrical {
                    point: pa,
                    direction: da,
                },
                Self::Cylindrical {
                    point: pb,
                    direction: db,
                },
            )
            | (
                Self::Revolute {
                    point: pa,
                    direction: da,
                },
                Self::Revolute {
                    point: pb,
                    direction: db,
                },
            ) => point_eq(*pa, *pb) && vec_eq(*da, *db),
            _ => false,
        }
    }
}

impl PartialEq for Coset {
    fn eq(&self, other: &Self) -> bool {
        let m = |x: &Affine3<f64>| [x.linear.c0, x.linear.c1, x.linear.c2, x.translation];
        self.subgroup == other.subgroup
            && m(&self.representative)
                .into_iter()
                .zip(m(&other.representative))
                .all(|(a, b)| vec_eq(a, b))
    }
}

impl Subgroup {
    /// The subgroup's dimension as a manifold, `None` for
    /// [`Subgroup::Empty`] (which is not a subgroup at all).
    pub fn dimension(&self) -> Option<u8> {
        Some(match self {
            Self::Se3 => 6,
            Self::Planar { .. } => 3,
            Self::Cylindrical { .. } => 2,
            Self::Prismatic { .. } | Self::Revolute { .. } => 1,
            Self::Trivial => 0,
            Self::Empty => return None,
        })
    }

    /// The subgroup's family name, for messages and table rows.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Se3 => "SE(3)",
            Self::Planar { .. } => "planar",
            Self::Cylindrical { .. } => "cylindrical",
            Self::Prismatic { .. } => "prismatic",
            Self::Revolute { .. } => "revolute",
            Self::Trivial => "trivial",
            Self::Empty => "empty",
        }
    }

    /// The subgroup NAMED WITH ITS PARAMETERS — what A11 rule 4's
    /// refusal quotes, so an author reads which freedom survived and
    /// along which direction, not merely that one did.
    pub fn describe(&self) -> String {
        let dir = |v: &Vec3<f64>| format!("[{}, {}, {}]", v.x, v.y, v.z);
        let pt = |p: &Point3<f64>| format!("({}, {}, {})", p.x, p.y, p.z);
        match self {
            Self::Se3 => "the full rigid freedom SE(3) — no mate constrains this pair".to_string(),
            Self::Planar { normal } => format!(
                "the planar freedom of the plane with normal {} (two in-plane translations and \
                 rotation about the normal)",
                dir(normal)
            ),
            Self::Cylindrical { point, direction } => format!(
                "the cylindrical freedom of the axis through {} along {} (rotation about it and \
                 translation along it)",
                pt(point),
                dir(direction)
            ),
            Self::Prismatic { direction } => format!(
                "the prismatic freedom of translation along {}",
                dir(direction)
            ),
            Self::Revolute { point, direction } => format!(
                "the revolute freedom of rotation about the axis through {} along {}",
                pt(point),
                dir(direction)
            ),
            Self::Trivial => "no freedom — the pose is determined".to_string(),
            Self::Empty => "nothing — the mates cannot both hold".to_string(),
        }
    }

    /// Whether this subgroup determines the pose outright.
    pub fn is_determined(&self) -> bool {
        matches!(self, Self::Trivial)
    }

    /// The rotations this subgroup contains, as the axis they all fix.
    fn rotations(&self) -> Rotations {
        match self {
            Self::Se3 => Rotations::Free,
            Self::Planar { normal } => Rotations::About(*normal),
            Self::Cylindrical { direction, .. } | Self::Revolute { direction, .. } => {
                Rotations::About(*direction)
            }
            Self::Prismatic { .. } | Self::Trivial | Self::Empty => Rotations::Fixed,
        }
    }

    /// The orthogonal projector onto this subgroup's PURE-TRANSLATION
    /// directions — the freedom a candidate translation may keep.
    fn translation_freedom(&self) -> Mat3<f64> {
        match self {
            Self::Se3 => Mat3::identity(),
            Self::Planar { normal } => sub(Mat3::identity(), outer(*normal, *normal)),
            Self::Cylindrical { direction, .. } | Self::Prismatic { direction } => {
                outer(*direction, *direction)
            }
            Self::Revolute { .. } | Self::Trivial | Self::Empty => zero3(),
        }
    }

    /// This subgroup's translation CONSTRAINT at a known rotation `a`
    /// (the candidate's rotation relative to the coset representative's):
    /// the pair `(P, q)` for which membership reads `P·(t − q) = 0`.
    ///
    /// Every entry is elementary once written down: a revolute's
    /// elements fix their axis pointwise, a cylindrical's move it
    /// only along itself, a planar's move no point across the plane.
    /// The one thing worth stating is why a point enters at all — a
    /// rotation about a line NOT through the origin carries a
    /// translation `(I − Q)p`, so the affine part cannot be split off
    /// and read separately.
    fn translation_constraint(&self, a: Mat3<f64>, r: Vec3<f64>) -> (Mat3<f64>, Vec3<f64>) {
        let ar = a * r;
        match self {
            Self::Se3 => (zero3(), Vec3::new(0.0, 0.0, 0.0)),
            Self::Trivial | Self::Empty => (Mat3::identity(), ar),
            Self::Prismatic { direction } => {
                (sub(Mat3::identity(), outer(*direction, *direction)), ar)
            }
            Self::Planar { normal } => (outer(*normal, *normal), ar),
            Self::Cylindrical { point, direction } => {
                let p = *point - Point3::origin();
                (
                    sub(Mat3::identity(), outer(*direction, *direction)),
                    p - a * p + ar,
                )
            }
            Self::Revolute { point, .. } => {
                let p = *point - Point3::origin();
                (Mat3::identity(), p - a * p + ar)
            }
        }
    }
}

/// The rotations a subgroup contains.
#[derive(Debug, Clone, Copy)]
enum Rotations {
    /// All of SO(3).
    Free,
    /// Exactly the rotations about this unit axis.
    About(Vec3<f64>),
    /// The identity alone.
    Fixed,
}

/// A coset of [`Subgroup`]: the set `{ g · representative : g ∈ subgroup }`
/// of relative poses a pair's mates admit.
///
/// The representative is meaningless when the subgroup is
/// [`Subgroup::Empty`]; every consumer must read the subgroup first.
#[derive(Debug, Clone, Copy)]
pub struct Coset {
    /// The residual freedom.
    pub subgroup: Subgroup,
    /// One admitted pose (`b`'s part coordinates into `a`'s).
    pub representative: Affine3<f64>,
}

impl Coset {
    /// The unconstrained coset — the fold's identity element.
    pub fn unconstrained() -> Self {
        Self {
            subgroup: Subgroup::Se3,
            representative: Affine3::identity(),
        }
    }
}

/// Why a fold step could not produce a coset.
#[derive(Debug, Clone, PartialEq)]
pub enum FoldStop {
    /// A case split landed in the ambiguity band.
    Indeterminate(Box<Indeterminate>),
    /// The intersection is empty: the named predicate measured a clash
    /// where the cosets would have had to meet.
    Clash {
        /// The membership predicate that refused.
        predicate: &'static str,
        /// Its measured margin, in metres.
        margin: f64,
    },
}

impl From<Indeterminate> for FoldStop {
    fn from(value: Indeterminate) -> Self {
        Self::Indeterminate(Box::new(value))
    }
}

// ---- The table's case splits, each a named decided predicate ----

/// Predicate: two unit directions are parallel (either sense). The
/// margin is the sine of the angle between them, levered by `arm` into
/// the displacement it induces there.
fn parallel(u: Vec3<f64>, v: Vec3<f64>, band: Band, arm: f64) -> Result<bool, Indeterminate> {
    let sin = u.cross(v).norm();
    Ok(decide("mate_axes_parallel", Margin::levered(sin, arm), band)? == Sign::Zero)
}

/// Predicate: a unit direction is perpendicular to a unit normal. The
/// margin is the cosine, levered into a displacement at `arm`.
fn perpendicular(u: Vec3<f64>, n: Vec3<f64>, band: Band, arm: f64) -> Result<bool, Indeterminate> {
    Ok(decide(
        "mate_axis_normal_perpendicular",
        Margin::levered(u.dot(n), arm),
        band,
    )? == Sign::Zero)
}

/// Predicate: a point lies on a line. The margin is its perpendicular
/// offset — already a length.
fn point_on_line(
    p: Point3<f64>,
    origin: Point3<f64>,
    direction: Vec3<f64>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let d = p - origin;
    let off = d - direction * d.dot(direction);
    Ok(decide("mate_axis_point_offset", Margin::norm3(off), band)? == Sign::Zero)
}

/// **The subgroup half of the binding table**: `G ∩ G′`, every case
/// split decided through the funnel.
///
/// Unordered as the table is: every entry states BOTH orders as one
/// arm, which is also what makes the match EXHAUSTIVE without a
/// catch-all — a new closure element would fail to compile here rather
/// than fall into a silent default, which is the enumeration's whole
/// point. `arm` is the lever the angular splits turn on (D4 ¶1: an
/// angle decides only through the displacement it induces at a named
/// length).
///
/// # Errors
///
/// [`Indeterminate`] when a case split lands in the ambiguity band —
/// the typed escalation the spec demands, never a silent pick.
pub fn intersect_subgroups(
    g1: Subgroup,
    g2: Subgroup,
    band: Band,
    arm: f64,
) -> Result<Subgroup, Indeterminate> {
    use Subgroup::{Cylindrical, Empty, Planar, Prismatic, Revolute, Se3, Trivial};
    Ok(match (g1, g2) {
        // The universal entries: empty absorbs, SE(3) is the identity,
        // trivial is the zero. Stated first, and between them they
        // cover every tuple touching those three — so what remains
        // below is exactly the four proper families.
        (Empty, _) | (_, Empty) => Empty,
        (Se3, other) | (other, Se3) => other,
        (Trivial, _) | (_, Trivial) => Trivial,
        // 1. planar ∩ planar — the flush merge and the V-block.
        (Planar { normal: n1 }, Planar { normal: n2 }) => {
            if parallel(n1, n2, band, arm)? {
                Planar { normal: n1 }
            } else {
                Prismatic {
                    direction: n1.cross(n2).normalize(),
                }
            }
        }
        // 2. planar ∩ cylindrical — pin-in-hole, slot, and the generic
        //    angle that kills both freedoms.
        (
            Planar { normal: n },
            Cylindrical {
                point: p,
                direction: u,
            },
        )
        | (
            Cylindrical {
                point: p,
                direction: u,
            },
            Planar { normal: n },
        ) => {
            if parallel(u, n, band, arm)? {
                Revolute {
                    point: p,
                    direction: u,
                }
            } else if perpendicular(u, n, band, arm)? {
                Prismatic { direction: u }
            } else {
                Trivial
            }
        }
        // 3. planar ∩ prismatic.
        (Planar { normal: n }, Prismatic { direction: d })
        | (Prismatic { direction: d }, Planar { normal: n }) => {
            if perpendicular(d, n, band, arm)? {
                Prismatic { direction: d }
            } else {
                Trivial
            }
        }
        // 4. planar ∩ revolute.
        (
            Planar { normal: n },
            Revolute {
                point: p,
                direction: u,
            },
        )
        | (
            Revolute {
                point: p,
                direction: u,
            },
            Planar { normal: n },
        ) => {
            if parallel(u, n, band, arm)? {
                Revolute {
                    point: p,
                    direction: u,
                }
            } else {
                Trivial
            }
        }
        // 5. cylindrical ∩ cylindrical — same line, parallel-distinct,
        //    or concurrent/skew.
        (
            Cylindrical {
                point: p1,
                direction: u1,
            },
            Cylindrical {
                point: p2,
                direction: u2,
            },
        ) => {
            if parallel(u1, u2, band, arm)? {
                if point_on_line(p2, p1, u1, band)? {
                    Cylindrical {
                        point: p1,
                        direction: u1,
                    }
                } else {
                    Prismatic { direction: u1 }
                }
            } else {
                Trivial
            }
        }
        // 6. cylindrical ∩ prismatic.
        (Cylindrical { direction: u, .. }, Prismatic { direction: d })
        | (Prismatic { direction: d }, Cylindrical { direction: u, .. }) => {
            if parallel(d, u, band, arm)? {
                Prismatic { direction: d }
            } else {
                Trivial
            }
        }
        // 7. cylindrical ∩ revolute.
        (
            Cylindrical {
                point: pc,
                direction: uc,
            },
            Revolute {
                point: pr,
                direction: ur,
            },
        )
        | (
            Revolute {
                point: pr,
                direction: ur,
            },
            Cylindrical {
                point: pc,
                direction: uc,
            },
        ) => {
            if parallel(uc, ur, band, arm)? && point_on_line(pr, pc, uc, band)? {
                Revolute {
                    point: pr,
                    direction: ur,
                }
            } else {
                Trivial
            }
        }
        // 8. prismatic ∩ prismatic.
        (Prismatic { direction: d1 }, Prismatic { direction: d2 }) => {
            if parallel(d1, d2, band, arm)? {
                Prismatic { direction: d1 }
            } else {
                Trivial
            }
        }
        // 9. prismatic ∩ revolute — a translation is never a rotation.
        (Prismatic { .. }, Revolute { .. }) | (Revolute { .. }, Prismatic { .. }) => Trivial,
        // 10. revolute ∩ revolute.
        (
            Revolute {
                point: p1,
                direction: u1,
            },
            Revolute {
                point: p2,
                direction: u2,
            },
        ) => {
            if parallel(u1, u2, band, arm)? && point_on_line(p2, p1, u1, band)? {
                Revolute {
                    point: p1,
                    direction: u1,
                }
            } else {
                Trivial
            }
        }
    })
}

// ---- Membership: the decided predicate the representative meets ----

/// Whether `x` lies in `g` (as a subgroup of SE(3), not a coset).
/// Returns the failing predicate and its measured margin when it does
/// not — the CONTRADICTORY refusal's own quotation.
fn member_of(
    g: Subgroup,
    x: Affine3<f64>,
    band: Band,
    arm: f64,
) -> Result<Result<(), (&'static str, f64)>, Indeterminate> {
    let axis_fixed = |axis: Vec3<f64>| {
        (
            "mate_member_axis_fixed",
            (x.linear * axis - axis).norm() * arm,
        )
    };
    let checks: Vec<(&'static str, f64)> = match g {
        // The empty set holds nothing, and no margin decides that —
        // the answer is structural, so it never reaches the funnel.
        Subgroup::Empty => return Ok(Err(("mate_member_empty", f64::INFINITY))),
        Subgroup::Se3 => Vec::new(),
        Subgroup::Trivial => vec![
            (
                "mate_member_rotation_identity",
                rotation_residual(x.linear, arm),
            ),
            ("mate_member_translation_zero", x.translation.norm()),
        ],
        Subgroup::Prismatic { direction } => vec![
            (
                "mate_member_rotation_identity",
                rotation_residual(x.linear, arm),
            ),
            (
                "mate_member_translation_along",
                x.translation.reject_from(direction).norm(),
            ),
        ],
        Subgroup::Planar { normal } => vec![
            axis_fixed(normal),
            (
                "mate_member_translation_in_plane",
                x.translation.dot(normal),
            ),
        ],
        Subgroup::Cylindrical { point, direction } => vec![
            axis_fixed(direction),
            (
                "mate_member_point_on_axis",
                (x.transform_point(point) - point)
                    .reject_from(direction)
                    .norm(),
            ),
        ],
        Subgroup::Revolute { point, direction } => vec![
            axis_fixed(direction),
            (
                "mate_member_point_fixed",
                (x.transform_point(point) - point).norm(),
            ),
        ],
    };
    for (name, margin) in checks {
        if decide(name, Margin::of(margin), band)? != Sign::Zero {
            return Ok(Err((name, margin)));
        }
    }
    Ok(Ok(()))
}

/// The displacement a rotation's departure from the identity induces at
/// lever arm `arm`: the Frobenius norm of `Q − I`, levered.
fn rotation_residual(q: Mat3<f64>, arm: f64) -> f64 {
    let d = sub(q, Mat3::identity());
    (d.c0.norm_squared() + d.c1.norm_squared() + d.c2.norm_squared()).sqrt() * arm
}

// ---- The intersection proper ----

/// **Intersect two cosets** — the table, executed.
///
/// The result's representative satisfies `held` exactly by
/// construction and `added` up to the decided membership check, so a
/// [`FoldStop::Clash`] quotes the ADDED mate's own violation. `arm` is
/// the lever the angular decisions turn on.
///
/// # Errors
///
/// [`FoldStop::Indeterminate`] when a case split or membership check
/// lands in the ambiguity band; [`FoldStop::Clash`] when the
/// intersection is empty.
pub fn intersect(held: Coset, added: Coset, band: Band, arm: f64) -> Result<Coset, FoldStop> {
    if matches!(held.subgroup, Subgroup::Empty) || matches!(added.subgroup, Subgroup::Empty) {
        return Ok(Coset {
            subgroup: Subgroup::Empty,
            representative: held.representative,
        });
    }
    // X ∩ SE(3) = X, verbatim: nothing to construct and nothing to
    // check, so the fold's identity element costs no arithmetic.
    if matches!(held.subgroup, Subgroup::Se3) {
        return Ok(added);
    }
    if matches!(added.subgroup, Subgroup::Se3) {
        return Ok(held);
    }
    let residual = intersect_subgroups(held.subgroup, added.subgroup, band, arm)?;
    let rotation = candidate_rotation(held, added, band, arm)?;
    let translation = candidate_translation(held, added, residual, rotation);
    let x = Affine3::from_parts(rotation, translation);
    for coset in [held, added] {
        let relative = x * coset.representative.inverse();
        if let Err((predicate, margin)) = member_of(coset.subgroup, relative, band, arm)? {
            return Err(FoldStop::Clash { predicate, margin });
        }
    }
    Ok(Coset {
        subgroup: residual,
        representative: x,
    })
}

/// Stage one: the candidate's rotation.
///
/// The rotation constraint of a coset `G·R` is that `Q·Q_R⁻¹` lie in
/// `G`'s rotations, which are all of SO(3), the rotations about one
/// axis, or the identity alone. Two one-axis constraints meet through
/// the two-axis condition below; everything else is forced.
fn candidate_rotation(
    held: Coset,
    added: Coset,
    band: Band,
    arm: f64,
) -> Result<Mat3<f64>, FoldStop> {
    let q1 = held.representative.linear;
    let q2 = added.representative.linear;
    Ok(
        match (held.subgroup.rotations(), added.subgroup.rotations()) {
            (Rotations::Free, _) => q2,
            (_, Rotations::Free) => q1,
            // The tighter side forces the rotation; the looser side's
            // violation, if any, surfaces at the membership check.
            (Rotations::Fixed, _) => q1,
            (_, Rotations::Fixed) => q2,
            (Rotations::About(a1), Rotations::About(a2)) => {
                if parallel(a1, a2, band, arm)? {
                    // PARALLEL rotation axes: the held side admits EVERY
                    // rotation about its own axis, so the candidate's
                    // clocking about that axis is free — and free means it
                    // must be SOLVED, not inherited. Pinning it to the
                    // held representative's clocking yields a pose that is
                    // merely IN the held coset rather than a witness of
                    // the intersection, and the (sound) membership check
                    // then refuses an assemblable pair: review MAJOR-1's
                    // two-pin pattern, inter-axis invariants matching but
                    // the patterns clocked apart.
                    clocking_about(held, added, a1) * q1
                } else {
                    // Two distinct rotation axes. Write the unknown as
                    // `rot(a1, α)·Q1`; requiring it to fix a2 after the
                    // added side's representative is undone gives one
                    // equation in α, solvable iff the two vectors share
                    // their a1-component — the reachability predicate.
                    let m = q1 * q2.transpose();
                    let v = m * a2;
                    let reach = v.dot(a1) - a2.dot(a1);
                    if decide(
                        "mate_rotation_two_axis_reachable",
                        Margin::levered(reach, arm),
                        band,
                    )? != Sign::Zero
                    {
                        return Err(FoldStop::Clash {
                            predicate: "mate_rotation_two_axis_reachable",
                            margin: reach * arm,
                        });
                    }
                    let vp = v.reject_from(a1);
                    let tp = a2.reject_from(a1);
                    let alpha = f64::atan2(vp.cross(tp).dot(a1), vp.dot(tp));
                    Mat3::rotation_about(a1, alpha) * q1
                }
            }
        },
    )
}

/// **The free clocking about a shared axis, solved** (review MAJOR-1).
///
/// When both sides constrain rotation to one shared axis `u`, the
/// candidate may be turned by any angle α about the HELD side's own
/// axis and stay in the held coset — `rot(line(p₁,u), α)` is an
/// element of every subgroup this arm sees. So α is a free parameter,
/// and the added side's own axis-POSITION constraint is what fixes it.
///
/// The equation, in one line: the added side asks that its axis point
/// `p₂` come back to itself (up to sliding along `u`), so the candidate
/// must carry `w = R₁R₂⁻¹(p₂)` onto `p₂`. Turning about `(p₁,u)` moves
/// `w` on a circle, so the condition is a planar rotation taking
/// `(w−p₁)⊥` onto `(p₂−p₁)⊥` — one angle, closed form.
///
/// **Solvable exactly when the two radii agree**, which is the table's
/// own "inter-axis displacement invariants match A vs B" predicate.
/// This function does NOT decide that: `atan2` gives the direction
/// change regardless, the radius mismatch survives into the candidate,
/// and the membership check refuses it with the mismatch as the
/// measured clash — the table's "an unsatisfiable candidate is empty,
/// named", reached through the one gate rather than a second one.
///
/// Two arms need no angle: a side whose subgroup carries NO axis point
/// states no position constraint (planar), and a held side with
/// perpendicular translation freedom (planar again) can satisfy the
/// added position constraint by TRANSLATING in stage two. Both return
/// the identity, and `atan2(0, 0) = 0` makes the degenerate radii fall
/// out the same way with no branch.
fn clocking_about(held: Coset, added: Coset, u: Vec3<f64>) -> Mat3<f64> {
    let (Some(p1), Some(p2)) = (axis_point(held.subgroup), axis_point(added.subgroup)) else {
        return Mat3::identity();
    };
    let w = (held.representative * added.representative.inverse()).transform_point(p2);
    let from = (w - p1).reject_from(u);
    let to = (p2 - p1).reject_from(u);
    Mat3::rotation_about(u, f64::atan2(from.cross(to).dot(u), from.dot(to)))
}

/// The point a subgroup's axis is pinned through, `None` for the
/// point-free subgroups (whose membership states no position
/// constraint at all).
fn axis_point(g: Subgroup) -> Option<Point3<f64>> {
    match g {
        Subgroup::Cylindrical { point, .. } | Subgroup::Revolute { point, .. } => Some(point),
        _ => None,
    }
}

/// Stage two: the candidate's translation, with the rotation fixed.
///
/// Each side contributes an affine condition `P·(t − q) = 0`. Solving
/// the HELD side exactly and the added side in the remaining freedom
/// is what makes a failure's margin the added mate's own clash. The
/// assembled matrix is nonsingular by construction: it is the identity
/// on the held side's constrained directions, the added side's
/// projector on the rest, and the residual's own freedom fills the
/// kernel that would otherwise remain.
fn candidate_translation(
    held: Coset,
    added: Coset,
    residual: Subgroup,
    rotation: Mat3<f64>,
) -> Vec3<f64> {
    let a1 = rotation * held.representative.linear.transpose();
    let a2 = rotation * added.representative.linear.transpose();
    let (p1, q1) = held
        .subgroup
        .translation_constraint(a1, held.representative.translation);
    let (p2, q2) = added
        .subgroup
        .translation_constraint(a2, added.representative.translation);
    let free1 = sub(Mat3::identity(), p1);
    let k = add(add(free1 * p2 * free1, p1), residual.translation_freedom());
    let rhs = free1 * (p2 * (q2 - q1));
    q1 + inverse3(k) * rhs
}

// ---- Small fixed-order matrix arithmetic (D9: one written order) ----

fn zero3() -> Mat3<f64> {
    let z = Vec3::new(0.0, 0.0, 0.0);
    Mat3::from_cols(z, z, z)
}

fn outer(u: Vec3<f64>, v: Vec3<f64>) -> Mat3<f64> {
    Mat3::from_cols(u * v.x, u * v.y, u * v.z)
}

fn add(a: Mat3<f64>, b: Mat3<f64>) -> Mat3<f64> {
    Mat3::from_cols(a.c0 + b.c0, a.c1 + b.c1, a.c2 + b.c2)
}

fn sub(a: Mat3<f64>, b: Mat3<f64>) -> Mat3<f64> {
    Mat3::from_cols(a.c0 - b.c0, a.c1 - b.c1, a.c2 - b.c2)
}

/// The inverse of a 3×3 by adjugate over determinant, in one written
/// order. Only ever called on the assembled system above, whose
/// nonsingularity is structural; a singular input would yield
/// non-finite entries, which the membership check then refuses rather
/// than passing off as a pose.
fn inverse3(m: Mat3<f64>) -> Mat3<f64> {
    let r0 = m.c1.cross(m.c2);
    let r1 = m.c2.cross(m.c0);
    let r2 = m.c0.cross(m.c1);
    let det = m.c0.dot(r0);
    let s = 1.0 / det;
    // The adjugate's ROWS are these cross products, so the inverse's
    // columns are their components.
    Mat3::from_cols(
        Vec3::new(r0.x, r1.x, r2.x) * s,
        Vec3::new(r0.y, r1.y, r2.y) * s,
        Vec3::new(r0.z, r1.z, r2.z) * s,
    )
}
