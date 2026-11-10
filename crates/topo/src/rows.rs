//! **The row registry of topo's funnel bundles**: every bundle of
//! predicate rows a reader in this crate decides under, held here alone.
//!
//! As geom-brep's registry (`geom_brep::rows`) holds the bundles of its
//! doors, this one holds topo's: the ray walk's ([`ParityRows`],
//! [`SegmentRows`]), the pieced arc's ([`ArcRows`]), a loop boundary's
//! ([`BoundaryRows`]), the wedge reading's ([`WedgeNames`]), the point-in-
//! solid ray's ([`RayRows`]) and the section ruling chart's
//! ([`ChartRows`]). A bundle's rows key its reader's `k_stats` stream and
//! its ledger row, so two readers sharing a name would merge their
//! samples. Hence:
//!
//! - **The bundle types live here, their fields private to this
//!   module**, read through their accessors: no code outside this module
//!   builds one field by field.
//! - **Every reader's bundle is a `const` here**, named for its reader.
//! - **[`ROSTER`] lists every const's names, derived**: the consts are
//!   given to [`geom_core::row_registry!`], which builds the roster from
//!   exactly them and fails this crate's build on a name used twice
//!   ([`geom_core::rows::first_shared_in`]); the test-only consts are a
//!   second invocation with a roster of their own. editor-core's
//!   tree-wide test walks this roster with every other crate's.
//!
//! **Not here, and why.** A carrier of a name minted elsewhere is a
//! plain site in a wrapper, not a bundle: [`RoleNames::one`] (one caller's
//! name at both ends), `props::SignNames` (two `RoleNames`, each a const
//! of its reader's), and `geom_core::period::TurnRows::exact` /
//! `::bounded` called with a passed-in name. Their uniqueness is the
//! plain-site census's question
//! (`work/onecopy/plain-predicate-sites-have-no-tree-wide-census.md`).
//! A reader minting its own two role names (check 7's [`PLUS_V`]) or its
//! own conic rows ([`ConicRows`]) is a bundle here.

/// **A loop boundary's conic rows**: the turn row a conic arc's span is
/// read against a period under, and the straddle row an ellipse's span
/// escalation carries where its two readings straddle the band. Read as
/// a [`geom_core::period::TurnRows`] ([`Self::rows`]); minted only here.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ConicRows(geom_core::period::TurnRows<&'static str>);

impl ConicRows {
    /// The rows for the span's turn and its straddle. Private: a reader
    /// names its rows at a const below.
    const fn bounded(turn: &'static str, straddle: &'static str) -> Self {
        Self(geom_core::period::TurnRows::registered(turn, straddle))
    }

    /// The rows, as the period door reads them.
    #[must_use]
    pub(crate) const fn rows(self) -> geom_core::period::TurnRows<&'static str> {
        self.0
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 2] {
        [self.0.turn(), self.0.straddle()]
    }
}

/// **The names a role read decides under**: the bracket's high end (and
/// an exact volume) under `high`, its low end under `low`. Its fields are
/// private here. A reader with two names of its own (check 7's) is a
/// const below; [`Self::one`] is the door for one name at both ends,
/// which a caller's own name passes through (a plain site, not a bundle:
/// its uniqueness is the plain-site census's).
#[derive(Clone, Copy, Debug)]
pub struct RoleNames {
    /// The name the high end, and an exact volume, are decided under.
    high: &'static str,
    /// The name the low end is decided under.
    low: &'static str,
}

impl RoleNames {
    /// One name for both ends: every decision but check 7's, which keeps
    /// its two historical names ([`PLUS_V`]).
    #[must_use]
    pub const fn one(name: &'static str) -> Self {
        Self {
            high: name,
            low: name,
        }
    }

    /// The high end's name.
    #[must_use]
    pub const fn high(&self) -> &'static str {
        self.high
    }

    /// The low end's name.
    #[must_use]
    pub const fn low(&self) -> &'static str {
        self.low
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 2] {
        [self.high, self.low]
    }
}
use geom_brep::rows::SubdivisionRows;

/// The four K rows one consumer meters its walk through. Distinct
/// names per consumer are the point: see the module docs.
///
/// **These name literals are the greppable K roster entry for the
/// walk.** A name passed as a parameter is invisible to a grep for a
/// literal at the funnel site, so this type is named explicitly in
/// `K-REPORT.md`'s inventory method. It is one of several such
/// carriers, not the only one — the restated method there enumerates
/// the classes and their measured residue. A new `ParityRows` value is
/// a roster change and belongs in that document.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ParityRows {
    /// A segment's own length — the degeneracy gate. Zero ⇒ the
    /// segment is null scaffolding and the point distance is measured
    /// exactly (the foot division below would poison on it, `w·e/0`);
    /// a certified-short-but-nonzero segment is a genuine sliver and
    /// escalates like any in-band comparison.
    segment: &'static str,
    /// The distance from `q` to a closed segment — perpendicular at
    /// an interior foot, endpoint otherwise. Zero ⇒ on the boundary.
    boundary: &'static str,
    /// A vertex's signed offset from the ray line. Zero ⇒ grazing ⇒
    /// next ray.
    side: &'static str,
    /// A straddling segment's crossing advance along the ray. Zero
    /// would be a crossing at `q` itself, contradicting the boundary
    /// pre-pass ⇒ next ray.
    advance: &'static str,
}

impl ParityRows {
    /// The `side` row.
    #[must_use]
    pub(crate) const fn side(&self) -> &'static str {
        self.side
    }

    /// The `advance` row.
    #[must_use]
    pub(crate) const fn advance(&self) -> &'static str {
        self.advance
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 4] {
        [self.segment, self.boundary, self.side, self.advance]
    }

    /// The two rows of a segment's boundary reading.
    pub(crate) const fn segment_rows(&self) -> SegmentRows {
        SegmentRows {
            segment: self.segment,
            boundary: self.boundary,
        }
    }
}

/// The two of a [`ParityRows`] that [`on_segment`] reads, for a caller
/// that reads a segment's boundary and casts no ray through it, so
/// mints no `side` or `advance`. A new value is a roster change, as a
/// [`ParityRows`] value is.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SegmentRows {
    /// As [`ParityRows::segment`].
    segment: &'static str,
    /// As [`ParityRows::boundary`].
    boundary: &'static str,
}

impl SegmentRows {
    /// The `segment` row.
    #[must_use]
    pub(crate) const fn segment(&self) -> &'static str {
        self.segment
    }

    /// The `boundary` row.
    #[must_use]
    pub(crate) const fn boundary(&self) -> &'static str {
        self.boundary
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 2] {
        [self.segment, self.boundary]
    }
}

/// The rows a caller reads a pieced edge's boundary under — its own,
/// per carrier, so each population stays separable in the telemetry.
#[derive(Clone, Copy)]
pub(crate) struct ArcRows {
    /// The distance from the point to either end of the arc.
    end: &'static str,
    /// A piece's ball's clearance from the point.
    clear: &'static str,
    /// The distance from the point to a point of the arc in the ball.
    on: &'static str,
    /// A piece's ball's radius: within the band's coincidence threshold
    /// the piece is a point, and is not halved again.
    leaf: &'static str,
}

impl ArcRows {
    /// The `end` row.
    #[must_use]
    pub(crate) const fn end(&self) -> &'static str {
        self.end
    }

    /// The `clear` row.
    #[must_use]
    pub(crate) const fn clear(&self) -> &'static str {
        self.clear
    }

    /// The `on` row.
    #[must_use]
    pub(crate) const fn on(&self) -> &'static str {
        self.on
    }

    /// The `leaf` row.
    #[must_use]
    pub(crate) const fn leaf(&self) -> &'static str {
        self.leaf
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 4] {
        [self.end, self.clear, self.on, self.leaf]
    }
}

/// The predicate names one caller of [`wedge_holds_named`] decides its
/// four signs under, so each question's K samples stay its own.
#[derive(Clone, Copy, Debug)]
pub(crate) struct WedgeNames {
    /// Whether the direction leaves the wedge's first bound inward.
    after_leaving: &'static str,
    /// Whether it lies short of the second bound.
    before_arriving: &'static str,
    /// Whether the wedge turns under or over a half turn.
    turn: &'static str,
    /// Whether a wedge of no turn is straight or a cusp.
    straight: &'static str,
    /// Where a sign read at both bounds of its lever straddles the band
    /// ([`decide_agreed`]). Recorded only where a departure is
    /// [`Departure::Between`]; a caller that hands exact departures alone
    /// names a row that never records, and says so at its site.
    straddle: &'static str,
}

impl WedgeNames {
    /// The `after_leaving` row.
    #[must_use]
    pub(crate) const fn after_leaving(&self) -> &'static str {
        self.after_leaving
    }

    /// The `before_arriving` row.
    #[must_use]
    pub(crate) const fn before_arriving(&self) -> &'static str {
        self.before_arriving
    }

    /// The `turn` row.
    #[must_use]
    pub(crate) const fn turn(&self) -> &'static str {
        self.turn
    }

    /// The `straight` row.
    #[must_use]
    pub(crate) const fn straight(&self) -> &'static str {
        self.straight
    }

    /// The `straddle` row.
    #[must_use]
    pub(crate) const fn straddle(&self) -> &'static str {
        self.straddle
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 5] {
        [
            self.after_leaving,
            self.before_arriving,
            self.turn,
            self.straight,
            self.straddle,
        ]
    }
}

/// **The names a ray's planar crossings are decided under**: a schedule
/// ray's ([`SCHEDULE_ROWS`]) or a ray aimed at the boundary from beyond
/// it ([`crate::side_reader::AIMED_ROWS`]), so the reader of the side at infinity, which
/// every at-rest gate runs, logs apart from point containment.
#[derive(Clone, Copy)]
pub(crate) struct RayRows {
    advance: &'static str,
    denom: &'static str,
    clearance: &'static str,
    beside: &'static str,
    order: &'static str,
    vertex: &'static str,
    edge: &'static str,
    edge_span: &'static str,
    winding: &'static str,
    represents: &'static str,
    reaches: &'static str,
}

impl RayRows {
    /// The `advance` row.
    #[must_use]
    pub(crate) const fn advance(&self) -> &'static str {
        self.advance
    }

    /// The `denom` row.
    #[must_use]
    pub(crate) const fn denom(&self) -> &'static str {
        self.denom
    }

    /// The `clearance` row.
    #[must_use]
    pub(crate) const fn clearance(&self) -> &'static str {
        self.clearance
    }

    /// The `beside` row.
    #[must_use]
    pub(crate) const fn beside(&self) -> &'static str {
        self.beside
    }

    /// The `order` row.
    #[must_use]
    pub(crate) const fn order(&self) -> &'static str {
        self.order
    }

    /// The `vertex` row.
    #[must_use]
    pub(crate) const fn vertex(&self) -> &'static str {
        self.vertex
    }

    /// The `edge` row.
    #[must_use]
    pub(crate) const fn edge(&self) -> &'static str {
        self.edge
    }

    /// The `edge_span` row.
    #[must_use]
    pub(crate) const fn edge_span(&self) -> &'static str {
        self.edge_span
    }

    /// The `winding` row.
    #[must_use]
    pub(crate) const fn winding(&self) -> &'static str {
        self.winding
    }

    /// The `represents` row.
    #[must_use]
    pub(crate) const fn represents(&self) -> &'static str {
        self.represents
    }

    /// The `reaches` row.
    #[must_use]
    pub(crate) const fn reaches(&self) -> &'static str {
        self.reaches
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 11] {
        [
            self.advance,
            self.denom,
            self.clearance,
            self.beside,
            self.order,
            self.vertex,
            self.edge,
            self.edge_span,
            self.winding,
            self.represents,
            self.reaches,
        ]
    }
}

/// The rows a caller reads a loop's boundary under: a straight edge's
/// ([`ray_walk::on_segment`]'s two), a conic edge's, a spiric edge's and
/// a spline edge's.
#[derive(Clone, Copy)]
pub(crate) struct BoundaryRows {
    /// A straight edge's rows.
    line: &'static ParityRows,
    /// A conic edge's rows.
    conic: ConicRows,
    /// A spiric edge's rows.
    spiric: ArcRows,
    /// A spline edge's rows.
    spline: ArcRows,
}

impl BoundaryRows {
    /// The `line` rows, a bundle registered on its own.
    #[must_use]
    pub(crate) const fn line(&self) -> &'static ParityRows {
        self.line
    }

    /// The `conic` rows.
    #[must_use]
    pub(crate) const fn conic(&self) -> ConicRows {
        self.conic
    }

    /// The `spiric` rows.
    #[must_use]
    pub(crate) const fn spiric(&self) -> ArcRows {
        self.spiric
    }

    /// The `spline` rows.
    #[must_use]
    pub(crate) const fn spline(&self) -> ArcRows {
        self.spline
    }

    /// Every row the bundle names, for the roster: its conic, spiric and
    /// spline rows (its `line` rows are a bundle registered on their own).
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 10] {
        let (s, p) = (self.spiric.names(), self.spline.names());
        [
            self.conic.0.turn(),
            self.conic.0.straddle(),
            s[0],
            s[1],
            s[2],
            s[3],
            p[0],
            p[1],
            p[2],
            p[3],
        ]
    }
}

/// The predicate rows one chart is read under.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartRows {
    /// A fold: `E`'s roots, their order, and the arcs between them.
    fold: &'static str,
    /// A line parallel to the partner's asymptotic directions: `p₂`'s
    /// roots, and `p₁`'s sign at them.
    asymptote: &'static str,
    /// A refusal the representation, not the band, makes: the reading's
    /// resolution is not inside the band's zero, or the reading answers
    /// with its rounding uncharged (module docs).
    precision: &'static str,
    /// The subdivision's own rows.
    walk: SubdivisionRows,
}

impl ChartRows {
    /// The `fold` row.
    #[must_use]
    pub(crate) const fn fold(&self) -> &'static str {
        self.fold
    }

    /// The `asymptote` row.
    #[must_use]
    pub(crate) const fn asymptote(&self) -> &'static str {
        self.asymptote
    }

    /// The `precision` row.
    #[must_use]
    pub(crate) const fn precision(&self) -> &'static str {
        self.precision
    }

    /// The `walk` rows.
    #[must_use]
    pub(crate) const fn walk(&self) -> &SubdivisionRows {
        &self.walk
    }

    /// Every row the bundle names, for the roster (its `walk` is
    /// geom-brep's, registered there).
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 3] {
        [self.fold, self.asymptote, self.precision]
    }
}

geom_core::row_registry! {
    /// **Every registered bundle's rows**, one entry per const below,
    /// derived from the consts given here, so none can be left out of it.
    pub const ROSTER;

    // ------------------------------------------------- the ray walk

    /// The chart region's polygon walk (`chart_region`): chart-space
    /// margins, metered apart from the 3-D loop's, though the walk is one.
    pub(crate) const CHART_REGION: ParityRows = ParityRows {
        segment: "chart_region_segment",
        boundary: "chart_region_boundary",
        side: "chart_region_side",
        advance: "chart_region_advance",
    };

    /// The point-in-loop walk (`splitting::containment`).
    pub(crate) const POINT_IN_LOOP: ParityRows = ParityRows {
        segment: "point_in_loop_segment",
        boundary: "point_in_loop_boundary",
        side: "point_in_loop_side",
        advance: "point_in_loop_advance",
    };

    /// The point-in-arc-loop walk's line edges (`splitting::containment`).
    pub(crate) const ARC_LOOP: ParityRows = ParityRows {
        segment: "point_in_arc_loop_segment",
        boundary: "point_in_arc_loop_boundary",
        side: "point_in_arc_loop_side",
        advance: "point_in_arc_loop_advance",
    };

    /// The Boolean's contact pre-pass's line edges (`boolean::contain`): it
    /// reads `segment` and `boundary` alone, so `side` and `advance` are
    /// never minted.
    pub(crate) const CONTACT_EDGE: ParityRows = ParityRows {
        segment: "bool_contact_edge_length",
        boundary: "bool_contact_edge",
        side: "bool_contact_edge_side",
        advance: "bool_contact_edge_advance",
    };

    /// A ruled region's line edge (`boolean::ruled_region`).
    pub(crate) const RULED_REGION_LINE: SegmentRows = SegmentRows {
        segment: "bool_ruled_region_edge_length",
        boundary: "bool_ruled_region_edge",
    };

    // ------------------------------------------------- loop boundaries

    /// The point-in-arc-loop walk's boundary rows (`splitting::containment`).
    pub(crate) const ARC_LOOP_WALK: BoundaryRows = BoundaryRows {
        line: &ARC_LOOP,
        conic: ConicRows::bounded(
            "point_in_arc_loop_conic_span",
            "point_in_arc_loop_conic_straddle",
        ),
        spiric: ArcRows {
            end: "point_in_arc_loop_spiric_end",
            clear: "point_in_arc_loop_spiric_clear",
            on: "point_in_arc_loop_spiric_on",
            leaf: "point_in_arc_loop_spiric_leaf",
        },
        spline: ArcRows {
            end: "point_in_arc_loop_spline_end",
            clear: "point_in_arc_loop_spline_clear",
            on: "point_in_arc_loop_spline_on",
            leaf: "point_in_arc_loop_spline_leaf",
        },
    };

    /// The Boolean's contact pre-pass's boundary rows (`boolean::contain`).
    pub(crate) const CONTACT: BoundaryRows = BoundaryRows {
        line: &CONTACT_EDGE,
        conic: ConicRows::bounded("bool_contact_arc_span", "bool_contact_arc_straddle"),
        spiric: ArcRows {
            end: "bool_contact_spiric_end",
            clear: "bool_contact_spiric_clear",
            on: "bool_contact_spiric",
            leaf: "bool_contact_spiric_leaf",
        },
        spline: ArcRows {
            end: "bool_contact_spline_end",
            clear: "bool_contact_spline_clear",
            on: "bool_contact_spline",
            leaf: "bool_contact_spline_leaf",
        },
    };

    /// A torus region's oval edge (`boolean::torus_region`).
    pub(crate) const TORUS_REGION_SPIRIC: ArcRows = ArcRows {
        end: "bool_torus_region_spiric_end",
        clear: "bool_torus_region_spiric_clear",
        on: "bool_torus_region_spiric_on",
        leaf: "bool_torus_region_spiric_leaf",
    };

    /// A torus region's circle edge (`boolean::torus_region`).
    pub(crate) const TORUS_REGION_CONIC: ConicRows =
        ConicRows::bounded("bool_torus_region_arc_span", "bool_torus_region_arc_straddle");

    /// A sphere region's boundary reading (`boolean::sphere_region`).
    pub(crate) const SPHERE_REGION_CONIC: ConicRows =
        ConicRows::bounded("bool_sphere_region_arc_span", "bool_sphere_region_arc_straddle");

    /// A ruled region's conic edge (`boolean::ruled_region`).
    pub(crate) const RULED_REGION_CONIC: ConicRows =
        ConicRows::bounded("bool_ruled_region_arc_span", "bool_ruled_region_arc_straddle");

    /// The census's edge-face crossing, a conic arc's span read by
    /// `splitting::containment::carrier_loop` (`census`).
    pub(crate) const EF_CROSS_CONIC: ConicRows =
        ConicRows::bounded("pm_census_ef_cross_arc_span", "pm_census_ef_cross_arc_straddle");

    // ------------------------------------------------- wedges

    /// The split's cone-corner reading (`splitting::section_cones`).
    pub(crate) const CONE_CORNER: WedgeNames = WedgeNames {
        after_leaving: "split_cone_corner_after_leaving",
        before_arriving: "split_cone_corner_before_arriving",
        turn: "split_cone_corner_turn",
        straight: "split_cone_corner_straight",
        straddle: "split_cone_corner_straddle",
    };

    /// The census's overlap-corner reading (`census::corners_meet`).
    pub(crate) const OVERLAP_CORNER: WedgeNames = WedgeNames {
        after_leaving: "pm_census_overlap_corner_after_leaving",
        before_arriving: "pm_census_overlap_corner_before_arriving",
        turn: "pm_census_overlap_corner_turn",
        straight: "pm_census_overlap_corner_straight",
        straddle: "pm_census_overlap_corner_straddle",
    };

    /// The census's line-wedge reading (`census::strictly_inside`): tier 3's
    /// pinch-corner reading, asked of a line's wedges under names of its
    /// own, so the K stream keeps the two questions apart. Its `straddle`
    /// never records: `strictly_inside` hands only exact departures, so no
    /// sign is read at two bounds; it names the row only because every
    /// `WedgeNames` carries one.
    pub(crate) const LINE_WEDGE: WedgeNames = WedgeNames {
        after_leaving: "pm_census_wedge_after_leaving",
        before_arriving: "pm_census_wedge_before_arriving",
        turn: "pm_census_wedge_turn",
        straight: "pm_census_wedge_straight",
        straddle: "pm_census_wedge_straddle",
    };

    /// Tier 3's pinch-corner reading (`validate::wedge_holds`).
    pub(crate) const PINCH_CORNER: WedgeNames = WedgeNames {
        after_leaving: "pinch_corner_after_leaving",
        before_arriving: "pinch_corner_before_arriving",
        turn: "pinch_corner_turn",
        straight: "pinch_corner_straight",
        straddle: "pinch_corner_straddle",
    };

    // ------------------------------------------------- rays through a solid

    /// A schedule ray through a solid (`boolean::solid_contain`).
    pub(crate) const SCHEDULE_RAY: RayRows = RayRows {
        advance: "bool_point_in_solid_advance",
        denom: "bool_point_in_solid_denom",
        clearance: "bool_point_in_solid_clearance",
        beside: "bool_point_in_solid_beside",
        order: "bool_point_in_solid_order",
        vertex: "bool_point_in_solid_vertex",
        edge: "bool_point_in_solid_edge",
        edge_span: "bool_point_in_solid_edge_span",
        winding: "bool_point_in_solid_winding",
        represents: "bool_point_in_solid_met_edge_represents",
        reaches: "bool_point_in_solid_met_edge_reaches",
    };

    /// An aimed ray to infinity (`side_reader::side_at_infinity`).
    pub(crate) const AIMED_RAY: RayRows = RayRows {
        advance: "side_at_infinity_advance",
        denom: "side_at_infinity_denom",
        clearance: "side_at_infinity_clearance",
        beside: "side_at_infinity_beside",
        order: "side_at_infinity_order",
        vertex: "side_at_infinity_vertex",
        edge: "side_at_infinity_edge",
        edge_span: "side_at_infinity_edge_span",
        winding: "side_at_infinity_winding",
        represents: "side_at_infinity_met_edge_represents",
        reaches: "side_at_infinity_met_edge_reaches",
    };

    // ------------------------------------------------- section ruling charts

    /// The cone × cylinder ruling chart (`boolean::section_cert`).
    pub(crate) const SECTION_CONE_CYLINDER: ChartRows = ChartRows {
        fold: "section_cone_cylinder_fold",
        asymptote: "section_cone_cylinder_asymptote",
        precision: "section_cone_cylinder_precision",
        walk: geom_brep::rows::SECTION_RULING_WALK,
    };

    /// The cylinder × cone ruling chart (`boolean::section_cert`).
    pub(crate) const SECTION_CYLINDER_CONE: ChartRows = ChartRows {
        fold: "section_cylinder_cone_fold",
        asymptote: "section_cylinder_cone_asymptote",
        precision: "section_cylinder_cone_precision",
        walk: geom_brep::rows::SECTION_RULING_WALK,
    };

    /// The cone pair's ruling chart (`boolean::section_cert`).
    pub(crate) const SECTION_CONE_PAIR: ChartRows = ChartRows {
        fold: "section_cone_pair_fold",
        asymptote: "section_cone_pair_asymptote",
        precision: "section_cone_pair_precision",
        walk: geom_brep::rows::SECTION_RULING_WALK,
    };

    // ------------------------------------------------- role reads

    /// Check 7's names (`validate`): the bracket's high end under
    /// `positive_volume`, the refusal this check has always made, and its
    /// low end under `positive_volume_enclosure`.
    pub const PLUS_V: RoleNames = RoleNames {
        high: "positive_volume",
        low: "positive_volume_enclosure",
    };
}

#[cfg(test)]
geom_core::row_registry! {
    /// The test-only bundles' rows, checked with the shipped roster in a
    /// test build: a test reader's names are distinct from every shipped
    /// reader's too.
    const TEST_ROSTER;

    // ------------------------------------------------- test-only readers

    /// The ray walk's own rows (`ray_walk`'s tests).
    pub(crate) const TEST_RAY_WALK: ParityRows = ParityRows {
        segment: "test_ray_walk_segment",
        boundary: "test_ray_walk_boundary",
        side: "test_ray_walk_side",
        advance: "test_ray_walk_advance",
    };

    /// The spline arc's own rows (`splitting::spline_arc`'s tests).
    pub(crate) const TEST_SPLINE_ARC: ArcRows = ArcRows {
        end: "test_spline_end",
        clear: "test_spline_clear",
        on: "test_spline_on",
        leaf: "test_spline_leaf",
    };

    /// The spiric arc's own rows (`splitting::spiric_arc`'s tests).
    pub(crate) const TEST_SPIRIC_ARC: ArcRows = ArcRows {
        end: "test_spiric_end",
        clear: "test_spiric_clear",
        on: "test_spiric_on",
        leaf: "test_spiric_leaf",
    };

    /// The spline arc fuzz's rows (`splitting::spline_arc::fuzz`).
    pub(crate) const FUZZ_SPLINE_ARC: ArcRows = ArcRows {
        end: "fuzz_spline_end",
        clear: "fuzz_spline_clear",
        on: "fuzz_spline_on",
        leaf: "fuzz_spline_leaf",
    };

    /// The spiric arc fuzz's rows (`splitting::spiric_arc::fuzz`).
    pub(crate) const FUZZ_SPIRIC_ARC: ArcRows = ArcRows {
        end: "fuzz_spiric_end",
        clear: "fuzz_spiric_clear",
        on: "fuzz_spiric_on",
        leaf: "fuzz_spiric_leaf",
    };

    /// The ruling chart's own rows (`boolean::section_cert::ruling`'s tests).
    pub(crate) const TEST_RULING: ChartRows = ChartRows {
        fold: "test_ruling_fold",
        asymptote: "test_ruling_asymptote",
        precision: "test_ruling_precision",
        walk: geom_brep::rows::SECTION_RULING_WALK,
    };
}

#[cfg(test)]
mod roster_rows {
    use super::{ROSTER, TEST_ROSTER};

    /// **The shipped and test-only rosters together name every row once.**
    #[test]
    fn every_registered_row_is_named_once() {
        let all: Vec<&[&str]> = ROSTER.iter().chain(TEST_ROSTER).copied().collect();
        assert_eq!(geom_core::rows::first_shared_in(&all), None);
    }
}

#[cfg(test)]
mod registry_source {
    /// **No bundle const of this registry lies outside its
    /// `row_registry!` blocks**, where it would escape the derived roster
    /// ([`geom_core::rows::consts_outside_registry`]).
    #[test]
    fn every_bundle_const_is_in_the_registry_block() {
        let stray = geom_core::rows::consts_outside_registry(
            include_str!("rows.rs"),
            &[
                "ParityRows",
                "SegmentRows",
                "ArcRows",
                "BoundaryRows",
                "WedgeNames",
                "RayRows",
                "ChartRows",
                "ConicRows",
                "RoleNames",
            ],
        );
        assert!(stray.is_empty(), "consts outside the registry: {stray:#?}");
    }
}
