# print-prep

[![Build Status](https://travis-ci.com/mlange-42/print-prep.svg?branch=master)](https://travis-ci.com/mlange-42/print-prep)

Command line tool for preparing photos for printing, and other bulk image operations.

* **[Download binaries](https://github.com/mlange-42/print-prep/releases/)**
* [Sources on GitHub](https://github.com/mlange-42/print-prep)

**Content**
* [Features](#features)
* [Installation](#installation)
* [Getting started](#getting-started)
* [Examples](#examples)
* [Commands](#commands)
* [Library / crate](#library--crate)
* [License](#license)

## Features

* Bulk image processing from the command line (Linux, Windows, OSX)
* Bulk-prepare images for printing, with:
  * Exactly predictable size and resolution,
  * Cut marks and 'mat' framing,
  * EXIF information (date, camera settings, GPS, ...),
  * Print control element 
* Convenient syntax with arbitrary mixing of length units (*cm*, *mm*, *in*, *px*)
* Automatically determines exact sizes for approximate *cm* formats (e.g., 10x15 is actually *10.2 cm x 15.2 cm*)
* More image processing operations to come...

## Installation

* Download the [latest binaries](https://github.com/mlange-42/print-prep/releases/).
* Unzip somewhere with write privileges (only required for running examples in place).

## Getting started

* See section [Examples](#examples) below, and try the batch files in sub-directory [/cmd_examples](https://github.com/mlange-42/print-prep/tree/master/cmd_examples). 
* For a full list of options, see section [Commands](#commands) or run `pprep -h`. Run `pprep --help` for a more comprehensive help message.
* Run `pprep <subcommand> -h` or `pprep <subcommand> --help` for information on a particular command.

## Examples

A simple example for preparing prints with 'mats' (padding) and cut marks:

```
pprep ^
  --input "test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "test_data/out/*-marks.png" ^
    --format 10cm/15cm ^
    --padding 5mm ^
    --margins 5mm ^
    --cut-marks ./1mm ^
    --dpi 90
```

> _Note 1:_ The ^ at the end of each line is required for breaking commands into multiple lines (at least on Windows).

> _Note 2:_ On Unix systems, the input pattern(s), as well as the output pattern with placeholder * **MUST be quoted**!.

Resulting in something like this:

<!-- ![Simple print preparation example](https://user-images.githubusercontent.com/44003176/80386704-0bad2000-88a8-11ea-85ed-81c40b471d6e.png)  -->
![Simple print preparation example](https://user-images.githubusercontent.com/44003176/80541201-0684c980-89ab-11ea-85f0-59d7c11c0a01.png)  
_Simple print preparation example._

Further, we can add a print control element and some EXIF information to the image:

```
..\target\release\pprep ^
  --input "../test_data/*.jpg" ^
  --cmd ^
  prep ^
    --output "../test_data/out/*-exif.png" ^
    --format 10cm/15cm ^
    --padding 5mm ^
    --margins 5mm ^
    --cut-marks ./1mm ^
    --exif "{F/2}, {Exp}, ISO {ISO}, {F}" ^
    --test-pattern 15px/3px ^
    --dpi 150
```

Note the two new lines above `--dpi 150`! We get this:

![Print preparation example with EXIF info and control element](https://user-images.githubusercontent.com/44003176/80541712-f4eff180-89ab-11ea-9888-8791f99a9b94.png)  
_Print preparation example with EXIF info and control element._

## Commands

Common options of all sub-commands are listed here.
For options of specific subcommands, see:

**[`prep`](#prep) &nbsp; [`scale`](#scale) &nbsp; [`list`](#list)**

Common options:
```
Command-line tool for photo print preparation and other bulk image operations.

Use `pprep -h`     for help, or
    `pprep --help` for more detailed help, or
    `pprep <subcommand> -h` for help on an operation.

For more documentation, see the GitHub repository:
     https://mlange-42.github.io/print-prep/

USAGE:
    pprep [FLAGS] [OPTIONS] <SUBCOMMAND>

FLAGS:
    -c, --cmd        Dummy option to end the `--input` list when no other top-level options are used.
    -d, --debug      Debug print parsed command line options
    -h, --help       Prints help information
    -V, --version    Prints version information
    -w, --wait       Wait for user input after processing

OPTIONS:
    -i, --input <input>...     List of input files or patterns. On Unix systems, patterns MUST be quoted!
    -t, --threads <threads>    Number of threads for parallel processing. Optional, default: number of processors

SUBCOMMANDS:
    help     Prints this message or the help of the given subcommand(s)
    list     List files found by input pattern
    prep     Prepare images for printing (add cut marks, 'mats', test patterns, EXIF information, ...).
    scale    Scale images to absolute or relative size
```

### `prep`

```
Prepare images for printing (add cut marks, 'mats', test patterns, EXIF information, ...).

<pre>
     ________________________________________
    |    |                              |    |
    |    |                              |    |-----  format
    |---- ------------------------------ ----|
    |    |     ____________________     |----|-----  framed-size
    |    |    |                    |    |    |
    |    |    |                    |----|----|-----  image-size
    |    |    |                    |    |    |       border
    |    |    |                    |    |    |
    |    |    |                    |   -|----|-----  padding
    |    |    |                    |    |    |
    |    |    |                    |    |   -|-----  margins
    |    |    |____________________|    |    |
    |    |                              |----|-----  cut-frame
    |---- ------------------------------ ----|
    |    |                              |----|-----  cut-marks
    |____|______________________________|____|
</pre>

USAGE:
    pprep prep [FLAGS] [OPTIONS] --format <w/h> --output <output>

FLAGS:
    -h, --help           Prints help information
        --incremental    Enable incremental scaling. For scaling to small sizes, scales down in multiple steps, to 50%
                         per step, averaging over 2x2 pixels
        --no-rotation    Prevents rotation of portrait format images (or of landscape format images if `--format` is
                         portrait)
    -V, --version        Prints version information

OPTIONS:
    -b, --bg <color>                    Background color. Default `white`
        --border <tp/rt/bm/lt>          Border width around image. Default none. This is included in padding!
        --border-color <color>          Border color. Default black
        --color <color>                 Cut marks, frame and exif color. Default: black
        --cut-frame <w/off>             Cut frame. Format <line-width>/<extend>. Use alternative to `--cut-marks`
        --cut-marks <w/off>             Cut marks with offset. Format <line-width>/<offset>. Use alternative to `--cut-
                                        frame`
    -d, --dpi <dpi>                     Image resolution. Default `300`
        --exif <format>                 Prints exif data. Formatting string. Example: --exif "{F/2}, {Exp}, ISO {ISO},
                                        {F}" Common abbreviations: `F/2`, `Exp`, `ISO`, `F`, `Bias`, `Date`, `Mod`.
                                        Further, all official exif tags
        --exif-size <size>              Size of exif font, in arbitrary units. Default: `12px`
    -f, --filter <filter>               Filter type for image scaling. One of `(nearest|linear|cubic|gauss|lanczos)`.
                                        Default: `cubic`
        --format <w/h>                  Print format `width/height`. Formats in cm are converted to exact print formats
                                        in inches. Examples: `15cm/10cm`, `6in/4in`, `6000px/4000px`
        --framed-size <w/h>             Maximum image size, incl. padding
        --image-size <w/h>              Maximum image size, excl. padding
        --margins <tp/rt/bm/lt>         Minimum margins around cut marks
    -o, --output <output>               Output path. Use `*` as placeholder for the original base file name.
                                        Used to determine output image type. On Unix systems, this MUST be quoted!
        --padding <tp/rt/bm/lt>         Padding between image and cut marks
    -q, --quality <quality>             Image quality for JPEG output in percent. Optional, default `95`
        --test-pattern <sx/gx/sy/gy>    Prints a print control element, with the given square size and gap. Format:
                                        `<sx>/<gx>/<sy>/<gy>` or `<size>/<gap>`. Example: `10px/2px/10px/2px`
```

### `scale`

```
Scale images to absolute or relative size

USAGE:
    pprep scale [FLAGS] [OPTIONS] --output <output>

FLAGS:
    -h, --help           Prints help information
        --incremental    Enable incremental scaling. For scaling to small sizes, scales down in multiple steps, to 50%
                         per step, averaging over 2x2 pixels
    -V, --version        Prints version information

OPTIONS:
    -b, --bg <bg>              Background color for `--mode fill`. Default `white`
    -d, --dpi <dpi>            Image resolution for size not in px. Default `300`
    -f, --filter <filter>      Filter type for image scaling. One of `(nearest|linear|cubic|gauss|lanczos)`. Default:
                               `cubic`
    -m, --mode <mode>          Scaling mode. Must be given when using `--size` with width and height. One of
                               `(keep|stretch|crop|fill)`. Default: `keep`
    -o, --output <output>      Output path. Use `*` as placeholder for the original base file name.
                               Used to determine output image type. On Unix systems, this MUST be quoted!
    -q, --quality <quality>    Image quality for JPEG output in percent. Optional, default `95`
        --scale <scale>        Output image scale. Use either `--size` or `--scale`. Examples: `0.5`, `50%`, `20%/10%`
        --size <size>          Output image size. Use either `--size` or `--scale`. Examples: `100px/.`, `./15cm`,
                               `8in/6in`
```

### `list`

```
List files found by input pattern

USAGE:
    pprep list [FLAGS]

FLAGS:
    -a, --absolute    Prints the absolute path
    -h, --help        Prints help information
    -p, --path        Prints the full path
    -V, --version     Prints version information
```

## Library / crate

To use this crate as a rust library, add the following to your `Cargo.toml` dependencies section:
```
print-prep = { git = "https://github.com/mlange-42/print-prep.git" }
```

_Warning:_ The API is still incomplete and highly unstable, so be prepared for frequent changes. 
Any help to stabilize the API is highly appreciated.

For the latest development version, see branch [`dev`](https://github.com/mlange-42/print-prep/tree/dev).

## License

[MIT](LICENSE) &copy; M. Lange