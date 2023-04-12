from superlibz import all_dir_jpg_to_webp, ffmpeg_setuping, compress_video_mp4_with_ffmpeg, video_conversion_with_ffmpeg



try: 
    all_dir_jpg_to_webp("/home/zargerion/vs-projects/superlibz/test_images/jpg_images" , "/home/zargerion/vs-projects/superlibz/test_images/to_webp")
except:
    print("Cannot run this function.")

try: 
    ffmpeg_setuping()
except:
    print("Cannot run function.")

try: 
    compress_video_mp4_with_ffmpeg("/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file.mp4", 8)
except:
    print("Cannot run this function.")

try: 
   video_conversion_with_ffmpeg("/home/zargerion/vs-projects/superlibz/test_videos/mp4_videos/file.mp4", "/home/zargerion/vs-projects/superlibz/test_videos/to_webm/file.webm")
except:
    print("Cannot run this function.")

print("NicE!")