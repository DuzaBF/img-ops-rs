use image::{GenericImageView, Luma, Pixel};

use crate::img_ops::ImageOp;
pub struct Median {
    mask_width: u32,
    mask_height: u32,
}

impl Median {
    pub fn new(w: u32, h: u32) -> Median {
        Median {
            mask_width: w,
            mask_height: h,
        }
    }
}

impl ImageOp for Median {
    type Image = image::GrayImage;

    fn apply(self: &Self, in_img: &Self::Image) -> Self::Image {
        println!(
            "Apply Median with {} by {} mask",
            self.mask_width, self.mask_height
        );
        let (width, height) = in_img.dimensions();
        let mut tmp: Self::Image = Self::Image::new(width, height);
        let h_start = 0;
        let h_end = height;
        let w_start = 0;
        let w_end = width;
        for h in h_start..h_end {
            for w in w_start..w_end {
                let patch = in_img.view(w, h, self.mask_width, self.mask_height);
                let mut patch_vec = vec![0u8; usize::try_from(self.mask_height*self.mask_width).ok().unwrap()];
                for i in 0..self.mask_width {
                    for j in 0..self.mask_height {
                        let pixel = patch.get_pixel(i, j);
                        let subpix = pixel.channels()[0];
                        let val = u8::from(subpix);
                        patch_vec[(i*self.mask_width + j) as usize] = val;
                    }
                }
            }
        }

        return tmp;
    }

    fn convert(self: &Self, in_img: image::DynamicImage) -> Self::Image {
        in_img.into_luma8()
    }
}
