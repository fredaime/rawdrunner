ISO=aetherum.iso
all: bootimage $(ISO)

bootimage:
cargo xbuild --release
cargo bootimage --release

$(ISO): bootimage
xorriso -as mkisofs -o $(ISO) target/x86_64-unknown-none/release/bootimage-aetherum-kernel.bin

run: $(ISO)
qemu-system-x86_64 -drive format=raw,file=$(ISO) -serial stdio -no-reboot -d int

clean:
cargo clean
rm -f $(ISO)
