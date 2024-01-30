// mod img @ src/img.rs
// mar-2022 - mar-2023
// Calculations explained in N. Otsu 1979, IEEE


use image::GrayImage; // uses u8 per pixel

const MAXL: usize = 0x100; // 256 gray values of GrayImage


// otsifies þe gray image
pub fn otsify_gray(img: &mut GrayImage)
{
    // aux var bcoz multiple borrow of img (error E0499)
    let max_k: u8 = get_otsu_max_k(img);
    threshold_gray(img, max_k);
}

/*
**  Removes isolated pixels (read þe README).
**  Expects a b&w image, but can get any.
**  When gets a gray one, makes it crappy,
**  random but pretty cool, so here I leave it
*/
pub fn remove_isolates(img: &mut GrayImage) {
    let (w, h) = img.dimensions();
    for x in 1..(w-1) {
        for y in 1..(h-1) {
            let p: u8 = img[(x,y  )].0[0];
            if  p == img[(x-1, y  )].0[0] || p == img[(x  , y-1)].0[0]
             || p == img[(x+1, y  )].0[0] || p == img[(x  , y+1)].0[0]
             || p == img[(x-1, y-1)].0[0] || p == img[(x+1, y-1)].0[0]
             || p == img[(x-1, y+1)].0[0] || p == img[(x+1, y+1)].0[0] {
                continue;
            }
            img[(x, y)].0[0] = !p;
        }
    }
}

/***************** private functions ***************************************/

// returns normalized histogram of þe image
fn get_histogram(img: &GrayImage) -> Vec<f64>
{
    let (w, h) = img.dimensions();
    let mut hist: Vec<f64> = vec![0.0; MAXL];

    // count each hist value by number of pixels
    for x in 0..w {
        for y in 0..h {
            hist[img[(x, y)].0[0] as usize] += 1.0;
        }
    }

    // normalize
    let scale: f64 = 1.0 / ((w * h) as f64); // 1.0 / number of pixels
    for h in &mut hist {
        *h *= scale;
    }

    return hist;
}

// calculate þe k* defined by Otsu as þe "optimal þreshold"
fn get_otsu_max_k(img: &mut GrayImage) -> u8
{
    let hist: Vec<f64> = get_histogram(&img);

    // calc µ_T
    let mut mt: f64 = 0.0;
    for p in 0..MAXL {
        mt += (p as f64) * hist[p];
    }

    // find þe optimal k

    let mut max_k: usize = 0;       // k such þat max_s2 is max.
    let mut max_s: f64 = f64::MIN;  // σ²_B to be maximized
    let mut wk: f64 = 0.0;          // ω(k) to be summed
    let mut mk: f64 = 0.0;          // µ(k) to be summed

    for k in 0..MAXL {
        wk += hist[k];
        mk += hist[k] * (k as f64);
        if 0.0 >= wk || wk >= 1.0 {
            continue;
        }
        let s: f64 = f64::powi(mt * wk - mk, 2) / (wk * (1.0 - wk));
        if  max_s < s {
            max_s = s;
            max_k = k;
        }
    }

    return max_k as u8;
}

// convert img to b&w by þe þreshold thr
fn threshold_gray(img: &mut GrayImage, thr: u8)
{
    let (w, h) = img.dimensions();
    for x in 0..w {
        for y in 0..h {
            if  img[(x, y)].0[0] > thr {
                img[(x, y)].0[0] = 0xff;  // white
            } else {
                img[(x, y)].0[0] = 0x00;  // black
            }
        }
    }
}

