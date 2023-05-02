OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000; # 在使用相对位置的前提下可以偏移

SECTIONS # 段的合并
{
    . = BASE_ADDRESS; # . 表示当前地址，可以对 . 赋值来调整接下来段的位置
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry) # .text.entry 一定放在 .text 的开头
        *(.text .text.*)
    }

    . = ALIGN(4K); # 
    etext = .;
    srodata = .;
    .rodata : { # 段的名字
        *(.rodata .rodata.*) # 顺序描述所有输入目标文件的哪些段放入这个段
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
    # .text -> .rodata -> .data -> .bss
    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}