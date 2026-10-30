//! nightsky renders the animated SVGs on my GitHub profile.
//!
//!     cargo run --release -- [banner|footer|cards|skyline|langs ...]
//!
//! No arguments renders everything into ../assets. The GitHub API is read with
//! PROFILE_TOKEN if set (sees private repos), otherwise GITHUB_TOKEN.

use base64::Engine;
use serde_json::{json, Value};
use std::{collections::HashMap, env, error::Error, fmt::Write, fs, path::PathBuf};

type Res<T> = Result<T, Box<dyn Error>>;

const USER: &str = "SenseiIssei";

/// Repos that only duplicate another one and would count its code twice.
const SKIP_REPOS: &[&str] = &["Caruspex-private-archive", "SF-Assistant-Private", "Lovebyte-certs"];

const INK: &str = "#07061a";
const LILAC: &str = "#c9b8ff";
const MAGENTA: &str = "#ff5fa2";
const TEAL: &str = "#5fe1d2";
const MOON: &str = "#f6efdc";
const WARM: &str = "#ffd98a";
const SANS: &str = "'Segoe UI','Hiragino Sans','Yu Gothic UI','Noto Sans JP',system-ui,-apple-system,Helvetica,Arial,sans-serif";
const MONO: &str = "'JetBrains Mono','Cascadia Code',Consolas,'SF Mono',Menlo,monospace";

/// A small bat, centred on the origin, about 22px wide.
const BAT: &str = "M-11 0c3-4 7-5 9-2c1-2 3-2 4 0c2-3 6-2 9 2c-3-1-6 0-7 2c-1-1-3-1-4 0c-1-2-4-3-7-2z";

const BASE_CSS: &str = r#"
text{font-family:$SANS}
.mono{font-family:$MONO}
.tw{fill:$MOON;animation:tw 5s ease-in-out infinite}
@keyframes tw{0%,100%{opacity:.12}50%{opacity:.95}}
.ff{fill:$WARM;opacity:0;animation:ff 10s linear infinite}
.ff.t{fill:$TEAL}
@keyframes ff{0%{transform:translate(0,0);opacity:0}15%{opacity:.85}100%{transform:translate(40px,-90px);opacity:0}}
.kb{transform-box:fill-box;transform-origin:center;animation:kb 30s ease-in-out infinite alternate}
@keyframes kb{from{transform:scale(1)}to{transform:scale(1.07)}}
.rise{opacity:0;animation:rise 1.4s ease-out forwards}
@keyframes rise{from{opacity:0;transform:translateY(12px)}to{opacity:1;transform:none}}
.fade{opacity:0;animation:fade 2.4s ease-out forwards}
@keyframes fade{to{opacity:1}}
.draw{stroke-dasharray:600;stroke-dashoffset:600;animation:draw 2s ease-out forwards}
@keyframes draw{to{stroke-dashoffset:0}}
.pulse{animation:pulse 7s ease-in-out infinite}
@keyframes pulse{0%,100%{opacity:.55}50%{opacity:1}}
.bat{fill:$INK}
"#;

fn main() -> Res<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let jobs: Vec<&str> = if args.is_empty() {
        vec!["banner", "footer", "cards", "skyline", "langs"]
    } else {
        args.iter().map(String::as_str).collect()
    };
    fs::create_dir_all(root().join("assets/cards"))?;
    for job in jobs {
        match job {
            "banner" => save("assets/banner.svg", &banner()?)?,
            "footer" => save("assets/footer.svg", &footer()?)?,
            "skyline" => save("assets/skyline.svg", &skyline()?)?,
            "langs" => save("assets/langs.svg", &langs()?)?,
            "cards" => {
                let projects: Vec<Value> = serde_json::from_str(&fs::read_to_string(here().join("projects.json"))?)?;
                for p in &projects {
                    save(&format!("assets/cards/{}.svg", p["id"].as_str().unwrap_or("x")), &card(p)?)?;
                }
            }
            other => return Err(format!("unknown job: {other}").into()),
        }
    }
    Ok(())
}

// ---------------------------------------------------------------- banner

fn banner() -> Res<String> {
    let (w, h) = (1200.0, 300.0);
    let mut rng = Rng(0x00b4_9e11);
    let mut s = open(w, h, 18.0);
    s += &glow_defs();
    s += &css(r#"
.type{clip-path:inset(0 100% 0 0);animation:type 2.6s steps(44,end) 1.6s forwards}
@keyframes type{to{clip-path:inset(0 0 0 0)}}
.cur{fill:$MAGENTA;opacity:0;animation:blink 1s step-end 4.2s infinite}
@keyframes blink{0%{opacity:1}50%{opacity:0}}
"#);
    s += r#"<g clip-path="url(#clip)">"#;
    write!(s, r#"<image class="kb" href="{}" width="{w}" height="{h}" preserveAspectRatio="xMidYMid slice"/>"#, jpg("assets/img/banner.jpg")?)?;
    s += r#"<circle class="pulse" cx="951" cy="126" r="170" fill="url(#glow)"/>"#;
    s += &fill(r#"<linearGradient id="shade" x1="0" x2="1"><stop offset="0" stop-color="$INK" stop-opacity=".88"/><stop offset=".5" stop-color="$INK" stop-opacity=".35"/><stop offset=".75" stop-color="$INK" stop-opacity="0"/></linearGradient><rect width="1200" height="300" fill="url(#shade)"/>"#);
    s += &stars(&mut rng, 70, (0.0, 640.0), (0.0, 230.0));
    s += &fireflies(&mut rng, 16, (60.0, 1150.0), (150.0, 300.0));
    s += &bats(&mut rng, 3, w, h);
    s += &fill(&format!(
        r##"<text class="fade" x="72" y="94" font-size="19" letter-spacing="8" fill="$LILAC">夜更かしの開発者</text>
<text class="rise" x="66" y="168" font-size="76" font-weight="300" letter-spacing="16" fill="$MOON" style="animation-delay:.3s">JAKOB</text>
<path class="draw" d="M72 188H340" stroke="$MAGENTA" stroke-width="2" style="animation-delay:1s"/>
<g class="type"><text class="mono" x="72" y="226" font-size="19" fill="#e8dcff" textLength="{tl}" lengthAdjust="spacingAndGlyphs">C++ machine vision by day, Rust after hours</text></g>
<rect class="cur" x="{cx}" y="210" width="9" height="20"/>"##,
        tl = 500,
        cx = 72 + 500 + 6
    ));
    s += "</g>";
    s += &frame(w, h, 18.0);
    Ok(s + "</svg>")
}

// ---------------------------------------------------------------- footer

fn footer() -> Res<String> {
    let (w, h) = (1200.0, 234.0);
    let mut rng = Rng(0x0f00_7e12);
    let mut s = open(w, h, 18.0);
    s += &css("");
    s += r#"<g clip-path="url(#clip)">"#;
    write!(s, r#"<image class="kb" href="{}" width="{w}" height="{h}" preserveAspectRatio="xMidYMid slice"/>"#, jpg("assets/img/footer.jpg")?)?;
    s += &fill(r#"<radialGradient id="vig" cx=".5" cy=".55" r=".6"><stop offset="0" stop-color="$INK" stop-opacity=".55"/><stop offset="1" stop-color="$INK" stop-opacity="0"/></radialGradient><rect width="1200" height="234" fill="url(#vig)"/>"#);
    s += &stars(&mut rng, 40, (0.0, w), (0.0, 120.0));
    s += &fireflies(&mut rng, 12, (0.0, w), (110.0, 234.0));
    s += &bats(&mut rng, 2, w, h);
    s += &fill(
        r#"<text class="fade" x="600" y="112" text-anchor="middle" font-size="30" font-weight="300" letter-spacing="14" fill="$MOON">おやすみ</text>
<text class="fade" x="600" y="146" text-anchor="middle" font-size="14" letter-spacing="5" fill="$LILAC" style="animation-delay:.8s">GOOD NIGHT, AND THANKS FOR STOPPING BY</text>"#,
    );
    s += "</g>";
    s += &frame(w, h, 18.0);
    Ok(s + "</svg>")
}

// ---------------------------------------------------------------- project cards

fn card(p: &Value) -> Res<String> {
    let (w, h) = (800.0, 420.0);
    let id = p["id"].as_str().ok_or("project without id")?;
    let mut rng = Rng(id.bytes().fold(0x9e37_79b9_7f4a_7c15, |a, b| a.rotate_left(5) ^ b as u64));
    let stars_n = p["repo"].as_str().and_then(|r| api(&format!("https://api.github.com/repos/{r}")).ok()).and_then(|v| v["stargazers_count"].as_u64()).unwrap_or(0);

    let mut s = open(w, h, 18.0);
    s += &css("");
    s += r#"<g clip-path="url(#clip)">"#;
    write!(s, r#"<image class="kb" href="{}" width="{w}" height="{h}" preserveAspectRatio="xMidYMid slice"/>"#, jpg(&format!("assets/img/{id}.jpg"))?)?;
    s += &fill(r#"<linearGradient id="shade" x1="0" y1="0" x2="0" y2="1"><stop offset=".3" stop-color="$INK" stop-opacity="0"/><stop offset=".62" stop-color="$INK" stop-opacity=".7"/><stop offset="1" stop-color="$INK" stop-opacity=".96"/></linearGradient><rect width="800" height="420" fill="url(#shade)"/>"#);
    s += &stars(&mut rng, 26, (0.0, w), (0.0, 200.0));
    s += &fireflies(&mut rng, 7, (0.0, w), (180.0, 420.0));

    // Kanji down the right edge, one glyph per line.
    for (i, ch) in p["kanji"].as_str().unwrap_or("").chars().enumerate() {
        write!(s, r#"<text class="fade" x="748" y="{}" font-size="30" text-anchor="middle" fill="{MOON}" opacity=".92" style="animation-delay:{:.1}s">{ch}</text>"#, 64 + i * 40, 0.4 + i as f64 * 0.3)?;
    }

    let name = esc(p["name"].as_str().unwrap_or(id));
    write!(s, r#"<text class="rise" x="40" y="250" font-size="50" font-weight="600" letter-spacing="2" fill="{MOON}">{name}"#)?;
    if stars_n > 0 {
        write!(s, r#"<tspan dx="18" font-size="24" font-weight="400" fill="{LILAC}">★ {stars_n}</tspan>"#)?;
    }
    s += "</text>";
    write!(s, r#"<path class="draw" d="M42 268H220" stroke="{MAGENTA}" stroke-width="2" style="animation-delay:.5s"/>"#)?;
    let mut lines = wrap(p["desc"].as_str().unwrap_or(""), 54);
    if lines.len() > 2 {
        lines.truncate(2);
        lines[1].push('…');
    }
    for (i, line) in lines.iter().enumerate() {
        write!(s, r##"<text class="fade" x="40" y="{}" font-size="24" fill="#e4dbff" style="animation-delay:.6s">{}</text>"##, 306 + i * 32, esc(line))?;
    }
    let stack: Vec<&str> = p["stack"].as_array().map(|a| a.iter().filter_map(Value::as_str).collect()).unwrap_or_default();
    write!(s, r#"<text class="mono fade" x="40" y="394" font-size="19" letter-spacing="1.5" fill="{TEAL}" style="animation-delay:1s">{}</text>"#, esc(&stack.join("  ·  ")))?;
    s += "</g>";
    s += &frame(w, h, 18.0);
    Ok(s + "</svg>")
}

// ---------------------------------------------------------------- contribution skyline

fn skyline() -> Res<String> {
    let q = "query($login:String!){user(login:$login){contributionsCollection{contributionCalendar{totalContributions weeks{contributionDays{contributionCount}}}}}}";
    let data = graphql(q, json!({ "login": USER }))?;
    let cal = &data["user"]["contributionsCollection"]["contributionCalendar"];
    let total = cal["totalContributions"].as_u64().unwrap_or(0);
    let weeks: Vec<u64> = cal["weeks"]
        .as_array()
        .ok_or("calendar without weeks")?
        .iter()
        .map(|wk| wk["contributionDays"].as_array().map(|d| d.iter().filter_map(|x| x["contributionCount"].as_u64()).sum()).unwrap_or(0))
        .collect();

    let (w, h, ground) = (1200.0, 340.0, 300.0);
    let max = weeks.iter().copied().max().unwrap_or(1).max(1) as f64;
    let (left, right) = (40.0, 1160.0);
    let step = (right - left) / weeks.len().max(1) as f64;
    let bw = (step - 4.0).max(6.0);
    let mut rng = Rng(0x00c0_ffee);

    let mut s = open(w, h, 18.0);
    s += &glow_defs();
    s += &fill(r##"<defs><linearGradient id="sky" x1="0" y1="0" x2="0" y2="1"><stop offset="0" stop-color="#07061a"/><stop offset=".55" stop-color="#1b1442"/><stop offset="1" stop-color="#52215f"/></linearGradient></defs>"##);
    s += &css(r#"
.w{fill:$WARM;opacity:0;animation:on .6s ease-out forwards}
.w.t{fill:$TEAL}.w.p{fill:$MAGENTA}
.w.f{animation:on .6s ease-out forwards,flick 7s ease-in-out infinite}
@keyframes on{to{opacity:.95}}
@keyframes flick{0%,100%{opacity:.95}50%{opacity:.3}}
.shoot{stroke:$MOON;stroke-width:1.6;stroke-linecap:round;opacity:0;animation:shoot 12s linear 4s infinite}
@keyframes shoot{0%{opacity:0;transform:translate(0,0)}2%{opacity:1}9%{opacity:0;transform:translate(-300px,120px)}100%{opacity:0;transform:translate(-300px,120px)}}
"#);
    s += r#"<g clip-path="url(#clip)"><rect width="1200" height="340" fill="url(#sky)"/>"#;
    s += &stars(&mut rng, 120, (0.0, w), (0.0, 210.0));
    s += r#"<circle class="pulse" cx="1070" cy="78" r="110" fill="url(#glow)"/>"#;
    write!(s, r#"<circle cx="1070" cy="78" r="34" fill="{MOON}"/><line class="shoot" x1="860" y1="40" x2="900" y2="24"/>"#)?;
    s += &bats(&mut rng, 2, w, 200.0);

    let mut busiest = (0usize, 0.0, 0.0);
    for (i, &count) in weeks.iter().enumerate() {
        let x = left + i as f64 * step + 2.0;
        // Quiet weeks still get a small dark house, so the street never looks empty.
        let bh = rng.range(14.0, 34.0) + 165.0 * (count as f64 / max).sqrt();
        let y = ground - bh;
        if count as f64 >= busiest.2 {
            busiest = (i, x + bw / 2.0, count as f64);
        }
        write!(s, r##"<rect x="{x:.1}" y="{y:.1}" width="{bw:.1}" height="{bh:.1}" fill="#100c28" stroke="#2c2360" stroke-width=".8"/>"##)?;
        let cols = (((bw - 4.0) / 5.0).floor() as usize).max(1);
        let rows = ((bh - 8.0) / 7.0).floor().max(0.0) as usize;
        let mut slots: Vec<usize> = (0..cols * rows).collect();
        let lit = (count as usize).min(slots.len());
        for k in 0..lit {
            let j = k + (rng.next() * (slots.len() - k) as f64) as usize;
            slots.swap(k, j);
            let (col, row) = (slots[k] % cols, slots[k] / cols);
            let tint = match rng.next() {
                r if r < 0.08 => " t",
                r if r < 0.12 => " p",
                _ => "",
            };
            let flicker = if rng.next() < 0.15 { " f" } else { "" };
            write!(
                s,
                r#"<rect class="w{tint}{flicker}" x="{:.1}" y="{:.1}" width="3" height="3.5" style="animation-delay:{:.2}s"/>"#,
                x + 2.5 + col as f64 * 5.0,
                y + 6.0 + row as f64 * 7.0,
                0.6 + i as f64 * 0.05 + rng.range(0.0, 0.8)
            )?;
        }
    }

    // Street, power line and someone sitting on the wire above the busiest week.
    write!(s, r##"<rect y="{ground}" width="1200" height="40" fill="#05040f"/>"##)?;
    let (p0, p1, p2) = ((70.0, 150.0), (600.0, 235.0), (1130.0, 150.0));
    write!(s, r##"<g stroke="#05040f" fill="none"><path d="M{} {}Q{} {} {} {}" stroke-width="1.6"/><path d="M{} {}Q{} {} {} {}" stroke-width="1.2"/><path d="M70 {ground}V138M58 146H82M1130 {ground}V138M1118 146H1142" stroke-width="4"/></g>"##,
        p0.0, p0.1, p1.0, p1.1, p2.0, p2.1, p0.0, p0.1 + 8.0, p1.0, p1.1 + 8.0, p2.0, p2.1 + 8.0)?;
    let t = ((busiest.1 - p0.0) / (p2.0 - p0.0)).clamp(0.12, 0.88);
    let sit = (
        (1.0 - t) * (1.0 - t) * p0.0 + 2.0 * (1.0 - t) * t * p1.0 + t * t * p2.0,
        (1.0 - t) * (1.0 - t) * p0.1 + 2.0 * (1.0 - t) * t * p1.1 + t * t * p2.1,
    );
    write!(s, r##"<g transform="translate({:.1} {:.1}) scale(1.6)" fill="#05040f"><circle cy="-13" r="3"/><path d="M-2.6-15C-5.5-9-4.5-3-6 3L-1 0Z"><animateTransform attributeName="transform" type="rotate" values="0 -2 -13;-6 -2 -13;0 -2 -13" dur="5s" repeatCount="indefinite"/></path><path d="M-2.5-10H2.5L2.8 0H-2.5Z"/><path d="M-1-1L4 0L3.5 8H2L2.2 2L-1 1Z"/></g>"##, sit.0, sit.1)?;

    s += &fill(&format!(
        r#"<text class="fade" x="40" y="46" font-size="15" letter-spacing="6" fill="$LILAC">NIGHTS SPENT CODING</text>
<text class="fade" x="40" y="82" font-size="28" font-weight="300" letter-spacing="1" fill="$MOON" style="animation-delay:.4s">{total} contributions in the last year</text>
<text class="mono fade" x="1160" y="327" text-anchor="end" font-size="15" letter-spacing="1" fill="$LILAC" opacity=".75" style="animation-delay:1.2s">one building per week · busier week, more lights on</text>"#
    ));
    s += "</g>";
    s += &frame(w, h, 18.0);
    Ok(s + "</svg>")
}

// ---------------------------------------------------------------- languages

fn langs() -> Res<String> {
    // /user/repos includes private repos but needs a user token, which CI only has via PROFILE_TOKEN.
    let url = if env::var("PROFILE_TOKEN").is_ok() || env::var("CI").is_err() {
        "https://api.github.com/user/repos?affiliation=owner&per_page=100".to_string()
    } else {
        format!("https://api.github.com/users/{USER}/repos?per_page=100")
    };
    let repos = api(&url)?;
    let mut bytes: HashMap<String, u64> = HashMap::new();
    let mut counted = 0;
    for r in repos.as_array().ok_or("repo list is not an array")? {
        let name = r["name"].as_str().unwrap_or("");
        if r["fork"].as_bool() == Some(true) || r["archived"].as_bool() == Some(true) || SKIP_REPOS.contains(&name) {
            continue;
        }
        let Some(langs_url) = r["languages_url"].as_str() else { continue };
        if let Some(map) = api(langs_url)?.as_object() {
            for (lang, n) in map {
                *bytes.entry(lang.clone()).or_default() += n.as_u64().unwrap_or(0);
            }
            counted += 1;
        }
    }
    let sum = bytes.values().sum::<u64>().max(1) as f64;
    let mut list: Vec<(String, f64)> = bytes.into_iter().map(|(k, v)| (k, v as f64 / sum)).collect();
    list.sort_by(|a, b| b.1.total_cmp(&a.1));
    let other: f64 = list.iter().skip(7).map(|x| x.1).sum();
    list.truncate(7);
    if other > 0.0 {
        list.push(("Other".into(), other));
    }

    let (w, h) = (1200.0, 190.0);
    let mut rng = Rng(0x0001_a265);
    let mut s = open(w, h, 18.0);
    s += &fill(r##"<defs><linearGradient id="bg" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="#0b0924"/><stop offset="1" stop-color="#1d1340"/></linearGradient></defs>"##);
    s += &css(r#"
.seg{transform-box:fill-box;transform-origin:left;transform:scaleX(0);animation:grow 1.2s cubic-bezier(.2,.8,.2,1) forwards}
@keyframes grow{to{transform:scaleX(1)}}
"#);
    s += r#"<g clip-path="url(#clip)"><rect width="1200" height="190" fill="url(#bg)"/>"#;
    s += &stars(&mut rng, 50, (0.0, w), (0.0, h));
    s += &fill(&format!(
        r#"<text class="fade" x="40" y="48" font-size="15" letter-spacing="6" fill="$LILAC">WHAT I WRITE</text>
<text class="fade" x="1160" y="48" text-anchor="end" font-size="15" letter-spacing="1" fill="$LILAC" opacity=".7">by bytes across {counted} repositories</text>"#
    ));
    let (bx, bwid) = (40.0, 1120.0);
    let mut x = bx;
    for (i, (lang, share)) in list.iter().enumerate() {
        let seg = bwid * share;
        write!(s, r#"<rect class="seg" x="{x:.1}" y="74" width="{:.1}" height="10" fill="{}" style="animation-delay:{:.2}s"/>"#, (seg - 2.0).max(1.0), lang_color(lang), 0.3 + i as f64 * 0.12)?;
        x += seg;
    }
    let colw = bwid / list.len().max(1) as f64;
    for (i, (lang, share)) in list.iter().enumerate() {
        let lx = bx + i as f64 * colw;
        write!(
            s,
            r#"<g class="fade" style="animation-delay:{:.2}s"><circle cx="{:.1}" cy="127" r="6" fill="{}"/><text x="{:.1}" y="134" font-size="20" fill="{MOON}">{}</text><text class="mono" x="{:.1}" y="162" font-size="16" fill="{LILAC}">{:.1}%</text></g>"#,
            0.8 + i as f64 * 0.1,
            lx + 5.0,
            lang_color(lang),
            lx + 18.0,
            esc(lang),
            lx + 18.0,
            share * 100.0
        )?;
    }
    s += "</g>";
    s += &frame(w, h, 18.0);
    Ok(s + "</svg>")
}

fn lang_color(lang: &str) -> &'static str {
    match lang {
        "Rust" => MAGENTA,
        "TypeScript" => "#8c9bff",
        "Python" => TEAL,
        "JavaScript" => "#c9a7ff",
        "C++" => WARM,
        "Java" => "#ff9b71",
        "Swift" => "#ffb3c7",
        "C#" => "#7fd88f",
        _ => "#5b5585",
    }
}

// ---------------------------------------------------------------- shared pieces

fn open(w: f64, h: f64, rx: f64) -> String {
    format!(r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}"><defs><clipPath id="clip"><rect width="{w}" height="{h}" rx="{rx}"/></clipPath></defs>"#)
}

fn frame(w: f64, h: f64, rx: f64) -> String {
    format!(r#"<rect x=".5" y=".5" width="{}" height="{}" rx="{rx}" fill="none" stroke="{LILAC}" stroke-opacity=".18"/>"#, w - 1.0, h - 1.0)
}

fn glow_defs() -> String {
    fill(r#"<defs><radialGradient id="glow"><stop offset="0" stop-color="$MOON" stop-opacity=".5"/><stop offset="1" stop-color="$MOON" stop-opacity="0"/></radialGradient></defs>"#)
}

fn css(extra: &str) -> String {
    fill(&format!("<style>{BASE_CSS}{extra}</style>"))
}

fn fill(t: &str) -> String {
    t.replace("$INK", INK)
        .replace("$LILAC", LILAC)
        .replace("$MAGENTA", MAGENTA)
        .replace("$TEAL", TEAL)
        .replace("$MOON", MOON)
        .replace("$WARM", WARM)
        .replace("$SANS", SANS)
        .replace("$MONO", MONO)
}

fn stars(rng: &mut Rng, n: usize, xs: (f64, f64), ys: (f64, f64)) -> String {
    let mut s = String::new();
    for _ in 0..n {
        let _ = write!(
            s,
            r#"<circle class="tw" cx="{:.0}" cy="{:.0}" r="{:.1}" style="animation-delay:-{:.1}s;animation-duration:{:.1}s"/>"#,
            rng.range(xs.0, xs.1),
            rng.range(ys.0, ys.1),
            rng.range(0.5, 1.5),
            rng.range(0.0, 6.0),
            rng.range(3.0, 7.0)
        );
    }
    s
}

fn fireflies(rng: &mut Rng, n: usize, xs: (f64, f64), ys: (f64, f64)) -> String {
    let mut s = String::new();
    for _ in 0..n {
        let class = if rng.next() < 0.3 { "ff t" } else { "ff" };
        let _ = write!(
            s,
            r#"<circle class="{class}" cx="{:.0}" cy="{:.0}" r="{:.1}" style="animation-delay:-{:.1}s;animation-duration:{:.1}s"/>"#,
            rng.range(xs.0, xs.1),
            rng.range(ys.0, ys.1),
            rng.range(1.0, 2.2),
            rng.range(0.0, 12.0),
            rng.range(8.0, 14.0)
        );
    }
    s
}

/// Bats crossing the sky on gentle curves, flapping as they go.
fn bats(rng: &mut Rng, n: usize, w: f64, h: f64) -> String {
    let mut s = String::new();
    for i in 0..n {
        let y0 = rng.range(h * 0.15, h * 0.5);
        let dur = rng.range(16.0, 24.0);
        let begin = i as f64 * 6.0 + rng.range(1.0, 4.0);
        let scale = rng.range(0.6, 1.0);
        let path = format!("M-40 {y0:.0}C{:.0} {:.0} {:.0} {:.0} {:.0} {:.0}", w * 0.3, y0 - 40.0, w * 0.6, y0 + 40.0, w + 40.0, y0 - 20.0);
        let _ = write!(
            s,
            r#"<g opacity="0"><animate attributeName="opacity" values="0;1;1;0" keyTimes="0;.05;.95;1" dur="{dur:.1}s" begin="{begin:.1}s" repeatCount="indefinite"/><animateMotion path="{path}" dur="{dur:.1}s" begin="{begin:.1}s" repeatCount="indefinite"/><g transform="scale({scale:.2})"><path class="bat" d="{BAT}"><animateTransform attributeName="transform" type="scale" values="1 1;1 .3;1 1" dur=".35s" repeatCount="indefinite"/></path></g></g>"#
        );
    }
    s
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![String::new()];
    for word in text.split_whitespace() {
        let line = lines.last_mut().unwrap();
        if !line.is_empty() && line.chars().count() + 1 + word.chars().count() > width {
            lines.push(word.to_string());
        } else {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
    }
    lines
}

fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn jpg(rel: &str) -> Res<String> {
    let bytes = fs::read(root().join(rel)).map_err(|e| format!("{rel}: {e}"))?;
    Ok(format!("data:image/jpeg;base64,{}", base64::engine::general_purpose::STANDARD.encode(bytes)))
}

fn save(rel: &str, svg: &str) -> Res<()> {
    fs::write(root().join(rel), svg)?;
    println!("wrote {rel} ({} KB)", svg.len() / 1024);
    Ok(())
}

fn here() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn root() -> PathBuf {
    here().join("..")
}

// ---------------------------------------------------------------- GitHub API

fn token() -> Option<String> {
    ["PROFILE_TOKEN", "GITHUB_TOKEN"].iter().filter_map(|k| env::var(k).ok()).find(|t| !t.is_empty())
}

fn api(url: &str) -> Res<Value> {
    let mut req = ureq::get(url).set("User-Agent", "nightsky").set("Accept", "application/vnd.github+json");
    if let Some(t) = token() {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    Ok(req.call()?.into_json()?)
}

fn graphql(query: &str, variables: Value) -> Res<Value> {
    let t = token().ok_or("the GraphQL API needs PROFILE_TOKEN or GITHUB_TOKEN")?;
    let v: Value = ureq::post("https://api.github.com/graphql")
        .set("User-Agent", "nightsky")
        .set("Authorization", &format!("Bearer {t}"))
        .send_json(json!({ "query": query, "variables": variables }))?
        .into_json()?;
    if let Some(e) = v.get("errors") {
        return Err(format!("GraphQL: {e}").into());
    }
    Ok(v["data"].clone())
}

/// xorshift64: deterministic, so unchanged data renders an unchanged SVG.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }

    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}
