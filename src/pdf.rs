use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crunch::PackedItem;
use printpdf::{Image, ImageTransform, Mm, PdfDocument, PdfLayerReference, Px};
use printpdf::image_crate::{DynamicImage, GenericImage, GenericImageView};
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
        image = remove_alpha_channel(image);
        let mut image = Image::from_dynamic_image(&image);
        // Clear this as well, as it seems to make images unreadable in some viewers
        image.image.smask = None;

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

/// Removes the alpha channel from an image by blending it with a white background.
fn remove_alpha_channel(image: DynamicImage) -> DynamicImage {
    if let Some(rgba) = image.as_rgba8() {
        let mut new_image = DynamicImage::new_rgb8(rgba.width(), rgba.height());
        for (x, y, pixel) in rgba.enumerate_pixels() {
            let rgb = rgba_to_rgb(&pixel.0);
            new_image.put_pixel(x, y, [rgb[0], rgb[1], rgb[2], 255].into());
        }
        return new_image;
    }
    image
}

fn rgba_to_rgb(rgba: &[u8]) -> [u8; 3] {
    // Blend with white background
    let [red, green, blue, alpha]: [u8; 4] = rgba.try_into().ok().unwrap();
    let alpha = alpha as f64 / 255.0;
    let new_red = ((1.0 - alpha) * 255.0 + alpha * red as f64) as u8;
    let new_green = ((1.0 - alpha) * 255.0 + alpha * green as f64) as u8;
    let new_blue = ((1.0 - alpha) * 255.0 + alpha * blue as f64) as u8;
    [new_red, new_green, new_blue]
}
