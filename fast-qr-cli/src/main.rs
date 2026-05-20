//! Command-line QR encoder built on `fast-qr-reworked`.

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{anyhow, Context, Result};
use clap::{Parser, ValueEnum};
use fast_qr_reworked::convert::svg::SvgBuilder;
use fast_qr_reworked::convert::Builder;
use fast_qr_reworked::{Mode, QRBuilder, QRCode, Version, ECL};

#[derive(Parser, Debug)]
#[command(
    name = "fast-qr-cli",
    about = "Encode text or bytes into a QR code (SVG, PNG, or terminal).",
    version
)]
struct Cli {
    /// Data to encode. Mutually exclusive with `--input`.
    data: Option<String>,

    /// Read raw bytes from a file. Use `-` to read from stdin.
    #[arg(short, long, value_name = "PATH")]
    input: Option<PathBuf>,

    /// Output file. Defaults to stdout.
    #[arg(short, long, value_name = "PATH")]
    output: Option<PathBuf>,

    /// Output format. Inferred from the `--output` extension when omitted; otherwise `svg`.
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Error correction level.
    #[arg(long, value_enum, ignore_case = true, default_value_t = CliEcl::M)]
    ecl: CliEcl,

    /// Quiet-zone margin in module units (SVG and PNG only).
    #[arg(long, default_value_t = 4)]
    margin: usize,

    /// Force Byte mode (skip alphanumeric/numeric auto-detection).
    #[arg(long)]
    byte_mode: bool,

    /// Force a specific QR version (1-40). Default: auto-selected for the data.
    #[arg(long, value_name = "N")]
    qr_version: Option<u8>,

    /// Pixels per QR module (PNG only).
    #[arg(long, default_value_t = 8)]
    scale: usize,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Format {
    Svg,
    Png,
    Terminal,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum CliEcl {
    #[value(name = "L")]
    L,
    #[value(name = "M")]
    M,
    #[value(name = "Q")]
    Q,
    #[value(name = "H")]
    H,
}

impl From<CliEcl> for ECL {
    fn from(e: CliEcl) -> ECL {
        match e {
            CliEcl::L => ECL::L,
            CliEcl::M => ECL::M,
            CliEcl::Q => ECL::Q,
            CliEcl::H => ECL::H,
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let data = load_data(&cli)?;
    let qr = build_qr(&data, &cli)?;
    let format = resolve_format(cli.format, cli.output.as_deref());
    let bytes = render(&qr, format, cli.margin, cli.scale)?;
    write_out(cli.output.as_deref(), &bytes)
}

fn load_data(cli: &Cli) -> Result<Vec<u8>> {
    match (&cli.data, &cli.input) {
        (Some(_), Some(_)) => Err(anyhow!(
            "provide data as a positional argument OR via --input, not both"
        )),
        (None, None) => Err(anyhow!(
            "no data: pass data as a positional argument or use --input <PATH|->"
        )),
        (Some(s), None) => Ok(s.clone().into_bytes()),
        (None, Some(path)) => {
            if path.as_os_str() == "-" {
                let mut buf = Vec::new();
                io::stdin()
                    .read_to_end(&mut buf)
                    .context("failed to read data from stdin")?;
                Ok(buf)
            } else {
                std::fs::read(path).with_context(|| format!("failed to read {}", path.display()))
            }
        }
    }
}

fn build_qr(data: &[u8], cli: &Cli) -> Result<QRCode> {
    let mut builder = QRBuilder::new(data.to_vec());
    builder.ecl(cli.ecl.into());
    if cli.byte_mode {
        builder.mode(Mode::Byte);
    }
    if let Some(v) = cli.qr_version {
        if !(1..=40).contains(&v) {
            return Err(anyhow!("--qr-version must be in 1..=40"));
        }
        builder.version(version_from_u8(v));
    }
    builder
        .build()
        .map_err(|e| anyhow!("QR encoding failed: {e}"))
}

fn resolve_format(explicit: Option<Format>, output: Option<&Path>) -> Format {
    if let Some(f) = explicit {
        return f;
    }
    output
        .and_then(|p| p.extension())
        .and_then(|e| e.to_str())
        .and_then(|ext| match ext.to_ascii_lowercase().as_str() {
            "png" => Some(Format::Png),
            "svg" => Some(Format::Svg),
            _ => None,
        })
        .unwrap_or(Format::Svg)
}

fn render(qr: &QRCode, format: Format, margin: usize, scale: usize) -> Result<Vec<u8>> {
    match format {
        Format::Svg => Ok(SvgBuilder::default().margin(margin).to_str(qr).into_bytes()),
        Format::Terminal => Ok(qr.to_str().into_bytes()),
        Format::Png => render_png(qr, margin, scale),
    }
}

fn render_png(qr: &QRCode, margin: usize, scale: usize) -> Result<Vec<u8>> {
    if scale == 0 {
        return Err(anyhow!("--scale must be greater than 0"));
    }
    let qr_size = qr.size;
    let full_modules = qr_size + 2 * margin;
    let side = full_modules
        .checked_mul(scale)
        .ok_or_else(|| anyhow!("PNG dimensions overflow"))?;
    let pixel_count = side
        .checked_mul(side)
        .ok_or_else(|| anyhow!("PNG dimensions overflow"))?;

    let mut pixels = vec![0xFFu8; pixel_count];

    for y in 0..qr_size {
        for x in 0..qr_size {
            if !qr.data[y * qr_size + x].value() {
                continue;
            }
            let start_y = (margin + y) * scale;
            let start_x = (margin + x) * scale;
            for dy in 0..scale {
                let row_start = (start_y + dy) * side + start_x;
                pixels[row_start..row_start + scale].fill(0);
            }
        }
    }

    let side_u32 = u32::try_from(side).map_err(|_| anyhow!("PNG side exceeds u32"))?;
    let mut buf = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut buf, side_u32, side_u32);
        encoder.set_color(png::ColorType::Grayscale);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().context("PNG header write failed")?;
        writer
            .write_image_data(&pixels)
            .context("PNG pixel write failed")?;
    }
    Ok(buf)
}

fn write_out(path: Option<&Path>, bytes: &[u8]) -> Result<()> {
    match path {
        Some(p) => {
            std::fs::write(p, bytes).with_context(|| format!("failed to write {}", p.display()))
        }
        None => {
            let mut out = io::stdout().lock();
            out.write_all(bytes).context("failed to write to stdout")?;
            out.flush().context("failed to flush stdout")
        }
    }
}

const VERSIONS: [Version; 40] = [
    Version::V01,
    Version::V02,
    Version::V03,
    Version::V04,
    Version::V05,
    Version::V06,
    Version::V07,
    Version::V08,
    Version::V09,
    Version::V10,
    Version::V11,
    Version::V12,
    Version::V13,
    Version::V14,
    Version::V15,
    Version::V16,
    Version::V17,
    Version::V18,
    Version::V19,
    Version::V20,
    Version::V21,
    Version::V22,
    Version::V23,
    Version::V24,
    Version::V25,
    Version::V26,
    Version::V27,
    Version::V28,
    Version::V29,
    Version::V30,
    Version::V31,
    Version::V32,
    Version::V33,
    Version::V34,
    Version::V35,
    Version::V36,
    Version::V37,
    Version::V38,
    Version::V39,
    Version::V40,
];

fn version_from_u8(v: u8) -> Version {
    VERSIONS[(v - 1) as usize]
}
