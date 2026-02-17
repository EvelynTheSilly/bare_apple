qemuflags := "-M virt -cpu cortex-a57 -kernel ./build/kernel.elf -semihosting -device ramfb"

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
    qemu-system-aarch64 {{ qemuflags }}

buildrun:
    just build
    just run

debug:
    just build
    echo "running vm"
    echo "exit with ctrl a, then x"
    echo "run mask start_gdb to attatch to the debugger"
    echo ""
    echo ""
    qemu-system-aarch64 {{ qemuflags }} -S -s

gdb:
    aarch64-none-elf-gdb -ex "target remote :1234" -ex "symbol-file build/kernel.elf"
