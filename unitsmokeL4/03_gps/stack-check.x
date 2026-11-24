/* SRAM1 contains statics and the normal Cortex-M stack; SRAM2 is unused. */
ASSERT(__euninit <= _stack_start - 16K, "L4 GPS requires at least 16 KiB stack headroom");
