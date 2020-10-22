#!/bin/sh

export PATH="${HOME}/.cargo/bin:${PATH}"

case "$TRAVIS_CPU_ARCH" in
	amd64)
		RUST_ARCH=x86_64
		;;
	arm64)
		RUST_ARCH=aarch64
		;;
esac

sh autogen.sh || exit 1
case "$BUILD" in
	static)
		./configure --enable-static || exit 1
		exec make
		;;
	all)
		sh $(dirname $0)/build-all.sh
		exec make
		;;
	musl)
		CC=musl-gcc RUST_TARGET="${RUST_ARCH}-unknown-linux-musl" sh $(dirname $0)/build-all.sh
		exec make
		;;
	musl-static)
		CC=musl-gcc RUST_TARGET="${RUST_ARCH}-unknown-linux-musl" sh $(dirname $0)/build-all.sh --enable-static
		exec make
		;;
	*)
		./configure || exit 1
		exec make
		;;
esac
