sudo apt-get install m4 curl libz-dev git pkg-config libglib2.0-dev libpixman-1-dev flex bison vim
curl https://sh.rustup.rs -sSf | sh
source ~/.cargo/env 
rustup default nightly
rustup component add rust-src
cargo install xargo
mkdir src
cd src
wget http://ftpmirror.gnu.org/binutils/binutils-2.30.tar.gz
wget http://ftpmirror.gnu.org/gcc/gcc-8.1.0/gcc-8.1.0.tar.gz
wget http://ftpmirror.gnu.org/mpfr/mpfr-4.0.1.tar.gz
wget http://ftpmirror.gnu.org/gmp/gmp-6.1.2.tar.gz
wget http://ftpmirror.gnu.org/gmp/gmp-6.1.2.tar.bz2
wget http://ftpmirror.gnu.org/mpc/mpc-1.1.0.tar.gz
wget ftp://gcc.gnu.org/pub/gcc/infrastructure/isl-0.18.tar.bz2
wget ftp://gcc.gnu.org/pub/gcc/infrastructure/cloog-0.18.1.tar.gz
for i in *.tar.gz; do tar -xzf $i; done
for i in *.tar.bz2; do tar -xjf $i; done
mv *.gz *.bz2 ~/Downloads/
cd gmp-6.1.2/
./configure 
make -j4
make check
sudo make install
cd ../mpfr-4.0.1/
wget http://www.mpfr.org/mpfr-4.0.1/allpatches
patch -N -Z -p1 < allpatches 
./configure
make -j4
make check
sudo make install
cd ../cloog-0.18.1/
./configure 
make -j4
make check
sudo make install
cd ../isl-0.18/
./configure 
make -j4
make check
sudo make install
cd ../mpc-1.1.0/
./configure 
make -j4
make check
sudo make install
cd ..
export PREFIX="$HOME/opt/cross"
export TARGET=aarch64-elf
export PATH="$PREFIX/bin:$PATH"
mkdir build-binutils
cd build-binutils/
../binutils-2.30/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror --enable-shared --disable-threads --enable-libmpx --with-system-zlib --with-isl --enable-__cxa_atexit --disable-libunwind-exceptions --enable-clocale=gnu --disable-libstdcxx-pch --disable-libssp --enable-plugin --disable-linker-build-id --enable-lto --enable-install-libiberty --with-linker-hash-style=gnu --with-gnu-ld --enable-gnu-indirect-function --disable-multilib --disable-werror --enable-checking=release --enable-default-pie --enable-default-ssp --enable-gnu-unique-object
make -j4
make check
make install
cd ..
mkdir build-gcc
cd build-gcc
../gcc-8.1.0/configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers --enable-shared --disable-threads --enable-libmpx --with-system-zlib --with-isl --enable-__cxa_atexit --disable-libunwind-exceptions --enable-clocale=gnu --disable-libstdcxx-pch --disable-libssp --enable-plugin --disable-linker-build-id --enable-lto --enable-install-libiberty --with-linker-hash-style=gnu --with-gnu-ld --enable-gnu-indirect-function --disable-multilib --disable-werror --enable-checking=release --enable-default-pie --enable-default-ssp --enable-gnu-unique-object
make -j4 all-gcc
make -j4 all-target-gcc
make install-gcc
make install-target-libgcc
cd ../build-binutils
export TARGET=aarch64-unknown-linux-gnueabi
make clean
rm */config.cache
../binutils-2.30/configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror --enable-shared --disable-threads --enable-libmpx --with-system-zlib --with-isl --enable-__cxa_atexit --disable-libunwind-exceptions --enable-clocale=gnu --disable-libstdcxx-pch --disable-libssp --enable-plugin --disable-linker-build-id --enable-lto --enable-install-libiberty --with-linker-hash-style=gnu --with-gnu-ld --enable-gnu-indirect-function --disable-multilib --disable-werror --enable-checking=release --enable-default-pie --enable-default-ssp --enable-gnu-unique-object
make -j4
make check
make install
cd
mkdir git
cd git
git clone https://github.com/bztsrc/raspi3-tutorial.git
git clone https://github.com/rsta2/circle.git
git clone https://github.com/andre-richter/rust-raspi3-tutorial.git
git clone https://github.com/qemu/qemu.git
git clone https://github.com/bhoward/SRF2018.git
cd qemu
mkdir build
cd build
../configure --python=/usr/bin/python3 --target-list=aarch64-softmmu
make
sudo make install
# add $HOME/opt/cross/bin to your PATH -- last line of ~/.profile
# reboot (to refresh dynamic library path)
# cd ~/git/raspi3-tutorial/0A_pcscreenfont
# make
# qemu-system-aarch64 -M raspi3 -kernel kernel8.img -serial stdio
# start Remmina and connect to VNC at 127.0.0.1:5900
# Or, in crumbs: make qemu, then start Remmina as above
