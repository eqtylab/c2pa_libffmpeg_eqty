# libffmpeg_eqty_c2pa.so

Library that wraps the C2PA rust library for PoC ffmpeg-c2pa branch. 

## Installation

```
cargo build --release
sudo cp target/release/libffmpeg_eqty_c2pa.so /lib
```

## Usage with ffmpeg

mkdir test-out
./ffmpeg -i inputfile.mp4 \
-f dash \
-c2pa_cert ~/c2patool/sample/ps256.pub \
-c2pa_key ~/c2patool/sample/ps256.pem \
-c2pa_manifest ~/c2patool/sample/test.json \
test-out/outfile.mpd