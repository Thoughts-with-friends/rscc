// memory that returns the value of the specified address
module MEM(addr, data);
    input [9:0] addr;        // rdi
    output [31:0] data;      // rax

    reg [31:0] mem[1023:0];  // 32bits 4 Kbyte memory

    assign data = mem[addr]; // mov rax, [rdi]
endmodule
