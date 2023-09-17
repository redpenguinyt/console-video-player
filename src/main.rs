use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::gameloop;
use gemini_video_player::{frame, get_video_filepath, Video};
use std::{env, fs, io, process};

const WIDTH: u32 = 350;
const HEIGHT: u32 = 90;
const FPS: f32 = 60.0;
const PIXEL_CHAR: char = ColChar::SOLID.fill_char;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let video_file_path = get_video_filepath(&args).unwrap_or_else(|err| {
        eprintln!("Error while finding video at path: {err}");
        process::exit(1);
    });

    let scaled_video = Video::new(WIDTH, HEIGHT, FPS, video_file_path).unwrap_or_else(|err| {
        eprintln!("Error while generating frames: {err}");
        process::exit(1);
    });

    let mut view = View::new(scaled_video.width * 2, scaled_video.height, ColChar::EMPTY);
    let mut frame_skip = false;
    for img in scaled_video.frames {
        let now = gameloop::Instant::now();
        view.clear();

        if !frame_skip {
            frame::blit_image_to(&mut view, img, PIXEL_CHAR, Wrapping::Ignore);
            view.display_render().unwrap();
        }

        let elapsed = now.elapsed();
        println!("Elapsed: {}µs", elapsed.as_micros());

        frame_skip = gameloop::sleep_fps(scaled_video.fps, Some(elapsed));
    }

    let _ = fs::remove_dir_all("frames/");
    Ok(())
}
