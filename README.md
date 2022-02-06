# image-diff

image-diff is a simple CLI tool to detect the pixel difference between two images. 
The difference in pixels is saved to a results image where each detected pixle difference is a fuchsia pixle.

## Usage
```
image-diff.exe <FIRST_IMAGE_LOC> <SECOND_IMAGE_LOC> <RESULTS_LOC>

ARGS:
    <FIRST_IMAGE_LOC>     
    <SECOND_IMAGE_LOC>
    <RESULTS_LOC>

OPTIONS:
    -h, --help    Print help information
```
To run with `cargo`, simply call `cargo run -- <cli args>`. Example:
```
# cargo run -- <PATH_TO_FIRST_IMAGE> <PATH_TO_SECOND_IMAGE> <PATH_WHERE_YOU_WANT_THE_THE_RESULTS_IMAGE_SAVED>

cargo run -- C:\Users\15034\Desktop\image-diff-test\first.jpg C:\Users\15034\Desktop\image-diff-test\png\second.png C:\Users\15034\Desktop\image-diff-test\png\diff_results.png
```

## Dependencies
* [image](https://crates.io/crates/image)
* [clap](https://crates.io/crates/clap)
