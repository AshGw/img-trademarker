# img_watermarker

**Minimal lib for watermarking images, where a logo image will be added on top of a main image.**

## Installation

Add this to your `Cargo.toml` file
````toml
[dependencies]
img_watermarker = "0.0.1"
````
## Usage

````rust
use img_watermarker::watermark;

fn main() {
    watermark();
}
````
Run it in the command line with:
````commandline
cargo run <logo_path> <image_path> <debug?> <output_path?>
````
- `<debug?>` is optional and can be set to enable debugging output.
- `<output_path?>` is optional. If not provided, the result of the two images will be saved to `output.jpg`
