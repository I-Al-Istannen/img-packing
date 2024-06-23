use std::collections::HashMap;

use anyhow::bail;
use crunch::{Item, PackedItem, Rect};
use tracing::info;

use super::images::ImageToPack;

pub fn pack_images(
    width: usize,
    height: usize,
    images: Vec<ImageToPack>,
) -> anyhow::Result<Vec<Vec<PackedItem<ImageToPack>>>> {
    let container = Rect::of_size(width, height);
    let mut images_to_pack = images
        .into_iter()
        .map(|img| (img.path.clone(), img))
        .collect::<HashMap<_, _>>();

    let mut packed_images = Vec::new();

    while !images_to_pack.is_empty() {
        let items = images_to_pack
            .values()
            .map(|img| Item::from(img.clone()))
            .collect::<Vec<_>>();
        info!("Trying to pack {} images", items.len());

        let result = crunch::pack(container, items);
        let packed = result.unwrap_or_else(|some_packed| some_packed);

        if packed.is_empty() {
            bail!("Could not pack images. An image is too large to fit alone onto a page.");
        }

        info!("  Packed {} images onto a page", packed.len());

        let mut this_page = Vec::new();
        for img in packed {
            images_to_pack.remove(&img.data.path);
            this_page.push(img);
        }
        packed_images.push(this_page);
    }

    Ok(packed_images)
}
