# Otsify
**Otsify** (pron. \[ˈoː.tsɪ.faɪ\]) is a simple CLI tool for otsifying images,
just kidding i.e. converting any image into black&white.

## Etymology
The name is a portmanteau of the surname *Ōtsu* (see **Description**) and the
suffix *-ify* (as in *simple* (adj) > *simplify* (v)).
Grammatically, it is a verb, so it can be used as in "Wait a second! I haven't
otsified this image yet.".

## Description
The program uses [Otsu's method](https://en.wikipedia.org/wiki/Otsu%27s_method)
so as to threshold an image automatically.
Afterwards, it also filters the "isolated" pixels, thus visually explained:

```
# = black
· = white

# # #     # # #
# · # ==> # # #
# # #     # # #

· · ·     · · ·
· # · ==> · · ·
· · ·     · · ·
```

This helps shrink the output image file size, because of how the `.png` format
works. Otsu's method leaves many of such pixels on images that either have a lot
of noise or don't have a strongly bimodal gray histogram. So it is particularly
helpful on those kinds of images.

## TODO
Brainstorm of new features:
* `--colored` or `--rgb` or `-c`: this flag would otsify each RGB channel individually.
* `--isolates` or `-i`: the filtering isolates as an optional flag.
* quiet/verbose flags

## Installation
Download the binary which suits your operating system & run it from the command
line. Otherwise, þou canst build it from source.

## Usage
Pass as CLI arguments the names of the input and optional output image files.
If no output name is passed, the default is generated from the input, e.g.
from `example.jpg` will result to `example_otsu.png`. Output files must be png.

* Windows: `otsify.exe example.jpg output.png`
* Linux: `./otsify example.jpg output.png`

## License
This software is licensed under the GPLv3.
