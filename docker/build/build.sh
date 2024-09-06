#!/bin/bash
cd /app
git clone https://github.com/eqtylab/c2pa_ffmpeg
cd c2pa_ffmpeg
git checkout c2pa
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
cp ffmpeg /out