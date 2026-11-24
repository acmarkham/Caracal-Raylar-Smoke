/* STM32L476 DS10198 section 3.5: SRAM2 is a separate 32 KiB CPU RAM bank.
 * Keep all statics/DMA buffers in the existing SRAM1 region. No alias is used.
 * Stack objects are initialized by normal execution; stack_probe paints and
 * verifies the unused region before peripheral work. No SRAM2 .data/.bss exists.
 */
MEMORY
{
    STACKRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 32K
}
_stack_start = ORIGIN(STACKRAM) + LENGTH(STACKRAM);
_stack_end = ORIGIN(STACKRAM);
ASSERT(__euninit <= ORIGIN(RAM) + LENGTH(RAM), "L4 static/DMA allocation exceeds 96 KiB SRAM1");
ASSERT(_stack_start == 0x10008000 && _stack_end == 0x10000000, "L4 stack must use the verified SRAM2 range");
