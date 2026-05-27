@start
    mov %1 3
    mov %2 5
    add %1 %1 %2
    add %3 %1 3
    add %1 %3 9
    eq %1 20
    assert
