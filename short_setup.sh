rustup default nightly
rustup component add rust-src
cargo install xargo
mkdir src
cd src
wget http://ftpmirror.gnu.org/binutils/binutils-2.30.tar.gz
tar -xzf binutils-2.30.tar.gz 
rm binutils-2.30.tar.gz 
export PREFIX="$HOME/opt/cross"
export TARGET=aarch64-unknown-linux-gnueabi
export PATH="$PREFIX/bin:$PATH"
mkdir build-binutils
cd build-binutils/
../binutils-2.30/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror --enable-shared --disable-threads --enable-libmpx --with-system-zlib --enable-__cxa_atexit --disable-libunwind-exceptions --enable-clocale=gnu --disable-libstdcxx-pch --disable-libssp --enable-plugin --disable-linker-build-id --enable-lto --enable-install-libiberty --with-linker-hash-style=gnu --with-gnu-ld --enable-gnu-indirect-function --disable-multilib --disable-werror --enable-checking=release --enable-default-pie --enable-default-ssp --enable-gnu-unique-object
make -j4
make check
make install
cd
cd git
git clone https://github.com/qemu/qemu.git
cd qemu
mkdir build
cd build
../configure --python=`which python3` --target-list=aarch64-softmmu
make -j4
sudo make install
