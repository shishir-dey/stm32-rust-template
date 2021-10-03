cargo build
arm-none-eabi-objcopy -O binary target/thumb*/debug/*app firmware.bin
st-flash write firmware.bin 0x8000000
