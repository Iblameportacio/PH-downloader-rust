# PH-downloader-rust

A heavy-duty, zero-bloat CLI video downloader written in **Rust**. No fancy wrappers, 
no unnecessary memory overhead—just pure native power calling `yt-dlp` directly to grab your 
porn videos materials at maximum speed.

Developed as a learning project to ditch Python and embrace the strict, lightning-fast world of Rust.

---

## Features

*   **Zero Bloat:** Built natively using Rust's `std::process::Command`. 
*   **Dual Mode:** Run it raw with terminal arguments, or just launch it empty to trigger the interactive prompt.
*   **Real-Time Feedback:** Inherits your system's shell I/O, meaning you get the actual `yt-dlp` progress bar and `ffmpeg` merging logs live.
*   **Safe Pathing:** Automatically handles cross-platform paths and dumps everything neatly inside a local `./descargas` directory if you don't specify one.

---

## Prerequisites

Since this tool talks directly to your OS, you need to have these bad boys installed and available in your `$PATH`:

1.  **`yt-dlp`**: The actual muscle doing the scraping.
2.  **`ffmpeg`**: Absolutely mandatory for stitching together video and audio streams from high-quality platforms.

On **Arch Linux**, just run:
```bash
sudo pacman -S yt-dlp ffmpeg

I use arch btw
