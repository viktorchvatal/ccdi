use nanocv::{ImgBuf, Img, ImgMut};
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

    pub fn red_mut(&mut self) -> &mut dyn ImgMut<T> {
        &mut self.r
    }

    pub fn green_mut(&mut self) -> &mut dyn ImgMut<T> {
        &mut self.g
    }

    pub fn blue_mut(&mut self) -> &mut dyn ImgMut<T> {
        &mut self.b
    }

    pub fn channels_mut(&mut self) -> [&mut dyn ImgMut<T>; 3] {
        [&mut self.r, &mut self.g, &mut self.b]
    }
}
