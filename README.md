# img-pipeline

This project is designed to process bitmap (`.bmp`) images using a series of
parallelized image processing filters. The program consists of 3 executables:

1. `blurrer`: Applies a blur filter to the image
2. `edger`: Applies an edge detection filter to the image
3. `publisher`: Coordinates the application of both filters by dividing the
   image into two halves (top and bottom) and applying one filter to each half
   concurrently. This process is done using shared memory.

## Disclaimer

This is just a toy project, inspired by an assignment from an Operating Systems
class that required implementation in C. It is not intended for real use. This
serves as a proof of concept for a pipeline of image processing filters.

## Documentation

For detailed requirements and specifications, please refer to the following files in the `docs` folder:

- [Requirements](docs/requirements.md)
- [Technical Specifications](docs/specifications.md)

## Installation

```sh
git clone https://github.com/anntnzrb/img-pipeline
cd img-pipeline
cargo build --workspace --release
```

### Nix

This project is [nix flakes](./flake.nix) ready. Just hook into the provided devshell.

## Usage

After completing the [installation](#installation), you can run the pipeline with:

```sh
cargo run --release -p publisher -- --input ./samples/test.bmp --output ./result.bmp
```

or try individual filters:

```sh
cargo run --release -p blurrer -- --input ./samples/test.bmp --output ./result.bmp
cargo run --release -p edger -- --input ./samples/test.bmp --output ./result.bmp
```

## COPYING

Refer to the [COPYING](./COPYING) file for licensing information.

Unless otherwise noted, all code herein is distributed under the terms of the
[GNU General Public License Version 3 or later](https://www.gnu.org/licenses/gpl-3.0.en.html).
