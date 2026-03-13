@REM  Modified by KotukoHumibana

rust-objcopy -O binary %1 %1.bin
minichlink -c 0x1209b803 -w %1.bin 0x08000000 -b