# Otsify
**Otsify** (pron. \[ˈo̞ː.tsɪ.faɪ\]) is a simple CLI tool for otsifying images,
(just kidding) i.e. converting any image into black&white.

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
helpfull on those kind of images.

## TODO
Brainstorm of new features:
* `--colored` or `--rgb` or `-c`: this flag would otsify each RGB channel individually.
* `--isolates` or `-i`: the filtering isolates as an optional flag.

## Installation
Download the binary which suits your operating system & run it from the command
line. Otherwise, þou canst build it from source.

## Usage
Pass as CLI arguments the names of the input (I) and output (O) image files.
* `./otsify I O` will read I, then otsify it & finally save it in O.
* `./otsify I` will read I, otsify it & save it in a default name made from I.

## License
This software is licensed under the GPLv3.

