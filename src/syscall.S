.global kaku
.global yameru

kaku:
    movq $0x0000001, %rax
    syscall
    retq

yameru:
    movq $0x000003c, %rax
    syscall