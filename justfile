build opt="debug":
    mkdir -p ./build
    rm -r ./build
    mkdir -p ./build
    cargo build $( [ {{ opt }} = release ] && printf '%s' --release )
    cp ./target/aarch64-none-custom/debug/kernel ./build/kernel.elf

run:
    @echo "running vm"
    @echo "exit with ctrl a, then x"
    @echo ""
    @echo ""
    qemu-system-aarch64  -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -semihosting

buildrun:
    just build
    just run

debug:
    mask build
    echo "running vm"
    echo "exit with ctrl a, then x"
    echo "run mask start_gdb to attatch to the debugger"
    echo ""
    echo ""
    qemu-system-aarch64 -M virt -cpu cortex-a57 -nographic -kernel ./build/kernel.elf -S -s -semihosting

gdb:
    aarch64-none-elf-gdb -ex "target remote :1234" -ex "symbol-file build/kernel.elf"
