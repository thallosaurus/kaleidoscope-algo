FROM ubuntu:25.10
WORKDIR /opt
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install curl xz-utils build-essential git -y
RUN apt-get -q install -y --no-install-recommends --fix-missing \
    automake \
    autoconf \
    build-essential \
    git \
    libbz2-dev \
    libegl1 \
    libfontconfig1 \
    libgl1 \
    libglvnd-dev \
    libgtk-3-0 \
    libsm6 \
    libtool \
    libx11-6 \
    libx11-dev \
    libxcursor1 \
    libxext6 \
    libxext-dev \
    libxi6 \
    libxinerama1 \
    libxkbcommon0 \
    libxrandr2 \
    libxrender1 \
    libxxf86vm1 \
    mesa-utils \
    pkg-config \
    wget \
    python3 \
    python3-pip\
    x11proto-dev \
    x11proto-gl-dev \
    xvfb

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN curl -o blender.tar.xz https://download.blender.org/release/Blender4.5/blender-4.5.4-linux-x64.tar.xz
RUN tar -xvf blender.tar.xz
RUN rm blender.tar.xz
RUN mv blender-4.5.4-linux-x64 blender
ENV PATH="/opt/blender:${PATH}"

RUN mkdir /opt/tarascope
WORKDIR /opt/tarascope
COPY . .
RUN cargo build --release --bin publisher


# Clone and build libglvnd for NVIDIA EGL support
RUN git clone https://github.com/NVIDIA/libglvnd.git /tmp/libglvnd \
    && cd /tmp/libglvnd \
    && ./autogen.sh \
    && ./configure \
    && make -j$(nproc) \
    && make install \
    && mkdir -p /usr/share/glvnd/egl_vendor.d/ \
    && printf "{\n\
    \"file_format_version\" : \"1.0.0\",\n\
    \"ICD\": {\n\
        \"library_path\": \"libEGL_nvidia.so.0\"\n\
    }\n\
    }" > /usr/share/glvnd/egl_vendor.d/10_nvidia.json \
    && cd / \
    && rm -rf /tmp/libglvnd

ENV EGL_DRIVER=nvidia
ENV __EGL_VENDOR_LIBRARY_DIRS=/usr/share/glvnd/egl_vendor.d

#CMD ["publisher"]

# docker run --rm --runtime=nvidia -e NVIDIA_VISIBLE_DEVICES=nvidia.com/gpu=all -it dockertest
