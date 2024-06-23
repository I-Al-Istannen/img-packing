use std::fmt::Debug;
use std::path::PathBuf;

use crunch::{Item, Rotation};
use printpdf::image_crate::imageops::FilterType;
use printpdf::image_crate::io::Reader as ImageReader;
use printpdf::image_crate::{DynamicImage, GenericImageView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageToPack {
    pub path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub margin: u32,
}
impl ImageToPack {
    pub fn from_path(
        path: PathBuf,
        max_width: Option<u32>,
        max_height: Option<u32>,
        margin: u32,
    ) -> anyhow::Result<Self> {
        let mut dynamic_image = ImageReader::open(&path)?.decode()?;

        let (width, height) = dynamic_image.dimensions();
        if max_width.is_some() || max_height.is_some() {
            dynamic_image = dynamic_image.resize(
                max_width.unwrap_or(width),
                max_height.unwrap_or(height),
                FilterType::Nearest,
            );
        }

        let (width, height) = dynamic_image.dimensions();
        Ok(Self {
            path,
            width: width + 2 * margin,
            height: height + 2 * margin,
            margin,
        })
    }

    pub fn get_image(&self) -> anyhow::Result<DynamicImage> {
        let image = ImageReader::open(&self.path)?.decode()?;
        Ok(image.resize(
            self.width - 2 * self.margin,
            self.height - 2 * self.margin,
            FilterType::Lanczos3,
        ))
    }
}

impl From<ImageToPack> for Item<ImageToPack> {
    fn from(value: ImageToPack) -> Self {
        let width = value.width as usize;
        let height = value.height as usize;
        Self::new(value, width, height, Rotation::Allowed)
    }
}
