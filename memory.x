/* memory.x */
MEMORY
{
    /* Flash sur RP2350 : 4MB sur une Pico 2 standard */
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    /* RAM : 512K */
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}