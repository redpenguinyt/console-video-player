pub mod frame;
use image::{io::Reader as ImageReader, DynamicImage};
use std::{
    ffi::OsString,
    fs::{self, DirEntry},
    io,
    process::Command,
};

pub fn get_video_filepath(args: &[String]) -> io::Result<&str> {
    if args.len() != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please submit exactly one argument - the video filepath",
        ));
    }

    let video_file_path = &args[1];
    let opened_video = fs::File::open(video_file_path);
    match opened_video {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    Ok(video_file_path)
}

pub fn generate_frames(video_file_path: &str, video_fps: f32) -> io::Result<()> {
    // convert video to image files
    let _ = fs::remove_dir_all("frames/");
    fs::create_dir_all("frames/").unwrap();

    let video_conversion_output = Command::new("ffmpeg")
        .args([
            "-y",
            "-i",
            video_file_path,
            "-filter:v",
            format!("fps=fps={}", video_fps).as_str(), // FPS
            "frames/frame_%6d.png",
        ])
        .output()?;
    println!("ffmpeg status: {}", video_conversion_output.status);

    Ok(())
}

pub struct Video {
    pub frames: Vec<DynamicImage>,
    pub width: usize,
    pub height: usize,
    pub fps: f32,
}

impl Video {
    pub fn new(width: u32, height: u32, fps: f32, video_file_path: &str) -> io::Result<Self> {
        generate_frames(video_file_path, fps)?;

        // load frames
        let (mut video_width, mut video_height) = (0, 0);

        let mut frame_files: Vec<io::Result<DirEntry>> = fs::read_dir("frames/")?.collect();

        frame_files.sort_by_key(|f| match f {
            Ok(file) => file.file_name(),
            Err(_) => OsString::default(),
        });

        let mut frames = vec![];
        let frame_count = frames.len();
        for (i, item_result) in frame_files.into_iter().enumerate() {
            let frame_path = item_result?.path();
            let frame_path = frame_path.to_str().unwrap();

            let mut img = ImageReader::open(frame_path)?.decode().unwrap();
            (img, video_width, video_height) = frame::resized_img_and_size(img, width, height);
            frames.push(img);

            print!("loading frame {i}/{frame_count}\r");
        }
        println!();

        Ok(Self {
            frames,
            width: video_width,
            height: video_height,
            fps,
        })
    }
}
