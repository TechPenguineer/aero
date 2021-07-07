initSidebarItems({"constant":[["IA32_APIC_BASE","APIC Location and Status (R/W)."],["IA32_EFER",""],["IA32_FMASK","System Call Flag Mask (R/W)."],["IA32_FS_BASE",""],["IA32_KERNEL_GSBASE","Swap Target of BASE Address of GS (R/W)."],["IA32_LSTAR","IA-32e Mode System Call Target Address (R/W)."],["IA32_STAR","System Call Target Address (R/W)."],["IA32_X2APIC_APICID","x2APIC ID register (R/O) See X2APIC Specification."],["IA32_X2APIC_EOI","x2APIC End of Interrupt."],["IA32_X2APIC_ESR","Error Status Register."],["IA32_X2APIC_ICR","x2APIC Interrupt Command register (R/W)"],["IA32_X2APIC_LVT_ERROR",""],["IA32_X2APIC_SIVR","x2APIC Spurious Interrupt Vector register (R/W)"]],"fn":[["delay",""],["inb","Wrapper function to the `inb` assembly instruction used to do the low level port input."],["inl","Wrapper function to the `inl` assembly instruction used to do the low level port input."],["outb","Wrapper function to the `outb` assembly instruction used to do the low level port output."],["outl","Wrapper function to the `outl` assembly instruction used to do the low level port output."],["rdmsr","Wrapper function to the `rdmsr` assembly instruction used"],["wait","This function is called after every `outb` and `outl` instruction as on older machines its necessary to give the PIC some time to react to commands as they might not be processed quickly."],["wrmsr","Wrapper function to the `wrmsr` assembly instruction used to write 64 bits to msr register."]]});