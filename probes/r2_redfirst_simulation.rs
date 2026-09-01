//! R2 (MESH-4) review probe — ADDITIVE, standalone.
//!
//! The PR claims each of the three new `sizing::tests` rows was
//! verified red-first against a specific mutation. Two of those
//! mutations are pure arithmetic, so they can be checked without
//! building the crate: reimplement the row's assertions and run them
//! against the mutant.
//!
//!     rustc -O probes/r2_redfirst_simulation.rs -o /tmp/r2red && /tmp/r2red

#[derive(Clone, Copy)]
struct Eps(f64);

// Which variant of the ops to use.
#[derive(Clone, Copy, PartialEq)]
enum Variant {
    Correct,
    /// The PR's mutation 1: `coincident` flipped to a strict `<`.
    CoincidentStrict,
    /// The PR's mutation 2: `separates` written as `!coincident`.
    SeparatesAsNotCoincident,
    /// A mutation the PR does NOT claim to have tried: `pad` widening DOWN.
    PadDown,
}

impl Eps {
    fn separates(self, length: f64, v: Variant) -> bool {
        match v {
            Variant::SeparatesAsNotCoincident => !self.coincident(length, Variant::Correct),
            _ => length > self.0,
        }
    }
    fn coincident(self, length: f64, v: Variant) -> bool {
        match v {
            Variant::CoincidentStrict => length < self.0,
            _ => length <= self.0,
        }
    }
    fn dominates(self, scaled: f64, _v: Variant) -> bool {
        scaled < self.0
    }
    fn pad(self, bound: f64, v: Variant) -> f64 {
        match v {
            Variant::PadDown => bound - self.0,
            _ => bound + self.0,
        }
    }
}

/// Row 1, transcribed from `the_band_edges_are_where_the_operations_differ`.
fn row_band_edges(v: Variant) -> Vec<&'static str> {
    let mut red = Vec::new();
    let e = Eps(1e-9);
    if !e.coincident(1e-9, v) {
        red.push("`coincident` includes the band");
    }
    if e.separates(1e-9, v) {
        red.push("`separates` excludes the band");
    }
    if e.dominates(1e-9, v) {
        red.push("`dominates` excludes the band");
    }
    if !(e.coincident(0.5e-9, v) && e.dominates(0.5e-9, v) && !e.separates(0.5e-9, v)) {
        red.push("inside the band");
    }
    if !(e.separates(2e-9, v) && !e.coincident(2e-9, v) && !e.dominates(2e-9, v)) {
        red.push("outside the band");
    }
    if Eps(0.0).dominates(0.0, v) {
        red.push("a zero band dominates nothing");
    }
    red
}

/// Row 2, transcribed from `a_poisoned_length_is_neither_near_nor_far`.
fn row_poisoned(v: Variant) -> Vec<&'static str> {
    let mut red = Vec::new();
    let e = Eps(1e-9);
    for x in [0.0, 1e-12, 1e-9, 1e-6, f64::INFINITY] {
        if e.separates(x, v) == e.coincident(x, v) {
            red.push("ordered input: separates == coincident");
            break;
        }
    }
    if !(!e.separates(f64::NAN, v) && !e.coincident(f64::NAN, v)) {
        red.push("NaN is neither separated nor coincident");
    }
    if e.dominates(f64::NAN, v) {
        red.push("NaN is not dominated");
    }
    red
}

/// Row 3, transcribed from `pad_widens_upward_by_one_band`.
fn row_pad(v: Variant) -> Vec<&'static str> {
    let mut red = Vec::new();
    let e = Eps(1e-9);
    if e.pad(1.0, v) != 1.0 + 1e-9 {
        red.push("pad(1.0) == 1.0 + band");
    }
    if !(e.pad(0.0, v) > 0.0) {
        red.push("a zero bound pads to the band itself");
    }
    if Eps(0.0).pad(4.0, v) != 4.0 {
        red.push("a zero band pads nothing");
    }
    red
}

fn main() {
    let variants = [
        ("CORRECT (as shipped)", Variant::Correct),
        ("MUTANT: coincident flipped to `<`", Variant::CoincidentStrict),
        (
            "MUTANT: separates written as `!coincident`",
            Variant::SeparatesAsNotCoincident,
        ),
        ("MUTANT: pad widens DOWN", Variant::PadDown),
    ];
    for (name, v) in variants {
        println!("--- {name}");
        for (row, red) in [
            ("the_band_edges_are_where_the_operations_differ", row_band_edges(v)),
            ("a_poisoned_length_is_neither_near_nor_far", row_poisoned(v)),
            ("pad_widens_upward_by_one_band", row_pad(v)),
        ] {
            if red.is_empty() {
                println!("    GREEN  {row}");
            } else {
                println!("    RED    {row}  <- {}", red.join("; "));
            }
        }
    }
}
