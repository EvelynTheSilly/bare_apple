ENTRY(_ENTRY)
SECTIONS
{
	. = ALIGN(16);
	. = 0x40000000;
    .text : {
        *(.text*)
    }
	.data : { *(.data) } 
	.bss : { *(.bss COMMON) } 
    .vectors ALIGN(11): {
        KEEP(*(.vectors))
    }
    .rodata : {
        *(.rodata)
    }
	. = . + 0x1000000; 
	. = ALIGN(16);
	el1_stack_top = .;
}