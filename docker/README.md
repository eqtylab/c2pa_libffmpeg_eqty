# Using docker

1) Build docker
```
docker compose build
```

2) Start docker
```
docker compose up -d
```
3) Enter docker shel
```
docker compose exec ffmpeg-container /bin/bash
```
4) Test signing of DASH

```
mkdir /data/test-out 
ffmpeg -i /data/sample-5.mp4 \
-f dash \
-c2pa_cert /data/ps256.pub \
-c2pa_key /data/ps256.pem \
-c2pa_manifest /data/test.json \
 /data/test-out/test.mpd
``````

5) Test run MP4
```
mkdir /data/test-out-mp4
ffmpeg -i /data/sample-5.mp4 \
-f mp4 \
-c2pa_cert /data/ps256.pub \
-c2pa_key /data/ps256.pem \
-c2pa_manifest /data/test.json \
/data/test-out-mp4/test.mp4
```