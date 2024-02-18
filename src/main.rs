/* otsify main: read into u8-gray, otsu-threshold and filter */
// papanumba mar-2022 - mar-2023

mod cli;
mod img;

use image;

fn main()
{
    let wtd = match cli::WhatToDo::read_args() {
        Ok(w) => w,
        Err(e) => cli::silent_panic!("ERROR: {e}"),
    };

    let x = match wtd {
        cli::WhatToDo::Help => return cli::print_help(),
        cli::WhatToDo::Otsu(o) => o,
    };

    // read image & convert it to grayscale
    let mut in_img: image::GrayImage = match image::open(&x.infile) {
        Ok(i) => i.into_luma8(),
        Err(ie) => cli::silent_panic!("ERROR: opening image: {ie}"),
    };

    // OSTIFY!
    img::otsify_gray(&mut in_img);
    if x.i_flag {
        img::remove_isolates(&mut in_img);
    }

    // save
    match save_image(
        &image::DynamicImage::ImageLuma8(in_img),
        &x.oufile
    ) {
        Ok(_) => if !x.q_flag {
            if x.is_dft {
                println!("WARNING: saving to default name");
            }
            println!("INFO: Saved image successfully \"{}\"", x.oufile)
        },
        Err(e) => cli::silent_panic!("ERROR: saving {}", e),
    }
}

// save image to ofn (out file name)
fn save_image(im: &image::DynamicImage, ofn: &str) -> image::ImageResult<()>
{
    return im.save_with_format(ofn, image::ImageFormat::Png);
}
