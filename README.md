# High Performance Python Library With Rust PyO3
Rust has really so pretty smart compiler.

## Install

Install rust and python from ```https://www.rust-lang.org/tools/install```, ```https://www.python.org/downloads/```.

Make directory.

Clone project to your directory using command ```git clone https://github.com/Zargerion/High_Performance_Python_Library_With_Rust_PyO3.git``` in there.

Install Maturin using command ```pip install maturin```.

Run command to use py env. For exemple on Linux ```source env/bin/activate``` or on Windows ```env/bin/Activate.ps1```.

Build project using command ```maturin build```.

Go to ```/target/wheels/``` from root project directory and you will see something like this ```superlibz-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl```.

*Or you can just see at ```/target/wheels/``` without all previously steps if you know what to do.

Run command ```pip install <path/to/file.whl>```

## Use

Look at test.py in root project directory.
*For videos it may use ffmpeg. To get:

On Windows. You can install ffmpeg here: https://www.gyan.dev/ffmpeg/builds/. Or by this link: https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-full.7z. Next you need to unpack it and specify bin folder in root to Path at Environment Variables.

On MacOS. Try to install this https://evermeet.cx/ffmpeg/ffmpeg-110189-ge1f691b2e8.7z. Maybe you will need to specify bin folder in root to Path at Environment Variables.

On linux just run ```sudo apt-get install ffmpeg``` or run ```ffmpeg_setuping()``` in python using this lib. (Look at test.py)
