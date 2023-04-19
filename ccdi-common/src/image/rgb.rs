use nanocv::{ImgBuf, Img};
use serde_derive::{Serialize, Deserialize};

// ============================================ PUBLIC =============================================

/// RGB with separate red, green, blue channels
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct RgbImage<T> {
    r: ImgBuf<T>,
    g: ImgBuf<T>,
    b: ImgBuf<T>,
}

impl<T> RgbImage<T> {
    pub fn from(
        r: ImgBuf<T>,
        g: ImgBuf<T>,
        b: ImgBuf<T>,
    ) -> Result<Self, String> {
        if (r.dimensions() != b.dimensions()) || (b.dimensions() != g.dimensions()) {
            return Err(
                format!(
                    "Image channels do not have same dimensions: r: {:?}, g: {:?}, b: {:?}",
                    r.dimensions(), g.dimensions(), b.dimensions()
                )
            )
        }

        Ok(Self{r, g, b})
    }

    pub fn width(&self) -> usize {
        self.r.width()
    }

    pub fn height(&self) -> usize {
        self.r.height()
    }

    pub fn red(&self) -> &dyn Img<T> {
        &self.r
    }

    pub fn green(&self) -> &dyn Img<T> {
        &self.g
    }

    pub fn blue(&self) -> &dyn Img<T> {
        &self.b
    }
}