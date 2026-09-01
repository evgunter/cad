//! The palette's own invariants — the rows that hold whatever colours
//! a theme happens to state.
//!
//! Every row here runs with **no `app` feature**: `viewer::theme` names
//! no toolkit, so the palette is asserted on in ordinary headless CI
//! with neither `egui` nor `wgpu` compiled. That is the whole reason
//! the module sits outside `app`.
//!
//! The colourblind check lives at the bottom, in [`cvd`]. It runs
//! only over themes that CLAIM [`Safety::ColorblindSafe`] — a palette
//! that makes no claim is not lesser and is not measured — and it
//! measures the **composited** colour a mark produces over the body
//! under shading, never the raw tint, because the raw tint is not
//! what any eye receives.

use viewer::theme::{Mark, Safety, Theme, from_linear, linear};

/// A theme's name is how `--theme` and a preferences file will reach
/// it, so two themes sharing one is a theme nobody can select.
#[test]
fn registered_names_are_unique() {
    let mut names: Vec<&str> = Theme::ALL.iter().map(|theme| theme.name).collect();
    names.sort_unstable();
    let before = names.len();
    names.dedup();
    assert_eq!(names.len(), before, "two registered themes share a name");
}

/// The registry and the lookup are one set: every theme in `ALL` is
/// reachable by its own name, and nothing else is.
#[test]
fn by_name_round_trips_every_registered_theme() {
    for theme in Theme::ALL {
        assert_eq!(
            Theme::by_name(theme.name),
            Some(*theme),
            "{} is registered but not reachable by name",
            theme.name,
        );
    }
    assert_eq!(Theme::by_name("no-such-theme"), None);
    // Not a fallback: an unrecognised `--theme` is a typo, and
    // opening the default in silence would hide it.
    assert_eq!(Theme::by_name(""), None);
}

/// The default has to be one of the themes the registry checks.
#[test]
fn default_is_registered() {
    assert!(
        Theme::ALL.contains(&Theme::DEFAULT),
        "the default theme is not in the registry, so nothing here checks it",
    );
}

/// Strengths and the ambient term are mix fractions. Checked once
/// over the registry rather than at each use: `Mark::strength` is
/// documented as `[0, 1]`, and a value outside it reaches the shader
/// as a `mix` that overshoots — a colour brighter than either input,
/// with nothing to report it.
#[test]
fn mix_fractions_are_in_range() {
    for theme in Theme::ALL {
        assert!(
            (0.0..=1.0).contains(&theme.ambient),
            "{}: ambient {} outside [0, 1]",
            theme.name,
            theme.ambient,
        );
        for (which, mark) in theme.marks() {
            assert!(
                (0.0..=1.0).contains(&mark.strength),
                "{}: {which} strength {} outside [0, 1]",
                theme.name,
                mark.strength,
            );
        }
    }
}

/// sRGB → linear → sRGB is the identity on all 256 codes.
///
/// The palette and the document both state colour in 8-bit sRGB and
/// the shader shades in linear, so this pair is the one crossing
/// every colour makes. A round trip that lost a code would move a
/// theme's colours by a step each time one was composited — and
/// [`Mark::over`] composites in linear and answers in sRGB, so the
/// loss would compound.
#[test]
fn srgb_linear_round_trip_is_exact() {
    for code in 0..=u8::MAX {
        let color = editor_core::appearance::Rgba8::opaque(code, code, code);
        assert_eq!(
            from_linear(linear(color)),
            color,
            "code {code} did not survive the round trip",
        );
    }
}

/// The mix endpoints are the two colours it mixes.
///
/// Strength 0 leaves the body untouched and strength 1 replaces it —
/// which is what the doc comment on [`Mark::strength`] promises, and
/// what makes the composited colour the honest subject of a
/// legibility check rather than the raw tint.
#[test]
fn a_mark_at_its_endpoints_is_body_or_tint() {
    for theme in Theme::ALL {
        for (which, mark) in theme.marks() {
            let none = Mark {
                tint: mark.tint,
                strength: 0.0,
            };
            assert_eq!(
                none.over(theme.body),
                theme.body,
                "{}: {which} at strength 0 moved the body colour",
                theme.name,
            );
            let full = Mark {
                tint: mark.tint,
                strength: 1.0,
            };
            assert_eq!(
                full.over(theme.body),
                mark.tint,
                "{}: {which} at strength 1 did not reach its tint",
                theme.name,
            );
        }
    }
}

/// A mark actually moves the body colour it is mixed over.
///
/// The weakest registered mark is the focus tint at 0.24, and a
/// palette whose marks composited to the body colour would draw a
/// highlight nobody can see — the failure this row exists to catch is
/// a theme edited to a tint too close to its own body.
#[test]
fn every_mark_is_visible_against_its_body() {
    for theme in Theme::ALL {
        for (which, mark) in theme.marks() {
            assert_ne!(
                mark.over(theme.body),
                theme.body,
                "{}: {which} composites to the body colour and marks nothing",
                theme.name,
            );
        }
    }
}

/// **A theme's ground stays off its own swatches.**
///
/// The ground is what the viewport is filled with where no geometry
/// is drawn, so every silhouette in the picture is a swatch meeting
/// it. A ground that landed on one of the theme's own colours would
/// erase exactly that outline — the shading-independent half of the
/// legibility question the marks check answers, and the one a
/// toolkit's default background used to decide behind the palette's
/// back.
///
/// Measured under the vision types each theme's own claim covers:
/// every one for a [`Safety::ColorblindSafe`] palette, normal vision
/// for a palette that claims nothing. Same bar, same metric — a
/// ground is not a different kind of colour.
#[test]
fn a_grounds_swatches_stay_off_it() {
    for theme in Theme::ALL {
        let (worst, at) = cvd::worst_against_ground(theme);
        assert!(
            worst >= cvd::MIN_SEPARATION,
            "{}: the ground is only {worst:.4} from {at}, under the {:.4} bar — a \
             silhouette there is invisible",
            theme.name,
            cvd::MIN_SEPARATION,
        );
    }
}

/// Every theme that claims colourblind safety is actually checked.
///
/// The registry and the claim are the only inputs, so a theme added
/// with the claim is measured without anyone remembering to add it
/// here — and a theme that drops the claim stops being measured, by
/// its own statement rather than by an edit to this file.
#[test]
fn a_claimed_theme_keeps_its_marks_apart_under_dichromacy() {
    let claimed: Vec<&Theme> = Theme::ALL
        .iter()
        .filter(|theme| theme.safety == Safety::ColorblindSafe)
        .collect();
    assert!(
        !claimed.is_empty(),
        "no theme claims colourblind safety, so this check measures nothing — \
         if the claim was deliberately dropped, drop this row with it",
    );
    for theme in claimed {
        let (worst, at) = cvd::worst_separation(theme);
        assert!(
            worst >= cvd::MIN_SEPARATION,
            "{}: {at} separated by only {worst:.4} in OKLab, under the {:.4} this \
             theme's own claim requires",
            theme.name,
            cvd::MIN_SEPARATION,
        );
    }
}

/// A theme that makes no claim is not measured — including today's
/// default, which would fail if it were.
///
/// This is the row that keeps the claim meaningful in both
/// directions. Were the check silently applied to everything, the
/// neutral palettes would have to be redesigned to satisfy a promise
/// they never made; were it applied to nothing, the claim would be
/// decoration. The number here is also the honest measure of what
/// claiming costs: the default's marks are far closer together than
/// a claimed theme may be.
#[test]
fn an_unclaimed_theme_is_not_held_to_the_bar() {
    let default = Theme::DEFAULT;
    assert_eq!(default.safety, Safety::Unchecked);
    let (worst, _) = cvd::worst_separation(&default);
    assert!(
        worst < cvd::MIN_SEPARATION,
        "the default theme now meets the bar ({worst:.4}) — if that is deliberate, \
         have it claim Safety::ColorblindSafe rather than meeting the bar in silence",
    );
}

/// The simulation, and the rows that check the oracle before the
/// oracle is used to check anything else.
///
/// `perceive-cvd` is a 0.1.0 with one release. It is a
/// dev-dependency, so it ships in nothing — but a palette measured
/// through a broken model would pass this suite while failing on
/// somebody's screen, which is the failure the rows below exist to
/// make impossible. They assert PROPERTIES the Brettel/Viénot
/// construction must have rather than pasted reference numbers: a
/// property that holds is checkable here, where a number copied out
/// of a paper is only a second thing to get wrong.
mod cvd {
    use perceive_color::Color;
    use perceive_cvd::{CvdType, Severity, simulate};
    use viewer::theme::{Safety, Theme, linear};

    /// How far apart two swatches must stay, in OKLab.
    ///
    /// An engineering threshold, not a standard: OKLab's lightness
    /// runs 0…1 and a just-noticeable difference on a large field is
    /// around 0.02, so this is roughly three JNDs — comfortably
    /// visible, and low enough that a palette has somewhere to live.
    /// The claimed theme clears it by about a quarter, which is the
    /// headroom that stops an incidental edit from flipping CI.
    pub(super) const MIN_SEPARATION: f64 = 0.06;

    /// Every vision type the claim covers, at full severity — the
    /// worst case, and the only severity a claim can honestly be
    /// about.
    ///
    /// **`CvdType::Achromat` is deliberately absent, and this is the
    /// scope of the claim rather than an oversight.** Total colour
    /// blindness leaves lightness and nothing else, so it would
    /// require all five swatches on a lightness ladder with no help
    /// from hue at all. Measured against the claimed palette,
    /// selection and focus land 0.0014 apart under achromatopsia —
    /// they are separated by the blue/amber axis, which
    /// achromatopsia removes entirely. Buying that back means pulling
    /// focus roughly 0.09 in lightness away from selection, which
    /// makes the quietest mark in the vocabulary a loud one for every
    /// viewer, to serve a condition orders of magnitude rarer than
    /// the three below. The trade was declined on purpose; a palette
    /// that wants it is a different palette, and may make a larger
    /// claim when someone builds one.
    const KINDS: [Option<CvdType>; 4] = [
        None,
        Some(CvdType::Protan),
        Some(CvdType::Deutan),
        Some(CvdType::Tritan),
    ];

    fn name_of(kind: Option<CvdType>) -> &'static str {
        match kind {
            None => "normal vision",
            Some(CvdType::Protan) => "protanopia",
            Some(CvdType::Deutan) => "deuteranopia",
            _ => "tritanopia",
        }
    }

    /// `color` as this vision type receives it.
    fn seen(color: Color, kind: Option<CvdType>) -> Color {
        match kind {
            None => color,
            Some(kind) => simulate(color, kind, Severity::FULL),
        }
    }

    /// OKLab, Cartesian — `perceive-color` states OKLCH, and a
    /// distance wants the rectangular form.
    fn oklab(color: Color) -> [f64; 3] {
        let p = color.to_oklch();
        let h = p.h.to_radians();
        [p.l, p.c * h.cos(), p.c * h.sin()]
    }

    /// Euclidean OKLab distance — the whole point of the space is
    /// that this is a perceptual one.
    pub(super) fn distance(a: Color, b: Color) -> f64 {
        let (x, y) = (oklab(a), oklab(b));
        ((x[0] - y[0]).powi(2) + (x[1] - y[1]).powi(2) + (x[2] - y[2]).powi(2)).sqrt()
    }

    /// A theme's swatches at one shading level: the bare body, and
    /// each mark composited over it.
    ///
    /// **Shaded, and that is the point.** The fragment shader
    /// multiplies the composited colour by `ambient + (1 - ambient) *
    /// lambert`, so what an eye receives on an unlit face is the
    /// whole palette scaled toward black — where separations are
    /// smallest and a claim fails first.
    fn swatches(theme: &Theme, shade: f64) -> Vec<(&'static str, Color)> {
        let scale = |c: [f32; 3]| {
            Color::new(
                f64::from(c[0]) * shade,
                f64::from(c[1]) * shade,
                f64::from(c[2]) * shade,
            )
        };
        let mut out = vec![("body", scale(linear(theme.body)))];
        for (label, mark) in theme.marks() {
            out.push((label, scale(linear(mark.over(theme.body)))));
        }
        out
    }

    /// The vision types a theme's own claim covers: all of them for a
    /// claimed palette, normal vision alone for one that claims
    /// nothing. The mark check does not need this — it runs only over
    /// claimed themes — but the ground check runs over the whole
    /// registry, and holding an unclaimed palette to a dichromatic
    /// bar would be measuring a promise it never made.
    fn kinds_of(theme: &Theme) -> &'static [Option<CvdType>] {
        match theme.safety {
            Safety::ColorblindSafe => &KINDS,
            Safety::Unchecked => &KINDS[..1],
        }
    }

    /// The closest a theme's GROUND comes to any of its swatches,
    /// across the shading range, with the swatch named.
    pub(super) fn worst_against_ground(theme: &Theme) -> (f64, String) {
        let [r, g, b] = linear(theme.ground);
        let ground = Color::new(f64::from(r), f64::from(g), f64::from(b));
        let ambient = f64::from(theme.ambient);
        let shades = [ambient, ambient + (1.0 - ambient) * 0.5, 1.0];
        let mut worst = (f64::INFINITY, String::new());
        for shade in shades {
            for (name, swatch) in swatches(theme, shade) {
                for kind in kinds_of(theme) {
                    let d = distance(seen(ground, *kind), seen(swatch, *kind));
                    if d < worst.0 {
                        worst = (
                            d,
                            format!("{name} under {} at shade {shade:.2}", name_of(*kind)),
                        );
                    }
                }
            }
        }
        worst
    }

    /// The closest any two of a theme's swatches come, over every
    /// vision type and across the shading range, with the pair named.
    pub(super) fn worst_separation(theme: &Theme) -> (f64, String) {
        let ambient = f64::from(theme.ambient);
        // The shading term's floor, midpoint and ceiling. Three
        // levels rather than a sweep: the term is linear in
        // `lambert`, so the interior holds no surprise the ends miss.
        let shades = [ambient, ambient + (1.0 - ambient) * 0.5, 1.0];
        let mut worst = (f64::INFINITY, String::new());
        for shade in shades {
            let swatches = swatches(theme, shade);
            for kind in KINDS {
                for (i, (a_name, a)) in swatches.iter().enumerate() {
                    for (b_name, b) in &swatches[i + 1..] {
                        let d = distance(seen(*a, kind), seen(*b, kind));
                        if d < worst.0 {
                            worst = (
                                d,
                                format!(
                                    "{a_name}/{b_name} under {} at shade {shade:.2}",
                                    name_of(kind)
                                ),
                            );
                        }
                    }
                }
            }
        }
        worst
    }

    /// No deficiency is no change.
    #[test]
    fn severity_none_is_the_identity() {
        for kind in [CvdType::Protan, CvdType::Deutan, CvdType::Tritan] {
            let color = Color::from_srgb8(200, 120, 40);
            let out = simulate(color, kind, Severity::NONE);
            assert!(
                distance(color, out) < 1.0e-9,
                "{kind:?} at severity 0 moved the colour",
            );
        }
    }

    /// Simulating twice is simulating once.
    ///
    /// A dichromat model PROJECTS onto the surface its two remaining
    /// cone types can span, so the projection is idempotent — a
    /// colour already on that surface has nowhere left to fall. This
    /// is the strongest property available without a reference table,
    /// and a model that got its matrices wrong would almost certainly
    /// fail it.
    ///
    /// The tolerance is loose because the projection can land outside
    /// the sRGB gamut and `Color::new` clamps there, so a second pass
    /// starts from a slightly different colour than the first
    /// produced — measured at 3.4e-4 on the darkest tint in the
    /// claimed palette. That is the clamp, not the model: crossed or
    /// mistyped matrices move a colour by order 0.1, a hundred times
    /// this bar.
    #[test]
    fn simulation_is_a_projection() {
        for kind in [CvdType::Protan, CvdType::Deutan, CvdType::Tritan] {
            for &(r, g, b) in &[
                (250u8, 198u8, 45u8),
                (60, 115, 210),
                (40, 22, 58),
                (222, 232, 245),
                (150, 148, 145),
            ] {
                let once = simulate(Color::from_srgb8(r, g, b), kind, Severity::FULL);
                let twice = simulate(once, kind, Severity::FULL);
                assert!(
                    distance(once, twice) < 2.0e-3,
                    "{kind:?} is not idempotent on ({r}, {g}, {b}): moved {:.6} on the \
                     second pass",
                    distance(once, twice),
                );
            }
        }
    }

    /// Red and green collapse toward each other under the red-green
    /// deficiencies; blue and yellow do not.
    ///
    /// This is the whole reason the claimed palette is built on the
    /// blue/amber axis, so it is worth asserting that the oracle
    /// actually reports it — a simulation with its axes crossed would
    /// send the palette design in exactly the wrong direction while
    /// every other row here still passed.
    #[test]
    fn the_red_green_axis_collapses_and_the_blue_yellow_axis_survives() {
        let red = Color::from_srgb8(200, 40, 40);
        let green = Color::from_srgb8(40, 170, 40);
        let blue = Color::from_srgb8(50, 90, 210);
        let yellow = Color::from_srgb8(230, 200, 50);
        for kind in [CvdType::Protan, CvdType::Deutan] {
            let rg = distance(
                simulate(red, kind, Severity::FULL),
                simulate(green, kind, Severity::FULL),
            );
            let by = distance(
                simulate(blue, kind, Severity::FULL),
                simulate(yellow, kind, Severity::FULL),
            );
            assert!(
                rg < distance(red, green),
                "{kind:?} did not bring red and green closer together",
            );
            assert!(
                by > rg,
                "{kind:?} left blue/yellow ({by:.4}) no further apart than red/green \
                 ({rg:.4}) — the axis this palette is built on",
            );
        }
    }
}
