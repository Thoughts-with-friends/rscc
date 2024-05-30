//    ---     ---
// __|   |___|   |___
module flip_flop(
    input wire CLK,
    input wire RST,
    input wire A,
    output reg Q
    );

    always @(posedge CLK) begin
        if(RST) Q <= 1'd00;  // 1bit, value 0
        else if(A)  Q <= !Q;
    end
endmodule
