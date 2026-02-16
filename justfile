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
