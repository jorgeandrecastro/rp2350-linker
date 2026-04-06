/* memory.x */
MEMORY
{
    /* Flash sur RP2350 : 4MB sur une Pico 2 standard */
    FLASH : ORIGIN = 0x10000000, LENGTH = 4096K
    RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}

/* On définit les sections spéciales pour le RP2350 */
SECTIONS {
    /* Bloc de démarrage requis par le BootROM du RP2350 */
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH

    /* Les infos binaires pour picotool */
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .vector_table;

/* Le code (.text) doit commencer APRÈS le bloc de démarrage */
_stext = ADDR(.start_block) + SIZEOF(.start_block);