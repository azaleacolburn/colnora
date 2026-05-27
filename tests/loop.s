@start
    mov %1 0
    bl @loop
@start_return
    assert
    end

@loop
    add %1 %1 1
    eq %1 10
    blif @start_return
    bl @loop
