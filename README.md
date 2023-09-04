Video player that plays videos in the terminal, written in Rust with [gemini_engine](https://github.com/redpenguinyt/gemini-rust)

## Installation/Use
This is a command line tool, but is currently only available to compile from source. To do so, make sure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed, as well as [ffmpeg](https://ffmpeg.org/), which this tool makes use of to convert the video to individual frames.

Download this project as a zip or with `git clone`, open a terminal in the project's directory (the same place where this README file is located) and run `cargo run <video>` (with the video you want to play instead of `<video>` for example `cargo run myvideo.mov`). The video will compile, then play!