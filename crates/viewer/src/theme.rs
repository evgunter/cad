//! The display palette as a value.
//!
//! G1's rule binds colour exactly as it binds everything else in this
//! crate: **the palette is a value, and rendering is a view of it.**
//! Nothing here names `egui` or `wgpu`, which is what makes this a
//! non-`app` module — the palette compiles, and is asserted on, in
//! ordinary headless CI with no toolkit graph present. [`app`] maps a
//! [`Theme`] onto the chrome and [`gpu`] feeds it to the shader;
//! neither of them decides what any colour *is*.
//!
//! [`app`]: crate::app
//! [`gpu`]: crate::gpu
//!
//! # Where a colour comes from
//!
//! Two sources, with the precedence between them ratified in
//! `docs/GUI-DESIGN.md`:
//!
//! - **The theme — a USER preference.** It supplies every semantic
//!   mark (selection, hover, probe, focus, unresolved) and the
//!   *default* body colour. It is never written into a document: the
//!   same file has to be legible to a colourblind reader and to
//!   somebody running the palette they find prettiest, on their own
//!   screens.
//! - **The document.** `Attr::Color` on a stable name, authored and
//!   persisted — which **this crate does not read yet**. When it
//!   does, it overrides [`Theme::body`] per patch, and the theme
//!   never overrides it back.
//!
//! Both sides are [`Rgba8`] for exactly that reason: the override is
//! then a substitution of one value for another *within one colour
//! space*, not a conversion between two spaces where drift can live.
//! That type is `editor-core`'s, reached on a direct edge rather than
//! through a new re-export on the façade's root — the ruling
//! `pncad`'s own crate docs state for a type the façade does not
//! carry, and the same one the `bvh` edge in this crate's manifest
//! cites. It adds nothing to the build: `pncad` already depends on
//! `editor-core`, so the crate was in this graph either way.
//!
//! # sRGB is what a theme states; linear is what the shader gets
//!
//! Every colour here is 8-bit sRGB, the space a palette is authored
//! in and the space `Attr::Color` persists. The shader shades in
//! linear RGB, so [`linear`] is the one conversion, applied at the
//! one boundary where a theme becomes a uniform.
//!
//! This re-expressed the constants that came before it, and **it is
//! not a bit-preserving move**: the four shader tints and the body
//! colour were written as linear `f32` triples, and the nearest sRGB8
//! encoding of each returns to linear up to 0.0035 away from where it
//! started — under a single 8-bit step, and a fifth of a percent of
//! the range. Naming the drift rather than claiming there is none is
//! the honest half; the reason to accept it is that it buys one
//! colour space for the palette and the document both, and
//! bit-preservation is not on its own a reason to keep a shape
//! (`memories/output-stability-as-justification.md`).
//!
//! It also made one existing claim true. `UNRESOLVED_COLOR` was
//! already sRGB8 (it goes to `egui`) while the tints beside it were
//! linear, so the comment calling the badge red "the same red" the
//! viewport uses was comparing two numbers that did not live in the
//! same space. Now they do.

use editor_core::appearance::Rgba8;

/// Which ground a theme is built on.
///
/// The chrome's own light/dark split, named here so the decision is
/// part of the value rather than something `app` infers from a
/// brightness threshold on some field. `app` maps this onto the
/// toolkit's own light and dark `Visuals`, which is the only place
/// the toolkit's names appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Polarity {
    /// A light ground: dark text, a pale viewport surround.
    Light,
    /// A dark ground: light text, a deep viewport surround.
    Dark,
}

/// One highlight mark: the colour a flagged patch is tinted toward,
/// and how far it travels.
///
/// **A tint and a strength are one decision, not two.** The shader
/// mixes rather than replaces — a highlight that discarded the
/// shading would flatten the facets a display-δ reading exists to
/// show — so how visible a mark is depends on both numbers together,
/// and a palette that carried the colours while the strengths stayed
/// hard-coded in WGSL could not actually state how its marks read.
/// That is also what makes [`Safety`] checkable: the thing to
/// simulate is the mix, not the tint.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mark {
    /// The colour the mixed result is pulled toward.
    pub tint: Rgba8,
    /// How far, in `[0, 1]`: `0.0` leaves the body colour untouched,
    /// `1.0` replaces it. Checked over every registered theme by
    /// `tests/theme.rs` rather than at each use.
    pub strength: f32,
}

impl Mark {
    /// This mark applied over `body`, in sRGB — **what an eye
    /// actually receives**, and so the only form worth simulating or
    /// measuring a distance between.
    ///
    /// The mix runs in *linear* light because that is where the
    /// shader's `mix` runs; doing it in sRGB would measure a screen
    /// nobody is looking at.
    pub fn over(&self, body: Rgba8) -> Rgba8 {
        let [br, bg, bb] = linear(body);
        let [tr, tg, tb] = linear(self.tint);
        let t = self.strength;
        from_linear([br + (tr - br) * t, bg + (tg - bg) * t, bb + (tb - bb) * t])
    }
}

/// What a theme claims about its own legibility.
///
/// **A claim, not a constraint on every palette.** A theme that says
/// [`Safety::ColorblindSafe`] is checked by `tests/theme.rs` under
/// simulated protanopia, deuteranopia and tritanopia; a theme that
/// says [`Safety::Unchecked`] is not, and is not lesser for it — the
/// point of shipping themes at all is that a palette chosen to be
/// pretty and a palette chosen to be discriminable are different
/// jobs, and one build can carry both. What must never happen is a
/// palette claiming the first job and not doing it, which is the one
/// thing the test is there to prevent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Safety {
    /// No claim; nothing is asserted about this palette's marks.
    Unchecked,
    /// Claims its marks stay mutually distinguishable under
    /// dichromatic vision, and is held to it.
    ColorblindSafe,
}

/// A complete display palette.
///
/// One value covers both halves of the window because they are one
/// question: the viewport's marks are mixed over [`Theme::body`] and
/// read against the chrome behind them, so a palette that stated only
/// the tints could not say how anything looks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    /// The stable identifier: what `--theme` accepts and what a
    /// preferences file will one day hold. Distinct from any label
    /// shown in the chrome, which may be translated or prettied.
    pub name: &'static str,
    /// Light or dark ground.
    pub polarity: Polarity,
    /// The default body colour — what an unflagged patch shades
    /// from, and what a document's own `Attr::Color` replaces per
    /// patch once this crate reads appearance.
    pub body: Rgba8,
    /// The ambient term: the fraction of the body colour that
    /// survives where the light does not reach, in `[0, 1]`.
    ///
    /// Part of the palette rather than a constant beside it because
    /// it is polarity-bound: a part unlit to 0.25 reads as solid
    /// against a dark surround and as a hole against a pale one.
    pub ambient: f32,
    /// The patch the user committed to.
    pub selected: Mark,
    /// The patch under the cursor.
    pub hovered: Mark,
    /// A free-move probe's placement (G3's honesty requirement: a
    /// probed placement must never be mistakable for a mated one).
    pub probe: Mark,
    /// The extent of what the side panel is editing.
    ///
    /// Deliberately the same hue family as [`Theme::selected`] at a
    /// lower strength: the two are one relation seen at two scales,
    /// and a focus as loud as the selection would bury the
    /// distinction. A palette is free to break that, and the safety
    /// check does not require a hue difference precisely so that
    /// value-only separation stays a legitimate answer.
    pub focus: Mark,
    /// An unresolved selection, a deleted feature, a failed or
    /// poisoned badge — everything that says "this does not denote
    /// anything". Chrome only; it tints no geometry.
    pub unresolved: Rgba8,
    /// This palette's legibility claim.
    pub safety: Safety,
}

impl Theme {
    /// Every registered theme.
    ///
    /// The registry is what `tests/theme.rs` iterates, so a palette
    /// added here is checked here — there is no second list to keep
    /// in step and no way to ship a theme the suite never saw.
    pub const ALL: &'static [Theme] = &[DARK_NEUTRAL, LIGHT_NEUTRAL, COLORBLIND_SAFE];

    /// The theme a viewer opens with when nothing selects one.
    pub const DEFAULT: Theme = DARK_NEUTRAL;

    /// The registered theme called `name`, if there is one.
    ///
    /// `None` rather than a fallback: a `--theme` nobody recognises
    /// is a typo, and silently opening the default would hide it.
    pub fn by_name(name: &str) -> Option<Theme> {
        Theme::ALL.iter().copied().find(|theme| theme.name == name)
    }

    /// The four marks, paired with what to call each in a refusal.
    ///
    /// Ordered, so a failure names the same pair the same way twice.
    pub fn marks(&self) -> [(&'static str, Mark); 4] {
        [
            ("selected", self.selected),
            ("hovered", self.hovered),
            ("probe", self.probe),
            ("focus", self.focus),
        ]
    }
}

/// The palette this viewer has always drawn, re-expressed in sRGB.
///
/// It claims nothing about colourblind legibility — selection-orange
/// against hover-blue is a hue distinction, and whether it survives
/// dichromacy is a question for a palette that has been designed to
/// answer it, not one that inherited its colours from a first light.
const DARK_NEUTRAL: Theme = Theme {
    name: "dark-neutral",
    polarity: Polarity::Dark,
    // A neutral machined grey, so shading reads as shape rather than
    // as colour.
    body: Rgba8::opaque(206, 209, 214),
    ambient: 0.25,
    selected: Mark {
        tint: Rgba8::opaque(255, 206, 111),
        strength: 0.55,
    },
    hovered: Mark {
        tint: Rgba8::opaque(179, 221, 255),
        strength: 0.55,
    },
    probe: Mark {
        tint: Rgba8::opaque(206, 160, 249),
        strength: 0.65,
    },
    focus: Mark {
        tint: Rgba8::opaque(255, 229, 173),
        strength: 0.24,
    },
    unresolved: Rgba8::opaque(210, 90, 70),
    safety: Safety::Unchecked,
};

/// The same palette on a light ground.
///
/// The marks are unchanged — they are mixed over the body, not over
/// the chrome, and the body has not moved. What changes is the
/// surround and the ambient term: a part unlit to 0.25 against a pale
/// chrome reads as a hole punched in the panel, so the floor comes up
/// far enough that the shading still describes a solid.
const LIGHT_NEUTRAL: Theme = Theme {
    name: "light-neutral",
    polarity: Polarity::Light,
    body: Rgba8::opaque(206, 209, 214),
    ambient: 0.45,
    selected: Mark {
        tint: Rgba8::opaque(255, 206, 111),
        strength: 0.55,
    },
    hovered: Mark {
        tint: Rgba8::opaque(179, 221, 255),
        strength: 0.55,
    },
    probe: Mark {
        tint: Rgba8::opaque(206, 160, 249),
        strength: 0.65,
    },
    focus: Mark {
        tint: Rgba8::opaque(255, 229, 173),
        strength: 0.24,
    },
    // Darker than the dark theme's red by as much as the ground
    // moved: the same hue at the same lightness on a pale panel is
    // the one chrome colour that stops being readable.
    unresolved: Rgba8::opaque(176, 46, 28),
    safety: Safety::Unchecked,
};

/// A palette designed so its four marks stay mutually
/// distinguishable under dichromatic vision — and held to it by
/// `tests/theme.rs`, which simulates protanopia, deuteranopia and
/// tritanopia over the composited colours.
///
/// # What the claim cost
///
/// Three of this palette's choices are consequences of the claim
/// rather than taste, and each gives something up:
///
/// 1. **The marks separate on LIGHTNESS first.** Lightness is the one
///    channel every dichromacy keeps, so the four marks sit on a
///    ladder — probe darkest, then hover, then the body itself, then
///    focus and selection at the top — and hue is the second signal,
///    not the first.
/// 2. **Focus and selection are separated by LIGHTNESS, not by hue.**
///    The neutral themes make them one relation at two scales, which
///    is the better design when it can be afforded; here it cannot.
///    The first attempt pulled them onto opposite ends of the
///    blue/amber axis — the axis protanopia and deuteranopia leave
///    most intact — and the check refused it at 0.0546: tritanopia
///    is precisely the deficiency that destroys blue/amber. No
///    single hue axis survives all three, so the pair had to move
///    apart on the ladder instead, and hue became the second signal
///    rather than the only one.
/// 3. **The ambient floor is high (0.42).** A mark is discriminable
///    in shadow only to the extent there is light there at all: every
///    swatch scales with the shading term, so a deep floor compresses
///    the whole palette toward black and the worst pair fails there
///    first. Raising the floor is what buys the shadowed half of the
///    part back — measurably, and monotonically.
///
/// The `unresolved` colour is NOT part of the claim: it tints no
/// geometry, and every badge that uses it carries its own words
/// ("deleted", "at rest: …"), so colour is redundant there rather
/// than load-bearing.
const COLORBLIND_SAFE: Theme = Theme {
    name: "colorblind-safe",
    polarity: Polarity::Dark,
    // Darker than the neutral themes' near-white, and that is what
    // makes the ladder fit: a mid body leaves range both above and
    // below it for four marks to occupy.
    body: Rgba8::opaque(120, 119, 117),
    ambient: 0.42,
    // The top of the ladder — a light amber.
    selected: Mark {
        tint: Rgba8::opaque(255, 214, 90),
        strength: 0.72,
    },
    // A step DOWN in lightness, where the neutral themes' hover is a
    // step up. Blue against the body's neutral is the second signal;
    // the lightness drop is the first.
    hovered: Mark {
        tint: Rgba8::opaque(58, 110, 205),
        strength: 0.55,
    },
    // The darkest thing on screen. G3 asks that a probed placement be
    // unmistakable, and under every vision type in scope what makes
    // it so is that nothing else is this dark.
    probe: Mark {
        tint: Rgba8::opaque(42, 24, 60),
        strength: 0.66,
    },
    // Pale and cool, at the lowest strength in the palette: still the
    // quietest mark, but now a full rung below the selection rather
    // than beside it.
    focus: Mark {
        tint: Rgba8::opaque(214, 224, 238),
        strength: 0.38,
    },
    unresolved: Rgba8::opaque(232, 122, 74),
    safety: Safety::ColorblindSafe,
};

/// `color`'s three channels as linear RGB — the space the shader
/// shades in, and the one boundary a theme crosses to reach it.
///
/// Alpha is dropped rather than carried: every colour that reaches
/// the viewport is opaque, and a lane the shader does not read is a
/// lane that can silently disagree with what the value says.
pub fn linear(color: Rgba8) -> [f32; 3] {
    [
        channel_to_linear(color.r),
        channel_to_linear(color.g),
        channel_to_linear(color.b),
    ]
}

/// The inverse of [`linear`], for the composited colours the safety
/// check measures — the mix happens in light, the answer is stated in
/// the space the palette is written in.
pub fn from_linear(linear: [f32; 3]) -> Rgba8 {
    let [r, g, b] = linear;
    Rgba8::opaque(
        channel_to_srgb8(r),
        channel_to_srgb8(g),
        channel_to_srgb8(b),
    )
}

/// One 8-bit sRGB channel as linear light. The IEC 61966-2-1 curve,
/// stated rather than approximated by a 2.2 power: the toe below
/// 0.04045 is linear, and rounding it into the exponent is the
/// difference that shows up in near-black.
fn channel_to_linear(channel: u8) -> f32 {
    let c = f32::from(channel) / 255.0;
    if c <= 0.040_45 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// One linear channel as 8-bit sRGB, clamped: a mix of two in-gamut
/// colours stays in gamut, but the clamp is what makes that a
/// property of the arithmetic rather than an assumption about it.
fn channel_to_srgb8(channel: f32) -> u8 {
    let c = channel.clamp(0.0, 1.0);
    let encoded = if c <= 0.003_130_8 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0).round() as u8
}
