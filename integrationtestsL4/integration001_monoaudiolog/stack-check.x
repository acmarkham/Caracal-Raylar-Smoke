/* CPU stack in physical SRAM2; every SD/audio DMA buffer stays in SRAM1. */
MEMORY { STACKRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 32K }
_stack_start = ORIGIN(STACKRAM) + LENGTH(STACKRAM);
_stack_end = ORIGIN(STACKRAM);
ASSERT(__euninit <= 0x20018000, "L4 integration exceeds 96 KiB SRAM1");
ASSERT(_stack_start == 0x10008000 && _stack_end == 0x10000000, "L4 stack must use SRAM2");
