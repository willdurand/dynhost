FROM rust:1.32.0-slim-stretch

ENV DEBIAN_FRONTEND noninteractive

# OpenSSL
ENV OPENSSL_VERSION=1.0.2q
ENv OPENSSL_STATIC=1
ENV OPENSSL_LIB_DIR=/openssl
ENV OPENSSL_INCLUDE_DIR=/openssl/include

ENV RPI_TOOLS=/rpi_tools
ENV PATH="${RPI_TOOLS}/arm-bcm2708/arm-linux-gnueabihf/bin:${PATH}"
ENV PKG_CONFIG_PATH=/usr/lib/arm-linux-gnueabihf/pkgconfig

RUN dpkg --add-architecture armhf
RUN apt-get update -qq && \
    # Install dependencies
    apt-get install -qq --no-install-recommends \
        git \
        make \
        wget \
        pkg-config \
        # libudev-dev will also bring in the arm libc6 and gcc packages
        libudev-dev:armhf && \
    # Install raspberrypi tools
    git -C "/" clone -q --depth=1 https://github.com/raspberrypi/tools.git "${RPI_TOOLS}" && \
    # Cross-compile OpenSSL
    wget -q -O /tmp/openssl.tar.gz "https://www.openssl.org/source/openssl-${OPENSSL_VERSION}.tar.gz" && \
        cd /tmp && \
        tar xzf openssl.tar.gz &&\
        mv "/tmp/openssl-${OPENSSL_VERSION}" /openssl && \
        cd /openssl && \
        ./Configure linux-generic32 shared --cross-compile-prefix=arm-linux-gnueabihf- && \
        make && \
    # Clean-up
    rm -fr "${RPI_TOOLS}/.git ${OPENSSL_DIR}/.git" \
           "${RPI_TOOLS}/arm-bcm2708/arm-bcm2708-linux-gnueabi" \
           "${RPI_TOOLS}/arm-bcm2708/arm-bcm2708hardfp-linux-gnueabi" \
           "${RPI_TOOLS}/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian" \
           "${RPI_TOOLS}/arm-bcm2708/gcc-linaro-arm-linux-gnueabihf-raspbian-x64" && \
    apt-get purge -qq \
        git \
        make \
        wget && \
    apt-get autoremove -qq --purge && \
    apt-get clean -qq && rm -fr /var/lib/apt/* /var/cache/apt/*

RUN rustup target add arm-unknown-linux-gnueabihf

ENTRYPOINT ["cargo", "build", "--target=arm-unknown-linux-gnueabihf"]
