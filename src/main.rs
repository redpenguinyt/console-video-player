mod frame_processing;
use std::{env, fs, io, process::Command};
use frame_processing as frame;
use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::gameloop;
use image::io::Reader as ImageReader;

const WIDTH: u32 = 350;
const HEIGHT: u32 = 90;
const FPS: u32 = 20;
const PIXEL_CHAR: char = ColChar::SOLID.fill_char;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please submit exactly one argument - the video filepath",
        ));
    }

    let video_filepath = &args[1];
    let opened_video = fs::File::open(video_filepath);
    match opened_video {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // convert video to image files
    let _ = fs::remove_dir_all("frames/");
    fs::create_dir_all("frames/").unwrap();

    let video_conversion_output = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            video_filepath,
            "-filter:v",
            format!("fps=fps={}", FPS).as_str(), // FPS
            "frames/frame_%0d.png",
        ])
        .output()
        .expect("Failed to convert video to frames");
    println!("ffmpeg status: {}", video_conversion_output.status);

    // load frames
    let (mut video_width, mut video_height) = (0, 0);

    let frames_dir = fs::read_dir("frames/").unwrap();
    let mut frames = vec![];
    let frame_count = fs::read_dir("frames/").unwrap().count();
    for (i, item_result) in frames_dir.enumerate() {
        let frame_path = item_result.unwrap().path();
        let frame_path = frame_path.to_str().unwrap();

        let mut img = ImageReader::open(frame_path).unwrap().decode().unwrap();
        (img, video_width, video_height) = frame::resized_img_and_size(img, WIDTH, HEIGHT);
        frames.push(img);

        print!("loading frame {i}/{frame_count}\r");
    }
    println!();

    let mut view = View::new(video_width * 2, video_height, ColChar::EMPTY);
    let mut frame_skip = false;
    for img in frames {
        let now = gameloop::Instant::now();
        view.clear();

        if !frame_skip {
            frame::blit_image_to(&mut view, img, PIXEL_CHAR, Wrapping::Ignore);
            view.display_render().unwrap();
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {}µs", elapsed.as_micros());

        frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
    }

    let _ = fs::remove_dir_all("frames/");
    Ok(())
}
