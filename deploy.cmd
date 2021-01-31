set PATH=%PATH%;E:/arduino/hardware/tools/avr/bin
cargo +nightly-2021-01-07 build --release
IF %errorlevel% neq 0 exit /b %errorlevel%

avrdude -p atmega328p -carduino -D "-Uflash:w:target/avr-atmega328p/release/blinky_led.elf:e" -CE:/arduino/hardware/tools/avr/etc/avrdude.conf -PCOM3