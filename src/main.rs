use std::path::PathBuf;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor;
use clap::Parser;
use crunch::PackedItem;
use printpdf::Mm;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use self::images::ImageToPack;
use self::sizes::parse_mm;

mod images;
mod packing;
mod pdf;
mod sizes;

const CLAP_STYLE: Styles = Styles::styled()
    .header(AnsiColor::Red.on_default().bold())
    .usage(AnsiColor::Red.on_default().bold())
    .literal(AnsiColor::Blue.on_default().bold())
    .placeholder(AnsiColor::Green.on_default());

/// A simple program to pack images into a PDF.
#[derive(Debug, Parser)]
#[command(version, about, long_about = None, styles = CLAP_STYLE)]
pub struct Arguments {
    /// The images to pack. Can be a single image or a folder containing *only* images.
    #[clap(required = true)]
    images: Vec<PathBuf>,

    /// The DPI to render the images at
    #[arg(long, value_name = "dpi", default_value = "300")]
    dpi: u32,

    /// The width of the paper in mm
    #[arg(long = "width", value_name = "width", default_value = "210.0", value_parser = parse_mm)]
    paper_width_mm: Mm,

    /// The height of the paper in mm
    #[arg(long = "height", value_name = "height", default_value = "297.0", value_parser = parse_mm)]
    paper_height_mm: Mm,

    /// The border width in mm
    #[arg(long = "border", value_name = "width", default_value = "3.0", value_parser = parse_mm)]
    border_mm: Mm,

    /// The margin width in mm
    #[arg(long = "margin", value_name = "width", default_value = "1.0", value_parser = parse_mm)]
    margin_mm: Mm,

    /// The maximum width an image may have in mm
    #[arg(long = "max-image-width", value_name = "width", value_parser = parse_mm)]
    max_image_width: Option<Mm>,

    /// The maximum height an image may have in mm
    #[arg(long = "max-image-height", value_name = "height", value_parser = parse_mm)]
    max_image_height: Option<Mm>,
}

fn main() -> anyhow::Result<()> {
    if let Err(e) = actual_main() {
        error!("An error occurred. Please nag the author.");
        error!("Error message: '{}'", e);
        std::process::exit(1);
    }
    Ok(())
}

fn actual_main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();
    let arguments = Arguments::parse();

    let dpi = arguments.dpi;
    let border_width_px = sizes::to_px(arguments.border_mm, dpi).0;
    // Margin is halved because it is applied on all images
    let margin_px = sizes::to_px(arguments.margin_mm, dpi).0 as u32 / 2;
    let width_px = sizes::to_px(arguments.paper_width_mm, dpi).0 - 2 * border_width_px;
    let height_px = sizes::to_px(arguments.paper_height_mm, dpi).0 - 2 * border_width_px;
    let max_width_px = arguments
        .max_image_width
        .map(|mm| sizes::to_px(mm, dpi).0 as u32);
    let max_height_px = arguments
        .max_image_height
        .map(|mm| sizes::to_px(mm, dpi).0 as u32);

    let images = arguments
        .images
        .into_iter()
        .map(|path| {
            ImageToPack::from_file_or_folder(path, max_width_px, max_height_px, margin_px)
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    info!("Paper size: {}x{}px @ {} DPI", width_px, height_px, dpi);
    let packed = packing::pack_images(width_px, height_px, images)?;
    info!("");
    print_assignments(&packed);

    info!("Writing PDF");
    pdf::write_pdf(
        packed,
        arguments.paper_width_mm,
        arguments.paper_height_mm,
        arguments.border_mm,
        dpi,
    );

    info!("");
    info!("Done. Have a nice day.");

    Ok(())
}

fn print_assignments(packed: &[Vec<PackedItem<ImageToPack>>]) {
    for (index, page) in packed.iter().enumerate() {
        info!("Page {}", index + 1);
        for item in page {
            info!(
                "  {:?} at {:?}",
                item.data.path.file_name().unwrap(),
                item.rect
            );
        }
        info!("");
    }
}
