pub trait ImageOp {
    type Image;
    fn apply(self: &Self, in_img: &Self::Image) -> Self::Image;
    fn convert(self: &Self, in_img: image::DynamicImage) -> Self::Image;
}
