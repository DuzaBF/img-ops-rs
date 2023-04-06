use crate::img_ops::ImageOp;
pub struct Median {}

impl ImageOp for Median {
    type Image = image::GrayImage;
    fn apply(self: &Self, in_img: &Self::Image) -> Self::Image
    {
        println!("Apply Median");
        let (width, height) = in_img.dimensions();
        let tmp: Self::Image = Self::Image::new(width, height);
        tmp
    }
}