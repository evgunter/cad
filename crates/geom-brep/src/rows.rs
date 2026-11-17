//! **The row registry of geom-brep's funnel bundles**: every bundle of
//! predicate rows a reader of this crate's root ladders decides under,
//! held here alone.
//!
//! A bundle is a set of predicate names one reader of a shared door
//! (the half-angle ladder, the quartic, the certified subdivision, the
//! first-harmonic roots, a pair section's gap) meters its decisions
//! under. Its rows key the reader's `k_stats` stream and its ledger row,
//! so two readers sharing a name would merge their samples. Hence:
//!
//! - **The bundle types live here, their fields private to this
//!   module**, read through their accessors. No code outside this module
//!   can build one field by field, this crate's and its callers' alike.
//! - **Every reader's bundle is a `const` here**, named for its reader:
//!   this crate's lanes, and the readers in `topo` that call these doors
//!   (the conic × quadric ladder, the ray × torus quartic, the section
//!   ruling walk). They are named here as strings only: those names are
//!   global to the `k_stats` stream and the ledger already.
//! - **[`ROSTER`] lists every const's names, derived**: the consts are
//!   given to [`geom_core::row_registry!`], which builds the roster from
//!   exactly them and adds the `const` assertion that fails this crate's
//!   build on a name used twice ([`geom_core::rows::first_shared_in`]).
//!   editor-core's tree-wide test walks this roster with every other
//!   crate's.
//!
//! A bundle with one reader (`TouchRows`, `FirstHarmonicRows`) has no
//! enum of readers: its one const here is its registration.
//!
//! A carrier of one row passed through from a name minted elsewhere
//! (`half_angle::RootSlack`'s `row`) is a plain site in a wrapper, not a
//! bundle, and has no registry here.

/// The predicate names one caller of [`crate::quartic::depressed_quartic_roots`] meters
/// its ladder under, so each lane's rows stay its own in the verdict
/// log.
#[derive(Clone, Copy, Debug)]
pub struct QuarticRows {
    /// The discriminant `Δ`'s sign.
    disc: &'static str,
    /// `p`'s sign on the positive branch.
    shape: &'static str,
    /// `D = 64s − 16p²`'s sign on the positive branch.
    depth: &'static str,
    /// Whether `q̂` is in band (the biquadratic arm).
    odd: &'static str,
    /// A quadratic factor's discriminant.
    split: &'static str,
    /// The resolvent's largest root.
    split_lead: &'static str,
}

impl QuarticRows {
    /// The `disc` row.
    #[must_use]
    pub const fn disc(&self) -> &'static str {
        self.disc
    }

    /// The `shape` row.
    #[must_use]
    pub const fn shape(&self) -> &'static str {
        self.shape
    }

    /// The `depth` row.
    #[must_use]
    pub const fn depth(&self) -> &'static str {
        self.depth
    }

    /// The `odd` row.
    #[must_use]
    pub const fn odd(&self) -> &'static str {
        self.odd
    }

    /// The `split` row.
    #[must_use]
    pub const fn split(&self) -> &'static str {
        self.split
    }

    /// The `split_lead` row.
    #[must_use]
    pub const fn split_lead(&self) -> &'static str {
        self.split_lead
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 6] {
        [
            self.disc,
            self.shape,
            self.depth,
            self.odd,
            self.split,
            self.split_lead,
        ]
    }
}

/// The predicate rows one caller of [`crate::half_angle::certified_subdivision`] meters
/// under, every one a residual in metres.
#[derive(Clone, Copy, Debug)]
pub struct SubdivisionRows {
    /// A piece is definitely root-free: `|F(m)|` exceeds the most `F` can
    /// fall over the half-width.
    clear: &'static str,
    /// A piece is definitely monotone: `|F′(m)|` exceeds the most `F′`
    /// can fall over the half-width, read as the rise it certifies.
    monotone: &'static str,
    /// The residual's sign at a monotone piece's ends, and ON the surface
    /// at a located root.
    side: &'static str,
    /// A piece is still wider, as arc length, than the zero band: below
    /// it a piece that is neither clear nor monotone holds a double root
    /// the band cannot resolve.
    width: &'static str,
}

impl SubdivisionRows {
    /// The `clear` row.
    #[must_use]
    pub const fn clear(&self) -> &'static str {
        self.clear
    }

    /// The `monotone` row.
    #[must_use]
    pub const fn monotone(&self) -> &'static str {
        self.monotone
    }

    /// The `side` row.
    #[must_use]
    pub const fn side(&self) -> &'static str {
        self.side
    }

    /// The `width` row.
    #[must_use]
    pub const fn width(&self) -> &'static str {
        self.width
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 4] {
        [self.clear, self.monotone, self.side, self.width]
    }
}

/// **The rows a caller of [`crate::half_angle::certified_subdivision_touching`] places a
/// touch under**: where a piece neither clear nor monotone is convex
/// toward its ends' common side, the walk reads its extremum as a
/// touch ([`crate::half_angle::certified_subdivision_touching`]).
#[derive(Clone, Copy, Debug)]
pub struct TouchRows {
    /// The least of `F″` toward the ends' side over the piece is
    /// positive, read exactly (a sign, not a length), and so are the
    /// slopes toward the extremum at the piece's ends.
    convex: &'static str,
    /// The most residual, in metres, over the arc of the piece where `F`
    /// lies on the far side of zero from its ends: zero is one touch.
    /// Where `F` does not reach zero on the piece (no root: `G(θ*)` is
    /// past zero and the fall's first term clamps at zero), the arc is
    /// empty and the fall is the slope's share alone; the touch is then
    /// the extremum's near-miss, whose own residual reads in the zero
    /// band (`side`).
    stretch: &'static str,
}

impl TouchRows {
    /// The `convex` row.
    #[must_use]
    pub const fn convex(&self) -> &'static str {
        self.convex
    }

    /// The `stretch` row.
    #[must_use]
    pub const fn stretch(&self) -> &'static str {
        self.stretch
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 2] {
        [self.convex, self.stretch]
    }
}

/// The predicate rows one caller of [`crate::half_angle::half_angle_roots`] meters under.
#[derive(Clone, Copy, Debug)]
pub struct HalfAngleRows {
    /// The pole is definitely off the surface (a residual, metres).
    pole: &'static str,
    /// The pole is well conditioned (module docs; metres).
    conditioning: &'static str,
    /// The harmonics' rounding, as a residual, is past the band, and the
    /// ladder does not run (the noise meter; metres).
    noise: &'static str,
    /// The quartic ladder's own rows.
    quartic: QuarticRows,
    /// The certified subdivision's rows ([`crate::half_angle::certified_subdivision`]).
    verify: SubdivisionRows,
}

impl HalfAngleRows {
    /// The `pole` row.
    #[must_use]
    pub const fn pole(&self) -> &'static str {
        self.pole
    }

    /// The `conditioning` row.
    #[must_use]
    pub const fn conditioning(&self) -> &'static str {
        self.conditioning
    }

    /// The `noise` row.
    #[must_use]
    pub const fn noise(&self) -> &'static str {
        self.noise
    }

    /// The `quartic` rows.
    #[must_use]
    pub const fn quartic(&self) -> &QuarticRows {
        &self.quartic
    }

    /// The `verify` rows.
    #[must_use]
    pub const fn verify(&self) -> &SubdivisionRows {
        &self.verify
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 13] {
        let q = self.quartic.names();
        let v = self.verify.names();
        [
            self.pole,
            self.conditioning,
            self.noise,
            q[0],
            q[1],
            q[2],
            q[3],
            q[4],
            q[5],
            v[0],
            v[1],
            v[2],
            v[3],
        ]
    }
}

/// The predicate rows one caller of [`crate::curve_face::circle_roots::first_harmonic_roots`] meters
/// under (module docs: noise, coaxial, extreme, root slack; all metres).
#[derive(Clone, Copy, Debug)]
pub struct FirstHarmonicRows {
    /// The noise meter's row.
    noise: &'static str,
    /// The coaxial carrier's constant residual's row.
    coaxial: &'static str,
    /// The extremes' row.
    extreme: &'static str,
    /// The root slack's row.
    root_slack: &'static str,
}

impl FirstHarmonicRows {
    /// The `noise` row.
    #[must_use]
    pub const fn noise(&self) -> &'static str {
        self.noise
    }

    /// The `coaxial` row.
    #[must_use]
    pub const fn coaxial(&self) -> &'static str {
        self.coaxial
    }

    /// The `extreme` row.
    #[must_use]
    pub const fn extreme(&self) -> &'static str {
        self.extreme
    }

    /// The `root_slack` row.
    #[must_use]
    pub const fn root_slack(&self) -> &'static str {
        self.root_slack
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 4] {
        [self.noise, self.coaxial, self.extreme, self.root_slack]
    }
}

/// The rows a sweep decides on, each a pair's own name.
#[derive(Clone, Copy, Debug)]
pub(crate) struct GapRows {
    /// The gap itself: a Zero verdict, and the floor's escalation.
    gap: &'static str,
    /// A cell's branch stands past the band from both levels.
    clear: &'static str,
    /// A cell's branch is monotone.
    monotone: &'static str,
    /// A cell's branch is smooth: off the kinks.
    smooth: &'static str,
    /// The sign of `D′` at a cell's or a window's end.
    turn: &'static str,
    /// A cell at the floor.
    floor: &'static str,
    /// The floor's two bounds on the gap straddling the band.
    straddle: &'static str,
}

impl GapRows {
    /// The `gap` row.
    #[must_use]
    pub(crate) const fn gap(&self) -> &'static str {
        self.gap
    }

    /// The `clear` row.
    #[must_use]
    pub(crate) const fn clear(&self) -> &'static str {
        self.clear
    }

    /// The `monotone` row.
    #[must_use]
    pub(crate) const fn monotone(&self) -> &'static str {
        self.monotone
    }

    /// The `smooth` row.
    #[must_use]
    pub(crate) const fn smooth(&self) -> &'static str {
        self.smooth
    }

    /// The `turn` row.
    #[must_use]
    pub(crate) const fn turn(&self) -> &'static str {
        self.turn
    }

    /// The `floor` row.
    #[must_use]
    pub(crate) const fn floor(&self) -> &'static str {
        self.floor
    }

    /// The `straddle` row.
    #[must_use]
    pub(crate) const fn straddle(&self) -> &'static str {
        self.straddle
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub(crate) const fn names(&self) -> [&'static str; 7] {
        [
            self.gap,
            self.clear,
            self.monotone,
            self.smooth,
            self.turn,
            self.floor,
            self.straddle,
        ]
    }
}

/// **A reader's period-door rows**: the turn row a span is read against a
/// period under, and the straddle row a bounded channel's two readings
/// name where they straddle the band. Read as a
/// [`geom_core::period::TurnRows`] ([`Self::rows`]); minted only here.
#[derive(Clone, Copy, Debug)]
pub struct PeriodRows(geom_core::period::TurnRows<&'static str>);

impl PeriodRows {
    /// The rows for the span's turn and its straddle. Private: a reader
    /// names its rows at a const below.
    const fn bounded(turn: &'static str, straddle: &'static str) -> Self {
        Self(geom_core::period::TurnRows::registered(turn, straddle))
    }

    /// The rows, as the period door reads them.
    #[must_use]
    pub const fn rows(self) -> geom_core::period::TurnRows<&'static str> {
        self.0
    }

    /// Every row the bundle names, for the roster.
    #[must_use]
    pub const fn names(&self) -> [&'static str; 2] {
        [self.0.turn(), self.0.straddle()]
    }
}

geom_core::row_registry! {
    /// **Every registered bundle's rows**, one entry per const below, derived
    /// from the consts given here, so none can be left out of it.
    pub const ROSTER;

    // ------------------------------------------------- this crate's readers

    /// The circle × torus lane's ladder rows (`curve_face::circle_torus`).
    pub const CIRCLE_TORUS: HalfAngleRows = HalfAngleRows {
        pole: "circle_torus_pole",
        conditioning: "circle_torus_pole_conditioning",
        noise: "circle_torus_noise",
        quartic: QuarticRows {
            disc: "circle_torus_disc",
            shape: "circle_torus_shape",
            depth: "circle_torus_depth",
            odd: "circle_torus_odd",
            split: "circle_torus_split",
            split_lead: "circle_torus_split_lead",
        },
        verify: SubdivisionRows {
            clear: "circle_torus_sub_clear",
            monotone: "circle_torus_sub_monotone",
            side: "circle_torus_sub_side",
            width: "circle_torus_sub_width",
        },
    };

    /// The ellipse × torus arm's subdivision rows (`curve_face::ellipse_torus`).
    pub const ELLIPSE_TORUS: SubdivisionRows = SubdivisionRows {
        clear: "ellipse_torus_sub_clear",
        monotone: "ellipse_torus_sub_monotone",
        side: "ellipse_torus_sub_side",
        width: "ellipse_torus_sub_width",
    };

    /// The coplanar conic pair's subdivision rows (`curve_pair::pose`).
    pub const COPLANAR: SubdivisionRows = SubdivisionRows {
        clear: "curve_pair_coplanar_sub_clear",
        monotone: "curve_pair_coplanar_sub_monotone",
        side: "curve_pair_coplanar_sub_side",
        width: "curve_pair_coplanar_sub_width",
    };

    /// The coplanar subdivision's touch rows (`curve_pair::pose`, module docs,
    /// "A touch in the subdivision").
    pub const COPLANAR_TOUCH: TouchRows = TouchRows {
        convex: "curve_pair_coplanar_sub_convex",
        stretch: "curve_pair_coplanar_sub_touch",
    };

    /// The cylinder × torus gap's rows (`pair_section::torus`).
    pub(crate) const CT_GAP: GapRows = GapRows {
        gap: "ct_critical_gap",
        clear: "ct_critical_gap_clear",
        monotone: "ct_critical_gap_monotone",
        smooth: "ct_critical_gap_smooth",
        turn: "ct_critical_gap_turn",
        floor: "ct_critical_gap_floor",
        straddle: "ct_critical_gap_straddle",
    };

    /// The torus × torus gap's rows (`pair_section::torus`).
    pub(crate) const TT_GAP: GapRows = GapRows {
        gap: "tt_critical_gap",
        clear: "tt_critical_gap_clear",
        monotone: "tt_critical_gap_monotone",
        smooth: "tt_critical_gap_smooth",
        turn: "tt_critical_gap_turn",
        floor: "tt_critical_gap_floor",
        straddle: "tt_critical_gap_straddle",
    };

    /// Check 2's interval-span winding (`certify`): a periodic span's
    /// turn, and the straddle of a bounded channel's two readings.
    pub const INTERVAL_SPAN_WINDING: PeriodRows =
        PeriodRows::bounded("interval_span_winding", "interval_span_winding_straddle");

    /// The pcurve's azimuth period gate (`pcurve_cache::projected`).
    pub const PCURVE_AZIMUTH_PERIOD: PeriodRows =
        PeriodRows::bounded("pcurve_azimuth_period", "pcurve_azimuth_period_straddle");

    /// A cone face's ellipse rim's span winding (`props::curved`).
    pub const PROPS_CONE_ELLIPSE_SPAN_WINDING: PeriodRows = PeriodRows::bounded(
        "props_cone_ellipse_span_winding",
        "props_cone_ellipse_span_winding_straddle",
    );

    // ------------------------------------------------- topo's readers

    /// `topo`'s conic × quadric first-harmonic arm's rows
    /// (`topo::boolean::conic_quadric`).
    pub const CONIC_QUADRIC_FIRST: FirstHarmonicRows = FirstHarmonicRows {
        noise: "bool_conic_quadric_first_noise",
        coaxial: "bool_conic_quadric_first_constant",
        extreme: "bool_conic_quadric_first_extreme",
        root_slack: "bool_conic_quadric_first_root_slack",
    };

    /// `topo`'s conic × quadric ladder's rows
    /// (`topo::boolean::conic_quadric`); its noise row is also the arm
    /// switch's ladder meter there.
    pub const CONIC_QUADRIC_LADDER: HalfAngleRows = HalfAngleRows {
        pole: "bool_conic_quadric_pole",
        conditioning: "bool_conic_quadric_pole_conditioning",
        noise: "bool_conic_quadric_ladder_noise",
        quartic: QuarticRows {
            disc: "bool_conic_quadric_disc",
            shape: "bool_conic_quadric_shape",
            depth: "bool_conic_quadric_depth",
            odd: "bool_conic_quadric_odd",
            split: "bool_conic_quadric_split",
            split_lead: "bool_conic_quadric_split_lead",
        },
        verify: SubdivisionRows {
            clear: "bool_conic_quadric_sub_clear",
            monotone: "bool_conic_quadric_sub_monotone",
            side: "bool_conic_quadric_sub_side",
            width: "bool_conic_quadric_sub_width",
        },
    };

    /// `topo`'s ray × torus quartic rows (`topo::solid_contain`).
    pub const RAY_TORUS: QuarticRows = QuarticRows {
        disc: "bool_ray_torus_disc",
        shape: "bool_ray_torus_shape",
        depth: "bool_ray_torus_depth",
        odd: "bool_ray_torus_odd",
        split: "bool_ray_torus_split",
        split_lead: "bool_ray_torus_split_lead",
    };

    /// The subdivision rows every `topo` section ruling chart walks under
    /// (`topo::boolean::section_cert`).
    pub const SECTION_RULING_WALK: SubdivisionRows = SubdivisionRows {
        clear: "section_ruling_clear",
        monotone: "section_ruling_monotone",
        side: "section_ruling_side",
        width: "section_ruling_width",
    };
}
