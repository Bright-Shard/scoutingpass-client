FROM alpine:latest
ARG USER

# Basic deps
RUN adduser -D -g '' $USER
WORKDIR /home/$USER
RUN apk update
RUN apk add curl bash gcompat busybox git

# Rust Config
ARG TAURI_CLI_VERSION=2.2.7

# Rust installation
RUN apk add gcc make musl-dev
USER $USER
RUN curl https://sh.rustup.rs -o rustup.sh
RUN chmod +x rustup.sh
RUN ./rustup.sh -y -t aarch64-linux-android -t armv7-linux-androideabi -t i686-linux-android -t x86_64-linux-android
RUN rm rustup.sh
ENV PATH=/home/${USER}/.cargo/bin:$PATH
RUN cargo install tauri-cli --version "$TAURI_CLI_VERSION"

# Java
USER root
RUN apk add openjdk17
USER $USER
ENV JAVA_HOME=/usr/lib/jvm/default-jvm

# SKIBBIDY TOILET - Nathan 2024

# Android SDK Config
ARG SDK_VERSION=25.2.5
ARG NDK_VERSION=25.0.8775105
ARG API_LEVEL=android-24
ARG BUILD_TOOLS_VERSION=30.0.3
ENV ANDROID_HOME=/home/${USER}/android-sdk

# Android SDK
ENV NDK_HOME=$ANDROID_HOME/ndk/$NDK_VERSION
RUN curl https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip -o android-sdk.zip
RUN unzip android-sdk.zip -d $ANDROID_HOME
RUN rm android-sdk.zip
ENV PATH=$ANDROID_HOME/cmdline-tools/bin:$PATH
RUN yes | sdkmanager --install "platform-tools" "platforms;$API_LEVEL" "build-tools;$BUILD_TOOLS_VERSION" "ndk;$NDK_VERSION" --sdk_root=$ANDROID_HOME

USER $USER
ENV IN_DOCKER=true

# For future reference: Removed these deps, might need them later
# RUN apk add wget openssl gcompat
# RUN ./rustup.sh -y -t aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
