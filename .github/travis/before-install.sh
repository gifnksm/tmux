#!/bin/sh

if [ "$TRAVIS_OS_NAME" = "linux" ]; then
	sudo apt-get update -qq
	sudo apt-get -y install bison \
				autotools-dev \
				libncurses5-dev \
				libevent-dev \
				pkg-config \
				libutempter-dev \
				build-essential \
				llvm-dev \
				libclang-dev \
				clang

	if [ "$BUILD" = "musl" -o "$BUILD" = "musl-static" ]; then
		sudo apt-get -y install musl-dev \
					musl-tools
	fi
fi

if [ "$TRAVIS_OS_NAME" = "freebsd" ]; then
	sudo pkg install -y \
		automake \
		libevent \
		pkgconf \
		llvm90
fi

export PATH="${HOME}/.cargo/bin:${PATH}"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s - -y

if [ "$BUILD" = "musl" -o "$BUILD" = "musl-static" ]; then
	IS_MUSL=yes
else
	IS_MUSL=no
fi

case "$TRAVIS_CPU_ARCH" in
	amd64)
		RUST_ARCH=x86_64
		;;
	arm64)
		RUST_ARCH=aarch64
		;;
esac

if [ "${IS_MUSL}" = yes -a "${TRAVIS_OS_NAME}" = linux ]; then
	rustup target add ${RUST_ARCH}-unknown-linux-musl
fi
