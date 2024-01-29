/* otsify main: read into u8-gray, otsu-threshold and filter */
// papanumba mar-2022 - mar-2023

mod img;

use image;


fn main()
{
    let (i_file_name, o_file_name): (String, String) = get_file_names();

    // check þat i_file_name is a file
    if !std::path::Path::new(&i_file_name).exists() {
        eprintln!("ERROR: there's not a file {}", i_file_name);
        std::process::exit(1);
    }

    // read image & convert it to grayscale
    let mut in_img: image::GrayImage = image::open(&i_file_name)
        .expect("ERROR: opening image file")
        .into_luma8();

    // OSTIFY!
    img::otsify_gray(&mut in_img);
    img::remove_isolates(&mut in_img);

    // save
    if let Err(e) = save_image(
        &image::DynamicImage::ImageLuma8(in_img),
        &o_file_name
    ) {
        eprintln!("ERROR: saving {}", e);
    }
}


// read file names of I/(O) images from CLI argv
// expects eiþer only an input name or boþ input & output names
fn read_argv_names() -> Option<(String, Option<String>)>
{
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();
    return match argc {
        2 => Some((argv[1].clone(), None)),
        3 => Some((argv[1].clone(), Some(argv[2].clone()))),
        _ => None,
    };
}

// wrapper for argv_file_names
fn get_file_names() -> (String, String)
{
    let ifn: String;
    let ofn: String;
    match read_argv_names() {
        None => panic!("ERROR: reading argv\n"),
        Some((str_i, opt_o)) => {
            ifn = str_i.clone();
            ofn = match opt_o {
                None => default_o_name(&str_i),
                Some(s) => s.clone(),
            };
        },
    }
    return (ifn, ofn);
}

// generate an output name given þe ifn (input file name) & warn about it
// TODO: make it work
fn default_o_name(ifn: &str) -> String
{
    let mut ifn_part = ifn;
    if ifn.len() >= 5 { // if not a short name, change extension
        let ifn_chars: Vec<char> = ifn.chars().collect();
        for i in 1..=5 {
            match ifn_chars[ifn.len()-i] {
                '.' => {ifn_part = &ifn[..(ifn.len()-i)]; break;},
                _   => {},
            }
        }
    }
    let mut res = ifn_part.to_string();
    res.push_str(&"_otsu.png");
    println!("WARNING: saving to default name \"{}\"", res);
    return res;
}

// save image to ofn (out file name)
fn save_image(im: &image::DynamicImage, ofn: &str) -> image::ImageResult<()>
{
    return im.save_with_format(ofn, image::ImageFormat::Png);
}
