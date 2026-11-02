//! **Drive the viewer from a script**, by label, with no window: each
//! line of the script is one thing a person at the window does, and
//! `dump` prints what is on screen.
//!
//! Usage:
//!
//! ```sh
//! cargo run --release -p viewer --features app --example drive -- script.txt
//! cargo run --release -p viewer --features app --example drive -- - < script.txt
//! ```
//!
//! Build `--release`: in a debug build tessellation and index builds
//! take minutes.
//!
//! The script language, one command per line (`#` starts a comment,
//! quoted arguments may contain spaces, `\"` and `\\` escape):
//!
//! | command | does |
//! |---|---|
//! | `click "label" [n]` | primary click on the `n`th (0-based) visible painting of `label`; a label ending in `*` matches any text starting with the rest |
//! | `dclick "label" [n]` | double click on it |
//! | `rclick "label" [n]` | secondary click on it |
//! | `hover "label" [n]` | rest the pointer on it past the tooltip delay |
//! | `clickat x y [left\|right\|middle] [mods]` | click at window point `(x, y)` |
//! | `vclick fx fy [left\|right\|middle] [mods]` | click at fractions `(fx, fy)` of the viewport |
//! | `vdrag fx1 fy1 fx2 fy2 [left\|right\|middle] [mods]` | drag across the viewport |
//! | `drag x1 y1 x2 y2 [left\|right\|middle] [mods]` | drag between window points |
//! | `scroll x y dy` | turn the wheel at `(x, y)` by `dy` points |
//! | `type "text"` | type into whatever has keyboard focus |
//! | `key Name [mods]` | press a key (`Enter`, `Escape`, `Tab`, `Delete`, `Z`, …) |
//! | `open path` / `save path` | what Open… / Save As… end in once the dialog answers |
//! | `window w h` | lay the app out in a `w`×`h` window |
//! | `settle` | frames until nothing is outstanding, the at-rest checks included |
//! | `expect "label"` | fail unless `label` is on screen |
//! | `dump` | print every visible painting as `x,y wxh text` |
//! | `dumpall` | print every painting, clipped or not, with its clip rect |
//! | `tree` | print the feature tree's rows |
//! | `picture path.png` | render the window as the last frame painted it, chrome and viewport, to a PNG |
//! | `echo text` | print `text` |
//!
//! `mods` is `+`-joined: `ctrl`, `shift`, `alt`, `cmd`. Every command
//! is echoed as `> command` before its output. A command that cannot
//! be done stops the run with exit status 1 and dumps the screen, so
//! a failing step shows what WAS there.
//!
//! `picture` renders on a GPU device opened at its first use, on
//! whatever adapter the machine has (`WGPU_BACKEND` steers it); a box
//! with no GPU can use lavapipe from `mesa-vulkan-drivers`.
//!
//! Preferences are read from and written to a fresh file under the
//! system temporary directory, never the preferences of the person
//! running it.

#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use eframe::{egui, wgpu};
use viewer::app::drive::{DriveError, Driver, Picture, picture_device};

/// The run's state beside the driver: the device pictures render on,
/// opened at the first `picture`.
#[derive(Default)]
struct Run {
    device: Option<(wgpu::Device, wgpu::Queue)>,
}

/// Write `picture` to `path` as an 8-bit RGBA PNG.
fn write_png(picture: &Picture, path: &Path) -> Result<(), String> {
    let file = std::fs::File::create(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    let mut encoder = png::Encoder::new(
        std::io::BufWriter::new(file),
        picture.size[0],
        picture.size[1],
    );
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .and_then(|mut writer| writer.write_image_data(&picture.rgba))
        .map_err(|error| format!("could not write {}: {error}", path.display()))
}

/// Split one script line into words, honouring double quotes.
fn words(line: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut chars = line.chars().peekable();
    loop {
        while chars.peek().is_some_and(|c| c.is_whitespace()) {
            chars.next();
        }
        match chars.peek() {
            None | Some('#') => return Ok(out),
            Some('"') => {
                chars.next();
                let mut word = String::new();
                loop {
                    match chars.next() {
                        None => return Err("unterminated quote".to_owned()),
                        Some('"') => break,
                        Some('\\') => match chars.next() {
                            Some(c) => word.push(c),
                            None => return Err("trailing backslash".to_owned()),
                        },
                        Some(c) => word.push(c),
                    }
                }
                out.push(word);
            }
            Some(_) => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() {
                        break;
                    }
                    word.push(c);
                    chars.next();
                }
                out.push(word);
            }
        }
    }
}

fn number(word: Option<&String>, what: &str) -> Result<f32, String> {
    let word = word.ok_or_else(|| format!("missing {what}"))?;
    word.parse()
        .map_err(|_| format!("{what} `{word}` is not a number"))
}

fn index(word: Option<&String>) -> Result<usize, String> {
    word.map_or(Ok(0), |word| {
        word.parse()
            .map_err(|_| format!("index `{word}` is not a whole number"))
    })
}

fn label(word: Option<&String>) -> Result<&str, String> {
    word.map(String::as_str)
        .ok_or_else(|| "missing label".to_owned())
}

fn button(word: Option<&String>) -> Result<egui::PointerButton, String> {
    match word.map(String::as_str) {
        None | Some("left") => Ok(egui::PointerButton::Primary),
        Some("right") => Ok(egui::PointerButton::Secondary),
        Some("middle") => Ok(egui::PointerButton::Middle),
        Some(other) => Err(format!("button `{other}` is not left, right or middle")),
    }
}

fn modifiers(word: Option<&String>) -> Result<egui::Modifiers, String> {
    let mut held = egui::Modifiers::default();
    let Some(word) = word else {
        return Ok(held);
    };
    for part in word.split('+') {
        match part {
            "ctrl" => held.ctrl = true,
            "shift" => held.shift = true,
            "alt" => held.alt = true,
            "cmd" => held.command = true,
            other => return Err(format!("modifier `{other}` is not ctrl, shift, alt or cmd")),
        }
    }
    if held.ctrl {
        held.command = true;
    }
    Ok(held)
}

/// Where the `nth` visible painting of `label` is: exactly `label`,
/// or, when it ends in `*`, any text starting with what comes before
/// it — for a tree row, whose tag is minted afresh in every document
/// a run creates.
fn located(driver: &Driver, label: &str, nth: usize) -> Result<egui::Pos2, String> {
    let Some(prefix) = label.strip_suffix('*') else {
        return driver.find(label, nth).map_err(|error| error.to_string());
    };
    let shown: Vec<egui::Pos2> = driver
        .screen()
        .iter()
        .filter(|landed| landed.text.starts_with(prefix))
        .filter_map(|landed| landed.visible().map(|rect| rect.center()))
        .collect();
    shown.get(nth).copied().ok_or_else(|| {
        format!(
            "no visible text starting `{prefix}` #{nth} ({} visible)",
            shown.len()
        )
    })
}

/// One command, done.
fn perform(driver: &mut Driver, run: &mut Run, words: &[String]) -> Result<(), String> {
    let drive = |result: Result<(), DriveError>| result.map_err(|error| error.to_string());
    let at = |x: Option<&String>, y: Option<&String>| -> Result<egui::Pos2, String> {
        Ok(egui::pos2(number(x, "x")?, number(y, "y")?))
    };
    let in_viewport = |driver: &Driver, x: Option<&String>, y: Option<&String>| {
        driver
            .in_viewport(number(x, "fx")?, number(y, "fy")?)
            .map_err(|error| error.to_string())
    };
    let (command, args) = words.split_first().ok_or("empty command")?;
    let arg = |n: usize| args.get(n);
    match command.as_str() {
        "click" | "dclick" | "rclick" => {
            let at = located(driver, label(arg(0))?, index(arg(1))?)?;
            let (button, count) = match command.as_str() {
                "dclick" => (egui::PointerButton::Primary, 2),
                "rclick" => (egui::PointerButton::Secondary, 1),
                _ => (egui::PointerButton::Primary, 1),
            };
            drive(driver.click_at(at, button, egui::Modifiers::default(), count))
        }
        "hover" => {
            let at = located(driver, label(arg(0))?, index(arg(1))?)?;
            driver.rest_at(at);
            Ok(())
        }
        "clickat" => {
            let point = at(arg(0), arg(1))?;
            drive(driver.click_at(point, button(arg(2))?, modifiers(arg(3))?, 1))
        }
        "vclick" => {
            let point = in_viewport(driver, arg(0), arg(1))?;
            drive(driver.click_at(point, button(arg(2))?, modifiers(arg(3))?, 1))
        }
        "drag" => {
            let from = at(arg(0), arg(1))?;
            let to = at(arg(2), arg(3))?;
            drive(driver.drag(from, to, button(arg(4))?, modifiers(arg(5))?, 10))
        }
        "vdrag" => {
            let from = in_viewport(driver, arg(0), arg(1))?;
            let to = in_viewport(driver, arg(2), arg(3))?;
            drive(driver.drag(from, to, button(arg(4))?, modifiers(arg(5))?, 10))
        }
        "scroll" => {
            let point = at(arg(0), arg(1))?;
            let dy = number(arg(2), "dy")?;
            drive(driver.scroll(point, egui::vec2(0.0, dy)))
        }
        "type" => drive(driver.type_text(label(arg(0))?)),
        "key" => {
            let name = label(arg(0))?;
            let key = egui::Key::from_name(name).ok_or_else(|| format!("no key named `{name}`"))?;
            drive(driver.key(key, modifiers(arg(1))?))
        }
        "open" => drive(driver.open(Path::new(label(arg(0))?))),
        "save" => drive(driver.save(Path::new(label(arg(0))?))),
        "window" => {
            driver.resize(egui::vec2(
                number(arg(0), "width")?,
                number(arg(1), "height")?,
            ));
            drive(driver.settle())
        }
        "settle" => drive(driver.settle()),
        "expect" => {
            let text = label(arg(0))?;
            driver
                .find(text, index(arg(1))?)
                .map(|_| ())
                .map_err(|error| format!("expected: {error}"))
        }
        "dump" => {
            print!("{}", driver.dump());
            Ok(())
        }
        "dumpall" => {
            print!("{}", driver.dump_all());
            Ok(())
        }
        "tree" => {
            for row in driver.session().tree_rows() {
                println!(
                    "{} {:?}{}{}",
                    row.spoken,
                    row.status,
                    if row.placed { " placed" } else { "" },
                    row.readout.map(|r| format!(" — {r:?}")).unwrap_or_default(),
                );
            }
            Ok(())
        }
        "picture" => {
            let path = Path::new(label(arg(0))?);
            if run.device.is_none() {
                run.device =
                    Some(pollster::block_on(picture_device()).map_err(|error| error.to_string())?);
            }
            let Some((device, queue)) = &run.device else {
                return Err("no device".to_owned());
            };
            let picture = driver
                .picture(device, queue)
                .map_err(|error| error.to_string())?;
            write_png(&picture, path)?;
            println!(
                "{}x{} picture written to {}",
                picture.size[0],
                picture.size[1],
                path.display()
            );
            Ok(())
        }
        "echo" => {
            println!("{}", args.join(" "));
            Ok(())
        }
        other => Err(format!("no command `{other}`")),
    }
}

fn main() -> ExitCode {
    let Some(source) = std::env::args().nth(1) else {
        eprintln!("usage: drive <script> | drive -   (see the example's header)");
        return ExitCode::FAILURE;
    };
    let script = if source == "-" {
        let mut text = String::new();
        if let Err(error) = std::io::stdin().read_to_string(&mut text) {
            eprintln!("could not read the script from stdin: {error}");
            return ExitCode::FAILURE;
        }
        text
    } else {
        match std::fs::read_to_string(&source) {
            Ok(text) => text,
            Err(error) => {
                eprintln!("could not read {source}: {error}");
                return ExitCode::FAILURE;
            }
        }
    };
    let prefs: PathBuf = std::env::temp_dir()
        .join(format!("viewer-drive-{}", std::process::id()))
        .join("prefs.toml");
    let mut driver = match Driver::start(prefs, pncad::tolerance::witness()) {
        Ok(driver) => driver,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };
    let mut run = Run::default();
    for (number, line) in script.lines().enumerate() {
        let parsed = match words(line) {
            Ok(parsed) => parsed,
            Err(error) => {
                eprintln!("line {}: {error}", number + 1);
                return ExitCode::FAILURE;
            }
        };
        if parsed.is_empty() {
            continue;
        }
        println!("> {}", line.trim());
        if let Err(error) = perform(&mut driver, &mut run, &parsed) {
            println!("!! line {}: {error}", number + 1);
            println!("-- the screen when it failed --");
            print!("{}", driver.dump());
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
