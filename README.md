# image-diff

image-diff is a simple CLI tool to detect the pixel difference between two images. 
The difference in pixels is saved to a results image where each detected pixle difference is a fuchsia pixle.

## Usage
```
USAGE:
    image-diff.exe --first-image <FIRST_IMAGE> --second-image <SECOND_IMAGE> 
--results-image <RESULTS_IMAGE>

OPTIONS:
    -f, --first-image <FIRST_IMAGE>        
    -h, --help                             Print help information
    -r, --results-image <RESULTS_IMAGE>    
    -s, --second-image <SECOND_IMAGE>      
    -V, --version                          Print version information
```
To run with `cargo`, simply call `cargo run -- <cli args>`. Example:
```
# cargo run -- <FIRST_IMAGE> <SECOND_IMAGE> <RESULTS_IMAGE>

cargo run -- C:\Users\15034\Desktop\image-diff-test\first.jpg C:\Users\15034\Desktop\image-diff-test\png\second.png C:\Users\15034\Desktop\image-diff-test\png\diff_results.png
```

## Dependencies
* [image](https://crates.io/crates/image)
* [clap](https://crates.io/crates/clap)
