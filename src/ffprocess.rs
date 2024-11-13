extern crate ffmpeg_next as ffmpeg;

pub fn initialize_ffmpeg() {
    ffmpeg::init().unwrap();
}