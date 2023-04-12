use std::path::Path;
use std::{self, fs, io, process::Command};


pub fn is_dir_empty( dir: String) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        return entries.count() == 0;
    }
    false
}

pub fn make_formatted(mut s: String) -> String {
    let len = s.len();
    s.replace_range((len - 4)..len, "");
    let formatted = format!("{}_formatted.mp4", s);
    formatted
}

pub fn make_persent_file_size(file: String, target_size_mb: i32) -> Result<f32, io::Error> {
    let video_file_size_mb = fs::metadata(&file)?.len() as f32 / 1024.0 / 1024.0;
    let file_size_percent_mb = target_size_mb as f32 / video_file_size_mb;
    let rounded_result = (file_size_percent_mb * 10.0).trunc() / 10.0;
    Ok(rounded_result)
}

fn ffmpeg_install_linux() {
    let output = Command::new("sudo")
        .arg("apt-get")
        .arg("install")
        .arg("-y")
        .arg("ffmpeg")
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{}", output_str);
}

pub fn ffmpeg_check_linux() {
    let output = Command::new("dpkg")
        .arg("-s")
        .arg("ffmpeg")
        .output()
        .expect("failed to execute process");
    println!("{}", output.status);
    let str_out = String::from_utf8_lossy(&output.stdout);
    let str_err = String::from_utf8_lossy(&output.stderr);
    print!("{}", str_out);
    println!("{}", str_err);
    if output.status.to_string() != "exit status: 0".to_string() {
        ffmpeg_install_linux();
    }
}

fn current_os() -> String {
    if cfg!(target_os = "windows") {
        "Windows".to_string()
    } else if cfg!(target_os = "macos") {
        "MacOS".to_string()
    } else if cfg!(target_os = "linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    }
}

pub fn ffmpeg_setup() {
    let os = current_os();
    if os == "Windows".to_string() {
        println!("On Windows. You can install ffmpeg here: https://www.gyan.dev/ffmpeg/builds/. Or by this link: https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z. Next you need to unpack it and specify bin folder in root to Path at Environment Variables.")
    } else if os == "MacOS".to_string() {
        println!("On MacOS. Try to install this https://evermeet.cx/ffmpeg/ffmpeg-110189-ge1f691b2e8.7z. Maybe you will need to specify bin folder in root to Path at Environment Variables.")
    } else if os == "Linux".to_string() {
        ffmpeg_check_linux();
    } else {
        println!("Unknown OS. It can be unsupported, but you can try install ffmpeg.")
    }
}

pub fn get_video_resolution(input_file_path: &Path) -> Result<(String, u32, u32), io::Error> {

    ffmpeg_next::init().unwrap();

    let input_file_path2 = input_file_path.clone();
    let ictx = ffmpeg_next::format::input(&input_file_path2)?;

    let video_stream = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .unwrap();

    let decoder = video_stream.codec().decoder().video()?;
    let width = decoder.width();
    let height = decoder.height();

    Ok((format!("Resolution: {}x{}", width, height), width, height))
}

pub fn nearest_even(n: i32) -> i32 {
    if n % 2 == 0 {
        n
    } else {
        (n + 1) / 2 * 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_formatted() {
        let input = String::from("video1234.mp4");
        let expected = "video1234_formatted.mp4";

        let result = make_formatted(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ffmpeg_check() {
        ffmpeg_check_linux();
        assert!(true);
    }

    #[test]
    fn test_ffmpeg_install_linux() {
        ffmpeg_install_linux();
        assert!(true);
    }

    #[test]
    fn test_make_persent_file_size() {
        let file = "/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file.mp4".to_string();
        let target_size_mb = 8;
        let result = match make_persent_file_size(file, target_size_mb) {
            Ok(result) => result,
            Err(e) => panic!("Error: {}", e),
        };
        println!("{}", result); 
        assert_eq!(true, true);
    }

    #[test]
    fn test_current_os() {
        let os = current_os();
        println!("{}", os);
        assert!(os == "Windows".to_string() || os == "MacOS".to_string() || os == "Linux".to_string());
    }

    #[test]
    fn test_get_video_resolution() {
        let input_file_path = Path::new("/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file.mp4");
        let (s, _, _) = match get_video_resolution(input_file_path) {
            Ok(s) => s,
            Err(e) => panic!("Error: {}", e),
        };
        println!("{}", s);
        assert!(true);
    }

    #[test]
    fn test_nearest_even() {
        assert_eq!(nearest_even(3), 4);
        assert_eq!(nearest_even(4), 4);
        assert_eq!(nearest_even(5), 6);
        assert_eq!(nearest_even(6), 6);
        assert_eq!(nearest_even(7), 8);
    }
} 

