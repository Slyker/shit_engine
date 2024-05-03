# Shit Engine

Welcome to the Shit Engine project! This is a basic image detection system written in Rust.

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Shit Engine is designed to provide developers with a flexible and efficient framework for image processing tasks. It leverages the power of Rust's performance and safety features to deliver a high-quality image manipulation experience.

## Features

- [x] Extract colors from an image
- [x] Extract based on criteria (color/tolerance)
- [ ] Better zone management
- [ ] Clustering
    - Includes progress bar and text processing
- [ ] OCR ?

## Usage

To use the Cheat Engine for extracting colors from an image, follow these steps:

1. Add the Cheat Engine as a dependency in your `Cargo.toml` file:
    ```toml
    [dependencies]
    shit_engine = "0.1.0" // you can use serde as feature
    ```

2. Import the Cheat Engine crate into your Rust code:
    ```rust
    use shit_engine::image_analyzer;
    use shit_engine::image_analyzer::{color::{hsv::Hsv, rgb::Rgb, Color}, pixel::PixelVec, ImageZone};
    ```

3. Use `image_analyzer` methods to extract pixels:
    ```rust
    let mut analyzer = image_analyzer::ImageAnalyzer::new(image.clone());
    let pixels = analyzer.detect_pixel_with_tolerance(
        ImageZone::Full,
        Color::Rgb(Rgb::from([0, 0, 0, 0])),
        Color::Hsv(Hsv::from([0.5, 0.5, 0.5, 0.5]))
    );
    // or use this for all colors
    let pixels = analyzer.detect_pixels(ImageZone::Full);
    ```
    Pixels are represented as `PixelVec` which look like:
    ```rust
    pub struct PixelVec {
        pub pixels: Vec<Pixel>,
        pub points_count: usize,
    }
    ```

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or bug reports, please open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

