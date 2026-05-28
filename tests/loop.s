.text

@start
    mov %a 0
    bl @loop
    eq %a 10
    assert

    end

@loop
    add %a %a 1
    neq %a 10
    brif @loop
    ret
