@start
    mov %1 0
    bl @loop
    eq %1 10
    assert

    end

@loop
    add %1 %1 1
    neq %1 10
    brif @loop
    ret
