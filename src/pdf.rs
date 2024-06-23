use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crunch::PackedItem;
use printpdf::{Image, ImageTransform, Mm, PdfDocument, PdfLayerReference, Px};
use printpdf::image_crate::GenericImageView;
use tracing::info;

use crate::images::ImageToPack;

pub fn write_pdf(
    path: &Path,
    images: Vec<Vec<PackedItem<ImageToPack>>>,
    width: Mm,
    height: Mm,
    border: Mm,
    dpi: u32,
) -> anyhow::Result<()> {
    let (doc, p1, l1) = PdfDocument::new("Amalgamation", width, height, "Root");
    let mut current_layer = doc.get_page(p1).get_layer(l1);

    for (index, page) in images.iter().enumerate() {
        info!("Writing page {}", index + 1);
        place_images(&current_layer, page, border, dpi)?;
        if index + 1 < images.len() {
            let (p, l) = doc.add_page(width, height, "Root");
            current_layer = doc.get_page(p).get_layer(l);
        }
    }

    info!("Saving PDF. This might take a while...");
    doc.save(&mut BufWriter::new(File::create(path)?))?;

    Ok(())
}

fn place_images(
    layer: &PdfLayerReference,
    images: &[PackedItem<ImageToPack>],
    border: Mm,
    dpi: u32,
) -> anyhow::Result<()> {
    let dpi = dpi as f32;

    for item in images {
        let margin = item.data.margin as usize;
        let mut image = item.data.get_image()?;
        let rotated = was_rotated(image.dimensions(), &item.rect);

        info!(
            "  Writing image {:?} ({})",
            item.data.path.file_name().unwrap(),
            if rotated { "rotated" } else { "not rotated" }
        );

        if rotated {
            image = image.rotate90();
        }
        let image = Image::from_dynamic_image(&image);

        let transform = ImageTransform {
            translate_x: Some(Mm::from(Px(item.rect.x + margin).into_pt(dpi)) + border),
            translate_y: Some(Mm::from(Px(item.rect.y + margin).into_pt(dpi)) + border),
            scale_x: None,
            scale_y: None,
            rotate: None,
            dpi: Some(dpi),
        };

        image.add_to_layer(layer.clone(), transform);
    }

    Ok(())
}

fn was_rotated((width, height): (u32, u32), rect: &crunch::Rect) -> bool {
    let width_was_larger = width > height;
    width_was_larger && rect.w <= rect.h
}
