    .section .text
    .globl __switch
__switch:
    sd ra, 0*8(a0)
    sd sp, 1*8(a0)
    sd s0, 2*8(a0)
    sd s1, 3*8(a0)
    sd s2, 4*8(a0)
    sd s3, 5*8(a0)
    sd s4, 6*8(a0)
    sd s5, 7*8(a0)
    sd s6, 8*8(a0)
    sd s7, 9*8(a0)
    sd s8, 10*8(a0)
    sd s9, 11*8(a0)
    sd s10, 12*8(a0)
    sd s11, 13*8(a0)
    # When user task scheduled for the second time,
    # overwrite task entry address with the return address
    sd ra, 14*8(a0)

    ld ra, 0*8(a1)
    ld sp, 1*8(a1)
    ld s0, 2*8(a1)
    ld s1, 3*8(a1)
    ld s2, 4*8(a1)
    ld s3, 5*8(a1)
    ld s4, 6*8(a1)
    ld s5, 7*8(a1)
    ld s6, 8*8(a1)
    ld s7, 9*8(a1)
    ld s8, 10*8(a1)
    ld s9, 11*8(a1)
    ld s10, 12*8(a1)
    ld s11, 13*8(a1)
    # When user task scheduled for the first time, t0 will be task entry address.
    # After that, t0 will be return address
    ld t0, 14*8(a1)

    # pseudo instruction, actually is jalr x0, 0(t0)
    jr t0