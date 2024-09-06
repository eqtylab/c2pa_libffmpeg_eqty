# Use a basic Ubuntu image as the base
FROM ubuntu:22.04


# Set working directory
WORKDIR /app

RUN apt update; apt install -y ffmpeg libfdk-aac2
# Copy the two files into the container
COPY ffmpeg /usr/bin/ffmpeg
COPY libffmpeg_eqty_c2pa.so /lib
RUN chmod +x /usr/bin/ffmpeg

# Set the default command to bash
CMD ["/bin/bash"]
