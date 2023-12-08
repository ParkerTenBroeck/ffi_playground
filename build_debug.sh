cd rust_side
RUSTFLAGS="--emit=llvm-bc"
cargo build 
cd ..
cd cpp_side
./build_debug.sh
cd ..