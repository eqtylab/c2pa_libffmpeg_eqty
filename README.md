# libffmpeg_eqty_c2pa.so

Library that wraps the C2PA rust library for PoC ffmpeg-c2pa branch. 

## Installation

### Step1 - Rust library

0) git clone
```
git clone https://github.com/eqtylab/c2pa_libffmpeg_eqty/
cd c2pa_libffmpeg_eqty
```
1) Build this library
```
cargo build --release
```
2) Place the resuting file into the library path
```
sudo cp target/release/libffmpeg_eqty_c2pa.so /lib
```

### Step2 Build FFMPEG
0) git clone
```
git clone https://github.com/eqtylab/c2pa_ffmpeg
cd c2pa_ffmpeg
git checkout c2pa
```
1) install build dependencies
```
sudo apt install -y \
    autoconf \
    automake \
    build-essential \
    cmake \
    libass-dev \
    libfreetype6-dev \
    libsdl2-dev \
    libtool \
    libva-dev \
    libvdpau-dev \
    libvorbis-dev \
    libxcb1-dev \
    libxcb-shm0-dev \
    libxcb-xfixes0-dev \
    meson \
    ninja-build \
    pkg-config \
    texinfo \
    wget \
    yasm \
    zlib1g-dev \
    git pkg-config \
    libopus-dev \
    libvpx-dev \
    libx264-dev \
    libx265-dev
```
2) configure and build ffmpeg
```
./configure \
  --enable-gpl \
  --enable-libass \
  --enable-libfreetype \
  --enable-libvorbis \
  --enable-libvpx \
  --enable-libx264 \
  --enable-libx265 \
  --enable-libopus \
  --enable-nonfree
make -j$(nproc)
```
## Usage c2pa with ffmpeg

Currently supports monolithic MP4 signing and signing of DASH segments. It requires 3 paramaters

`-c2pa_key`: Certificate private key  
`-c2pa_cert`: Certificate  
`-c2pa_manifest`: json manifest that will be used   

Example:

```
mkdir test-output
./ffmpeg -i INPUTFILE.mp4 \
-f dash \
-c2pa_cert ps256.pub \
-c2pa_key ps256.pem \
-c2pa_manifest test.json \
test-out/outfile.mpd
```
