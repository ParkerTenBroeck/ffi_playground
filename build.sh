cd rust_side
RUSTFLAGS="--emit=llvm-bc"
cargo build --release
cd ..
cd cpp_side
./build.sh
cd ..