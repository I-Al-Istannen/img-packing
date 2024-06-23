use printpdf::{Mm, Px};

pub fn parse_mm(s: &str) -> Result<Mm, String> {
    Ok(Mm(s.parse::<f32>().map_err(|e| e.to_string())?))
}

const MM_PER_INCH: f32 = 25.4;

pub fn to_px(val: Mm, dpi: u32) -> Px {
    Px((val.0 / MM_PER_INCH * dpi as f32).round() as usize)
}
