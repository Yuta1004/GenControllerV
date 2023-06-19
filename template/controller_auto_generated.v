module {NAME}_controller_auto_generated #
    (
        parameter integer C_S_AXI_DATA_WIDTH    = 32,
        parameter integer C_S_AXI_ADDR_WIDTH    = 16
    )
    (
        // 回路接続
        input wire {CLOCK},
        {WIRE_DEFINITIONS}

        // AXIバス
        input wire                              S_AXI_ACLK,
        input wire                              S_AXI_ARSTN,
        input wire [C_S_AXI_ADDR_WIDTH-1:0]     S_AXI_AWADDR,
        input wire                              S_AXI_AWVALID,
        output wire                             S_AXI_AWREADY,
        input wire [C_S_AXI_DATA_WIDTH-1:0]     S_AXI_WDATA,
        input wire                              S_AXI_WVALID,
        output wire                             S_AXI_WREADY,
        input wire [C_S_AXI_ADDR_WIDTH-1:0]     S_AXI_ARADDR,

        output reg [C_S_AXI_DATA_WIDTH-1:0]     reg_data_out
    );

    /* ----- AXIバス ==> 接続回路 ----- */

    // 書き込みチェック信号
    assign slv_reg_wren = S_AXI_WREADY && S_AXI_WVALID && S_AXI_AWREADY && S_AXI_AWVALID;

    // SLVレジスタ書き込み
    reg [C_S_AXI_DATA_WIDTH-1:0] {SLV_REGISTERS};

    always @(posedge S_AXI_ACLK) begin
        if (S_AXI_ARSTN == 1'b0) begin
            {SLV_REGISTERS_RESET}
        end
        else begin
            if (slv_reg_wren) begin
                case (S_AXI_AWADDR)
                    {SLV_REGISTERS_SET}
                endcase
            end
        end
    end

    // 接続回路への出力
    {SLV_OCACHE_REGISTERS}

    {SLV_OCACHE_REGISTERS_ASSIGN}

    always @ (posedge {CLOCK}) begin
        if (S_AXI_ARSTN == 1'b0) begin
            {SLV_OCACHE_REGISTERS_RESET}
        end
        else begin
            {SLV_OCACHE_REGISTERS_SET}
        end
    end

    /* ----- 接続回路 ==> AXIバス ----- */

    // 接続回路からの入力
    {SLV_ICACHE_REGISTERS}

    always @ (posedge S_AXI_ACLK) begin
        {SLV_ICACHE_REGISTERS_SET}
    end

    // AXIバスへの出力生成
    always @* begin
        case (S_AXI_ARADDR)
            {SLV_ICACHE_REGISTERS_ASSIGN}
            default     : reg_data_out <= 0;
        endcase
    end

endmodule
