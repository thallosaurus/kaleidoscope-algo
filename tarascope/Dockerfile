FROM ubuntu:25.10
WORKDIR /opt
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install curl xz-utils -y
RUN curl -o blender.tar.xz https://download.blender.org/release/Blender4.5/blender-4.5.4-linux-x64.tar.xz
RUN tar -xvf blender.tar.xz
RUN rm blender.tar.xz
RUN mv blender-4.5.4-linux-x64 blender
ENV PATH=/opt/blender:$PATH

CMD ["blender", "-b"]