<div align="center">
  <img align="middle" src="https://github.com/I-Al-Istannen/img-packing/blob/master/assets/demo.jpg?raw=true" width="200">
  <h1>img-packing</h1>
</div>

You have lots of images, a printer and a budget? You need to somehow put
everything into a single pdf for printing, but do not want all that wasted
space? `img-packing` will layout images densely in a multi-page pdf to reduce
wasted space, even rotating images by 90° if that makes them fit better.
Recognizing that rotating random images might mess up the griddy look of your
pdf, you can disable this with the `--no-rotate` switch.

## Installation
Download a binary from the
[releases](https://github.com/I-Al-Istannen/img-packing/releases) page or build
from source using `cargo build --release`.

## Usage
```
A simple program to pack images into a PDF

Usage: img-packing [OPTIONS] <OUTPUT_PDF> <IMAGES>...

Arguments:
  <OUTPUT_PDF>  The output PDF file
  <IMAGES>...   The images to pack. Can be a single image or a folder containing *only* images

Options:
      --dpi <dpi>                  The DPI to render the images at [default: 300]
      --width <width>              The width of the paper in mm [default: 210.0]
      --height <height>            The height of the paper in mm [default: 297.0]
      --border <width>             The border width in mm [default: 3.0]
      --margin <width>             The margin width in mm [default: 1.0]
      --max-image-width <width>    The maximum width an image may have in mm
      --max-image-height <height>  The maximum height an image may have in mm
      --no-rotate                  Do not rotate images
  -h, --help                       Print help
  -V, --version                    Print version
```
