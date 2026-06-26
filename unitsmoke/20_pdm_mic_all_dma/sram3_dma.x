SECTIONS
{
  .sram3_dma 0x200D0000 (NOLOAD) :
  {
    . = ALIGN(32);
    __ssram3_dma = .;
    *(.sram3_dma .sram3_dma.*);
    . = ALIGN(32);
    __esram3_dma = .;
  } > RAM
}

ASSERT(__esram3_dma <= 0x201A0000, "
ERROR(unit-smoke-20-pdm-mic-all-dma): .sram3_dma exceeds SRAM3");
