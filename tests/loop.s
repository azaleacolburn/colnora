@start
    mov %1 0
    jmp @loop
@start_return
    assert
    end

@loop
    add %1 %1 1
    eq %1 10
    jmpif @start_return
    jmp @loop
