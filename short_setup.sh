rustup default nightly
rustup component add rust-src llvm-tools
cargo install xargo cargo-binutils
cd
cd git
git clone https://github.com/qemu/qemu.git
cd qemu
mkdir build
cd build
../configure --python=`which python3` --target-list=aarch64-softmmu
make -j4
sudo make install
