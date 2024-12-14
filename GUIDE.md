# rsframe guide
## Requirements
 - ***ffmpeg***: ffmpeg is the main video editor component of `rsframe`. Windows users need to install a precompiled build, which you can find online, and then add it to PATH. Linux users can just run the `sudo apt install` command.
 - ***ImageMagick***: ImageMagick is used for creating text frames. You can leave ImageMagick uninstall if you're not going to do any text editing, however it is still recommended to install it. Go to their official website and install it. Windows users must also have to add it to path
## Basic Video Editing
In this short chapter, I will demonstrate how to turn a video monochrome.  
Firstly, let's get a basic project set up. In your project directory, there should be a video called `placeholder.mp4`
```rust
use rsframe::vfx::video;

fn main() {
    
}
```
As you can see in this code block, the `video` module is where all the video editing tools are.
Let's load a video. To do this, we'll call the `from_file` method from the `Video` struct.
```rust
let mut vid = video::Video::from_file("placeholder.mp4".to_string()).expect("Cannot open video.");
```
This code will load a video into memory. You shouldn't do this with really long videos as loading in a video takes quite a bit of time.
```rust
let mut vid = video::Video::from_file("placeholder.mp4".to_string()).expect("Cannot open video.");

vid.monochrome();
vid.save("output.mp4".to_string(), 24, false);
```
Above is a code block demonstrating how to modify a Video and how to save it.
Saving a video takes three parameters:
1. Output path
2. FPS
3. Keep rendering folder  

Keeping the debug folder can be valuable if you're trying to figure what went wrong in your program and at what time.

***TODO: Add more tutorials***  
But for right now, this and the `docs.rs` page should serve you well enough to figure out how to use `rsframe`.  
***FYI: Text Frame creation is experiencing some issues, if you're able to find a fix, PLEASE create a pull request. Thank you***