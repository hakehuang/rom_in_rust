/* Memory layout of the RT685 ROM */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  OTP_API (rx) : ORIGIN = 0x13009000, LENGTH = 256K
  ROM     (rx) : ORIGIN = 0x1303f000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The entry point is the reset handler */
ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{

  .OPT_API_CALL ORIGIN(OTP_API):
  {
    . = ALIGN(4);
    __FSL_ROM_OTP_INIT_ADDR_start__ = 0xFF8;
  *(.FSL_ROM_OTP_INIT_ADDR*)
    __FSL_ROM_OTP_INIT_ADDR_end__ = .;
    . = ALIGN(4);
    __FSL_ROM_OTP_DEINIT_ADDR_start__ = 0x1046;
  *(.FSL_ROM_OTP_DEINIT_ADDR*)
    __FSL_ROM_OTP_DEINIT_ADDR_end__ = .;
    . = ALIGN(4);
    __FSL_ROM_OTP_FUSE_READ_ADDR_start__ = 0x1056;
  *(.FSL_ROM_OTP_FUSE_READ_ADDR*)
    __FSL_ROM_OTP_FUSE_READ_ADDR_end__ = .;
  } > OTP_API

  .vector_table ORIGIN(ROM) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > ROM

  .text :
  {
    *(.text .text.*);
  } > ROM

  /* CHANGED! */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > ROM

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}