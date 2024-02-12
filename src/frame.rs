use gemini_engine::elements::{
    view::{ColChar, Modifier, View, Wrapping},
    PixelContainer, Vec2D,
};
use image::{imageops::FilterType, DynamicImage};

#[must_use] pub fn resized_img_and_size(
    img: DynamicImage,
    width: u32,
    height: u32,
) -> (DynamicImage, usize, usize) {
    let resized_img = img.resize(width, height, FilterType::Nearest);
    let width = resized_img.width() as usize;
    let height = resized_img.height() as usize;

    (resized_img, width, height)
}

pub fn blit_image_to(view: &mut View, img: DynamicImage, pixel_char: char, wrapping: Wrapping) {
    for (i, pixel) in img.as_rgb8().unwrap().pixels().enumerate() {
        let pos = Vec2D::new(
            i.rem_euclid(img.width() as usize) as isize * 2,
            (i / img.width() as usize) as isize,
        );
        let colour = ColChar::new(
            pixel_char,
            Modifier::from_rgb(pixel.0[0], pixel.0[1], pixel.0[2]),
        );

        let wide_pixel =
            PixelContainer::from(&vec![(pos, colour), (pos + Vec2D::new(1, 0), colour)][..]);

        view.blit(&wide_pixel, wrapping);
    }
}
