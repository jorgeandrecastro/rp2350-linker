/* link-rp2350.x */
SECTIONS {
    /* Bloc de démarrage requis par le RP2350 */
    .start_block : ALIGN(4)
    {
        __start_block_addr = .;
        KEEP(*(.start_block));
        KEEP(*(.boot_info));
    } > FLASH

    /* Infos binaires pour picotool */
    .bi_entries : ALIGN(4)
    {
        __bi_entries_start = .;
        KEEP(*(.bi_entries));
        . = ALIGN(4);
        __bi_entries_end = .;
    } > FLASH
} INSERT AFTER .vector_table;

/* Le code commence après le bloc de boot */
_stext = ADDR(.start_block) + SIZEOF(.start_block);