MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the LPC11U37F */
  FLASH : ORIGIN = 0x00000000, LENGTH = 8K
  RAM : ORIGIN = 0x10000000, LENGTH = 8K
}

/* Match stock Valve bootloader stack top (leaves a small gap below 0x10002000). */
_stack_start = 0x10001C20;
