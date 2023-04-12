use image::ImageFormat;
use pyo3::prelude::*;
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::fs::{self,};
use std::path::Path;
use std::io::{self,};
use std::process::Command;
use image::io::Reader as ImageReader;

mod utils;

#[pyfunction]
fn all_dir_jpg_to_webp(src: String, dest: String) -> std::io::Result<()> {

    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Destination directory not found"));
    }

    fs::read_dir(src_dir)?.par_bridge().for_each(|entry|  { 
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return println!("{}", e.to_string()),
        };
        let path = entry.path();

        if entry.file_type().unwrap().is_file() && path.extension().map_or(false, |ext| ext == "jpg") {
            let dest_file = dest_dir.join(path.file_name().unwrap().to_str().unwrap().replace("jpg", "webp"));

            let image = match ImageReader::open(&path).unwrap().decode() {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };

            let dest_file2 = dest_file.clone();
            let mut file = match fs::File::create(&dest_file) {
                Ok(f) => f,
                Err(e) => return println!("{}", e.to_string()),
            };
            match image.write_to(&mut file, image::ImageOutputFormat::WebP) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };

            println!("Converted {:?} to {:?}", path, dest_file2);
        }
    });

    Ok(())
    
}

#[pyfunction]
fn all_dir_webp_to_jpg(src: String, dest: String) -> std::io::Result<()> {

    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Destination directory not found"));
    }

    fs::read_dir(src_dir)?.par_bridge().for_each(|entry|  {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return println!("{}", e.to_string()),
        };
        let path = entry.path();

        if entry.file_type().unwrap().is_file() && path.extension().map_or(false, |ext| ext == "webp") {
            let dest_file = dest_dir.join(path.file_name().unwrap().to_str().unwrap().replace("webp", "jpg"));

            let image = match ImageReader::open(&path).unwrap().decode() {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };

            let dest_file2 = dest_file.clone();
            let mut file = match fs::File::create(&dest_file) {
                Ok(f) => f,
                Err(e) => return println!("{}", e.to_string()),
            };
            match image.write_to(&mut file, image::ImageOutputFormat::Jpeg(100)) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };

            println!("Converted {:?} to {:?}", path, dest_file2);
        }
    });

    Ok(())
}

#[pyfunction]
fn all_dir_webp_to_jpg_no_threading(src: String, dest: String) -> std::io::Result<()> {

    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Destination directory not found"));
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_file() && path.extension().map_or(false, |ext| ext == "webp") {
            let dest_file = dest_dir.join(path.file_name().unwrap().to_str().unwrap().replace("webp", "jpg"));

            let image = match ImageReader::open(&path)?.decode() {
                Ok(img) => img,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            };

            let dest_file2 = dest_file.clone();
            let mut file = fs::File::create(&dest_file)?;
            match image.write_to(&mut file, image::ImageOutputFormat::Jpeg(100)) {
                Ok(img) => img,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            };

            println!("Converted {:?} to {:?}", path, dest_file2);
        }
    }

    Ok(())
}

#[pyfunction]
fn all_dir_jpg_to_png(src: String, dest: String) -> std::io::Result<()> {
    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Destination directory not found",
        ));
    }

    fs::read_dir(src_dir)?.par_bridge().for_each(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return println!("{}", e.to_string()),
        };
        let path = entry.path();

        if entry.file_type().unwrap().is_file() && path.extension().map_or(false, |ext| ext == "jpg") {
            let dest_file = dest_dir.join(path.file_stem().unwrap()).with_extension("png");

            let image = match image::open(&path) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };
            match image.save_with_format(&dest_file, ImageFormat::Png) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            } 

            println!("Converted {:?} to {:?}", path, dest_file);
        }
    });

    Ok(())
}

#[pyfunction]
fn all_dir_png_to_jpg(src: String, dest: String) -> std::io::Result<()> {
    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Destination directory not found",
        ));
    }

    fs::read_dir(src_dir)?.par_bridge().for_each(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => return println!("{}", e.to_string()),
        };
        let path = entry.path();

        if entry.file_type().unwrap().is_file() && path.extension().map_or(false, |ext| ext == "png") {
            let dest_file = dest_dir.join(path.file_stem().unwrap()).with_extension("jpg");

            let image = match image::open(&path) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };
            match image.save_with_format(&dest_file, ImageFormat::Jpeg) {
                Ok(img) => img,
                Err(e) => return println!("{}", e.to_string()),
            };

            println!("Converted {:?} to {:?}", path, dest_file);
        }
    });

    Ok(())
}

#[pyfunction]
fn all_dir_png_to_jpg_no_threading(src: String, dest: String) -> std::io::Result<()> {
    let src_dir = Path::new(&src);
    let dest_dir = Path::new(&dest);

    if !dest_dir.exists() || !src_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Destination directory not found",
        ));
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_file() && path.extension().map_or(false, |ext| ext == "png") {
            let dest_file = dest_dir.join(path.file_stem().unwrap()).with_extension("jpg");

            let image = match image::open(&path) {
                Ok(img) => img,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            };
            match image.save_with_format(&dest_file, ImageFormat::Jpeg) {
                Ok(img) => img,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
            };

            println!("Converted {:?} to {:?}", path, dest_file);
        }
    }

    Ok(())
}

#[pyfunction]
fn compress_video_mp4_with_ffmpeg(input_file_path: String, target_size_mb: i32) -> std::io::Result<()> {

    let num_threads = num_cpus::get();
    let input_file_path2 = input_file_path.clone();
    let s_formatted = utils::make_formatted(input_file_path);
    let s_slice_formatted = s_formatted.as_str();
    let input_file_path = Path::new(&input_file_path2);
    let output_file_path = Path::new(s_slice_formatted);

    //------------------------------------------------------------

    let (s, w, h) = match utils::get_video_resolution(input_file_path) {
        Ok(swh) => swh,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };
    println!("{}", s);

    let str_input_file_path = input_file_path.to_string_lossy();
    let persent = match utils::make_persent_file_size(str_input_file_path.to_string(), target_size_mb) {
        Ok(swh) => swh,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e.to_string())),
    };
    let new_w = (w as f32) * persent;
    let new_h = (h as f32) * persent;
    let wh = format!("{}x{}", utils::nearest_even(new_w as i32).to_string(), utils::nearest_even(new_h as i32).to_string());
    let wh2 = wh.clone();
    println!("Resolution: {}", wh2);
    let scale_resolution = format!("scale={}", wh);

    //------------------------------------------------------------

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_file_path)
        .arg("-vf")
        .arg(scale_resolution.as_str())
        .arg("-threads")
        .arg(num_threads.to_string().as_str())
        .arg(output_file_path)
        .output()
        .expect("Failed to execute command");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}

#[pyfunction]
fn video_conversion_with_ffmpeg(src: String, dest: String) -> std::io::Result<()> {
    let src_file = Path::new(&src);
    let dest_file = Path::new(&dest);
    let num_threads = num_cpus::get();

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(src_file)
        .arg("-threads")
        .arg(num_threads.to_string().as_str())
        .arg(dest_file)
        .output()
        .expect("Failed to execute ffmpeg command");

    
    println!("Required ffmpeg. Try to run ffmpeg_setuping().");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

#[pyfunction]
fn ffmpeg_setuping() {
    utils::ffmpeg_setup();
}

#[pyfunction]
fn delete_files_in_dirs(directories: Vec<String>) -> std::io::Result<()> {
    for dir in directories {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        fs::remove_file(path)?;
                    }
                }
            }
        }
    }

    Ok(())
}

#[pymodule]
fn superlibz(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(all_dir_jpg_to_webp, m)?)?;
    m.add_function(wrap_pyfunction!(all_dir_webp_to_jpg, m)?)?;
    m.add_function(wrap_pyfunction!(all_dir_webp_to_jpg_no_threading, m)?)?;
    m.add_function(wrap_pyfunction!(all_dir_jpg_to_png, m)?)?;
    m.add_function(wrap_pyfunction!(all_dir_png_to_jpg, m)?)?;
    m.add_function(wrap_pyfunction!(all_dir_png_to_jpg_no_threading, m)?)?;
    m.add_function(wrap_pyfunction!(compress_video_mp4_with_ffmpeg, m)?)?;
    m.add_function(wrap_pyfunction!(video_conversion_with_ffmpeg, m)?)?;
    m.add_function(wrap_pyfunction!(ffmpeg_setuping, m)?)?;
    m.add_function(wrap_pyfunction!(delete_files_in_dirs, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_dir_jpg_to_webp() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/jpg_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_webp".to_string();
        match all_dir_jpg_to_webp(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_webp/file1.webp");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_webp/file2.webp");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_webp/file3.webp");
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(file3.exists());
    }

    #[test]
    fn test_all_dir_webp_to_jpg() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/webp_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_jpg".to_string();
        match all_dir_webp_to_jpg(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file1.jpg");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file2.jpg");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file3.jpg");
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(file3.exists());
    }

    #[test]
    fn test_all_dir_webp_to_jpg_no_threading() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/webp_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_jpg".to_string();
        match all_dir_webp_to_jpg(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file1.jpg");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file2.jpg");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file3.jpg");
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(file3.exists());
    }

    #[test]
    fn test_all_dir_jpg_to_png() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/jpg_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_png".to_string();
        match all_dir_jpg_to_png(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_png/file1.png");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_png/file2.png");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_png/file3.png");
        assert!(!file1.exists());
        assert!(!file2.exists());
        assert!(file3.exists());
    }

    #[test]
    fn test_all_dir_png_to_jpg() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/png_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_jpg".to_string();
        match all_dir_png_to_jpg(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file1.jpg");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file2.jpg");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file3.jpg");
        assert!(file1.exists());
        assert!(file2.exists());
        assert!(!file3.exists());
    }

    #[test]
    fn test_all_dir_png_to_jpg_no_threading() {
        let src: String = "/home/zargerion/vs-projects/superlibz/test_images/png_images".to_string();
        let dst: String = "/home/zargerion/vs-projects/superlibz/test_images/to_jpg".to_string();
        match all_dir_png_to_jpg_no_threading(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file1 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file1.jpg");
        let file2 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file2.jpg");
        let file3 = Path::new("/home/zargerion/vs-projects/superlibz/test_images/to_jpg/file3.jpg");
        assert!(file1.exists());
        assert!(file2.exists());
        assert!(!file3.exists());
    }

    #[test]
    fn test_compress_video_mp4_with_ffmpeg() {
        match compress_video_mp4_with_ffmpeg("/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file.mp4".to_string(), 8) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file = Path::new("/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file_formatted.mp4");
        assert!(file.exists());
    }

    #[test]
    fn test_video_conversion_with_ffmpeg() {
        let src = "/home/zargerion/vs-projects/superlibz/test_videos/webm_videos/file.webm".to_string();
        let dst = "/home/zargerion/vs-projects/superlibz/test_videos/to_mp4/file.mp4".to_string();
        match video_conversion_with_ffmpeg(src, dst) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        let file = Path::new("/home/zargerion/vs-projects/superlibz/test_videos/to_mp4/file.mp4");
        assert!(file.exists());
    }

    #[test]
    fn test_delete_files_in_dirs() {
        let folders: Vec<String> = vec!["/home/zargerion/vs-projects/superlibz/test_images/to_webp".to_string(), "/home/zargerion/vs-projects/superlibz/test_images/to_jpg".to_string(), "/home/zargerion/vs-projects/superlibz/test_images/to_png".to_string(), "/home/zargerion/vs-projects/superlibz/test_videos/to_mp4".to_string(), "/home/zargerion/vs-projects/superlibz/test_videos/to_webm".to_string()];
        let folders2 = folders.clone();
        match delete_files_in_dirs(folders) {
            Ok(_) => assert!(true),
            Err(e) => panic!("Error: {}", e),
        }
        for f in folders2 {
            assert!(utils::is_dir_empty(f));
        }
    }

}    