#!/usr/bin/env bash
binary="jobasha"
folder="Jobasha"
docs="${folder}/Docs"
zip="Jobasha.zip"

build() {
set -x
cargo build || return 1
cargo build --profile release-lto --target x86_64-unknown-linux-gnu || return 1
cargo build --profile release-lto --target x86_64-unknown-linux-musl || return 1
cargo ndk -t arm64-v8a build --profile release-lto || return 1
cargo xwin build --profile release-lto --target x86_64-pc-windows-msvc || return 1
cargo build --profile release-lto --target x86_64-pc-windows-gnu || return 1
PATH="$HOME/projects/osxcross/target/bin:$PATH" cargo build --profile release-lto-darwin --target x86_64-apple-darwin --config target.x86_64-apple-darwin.linker=\"x86_64-apple-darwin21.4-clang\" --config target.x86_64-apple-darwin.ar=\"x86_64-apple-darwin21.4-ar\" || return 1
PATH="$HOME/projects/osxcross/target/bin:$PATH" cargo build --profile release-lto-darwin --target aarch64-apple-darwin --config target.aarch64-apple-darwin.linker=\"aarch64-apple-darwin21.4-clang\" --config target.aarch64-apple-darwin.ar=\"aarch64-apple-darwin21.4-ar\" || return 1
set +x
}

zip() (

  if [ -d "${folder}" ]; then
    echo "${folder} exists"
    return 1
  fi
  if [ -f "${zip}" ]; then
    rm -v "${zip}" || return 1
  fi
  mkdir -pv "${folder}/linux_x86-64" || return 1
  cp    -vt "${folder}/linux_x86-64"\
    "target/x86_64-unknown-linux-gnu/release-lto/${binary}" || return 1
  mkdir -pv "${folder}/linux_x86-64_musl" || return 1
  cp    -vt "${folder}/linux_x86-64_musl"\
    "target/x86_64-unknown-linux-musl/release-lto/${binary}" || return 1
  mkdir -pv "${folder}/android_aarch64" || return 1
  cp    -vt "${folder}/android_aarch64"\
    "target/aarch64-linux-android/release-lto/${binary}" || return 1
  mkdir -pv "${folder}/windows_x86-64_msvc" || return 1
  cp    -vt "${folder}/windows_x86-64_msvc"\
    "target/x86_64-pc-windows-msvc/release-lto/${binary}.exe" || return 1
  mkdir -pv "${folder}/windows_x86-64_gnu" || return 1
  cp    -vt "${folder}/windows_x86-64_gnu"\
    "target/x86_64-pc-windows-gnu/release-lto/${binary}.exe" || return 1
  mkdir -pv "${folder}/macos_x86-64" || return 1
  cp    -vt "${folder}/macos_x86-64"\
    "target/x86_64-apple-darwin/release-lto-darwin/${binary}" || return 1
  mkdir -pv "${folder}/macos_aarch64" || return 1
  cp    -vt "${folder}/macos_aarch64"\
    "target/aarch64-apple-darwin/release-lto-darwin/${binary}" || return 1
  mkdir -pv "${docs}" || return 1
  set -x
  ./target/debug/${binary} -h 2> "${docs}/help_brief.txt"
  ./target/debug/${binary} --help 2> "${docs}/help_extended.txt"
  ./target/debug/${binary} --no-log --settings-write --settings "${docs}/${binary}.toml"
  set +x

  7z a "${zip}" "${folder}" -tzip || return 1
  7z t "${zip}" || return 1
  7z l "${zip}" || return 1
  md5sum "${zip}" || return 1
  rm -r "${folder}" || return 1
)

main() {
build || return 1
if [ "${1}" == "zip" ]; then
  zip || return 1
fi
}

main "$@" || echo "error"

# [Build for your platform]
# RUSTFLAGS="-C target-cpu=native" cargo build --profile release-lto

# [Preparations on arch-linux to build for other platforms]
# rustup target add x86_64-unknown-linux-gnu x86_64-unknown-linux-musl x86_64-pc-windows-gnu x86_64-pc-windows-gnu x86_64-apple-darwin aarch64-apple-darwin
# [Preparations:android]
# yay -S android-ndk cargo-ndk
# [Preparations:windows_GNU]
# yay -S mingw-w64-gcc
# [Preparations:windows_MSVC]
# cargo install cargo-xwin
# [Preparations:macOS]
# yay -S clang
# # https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html
# # https://github.com/tpoechtrager/osxcross - how to get sdk
# # go to mac, install homebrew with Xcode Command Line Tools
# git clone https://github.com/tpoechtrager/osxcross.git
# cd osxcross/
# ./tools/gen_sdk_package_tools.sh
# # transfer files to linux
# git clone https://github.com/tpoechtrager/osxcross.git
# cd osxcross/
# cp ../MacOSX12.3.sdk.tar.xz tarballs/
# ./build.sh

# [PGO template] Doesn't improve anything for this project.
# rustup component add llvm-tools-preview
# rm -rf /tmp/pgo-data/
# RUSTFLAGS="-C target-cpu=native -C profile-generate=/tmp/pgo-data" cargo build --profile release-lto
# ./target/release/jobasha
# ./target/release/jobasha --skip-last 1 --no-delete-plugins ""
# ./target/release/jobasha --skip-last 2 --log -v
# ./target/release/jobasha --skip-last 3 --threshold-creatures 33 -q
# ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
# RUSTFLAGS="-C target-cpu=native -C profile-use=/tmp/pgo-data/merged.profdata" cargo build --profile release-lto
