pub trait ImageOp {
    type Image; 
    fn apply(self: &Self, in_img: &Self::Image) -> Self::Image;
}