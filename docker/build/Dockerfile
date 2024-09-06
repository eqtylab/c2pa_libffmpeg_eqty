# Use a basic Ubuntu image as the base
FROM ubuntu:22.04


# Set working directory
WORKDIR /app

RUN apt update;
RUN apt install -y \
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
    git pkg-config sudo \
    libopus-dev \
    libvpx-dev \
    libx264-dev \
    libx265-dev

  
COPY build.sh /app
RUN chmod +x /app/build.sh

# Set the default command to bash
CMD ["/app/build.sh"]
