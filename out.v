/* Machine-generated using Migen */
module top(
	input rio_clk,
	input rio_rst,
	input sys_clk,
	input sys_rst
);

reg camera_link_pads_clk_p = 1'd0;
reg camera_link_pads_sdi_p = 1'd0;
reg camera_link_pads_sdi_n = 1'd0;
reg camera_link_pads_clk_n = 1'd0;
reg ointerface0_stb = 1'd0;
reg [11:0] ointerface0_data = 12'd0;
reg [7:0] ointerface0_address = 8'd0;
reg ointerface1_stb = 1'd0;
reg [63:0] ointerface1_data = 64'd0;
reg iinterface_stb;
reg [31:0] iinterface_data;
reg pll_reset_storage = 1'd1;
wire pll_locked_status;
reg phase_shift_re = 1'd0;
reg phase_shift_r = 1'd0;
reg phase_shift_done_status = 1'd1;
wire [6:0] clk_sampled_status;
wire [6:0] q_clk;
wire [6:0] q;
wire cl_clk;
wire cl_rst;
wire cl7x_clk;
wire clk_se;
wire clk_se_iserdes;
wire sdi_se;
(* no_retiming = "true" *) reg pll_reset = 1'd1;
wire mmcm_fb;
wire mmcm_locked;
wire mmcm_ps_psdone;
wire cl7x_clk_1;
reg [7:0] frequency_counter_status = 8'd0;
(* no_retiming = "true" *) reg frequency_counter_toggle = 1'd0;
wire frequency_counter_toggle_sys;
reg [8:0] frequency_counter_timer = 9'd0;
reg frequency_counter_tick = 1'd1;
reg [7:0] frequency_counter_count = 8'd0;
reg frequency_counter_toggle_sys_r = 1'd0;
wire [27:0] cl;
wire [11:0] last_x_status;
wire [11:0] last_y_status;
reg [11:0] pix_x = 12'd0;
reg [11:0] pix_y = 12'd0;
wire [7:0] pix_a;
wire [7:0] pix_b;
wire [7:0] pix_c;
wire pix_stb;
wire pix_eop;
(* no_retiming = "true" *) reg [11:0] last_x = 12'd0;
(* no_retiming = "true" *) reg [11:0] last_y = 12'd0;
wire lval;
wire fval;
wire dval;
reg last_lval = 1'd0;
reg last_fval = 1'd0;
wire [11:0] roi0_cfg_x0;
wire [11:0] roi0_cfg_x1;
wire [11:0] roi0_cfg_y0;
wire [11:0] roi0_cfg_y1;
reg roi0_out_update = 1'd0;
reg [30:0] roi0_out_count = 31'd0;
reg roi0_y_good = 1'd0;
reg roi0_x_good = 1'd0;
reg roi0_stb = 1'd0;
reg roi0_eop = 1'd0;
reg [15:0] roi0_gray = 16'd0;
reg [30:0] roi0_count = 31'd0;
wire [11:0] roi1_cfg_x0;
wire [11:0] roi1_cfg_x1;
wire [11:0] roi1_cfg_y0;
wire [11:0] roi1_cfg_y1;
reg roi1_out_update = 1'd0;
reg [30:0] roi1_out_count = 31'd0;
reg roi1_y_good = 1'd0;
reg roi1_x_good = 1'd0;
reg roi1_stb = 1'd0;
reg roi1_eop = 1'd0;
reg [15:0] roi1_gray = 16'd0;
reg [30:0] roi1_count = 31'd0;
wire [11:0] roi2_cfg_x0;
wire [11:0] roi2_cfg_x1;
wire [11:0] roi2_cfg_y0;
wire [11:0] roi2_cfg_y1;
reg roi2_out_update = 1'd0;
reg [30:0] roi2_out_count = 31'd0;
reg roi2_y_good = 1'd0;
reg roi2_x_good = 1'd0;
reg roi2_stb = 1'd0;
reg roi2_eop = 1'd0;
reg [15:0] roi2_gray = 16'd0;
reg [30:0] roi2_count = 31'd0;
wire [11:0] roi3_cfg_x0;
wire [11:0] roi3_cfg_x1;
wire [11:0] roi3_cfg_y0;
wire [11:0] roi3_cfg_y1;
reg roi3_out_update = 1'd0;
reg [30:0] roi3_out_count = 31'd0;
reg roi3_y_good = 1'd0;
reg roi3_x_good = 1'd0;
reg roi3_stb = 1'd0;
reg roi3_eop = 1'd0;
reg [15:0] roi3_gray = 16'd0;
reg [30:0] roi3_count = 31'd0;
wire [11:0] roi4_cfg_x0;
wire [11:0] roi4_cfg_x1;
wire [11:0] roi4_cfg_y0;
wire [11:0] roi4_cfg_y1;
reg roi4_out_update = 1'd0;
reg [30:0] roi4_out_count = 31'd0;
reg roi4_y_good = 1'd0;
reg roi4_x_good = 1'd0;
reg roi4_stb = 1'd0;
reg roi4_eop = 1'd0;
reg [15:0] roi4_gray = 16'd0;
reg [30:0] roi4_count = 31'd0;
wire [11:0] roi5_cfg_x0;
wire [11:0] roi5_cfg_x1;
wire [11:0] roi5_cfg_y0;
wire [11:0] roi5_cfg_y1;
reg roi5_out_update = 1'd0;
reg [30:0] roi5_out_count = 31'd0;
reg roi5_y_good = 1'd0;
reg roi5_x_good = 1'd0;
reg roi5_stb = 1'd0;
reg roi5_eop = 1'd0;
reg [15:0] roi5_gray = 16'd0;
reg [30:0] roi5_count = 31'd0;
wire [11:0] roi6_cfg_x0;
wire [11:0] roi6_cfg_x1;
wire [11:0] roi6_cfg_y0;
wire [11:0] roi6_cfg_y1;
reg roi6_out_update = 1'd0;
reg [30:0] roi6_out_count = 31'd0;
reg roi6_y_good = 1'd0;
reg roi6_x_good = 1'd0;
reg roi6_stb = 1'd0;
reg roi6_eop = 1'd0;
reg [15:0] roi6_gray = 16'd0;
reg [30:0] roi6_count = 31'd0;
wire [11:0] roi7_cfg_x0;
wire [11:0] roi7_cfg_x1;
wire [11:0] roi7_cfg_y0;
wire [11:0] roi7_cfg_y1;
reg roi7_out_update = 1'd0;
reg [30:0] roi7_out_count = 31'd0;
reg roi7_y_good = 1'd0;
reg roi7_x_good = 1'd0;
reg roi7_stb = 1'd0;
reg roi7_eop = 1'd0;
reg [15:0] roi7_gray = 16'd0;
reg [30:0] roi7_count = 31'd0;
wire [11:0] roi8_cfg_x0;
wire [11:0] roi8_cfg_x1;
wire [11:0] roi8_cfg_y0;
wire [11:0] roi8_cfg_y1;
reg roi8_out_update = 1'd0;
reg [30:0] roi8_out_count = 31'd0;
reg roi8_y_good = 1'd0;
reg roi8_x_good = 1'd0;
reg roi8_stb = 1'd0;
reg roi8_eop = 1'd0;
reg [15:0] roi8_gray = 16'd0;
reg [30:0] roi8_count = 31'd0;
wire [11:0] roi9_cfg_x0;
wire [11:0] roi9_cfg_x1;
wire [11:0] roi9_cfg_y0;
wire [11:0] roi9_cfg_y1;
reg roi9_out_update = 1'd0;
reg [30:0] roi9_out_count = 31'd0;
reg roi9_y_good = 1'd0;
reg roi9_x_good = 1'd0;
reg roi9_stb = 1'd0;
reg roi9_eop = 1'd0;
reg [15:0] roi9_gray = 16'd0;
reg [30:0] roi9_count = 31'd0;
wire [11:0] roi10_cfg_x0;
wire [11:0] roi10_cfg_x1;
wire [11:0] roi10_cfg_y0;
wire [11:0] roi10_cfg_y1;
reg roi10_out_update = 1'd0;
reg [30:0] roi10_out_count = 31'd0;
reg roi10_y_good = 1'd0;
reg roi10_x_good = 1'd0;
reg roi10_stb = 1'd0;
reg roi10_eop = 1'd0;
reg [15:0] roi10_gray = 16'd0;
reg [30:0] roi10_count = 31'd0;
wire [11:0] roi11_cfg_x0;
wire [11:0] roi11_cfg_x1;
wire [11:0] roi11_cfg_y0;
wire [11:0] roi11_cfg_y1;
reg roi11_out_update = 1'd0;
reg [30:0] roi11_out_count = 31'd0;
reg roi11_y_good = 1'd0;
reg roi11_x_good = 1'd0;
reg roi11_stb = 1'd0;
reg roi11_eop = 1'd0;
reg [15:0] roi11_gray = 16'd0;
reg [30:0] roi11_count = 31'd0;
wire [11:0] roi12_cfg_x0;
wire [11:0] roi12_cfg_x1;
wire [11:0] roi12_cfg_y0;
wire [11:0] roi12_cfg_y1;
reg roi12_out_update = 1'd0;
reg [30:0] roi12_out_count = 31'd0;
reg roi12_y_good = 1'd0;
reg roi12_x_good = 1'd0;
reg roi12_stb = 1'd0;
reg roi12_eop = 1'd0;
reg [15:0] roi12_gray = 16'd0;
reg [30:0] roi12_count = 31'd0;
wire [11:0] roi13_cfg_x0;
wire [11:0] roi13_cfg_x1;
wire [11:0] roi13_cfg_y0;
wire [11:0] roi13_cfg_y1;
reg roi13_out_update = 1'd0;
reg [30:0] roi13_out_count = 31'd0;
reg roi13_y_good = 1'd0;
reg roi13_x_good = 1'd0;
reg roi13_stb = 1'd0;
reg roi13_eop = 1'd0;
reg [15:0] roi13_gray = 16'd0;
reg [30:0] roi13_count = 31'd0;
wire [11:0] roi14_cfg_x0;
wire [11:0] roi14_cfg_x1;
wire [11:0] roi14_cfg_y0;
wire [11:0] roi14_cfg_y1;
reg roi14_out_update = 1'd0;
reg [30:0] roi14_out_count = 31'd0;
reg roi14_y_good = 1'd0;
reg roi14_x_good = 1'd0;
reg roi14_stb = 1'd0;
reg roi14_eop = 1'd0;
reg [15:0] roi14_gray = 16'd0;
reg [30:0] roi14_count = 31'd0;
wire [11:0] roi15_cfg_x0;
wire [11:0] roi15_cfg_x1;
wire [11:0] roi15_cfg_y0;
wire [11:0] roi15_cfg_y1;
reg roi15_out_update = 1'd0;
reg [30:0] roi15_out_count = 31'd0;
reg roi15_y_good = 1'd0;
reg roi15_x_good = 1'd0;
reg roi15_stb = 1'd0;
reg roi15_eop = 1'd0;
reg [15:0] roi15_gray = 16'd0;
reg [30:0] roi15_count = 31'd0;
wire [11:0] roi16_cfg_x0;
wire [11:0] roi16_cfg_x1;
wire [11:0] roi16_cfg_y0;
wire [11:0] roi16_cfg_y1;
reg roi16_out_update = 1'd0;
reg [30:0] roi16_out_count = 31'd0;
reg roi16_y_good = 1'd0;
reg roi16_x_good = 1'd0;
reg roi16_stb = 1'd0;
reg roi16_eop = 1'd0;
reg [15:0] roi16_gray = 16'd0;
reg [30:0] roi16_count = 31'd0;
wire [11:0] roi17_cfg_x0;
wire [11:0] roi17_cfg_x1;
wire [11:0] roi17_cfg_y0;
wire [11:0] roi17_cfg_y1;
reg roi17_out_update = 1'd0;
reg [30:0] roi17_out_count = 31'd0;
reg roi17_y_good = 1'd0;
reg roi17_x_good = 1'd0;
reg roi17_stb = 1'd0;
reg roi17_eop = 1'd0;
reg [15:0] roi17_gray = 16'd0;
reg [30:0] roi17_count = 31'd0;
wire [11:0] roi18_cfg_x0;
wire [11:0] roi18_cfg_x1;
wire [11:0] roi18_cfg_y0;
wire [11:0] roi18_cfg_y1;
reg roi18_out_update = 1'd0;
reg [30:0] roi18_out_count = 31'd0;
reg roi18_y_good = 1'd0;
reg roi18_x_good = 1'd0;
reg roi18_stb = 1'd0;
reg roi18_eop = 1'd0;
reg [15:0] roi18_gray = 16'd0;
reg [30:0] roi18_count = 31'd0;
wire [11:0] roi19_cfg_x0;
wire [11:0] roi19_cfg_x1;
wire [11:0] roi19_cfg_y0;
wire [11:0] roi19_cfg_y1;
reg roi19_out_update = 1'd0;
reg [30:0] roi19_out_count = 31'd0;
reg roi19_y_good = 1'd0;
reg roi19_x_good = 1'd0;
reg roi19_stb = 1'd0;
reg roi19_eop = 1'd0;
reg [15:0] roi19_gray = 16'd0;
reg [30:0] roi19_count = 31'd0;
wire [11:0] roi20_cfg_x0;
wire [11:0] roi20_cfg_x1;
wire [11:0] roi20_cfg_y0;
wire [11:0] roi20_cfg_y1;
reg roi20_out_update = 1'd0;
reg [30:0] roi20_out_count = 31'd0;
reg roi20_y_good = 1'd0;
reg roi20_x_good = 1'd0;
reg roi20_stb = 1'd0;
reg roi20_eop = 1'd0;
reg [15:0] roi20_gray = 16'd0;
reg [30:0] roi20_count = 31'd0;
wire [11:0] roi21_cfg_x0;
wire [11:0] roi21_cfg_x1;
wire [11:0] roi21_cfg_y0;
wire [11:0] roi21_cfg_y1;
reg roi21_out_update = 1'd0;
reg [30:0] roi21_out_count = 31'd0;
reg roi21_y_good = 1'd0;
reg roi21_x_good = 1'd0;
reg roi21_stb = 1'd0;
reg roi21_eop = 1'd0;
reg [15:0] roi21_gray = 16'd0;
reg [30:0] roi21_count = 31'd0;
wire [11:0] roi22_cfg_x0;
wire [11:0] roi22_cfg_x1;
wire [11:0] roi22_cfg_y0;
wire [11:0] roi22_cfg_y1;
reg roi22_out_update = 1'd0;
reg [30:0] roi22_out_count = 31'd0;
reg roi22_y_good = 1'd0;
reg roi22_x_good = 1'd0;
reg roi22_stb = 1'd0;
reg roi22_eop = 1'd0;
reg [15:0] roi22_gray = 16'd0;
reg [30:0] roi22_count = 31'd0;
wire [11:0] roi23_cfg_x0;
wire [11:0] roi23_cfg_x1;
wire [11:0] roi23_cfg_y0;
wire [11:0] roi23_cfg_y1;
reg roi23_out_update = 1'd0;
reg [30:0] roi23_out_count = 31'd0;
reg roi23_y_good = 1'd0;
reg roi23_x_good = 1'd0;
reg roi23_stb = 1'd0;
reg roi23_eop = 1'd0;
reg [15:0] roi23_gray = 16'd0;
reg [30:0] roi23_count = 31'd0;
wire [11:0] roi24_cfg_x0;
wire [11:0] roi24_cfg_x1;
wire [11:0] roi24_cfg_y0;
wire [11:0] roi24_cfg_y1;
reg roi24_out_update = 1'd0;
reg [30:0] roi24_out_count = 31'd0;
reg roi24_y_good = 1'd0;
reg roi24_x_good = 1'd0;
reg roi24_stb = 1'd0;
reg roi24_eop = 1'd0;
reg [15:0] roi24_gray = 16'd0;
reg [30:0] roi24_count = 31'd0;
wire [11:0] roi25_cfg_x0;
wire [11:0] roi25_cfg_x1;
wire [11:0] roi25_cfg_y0;
wire [11:0] roi25_cfg_y1;
reg roi25_out_update = 1'd0;
reg [30:0] roi25_out_count = 31'd0;
reg roi25_y_good = 1'd0;
reg roi25_x_good = 1'd0;
reg roi25_stb = 1'd0;
reg roi25_eop = 1'd0;
reg [15:0] roi25_gray = 16'd0;
reg [30:0] roi25_count = 31'd0;
wire [11:0] roi26_cfg_x0;
wire [11:0] roi26_cfg_x1;
wire [11:0] roi26_cfg_y0;
wire [11:0] roi26_cfg_y1;
reg roi26_out_update = 1'd0;
reg [30:0] roi26_out_count = 31'd0;
reg roi26_y_good = 1'd0;
reg roi26_x_good = 1'd0;
reg roi26_stb = 1'd0;
reg roi26_eop = 1'd0;
reg [15:0] roi26_gray = 16'd0;
reg [30:0] roi26_count = 31'd0;
wire [11:0] roi27_cfg_x0;
wire [11:0] roi27_cfg_x1;
wire [11:0] roi27_cfg_y0;
wire [11:0] roi27_cfg_y1;
reg roi27_out_update = 1'd0;
reg [30:0] roi27_out_count = 31'd0;
reg roi27_y_good = 1'd0;
reg roi27_x_good = 1'd0;
reg roi27_stb = 1'd0;
reg roi27_eop = 1'd0;
reg [15:0] roi27_gray = 16'd0;
reg [30:0] roi27_count = 31'd0;
wire [11:0] roi28_cfg_x0;
wire [11:0] roi28_cfg_x1;
wire [11:0] roi28_cfg_y0;
wire [11:0] roi28_cfg_y1;
reg roi28_out_update = 1'd0;
reg [30:0] roi28_out_count = 31'd0;
reg roi28_y_good = 1'd0;
reg roi28_x_good = 1'd0;
reg roi28_stb = 1'd0;
reg roi28_eop = 1'd0;
reg [15:0] roi28_gray = 16'd0;
reg [30:0] roi28_count = 31'd0;
wire [11:0] roi29_cfg_x0;
wire [11:0] roi29_cfg_x1;
wire [11:0] roi29_cfg_y0;
wire [11:0] roi29_cfg_y1;
reg roi29_out_update = 1'd0;
reg [30:0] roi29_out_count = 31'd0;
reg roi29_y_good = 1'd0;
reg roi29_x_good = 1'd0;
reg roi29_stb = 1'd0;
reg roi29_eop = 1'd0;
reg [15:0] roi29_gray = 16'd0;
reg [30:0] roi29_count = 31'd0;
wire [11:0] roi30_cfg_x0;
wire [11:0] roi30_cfg_x1;
wire [11:0] roi30_cfg_y0;
wire [11:0] roi30_cfg_y1;
reg roi30_out_update = 1'd0;
reg [30:0] roi30_out_count = 31'd0;
reg roi30_y_good = 1'd0;
reg roi30_x_good = 1'd0;
reg roi30_stb = 1'd0;
reg roi30_eop = 1'd0;
reg [15:0] roi30_gray = 16'd0;
reg [30:0] roi30_count = 31'd0;
wire [11:0] roi31_cfg_x0;
wire [11:0] roi31_cfg_x1;
wire [11:0] roi31_cfg_y0;
wire [11:0] roi31_cfg_y1;
reg roi31_out_update = 1'd0;
reg [30:0] roi31_out_count = 31'd0;
reg roi31_y_good = 1'd0;
reg roi31_x_good = 1'd0;
reg roi31_stb = 1'd0;
reg roi31_eop = 1'd0;
reg [15:0] roi31_gray = 16'd0;
reg [30:0] roi31_count = 31'd0;
wire [11:0] roi32_cfg_x0;
wire [11:0] roi32_cfg_x1;
wire [11:0] roi32_cfg_y0;
wire [11:0] roi32_cfg_y1;
reg roi32_out_update = 1'd0;
reg [30:0] roi32_out_count = 31'd0;
reg roi32_y_good = 1'd0;
reg roi32_x_good = 1'd0;
reg roi32_stb = 1'd0;
reg roi32_eop = 1'd0;
reg [15:0] roi32_gray = 16'd0;
reg [30:0] roi32_count = 31'd0;
wire [11:0] roi33_cfg_x0;
wire [11:0] roi33_cfg_x1;
wire [11:0] roi33_cfg_y0;
wire [11:0] roi33_cfg_y1;
reg roi33_out_update = 1'd0;
reg [30:0] roi33_out_count = 31'd0;
reg roi33_y_good = 1'd0;
reg roi33_x_good = 1'd0;
reg roi33_stb = 1'd0;
reg roi33_eop = 1'd0;
reg [15:0] roi33_gray = 16'd0;
reg [30:0] roi33_count = 31'd0;
wire [11:0] roi34_cfg_x0;
wire [11:0] roi34_cfg_x1;
wire [11:0] roi34_cfg_y0;
wire [11:0] roi34_cfg_y1;
reg roi34_out_update = 1'd0;
reg [30:0] roi34_out_count = 31'd0;
reg roi34_y_good = 1'd0;
reg roi34_x_good = 1'd0;
reg roi34_stb = 1'd0;
reg roi34_eop = 1'd0;
reg [15:0] roi34_gray = 16'd0;
reg [30:0] roi34_count = 31'd0;
wire [11:0] roi35_cfg_x0;
wire [11:0] roi35_cfg_x1;
wire [11:0] roi35_cfg_y0;
wire [11:0] roi35_cfg_y1;
reg roi35_out_update = 1'd0;
reg [30:0] roi35_out_count = 31'd0;
reg roi35_y_good = 1'd0;
reg roi35_x_good = 1'd0;
reg roi35_stb = 1'd0;
reg roi35_eop = 1'd0;
reg [15:0] roi35_gray = 16'd0;
reg [30:0] roi35_count = 31'd0;
wire [11:0] roi36_cfg_x0;
wire [11:0] roi36_cfg_x1;
wire [11:0] roi36_cfg_y0;
wire [11:0] roi36_cfg_y1;
reg roi36_out_update = 1'd0;
reg [30:0] roi36_out_count = 31'd0;
reg roi36_y_good = 1'd0;
reg roi36_x_good = 1'd0;
reg roi36_stb = 1'd0;
reg roi36_eop = 1'd0;
reg [15:0] roi36_gray = 16'd0;
reg [30:0] roi36_count = 31'd0;
wire [11:0] roi37_cfg_x0;
wire [11:0] roi37_cfg_x1;
wire [11:0] roi37_cfg_y0;
wire [11:0] roi37_cfg_y1;
reg roi37_out_update = 1'd0;
reg [30:0] roi37_out_count = 31'd0;
reg roi37_y_good = 1'd0;
reg roi37_x_good = 1'd0;
reg roi37_stb = 1'd0;
reg roi37_eop = 1'd0;
reg [15:0] roi37_gray = 16'd0;
reg [30:0] roi37_count = 31'd0;
wire [11:0] roi38_cfg_x0;
wire [11:0] roi38_cfg_x1;
wire [11:0] roi38_cfg_y0;
wire [11:0] roi38_cfg_y1;
reg roi38_out_update = 1'd0;
reg [30:0] roi38_out_count = 31'd0;
reg roi38_y_good = 1'd0;
reg roi38_x_good = 1'd0;
reg roi38_stb = 1'd0;
reg roi38_eop = 1'd0;
reg [15:0] roi38_gray = 16'd0;
reg [30:0] roi38_count = 31'd0;
wire [11:0] roi39_cfg_x0;
wire [11:0] roi39_cfg_x1;
wire [11:0] roi39_cfg_y0;
wire [11:0] roi39_cfg_y1;
reg roi39_out_update = 1'd0;
reg [30:0] roi39_out_count = 31'd0;
reg roi39_y_good = 1'd0;
reg roi39_x_good = 1'd0;
reg roi39_stb = 1'd0;
reg roi39_eop = 1'd0;
reg [15:0] roi39_gray = 16'd0;
reg [30:0] roi39_count = 31'd0;
wire [11:0] roi40_cfg_x0;
wire [11:0] roi40_cfg_x1;
wire [11:0] roi40_cfg_y0;
wire [11:0] roi40_cfg_y1;
reg roi40_out_update = 1'd0;
reg [30:0] roi40_out_count = 31'd0;
reg roi40_y_good = 1'd0;
reg roi40_x_good = 1'd0;
reg roi40_stb = 1'd0;
reg roi40_eop = 1'd0;
reg [15:0] roi40_gray = 16'd0;
reg [30:0] roi40_count = 31'd0;
wire [11:0] roi41_cfg_x0;
wire [11:0] roi41_cfg_x1;
wire [11:0] roi41_cfg_y0;
wire [11:0] roi41_cfg_y1;
reg roi41_out_update = 1'd0;
reg [30:0] roi41_out_count = 31'd0;
reg roi41_y_good = 1'd0;
reg roi41_x_good = 1'd0;
reg roi41_stb = 1'd0;
reg roi41_eop = 1'd0;
reg [15:0] roi41_gray = 16'd0;
reg [30:0] roi41_count = 31'd0;
wire [11:0] roi42_cfg_x0;
wire [11:0] roi42_cfg_x1;
wire [11:0] roi42_cfg_y0;
wire [11:0] roi42_cfg_y1;
reg roi42_out_update = 1'd0;
reg [30:0] roi42_out_count = 31'd0;
reg roi42_y_good = 1'd0;
reg roi42_x_good = 1'd0;
reg roi42_stb = 1'd0;
reg roi42_eop = 1'd0;
reg [15:0] roi42_gray = 16'd0;
reg [30:0] roi42_count = 31'd0;
wire [11:0] roi43_cfg_x0;
wire [11:0] roi43_cfg_x1;
wire [11:0] roi43_cfg_y0;
wire [11:0] roi43_cfg_y1;
reg roi43_out_update = 1'd0;
reg [30:0] roi43_out_count = 31'd0;
reg roi43_y_good = 1'd0;
reg roi43_x_good = 1'd0;
reg roi43_stb = 1'd0;
reg roi43_eop = 1'd0;
reg [15:0] roi43_gray = 16'd0;
reg [30:0] roi43_count = 31'd0;
wire [11:0] roi44_cfg_x0;
wire [11:0] roi44_cfg_x1;
wire [11:0] roi44_cfg_y0;
wire [11:0] roi44_cfg_y1;
reg roi44_out_update = 1'd0;
reg [30:0] roi44_out_count = 31'd0;
reg roi44_y_good = 1'd0;
reg roi44_x_good = 1'd0;
reg roi44_stb = 1'd0;
reg roi44_eop = 1'd0;
reg [15:0] roi44_gray = 16'd0;
reg [30:0] roi44_count = 31'd0;
wire [11:0] roi45_cfg_x0;
wire [11:0] roi45_cfg_x1;
wire [11:0] roi45_cfg_y0;
wire [11:0] roi45_cfg_y1;
reg roi45_out_update = 1'd0;
reg [30:0] roi45_out_count = 31'd0;
reg roi45_y_good = 1'd0;
reg roi45_x_good = 1'd0;
reg roi45_stb = 1'd0;
reg roi45_eop = 1'd0;
reg [15:0] roi45_gray = 16'd0;
reg [30:0] roi45_count = 31'd0;
wire [11:0] roi46_cfg_x0;
wire [11:0] roi46_cfg_x1;
wire [11:0] roi46_cfg_y0;
wire [11:0] roi46_cfg_y1;
reg roi46_out_update = 1'd0;
reg [30:0] roi46_out_count = 31'd0;
reg roi46_y_good = 1'd0;
reg roi46_x_good = 1'd0;
reg roi46_stb = 1'd0;
reg roi46_eop = 1'd0;
reg [15:0] roi46_gray = 16'd0;
reg [30:0] roi46_count = 31'd0;
wire [11:0] roi47_cfg_x0;
wire [11:0] roi47_cfg_x1;
wire [11:0] roi47_cfg_y0;
wire [11:0] roi47_cfg_y1;
reg roi47_out_update = 1'd0;
reg [30:0] roi47_out_count = 31'd0;
reg roi47_y_good = 1'd0;
reg roi47_x_good = 1'd0;
reg roi47_stb = 1'd0;
reg roi47_eop = 1'd0;
reg [15:0] roi47_gray = 16'd0;
reg [30:0] roi47_count = 31'd0;
wire [11:0] roi48_cfg_x0;
wire [11:0] roi48_cfg_x1;
wire [11:0] roi48_cfg_y0;
wire [11:0] roi48_cfg_y1;
reg roi48_out_update = 1'd0;
reg [30:0] roi48_out_count = 31'd0;
reg roi48_y_good = 1'd0;
reg roi48_x_good = 1'd0;
reg roi48_stb = 1'd0;
reg roi48_eop = 1'd0;
reg [15:0] roi48_gray = 16'd0;
reg [30:0] roi48_count = 31'd0;
wire [11:0] roi49_cfg_x0;
wire [11:0] roi49_cfg_x1;
wire [11:0] roi49_cfg_y0;
wire [11:0] roi49_cfg_y1;
reg roi49_out_update = 1'd0;
reg [30:0] roi49_out_count = 31'd0;
reg roi49_y_good = 1'd0;
reg roi49_x_good = 1'd0;
reg roi49_stb = 1'd0;
reg roi49_eop = 1'd0;
reg [15:0] roi49_gray = 16'd0;
reg [30:0] roi49_count = 31'd0;
wire [11:0] roi50_cfg_x0;
wire [11:0] roi50_cfg_x1;
wire [11:0] roi50_cfg_y0;
wire [11:0] roi50_cfg_y1;
reg roi50_out_update = 1'd0;
reg [30:0] roi50_out_count = 31'd0;
reg roi50_y_good = 1'd0;
reg roi50_x_good = 1'd0;
reg roi50_stb = 1'd0;
reg roi50_eop = 1'd0;
reg [15:0] roi50_gray = 16'd0;
reg [30:0] roi50_count = 31'd0;
wire [11:0] roi51_cfg_x0;
wire [11:0] roi51_cfg_x1;
wire [11:0] roi51_cfg_y0;
wire [11:0] roi51_cfg_y1;
reg roi51_out_update = 1'd0;
reg [30:0] roi51_out_count = 31'd0;
reg roi51_y_good = 1'd0;
reg roi51_x_good = 1'd0;
reg roi51_stb = 1'd0;
reg roi51_eop = 1'd0;
reg [15:0] roi51_gray = 16'd0;
reg [30:0] roi51_count = 31'd0;
wire [11:0] roi52_cfg_x0;
wire [11:0] roi52_cfg_x1;
wire [11:0] roi52_cfg_y0;
wire [11:0] roi52_cfg_y1;
reg roi52_out_update = 1'd0;
reg [30:0] roi52_out_count = 31'd0;
reg roi52_y_good = 1'd0;
reg roi52_x_good = 1'd0;
reg roi52_stb = 1'd0;
reg roi52_eop = 1'd0;
reg [15:0] roi52_gray = 16'd0;
reg [30:0] roi52_count = 31'd0;
wire [11:0] roi53_cfg_x0;
wire [11:0] roi53_cfg_x1;
wire [11:0] roi53_cfg_y0;
wire [11:0] roi53_cfg_y1;
reg roi53_out_update = 1'd0;
reg [30:0] roi53_out_count = 31'd0;
reg roi53_y_good = 1'd0;
reg roi53_x_good = 1'd0;
reg roi53_stb = 1'd0;
reg roi53_eop = 1'd0;
reg [15:0] roi53_gray = 16'd0;
reg [30:0] roi53_count = 31'd0;
wire [11:0] roi54_cfg_x0;
wire [11:0] roi54_cfg_x1;
wire [11:0] roi54_cfg_y0;
wire [11:0] roi54_cfg_y1;
reg roi54_out_update = 1'd0;
reg [30:0] roi54_out_count = 31'd0;
reg roi54_y_good = 1'd0;
reg roi54_x_good = 1'd0;
reg roi54_stb = 1'd0;
reg roi54_eop = 1'd0;
reg [15:0] roi54_gray = 16'd0;
reg [30:0] roi54_count = 31'd0;
wire [11:0] roi55_cfg_x0;
wire [11:0] roi55_cfg_x1;
wire [11:0] roi55_cfg_y0;
wire [11:0] roi55_cfg_y1;
reg roi55_out_update = 1'd0;
reg [30:0] roi55_out_count = 31'd0;
reg roi55_y_good = 1'd0;
reg roi55_x_good = 1'd0;
reg roi55_stb = 1'd0;
reg roi55_eop = 1'd0;
reg [15:0] roi55_gray = 16'd0;
reg [30:0] roi55_count = 31'd0;
wire [11:0] roi56_cfg_x0;
wire [11:0] roi56_cfg_x1;
wire [11:0] roi56_cfg_y0;
wire [11:0] roi56_cfg_y1;
reg roi56_out_update = 1'd0;
reg [30:0] roi56_out_count = 31'd0;
reg roi56_y_good = 1'd0;
reg roi56_x_good = 1'd0;
reg roi56_stb = 1'd0;
reg roi56_eop = 1'd0;
reg [15:0] roi56_gray = 16'd0;
reg [30:0] roi56_count = 31'd0;
wire [11:0] roi57_cfg_x0;
wire [11:0] roi57_cfg_x1;
wire [11:0] roi57_cfg_y0;
wire [11:0] roi57_cfg_y1;
reg roi57_out_update = 1'd0;
reg [30:0] roi57_out_count = 31'd0;
reg roi57_y_good = 1'd0;
reg roi57_x_good = 1'd0;
reg roi57_stb = 1'd0;
reg roi57_eop = 1'd0;
reg [15:0] roi57_gray = 16'd0;
reg [30:0] roi57_count = 31'd0;
wire [11:0] roi58_cfg_x0;
wire [11:0] roi58_cfg_x1;
wire [11:0] roi58_cfg_y0;
wire [11:0] roi58_cfg_y1;
reg roi58_out_update = 1'd0;
reg [30:0] roi58_out_count = 31'd0;
reg roi58_y_good = 1'd0;
reg roi58_x_good = 1'd0;
reg roi58_stb = 1'd0;
reg roi58_eop = 1'd0;
reg [15:0] roi58_gray = 16'd0;
reg [30:0] roi58_count = 31'd0;
wire [11:0] roi59_cfg_x0;
wire [11:0] roi59_cfg_x1;
wire [11:0] roi59_cfg_y0;
wire [11:0] roi59_cfg_y1;
reg roi59_out_update = 1'd0;
reg [30:0] roi59_out_count = 31'd0;
reg roi59_y_good = 1'd0;
reg roi59_x_good = 1'd0;
reg roi59_stb = 1'd0;
reg roi59_eop = 1'd0;
reg [15:0] roi59_gray = 16'd0;
reg [30:0] roi59_count = 31'd0;
wire [11:0] roi60_cfg_x0;
wire [11:0] roi60_cfg_x1;
wire [11:0] roi60_cfg_y0;
wire [11:0] roi60_cfg_y1;
reg roi60_out_update = 1'd0;
reg [30:0] roi60_out_count = 31'd0;
reg roi60_y_good = 1'd0;
reg roi60_x_good = 1'd0;
reg roi60_stb = 1'd0;
reg roi60_eop = 1'd0;
reg [15:0] roi60_gray = 16'd0;
reg [30:0] roi60_count = 31'd0;
wire [11:0] roi61_cfg_x0;
wire [11:0] roi61_cfg_x1;
wire [11:0] roi61_cfg_y0;
wire [11:0] roi61_cfg_y1;
reg roi61_out_update = 1'd0;
reg [30:0] roi61_out_count = 31'd0;
reg roi61_y_good = 1'd0;
reg roi61_x_good = 1'd0;
reg roi61_stb = 1'd0;
reg roi61_eop = 1'd0;
reg [15:0] roi61_gray = 16'd0;
reg [30:0] roi61_count = 31'd0;
wire [11:0] roi62_cfg_x0;
wire [11:0] roi62_cfg_x1;
wire [11:0] roi62_cfg_y0;
wire [11:0] roi62_cfg_y1;
reg roi62_out_update = 1'd0;
reg [30:0] roi62_out_count = 31'd0;
reg roi62_y_good = 1'd0;
reg roi62_x_good = 1'd0;
reg roi62_stb = 1'd0;
reg roi62_eop = 1'd0;
reg [15:0] roi62_gray = 16'd0;
reg [30:0] roi62_count = 31'd0;
wire [11:0] roi63_cfg_x0;
wire [11:0] roi63_cfg_x1;
wire [11:0] roi63_cfg_y0;
wire [11:0] roi63_cfg_y1;
reg roi63_out_update = 1'd0;
reg [30:0] roi63_out_count = 31'd0;
reg roi63_y_good = 1'd0;
reg roi63_x_good = 1'd0;
reg roi63_stb = 1'd0;
reg roi63_eop = 1'd0;
reg [15:0] roi63_gray = 16'd0;
reg [30:0] roi63_count = 31'd0;
reg synchronizer_update = 1'd0;
wire [30:0] synchronizer0;
wire [30:0] synchronizer1;
wire [30:0] synchronizer2;
wire [30:0] synchronizer3;
wire [30:0] synchronizer4;
wire [30:0] synchronizer5;
wire [30:0] synchronizer6;
wire [30:0] synchronizer7;
wire [30:0] synchronizer8;
wire [30:0] synchronizer9;
wire [30:0] synchronizer10;
wire [30:0] synchronizer11;
wire [30:0] synchronizer12;
wire [30:0] synchronizer13;
wire [30:0] synchronizer14;
wire [30:0] synchronizer15;
wire [30:0] synchronizer16;
wire [30:0] synchronizer17;
wire [30:0] synchronizer18;
wire [30:0] synchronizer19;
wire [30:0] synchronizer20;
wire [30:0] synchronizer21;
wire [30:0] synchronizer22;
wire [30:0] synchronizer23;
wire [30:0] synchronizer24;
wire [30:0] synchronizer25;
wire [30:0] synchronizer26;
wire [30:0] synchronizer27;
wire [30:0] synchronizer28;
wire [30:0] synchronizer29;
wire [30:0] synchronizer30;
wire [30:0] synchronizer31;
wire [30:0] synchronizer32;
wire [30:0] synchronizer33;
wire [30:0] synchronizer34;
wire [30:0] synchronizer35;
wire [30:0] synchronizer36;
wire [30:0] synchronizer37;
wire [30:0] synchronizer38;
wire [30:0] synchronizer39;
wire [30:0] synchronizer40;
wire [30:0] synchronizer41;
wire [30:0] synchronizer42;
wire [30:0] synchronizer43;
wire [30:0] synchronizer44;
wire [30:0] synchronizer45;
wire [30:0] synchronizer46;
wire [30:0] synchronizer47;
wire [30:0] synchronizer48;
wire [30:0] synchronizer49;
wire [30:0] synchronizer50;
wire [30:0] synchronizer51;
wire [30:0] synchronizer52;
wire [30:0] synchronizer53;
wire [30:0] synchronizer54;
wire [30:0] synchronizer55;
wire [30:0] synchronizer56;
wire [30:0] synchronizer57;
wire [30:0] synchronizer58;
wire [30:0] synchronizer59;
wire [30:0] synchronizer60;
wire [30:0] synchronizer61;
wire [30:0] synchronizer62;
wire [30:0] synchronizer63;
reg [63:0] gate0 = 64'd0;
reg [63:0] gate1 = 64'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary0 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary1 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary2 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary3 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary4 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary5 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary6 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary7 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary8 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary9 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary10 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary11 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary12 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary13 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary14 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary15 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary16 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary17 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary18 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary19 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary20 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary21 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary22 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary23 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary24 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary25 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary26 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary27 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary28 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary29 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary30 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary31 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary32 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary33 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary34 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary35 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary36 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary37 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary38 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary39 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary40 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary41 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary42 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary43 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary44 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary45 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary46 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary47 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary48 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary49 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary50 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary51 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary52 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary53 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary54 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary55 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary56 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary57 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary58 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary59 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary60 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary61 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary62 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary63 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary64 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary65 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary66 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary67 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary68 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary69 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary70 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary71 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary72 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary73 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary74 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary75 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary76 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary77 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary78 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary79 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary80 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary81 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary82 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary83 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary84 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary85 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary86 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary87 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary88 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary89 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary90 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary91 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary92 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary93 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary94 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary95 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary96 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary97 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary98 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary99 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary100 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary101 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary102 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary103 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary104 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary105 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary106 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary107 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary108 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary109 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary110 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary111 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary112 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary113 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary114 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary115 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary116 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary117 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary118 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary119 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary120 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary121 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary122 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary123 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary124 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary125 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary126 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary127 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary128 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary129 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary130 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary131 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary132 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary133 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary134 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary135 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary136 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary137 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary138 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary139 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary140 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary141 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary142 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary143 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary144 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary145 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary146 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary147 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary148 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary149 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary150 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary151 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary152 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary153 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary154 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary155 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary156 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary157 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary158 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary159 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary160 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary161 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary162 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary163 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary164 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary165 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary166 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary167 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary168 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary169 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary170 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary171 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary172 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary173 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary174 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary175 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary176 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary177 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary178 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary179 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary180 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary181 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary182 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary183 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary184 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary185 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary186 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary187 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary188 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary189 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary190 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary191 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary192 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary193 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary194 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary195 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary196 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary197 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary198 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary199 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary200 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary201 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary202 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary203 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary204 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary205 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary206 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary207 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary208 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary209 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary210 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary211 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary212 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary213 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary214 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary215 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary216 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary217 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary218 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary219 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary220 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary221 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary222 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary223 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary224 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary225 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary226 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary227 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary228 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary229 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary230 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary231 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary232 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary233 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary234 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary235 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary236 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary237 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary238 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary239 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary240 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary241 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary242 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary243 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary244 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary245 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary246 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary247 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary248 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary249 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary250 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary251 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary252 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary253 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary254 = 12'd0;
(* no_retiming = "true" *) reg [11:0] roi_boundary255 = 12'd0;
reg [6:0] state = 7'd0;
reg [6:0] next_state;
reg [63:0] gate1_next_value;
reg gate1_next_value_ce;
wire [15:0] slice_proxy0;
wire [15:0] slice_proxy1;
wire [15:0] slice_proxy2;
wire [15:0] slice_proxy3;
wire [15:0] slice_proxy4;
wire [15:0] slice_proxy5;
wire [15:0] slice_proxy6;
wire [15:0] slice_proxy7;
wire [15:0] slice_proxy8;
wire [15:0] slice_proxy9;
wire [15:0] slice_proxy10;
wire [15:0] slice_proxy11;
wire [15:0] slice_proxy12;
wire [15:0] slice_proxy13;
wire [15:0] slice_proxy14;
wire [15:0] slice_proxy15;
wire [15:0] slice_proxy16;
wire [15:0] slice_proxy17;
wire [15:0] slice_proxy18;
wire [15:0] slice_proxy19;
wire [15:0] slice_proxy20;
wire [15:0] slice_proxy21;
wire [15:0] slice_proxy22;
wire [15:0] slice_proxy23;
wire [15:0] slice_proxy24;
wire [15:0] slice_proxy25;
wire [15:0] slice_proxy26;
wire [15:0] slice_proxy27;
wire [15:0] slice_proxy28;
wire [15:0] slice_proxy29;
wire [15:0] slice_proxy30;
wire [15:0] slice_proxy31;
wire [15:0] slice_proxy32;
wire [15:0] slice_proxy33;
wire [15:0] slice_proxy34;
wire [15:0] slice_proxy35;
wire [15:0] slice_proxy36;
wire [15:0] slice_proxy37;
wire [15:0] slice_proxy38;
wire [15:0] slice_proxy39;
wire [15:0] slice_proxy40;
wire [15:0] slice_proxy41;
wire [15:0] slice_proxy42;
wire [15:0] slice_proxy43;
wire [15:0] slice_proxy44;
wire [15:0] slice_proxy45;
wire [15:0] slice_proxy46;
wire [15:0] slice_proxy47;
wire [15:0] slice_proxy48;
wire [15:0] slice_proxy49;
wire [15:0] slice_proxy50;
wire [15:0] slice_proxy51;
wire [15:0] slice_proxy52;
wire [15:0] slice_proxy53;
wire [15:0] slice_proxy54;
wire [15:0] slice_proxy55;
wire [15:0] slice_proxy56;
wire [15:0] slice_proxy57;
wire [15:0] slice_proxy58;
wire [15:0] slice_proxy59;
wire [15:0] slice_proxy60;
wire [15:0] slice_proxy61;
wire [15:0] slice_proxy62;
wire [15:0] slice_proxy63;
wire async_reset;
wire rst_meta;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [6:0] xilinxmultiregimpl00 = 7'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [6:0] xilinxmultiregimpl01 = 7'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg xilinxmultiregimpl10 = 1'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg xilinxmultiregimpl11 = 1'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg xilinxmultiregimpl20 = 1'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg xilinxmultiregimpl21 = 1'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl30 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl31 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl40 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl41 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl50 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl51 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl60 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl61 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl70 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl71 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl80 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl81 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl90 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl91 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl100 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl101 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl110 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl111 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl120 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl121 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl130 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl131 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl140 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl141 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl150 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl151 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl160 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl161 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl170 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl171 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl180 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl181 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl190 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl191 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl200 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl201 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl210 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl211 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl220 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl221 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl230 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl231 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl240 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl241 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl250 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl251 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl260 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl261 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl270 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl271 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl280 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl281 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl290 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl291 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl300 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl301 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl310 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl311 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl320 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl321 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl330 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl331 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl340 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl341 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl350 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl351 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl360 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl361 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl370 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl371 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl380 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl381 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl390 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl391 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl400 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl401 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl410 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl411 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl420 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl421 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl430 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl431 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl440 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl441 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl450 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl451 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl460 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl461 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl470 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl471 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl480 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl481 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl490 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl491 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl500 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl501 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl510 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl511 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl520 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl521 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl530 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl531 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl540 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl541 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl550 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl551 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl560 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl561 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl570 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl571 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl580 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl581 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl590 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl591 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl600 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl601 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl610 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl611 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl620 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl621 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl630 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl631 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl640 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl641 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl650 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl651 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl660 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl661 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl670 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl671 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl680 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl681 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl690 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl691 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl700 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl701 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl710 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl711 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl720 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl721 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl730 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl731 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl740 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl741 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl750 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl751 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl760 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl761 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl770 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl771 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl780 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl781 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl790 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl791 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl800 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl801 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl810 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl811 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl820 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl821 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl830 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl831 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl840 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl841 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl850 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl851 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl860 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl861 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl870 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl871 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl880 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl881 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl890 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl891 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl900 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl901 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl910 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl911 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl920 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl921 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl930 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl931 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl940 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl941 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl950 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl951 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl960 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl961 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl970 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl971 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl980 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl981 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl990 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl991 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1000 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1001 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1010 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1011 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1020 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1021 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1030 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1031 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1040 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1041 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1050 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1051 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1060 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1061 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1070 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1071 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1080 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1081 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1090 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1091 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1100 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1101 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1110 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1111 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1120 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1121 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1130 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1131 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1140 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1141 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1150 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1151 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1160 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1161 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1170 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1171 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1180 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1181 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1190 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1191 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1200 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1201 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1210 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1211 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1220 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1221 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1230 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1231 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1240 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1241 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1250 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1251 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1260 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1261 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1270 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1271 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1280 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1281 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1290 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1291 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1300 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1301 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1310 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1311 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1320 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1321 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1330 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1331 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1340 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1341 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1350 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1351 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1360 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1361 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1370 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1371 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1380 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1381 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1390 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1391 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1400 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1401 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1410 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1411 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1420 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1421 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1430 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1431 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1440 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1441 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1450 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1451 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1460 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1461 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1470 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1471 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1480 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1481 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1490 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1491 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1500 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1501 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1510 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1511 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1520 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1521 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1530 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1531 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1540 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1541 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1550 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1551 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1560 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1561 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1570 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1571 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1580 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1581 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1590 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1591 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1600 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1601 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1610 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1611 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1620 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1621 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1630 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1631 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1640 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1641 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1650 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1651 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1660 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1661 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1670 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1671 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1680 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1681 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1690 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1691 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1700 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1701 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1710 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1711 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1720 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1721 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1730 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1731 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1740 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1741 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1750 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1751 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1760 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1761 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1770 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1771 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1780 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1781 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1790 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1791 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1800 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1801 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1810 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1811 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1820 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1821 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1830 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1831 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1840 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1841 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1850 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1851 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1860 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1861 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1870 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1871 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1880 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1881 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1890 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1891 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1900 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1901 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1910 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1911 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1920 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1921 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1930 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1931 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1940 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1941 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1950 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1951 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1960 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1961 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1970 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1971 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1980 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1981 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1990 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl1991 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2000 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2001 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2010 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2011 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2020 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2021 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2030 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2031 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2040 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2041 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2050 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2051 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2060 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2061 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2070 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2071 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2080 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2081 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2090 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2091 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2100 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2101 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2110 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2111 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2120 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2121 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2130 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2131 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2140 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2141 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2150 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2151 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2160 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2161 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2170 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2171 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2180 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2181 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2190 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2191 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2200 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2201 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2210 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2211 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2220 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2221 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2230 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2231 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2240 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2241 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2250 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2251 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2260 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2261 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2270 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2271 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2280 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2281 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2290 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2291 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2300 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2301 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2310 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2311 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2320 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2321 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2330 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2331 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2340 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2341 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2350 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2351 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2360 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2361 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2370 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2371 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2380 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2381 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2390 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2391 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2400 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2401 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2410 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2411 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2420 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2421 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2430 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2431 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2440 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2441 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2450 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2451 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2460 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2461 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2470 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2471 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2480 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2481 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2490 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2491 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2500 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2501 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2510 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2511 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2520 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2521 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2530 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2531 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2540 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2541 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2550 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2551 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2560 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2561 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2570 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2571 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2580 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2581 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2590 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2591 = 12'd0;
(* async_reg = "true", mr_ff = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2600 = 12'd0;
(* async_reg = "true", no_retiming = "true", no_shreg_extract = "true" *) reg [11:0] xilinxmultiregimpl2601 = 12'd0;

// synthesis translate_off
reg dummy_s;
initial dummy_s <= 1'd0;
// synthesis translate_on

assign cl = q;
assign {lval, fval, dval} = cl[16:14];
assign pix_stb = ((dval & fval) & lval);
assign pix_eop = ((~fval) & last_fval);
assign {pix_c, pix_b, pix_a} = {cl[22], cl[23], cl[17], cl[18], cl[19], cl[20], cl[7], cl[8], cl[24], cl[25], cl[9], cl[10], cl[11], cl[12], cl[13], cl[0], cl[26], cl[27], cl[1], cl[2], cl[3], cl[4], cl[5], cl[6]};
assign synchronizer0 = roi0_out_count;
assign synchronizer1 = roi1_out_count;
assign synchronizer2 = roi2_out_count;
assign synchronizer3 = roi3_out_count;
assign synchronizer4 = roi4_out_count;
assign synchronizer5 = roi5_out_count;
assign synchronizer6 = roi6_out_count;
assign synchronizer7 = roi7_out_count;
assign synchronizer8 = roi8_out_count;
assign synchronizer9 = roi9_out_count;
assign synchronizer10 = roi10_out_count;
assign synchronizer11 = roi11_out_count;
assign synchronizer12 = roi12_out_count;
assign synchronizer13 = roi13_out_count;
assign synchronizer14 = roi14_out_count;
assign synchronizer15 = roi15_out_count;
assign synchronizer16 = roi16_out_count;
assign synchronizer17 = roi17_out_count;
assign synchronizer18 = roi18_out_count;
assign synchronizer19 = roi19_out_count;
assign synchronizer20 = roi20_out_count;
assign synchronizer21 = roi21_out_count;
assign synchronizer22 = roi22_out_count;
assign synchronizer23 = roi23_out_count;
assign synchronizer24 = roi24_out_count;
assign synchronizer25 = roi25_out_count;
assign synchronizer26 = roi26_out_count;
assign synchronizer27 = roi27_out_count;
assign synchronizer28 = roi28_out_count;
assign synchronizer29 = roi29_out_count;
assign synchronizer30 = roi30_out_count;
assign synchronizer31 = roi31_out_count;
assign synchronizer32 = roi32_out_count;
assign synchronizer33 = roi33_out_count;
assign synchronizer34 = roi34_out_count;
assign synchronizer35 = roi35_out_count;
assign synchronizer36 = roi36_out_count;
assign synchronizer37 = roi37_out_count;
assign synchronizer38 = roi38_out_count;
assign synchronizer39 = roi39_out_count;
assign synchronizer40 = roi40_out_count;
assign synchronizer41 = roi41_out_count;
assign synchronizer42 = roi42_out_count;
assign synchronizer43 = roi43_out_count;
assign synchronizer44 = roi44_out_count;
assign synchronizer45 = roi45_out_count;
assign synchronizer46 = roi46_out_count;
assign synchronizer47 = roi47_out_count;
assign synchronizer48 = roi48_out_count;
assign synchronizer49 = roi49_out_count;
assign synchronizer50 = roi50_out_count;
assign synchronizer51 = roi51_out_count;
assign synchronizer52 = roi52_out_count;
assign synchronizer53 = roi53_out_count;
assign synchronizer54 = roi54_out_count;
assign synchronizer55 = roi55_out_count;
assign synchronizer56 = roi56_out_count;
assign synchronizer57 = roi57_out_count;
assign synchronizer58 = roi58_out_count;
assign synchronizer59 = roi59_out_count;
assign synchronizer60 = roi60_out_count;
assign synchronizer61 = roi61_out_count;
assign synchronizer62 = roi62_out_count;
assign synchronizer63 = roi63_out_count;

// synthesis translate_off
reg dummy_d;
// synthesis translate_on
always @(*) begin
	iinterface_stb <= 1'd0;
	iinterface_data <= 32'd0;
	next_state <= 7'd0;
	gate1_next_value <= 64'd0;
	gate1_next_value_ce <= 1'd0;
	next_state <= state;
	case (state)
		1'd1: begin
			iinterface_data <= synchronizer0;
			iinterface_stb <= gate1[0];
			next_state <= 2'd2;
		end
		2'd2: begin
			iinterface_data <= synchronizer1;
			iinterface_stb <= gate1[1];
			next_state <= 2'd3;
		end
		2'd3: begin
			iinterface_data <= synchronizer2;
			iinterface_stb <= gate1[2];
			next_state <= 3'd4;
		end
		3'd4: begin
			iinterface_data <= synchronizer3;
			iinterface_stb <= gate1[3];
			next_state <= 3'd5;
		end
		3'd5: begin
			iinterface_data <= synchronizer4;
			iinterface_stb <= gate1[4];
			next_state <= 3'd6;
		end
		3'd6: begin
			iinterface_data <= synchronizer5;
			iinterface_stb <= gate1[5];
			next_state <= 3'd7;
		end
		3'd7: begin
			iinterface_data <= synchronizer6;
			iinterface_stb <= gate1[6];
			next_state <= 4'd8;
		end
		4'd8: begin
			iinterface_data <= synchronizer7;
			iinterface_stb <= gate1[7];
			next_state <= 4'd9;
		end
		4'd9: begin
			iinterface_data <= synchronizer8;
			iinterface_stb <= gate1[8];
			next_state <= 4'd10;
		end
		4'd10: begin
			iinterface_data <= synchronizer9;
			iinterface_stb <= gate1[9];
			next_state <= 4'd11;
		end
		4'd11: begin
			iinterface_data <= synchronizer10;
			iinterface_stb <= gate1[10];
			next_state <= 4'd12;
		end
		4'd12: begin
			iinterface_data <= synchronizer11;
			iinterface_stb <= gate1[11];
			next_state <= 4'd13;
		end
		4'd13: begin
			iinterface_data <= synchronizer12;
			iinterface_stb <= gate1[12];
			next_state <= 4'd14;
		end
		4'd14: begin
			iinterface_data <= synchronizer13;
			iinterface_stb <= gate1[13];
			next_state <= 4'd15;
		end
		4'd15: begin
			iinterface_data <= synchronizer14;
			iinterface_stb <= gate1[14];
			next_state <= 5'd16;
		end
		5'd16: begin
			iinterface_data <= synchronizer15;
			iinterface_stb <= gate1[15];
			next_state <= 5'd17;
		end
		5'd17: begin
			iinterface_data <= synchronizer16;
			iinterface_stb <= gate1[16];
			next_state <= 5'd18;
		end
		5'd18: begin
			iinterface_data <= synchronizer17;
			iinterface_stb <= gate1[17];
			next_state <= 5'd19;
		end
		5'd19: begin
			iinterface_data <= synchronizer18;
			iinterface_stb <= gate1[18];
			next_state <= 5'd20;
		end
		5'd20: begin
			iinterface_data <= synchronizer19;
			iinterface_stb <= gate1[19];
			next_state <= 5'd21;
		end
		5'd21: begin
			iinterface_data <= synchronizer20;
			iinterface_stb <= gate1[20];
			next_state <= 5'd22;
		end
		5'd22: begin
			iinterface_data <= synchronizer21;
			iinterface_stb <= gate1[21];
			next_state <= 5'd23;
		end
		5'd23: begin
			iinterface_data <= synchronizer22;
			iinterface_stb <= gate1[22];
			next_state <= 5'd24;
		end
		5'd24: begin
			iinterface_data <= synchronizer23;
			iinterface_stb <= gate1[23];
			next_state <= 5'd25;
		end
		5'd25: begin
			iinterface_data <= synchronizer24;
			iinterface_stb <= gate1[24];
			next_state <= 5'd26;
		end
		5'd26: begin
			iinterface_data <= synchronizer25;
			iinterface_stb <= gate1[25];
			next_state <= 5'd27;
		end
		5'd27: begin
			iinterface_data <= synchronizer26;
			iinterface_stb <= gate1[26];
			next_state <= 5'd28;
		end
		5'd28: begin
			iinterface_data <= synchronizer27;
			iinterface_stb <= gate1[27];
			next_state <= 5'd29;
		end
		5'd29: begin
			iinterface_data <= synchronizer28;
			iinterface_stb <= gate1[28];
			next_state <= 5'd30;
		end
		5'd30: begin
			iinterface_data <= synchronizer29;
			iinterface_stb <= gate1[29];
			next_state <= 5'd31;
		end
		5'd31: begin
			iinterface_data <= synchronizer30;
			iinterface_stb <= gate1[30];
			next_state <= 6'd32;
		end
		6'd32: begin
			iinterface_data <= synchronizer31;
			iinterface_stb <= gate1[31];
			next_state <= 6'd33;
		end
		6'd33: begin
			iinterface_data <= synchronizer32;
			iinterface_stb <= gate1[32];
			next_state <= 6'd34;
		end
		6'd34: begin
			iinterface_data <= synchronizer33;
			iinterface_stb <= gate1[33];
			next_state <= 6'd35;
		end
		6'd35: begin
			iinterface_data <= synchronizer34;
			iinterface_stb <= gate1[34];
			next_state <= 6'd36;
		end
		6'd36: begin
			iinterface_data <= synchronizer35;
			iinterface_stb <= gate1[35];
			next_state <= 6'd37;
		end
		6'd37: begin
			iinterface_data <= synchronizer36;
			iinterface_stb <= gate1[36];
			next_state <= 6'd38;
		end
		6'd38: begin
			iinterface_data <= synchronizer37;
			iinterface_stb <= gate1[37];
			next_state <= 6'd39;
		end
		6'd39: begin
			iinterface_data <= synchronizer38;
			iinterface_stb <= gate1[38];
			next_state <= 6'd40;
		end
		6'd40: begin
			iinterface_data <= synchronizer39;
			iinterface_stb <= gate1[39];
			next_state <= 6'd41;
		end
		6'd41: begin
			iinterface_data <= synchronizer40;
			iinterface_stb <= gate1[40];
			next_state <= 6'd42;
		end
		6'd42: begin
			iinterface_data <= synchronizer41;
			iinterface_stb <= gate1[41];
			next_state <= 6'd43;
		end
		6'd43: begin
			iinterface_data <= synchronizer42;
			iinterface_stb <= gate1[42];
			next_state <= 6'd44;
		end
		6'd44: begin
			iinterface_data <= synchronizer43;
			iinterface_stb <= gate1[43];
			next_state <= 6'd45;
		end
		6'd45: begin
			iinterface_data <= synchronizer44;
			iinterface_stb <= gate1[44];
			next_state <= 6'd46;
		end
		6'd46: begin
			iinterface_data <= synchronizer45;
			iinterface_stb <= gate1[45];
			next_state <= 6'd47;
		end
		6'd47: begin
			iinterface_data <= synchronizer46;
			iinterface_stb <= gate1[46];
			next_state <= 6'd48;
		end
		6'd48: begin
			iinterface_data <= synchronizer47;
			iinterface_stb <= gate1[47];
			next_state <= 6'd49;
		end
		6'd49: begin
			iinterface_data <= synchronizer48;
			iinterface_stb <= gate1[48];
			next_state <= 6'd50;
		end
		6'd50: begin
			iinterface_data <= synchronizer49;
			iinterface_stb <= gate1[49];
			next_state <= 6'd51;
		end
		6'd51: begin
			iinterface_data <= synchronizer50;
			iinterface_stb <= gate1[50];
			next_state <= 6'd52;
		end
		6'd52: begin
			iinterface_data <= synchronizer51;
			iinterface_stb <= gate1[51];
			next_state <= 6'd53;
		end
		6'd53: begin
			iinterface_data <= synchronizer52;
			iinterface_stb <= gate1[52];
			next_state <= 6'd54;
		end
		6'd54: begin
			iinterface_data <= synchronizer53;
			iinterface_stb <= gate1[53];
			next_state <= 6'd55;
		end
		6'd55: begin
			iinterface_data <= synchronizer54;
			iinterface_stb <= gate1[54];
			next_state <= 6'd56;
		end
		6'd56: begin
			iinterface_data <= synchronizer55;
			iinterface_stb <= gate1[55];
			next_state <= 6'd57;
		end
		6'd57: begin
			iinterface_data <= synchronizer56;
			iinterface_stb <= gate1[56];
			next_state <= 6'd58;
		end
		6'd58: begin
			iinterface_data <= synchronizer57;
			iinterface_stb <= gate1[57];
			next_state <= 6'd59;
		end
		6'd59: begin
			iinterface_data <= synchronizer58;
			iinterface_stb <= gate1[58];
			next_state <= 6'd60;
		end
		6'd60: begin
			iinterface_data <= synchronizer59;
			iinterface_stb <= gate1[59];
			next_state <= 6'd61;
		end
		6'd61: begin
			iinterface_data <= synchronizer60;
			iinterface_stb <= gate1[60];
			next_state <= 6'd62;
		end
		6'd62: begin
			iinterface_data <= synchronizer61;
			iinterface_stb <= gate1[61];
			next_state <= 6'd63;
		end
		6'd63: begin
			iinterface_data <= synchronizer62;
			iinterface_stb <= gate1[62];
			next_state <= 7'd64;
		end
		7'd64: begin
			iinterface_data <= synchronizer63;
			iinterface_stb <= gate1[63];
			next_state <= 1'd0;
		end
		default: begin
			iinterface_data <= 32'd2147483648;
			if ((synchronizer_update & (gate0 != 1'd0))) begin
				gate1_next_value <= gate0;
				gate1_next_value_ce <= 1'd1;
				iinterface_stb <= 1'd1;
				next_state <= 1'd1;
			end
		end
	endcase
// synthesis translate_off
	dummy_d <= dummy_s;
// synthesis translate_on
end
assign slice_proxy0 = {pix_b, pix_a};
assign slice_proxy1 = {pix_b, pix_a};
assign slice_proxy2 = {pix_b, pix_a};
assign slice_proxy3 = {pix_b, pix_a};
assign slice_proxy4 = {pix_b, pix_a};
assign slice_proxy5 = {pix_b, pix_a};
assign slice_proxy6 = {pix_b, pix_a};
assign slice_proxy7 = {pix_b, pix_a};
assign slice_proxy8 = {pix_b, pix_a};
assign slice_proxy9 = {pix_b, pix_a};
assign slice_proxy10 = {pix_b, pix_a};
assign slice_proxy11 = {pix_b, pix_a};
assign slice_proxy12 = {pix_b, pix_a};
assign slice_proxy13 = {pix_b, pix_a};
assign slice_proxy14 = {pix_b, pix_a};
assign slice_proxy15 = {pix_b, pix_a};
assign slice_proxy16 = {pix_b, pix_a};
assign slice_proxy17 = {pix_b, pix_a};
assign slice_proxy18 = {pix_b, pix_a};
assign slice_proxy19 = {pix_b, pix_a};
assign slice_proxy20 = {pix_b, pix_a};
assign slice_proxy21 = {pix_b, pix_a};
assign slice_proxy22 = {pix_b, pix_a};
assign slice_proxy23 = {pix_b, pix_a};
assign slice_proxy24 = {pix_b, pix_a};
assign slice_proxy25 = {pix_b, pix_a};
assign slice_proxy26 = {pix_b, pix_a};
assign slice_proxy27 = {pix_b, pix_a};
assign slice_proxy28 = {pix_b, pix_a};
assign slice_proxy29 = {pix_b, pix_a};
assign slice_proxy30 = {pix_b, pix_a};
assign slice_proxy31 = {pix_b, pix_a};
assign slice_proxy32 = {pix_b, pix_a};
assign slice_proxy33 = {pix_b, pix_a};
assign slice_proxy34 = {pix_b, pix_a};
assign slice_proxy35 = {pix_b, pix_a};
assign slice_proxy36 = {pix_b, pix_a};
assign slice_proxy37 = {pix_b, pix_a};
assign slice_proxy38 = {pix_b, pix_a};
assign slice_proxy39 = {pix_b, pix_a};
assign slice_proxy40 = {pix_b, pix_a};
assign slice_proxy41 = {pix_b, pix_a};
assign slice_proxy42 = {pix_b, pix_a};
assign slice_proxy43 = {pix_b, pix_a};
assign slice_proxy44 = {pix_b, pix_a};
assign slice_proxy45 = {pix_b, pix_a};
assign slice_proxy46 = {pix_b, pix_a};
assign slice_proxy47 = {pix_b, pix_a};
assign slice_proxy48 = {pix_b, pix_a};
assign slice_proxy49 = {pix_b, pix_a};
assign slice_proxy50 = {pix_b, pix_a};
assign slice_proxy51 = {pix_b, pix_a};
assign slice_proxy52 = {pix_b, pix_a};
assign slice_proxy53 = {pix_b, pix_a};
assign slice_proxy54 = {pix_b, pix_a};
assign slice_proxy55 = {pix_b, pix_a};
assign slice_proxy56 = {pix_b, pix_a};
assign slice_proxy57 = {pix_b, pix_a};
assign slice_proxy58 = {pix_b, pix_a};
assign slice_proxy59 = {pix_b, pix_a};
assign slice_proxy60 = {pix_b, pix_a};
assign slice_proxy61 = {pix_b, pix_a};
assign slice_proxy62 = {pix_b, pix_a};
assign slice_proxy63 = {pix_b, pix_a};
assign async_reset = (~mmcm_locked);
assign clk_sampled_status = xilinxmultiregimpl01;
assign pll_locked_status = xilinxmultiregimpl11;
assign frequency_counter_toggle_sys = xilinxmultiregimpl21;
assign last_x_status = xilinxmultiregimpl31;
assign last_y_status = xilinxmultiregimpl41;
assign roi0_cfg_x0 = xilinxmultiregimpl51;
assign roi0_cfg_y0 = xilinxmultiregimpl61;
assign roi0_cfg_x1 = xilinxmultiregimpl71;
assign roi0_cfg_y1 = xilinxmultiregimpl81;
assign roi1_cfg_x0 = xilinxmultiregimpl91;
assign roi1_cfg_y0 = xilinxmultiregimpl101;
assign roi1_cfg_x1 = xilinxmultiregimpl111;
assign roi1_cfg_y1 = xilinxmultiregimpl121;
assign roi2_cfg_x0 = xilinxmultiregimpl131;
assign roi2_cfg_y0 = xilinxmultiregimpl141;
assign roi2_cfg_x1 = xilinxmultiregimpl151;
assign roi2_cfg_y1 = xilinxmultiregimpl161;
assign roi3_cfg_x0 = xilinxmultiregimpl171;
assign roi3_cfg_y0 = xilinxmultiregimpl181;
assign roi3_cfg_x1 = xilinxmultiregimpl191;
assign roi3_cfg_y1 = xilinxmultiregimpl201;
assign roi4_cfg_x0 = xilinxmultiregimpl211;
assign roi4_cfg_y0 = xilinxmultiregimpl221;
assign roi4_cfg_x1 = xilinxmultiregimpl231;
assign roi4_cfg_y1 = xilinxmultiregimpl241;
assign roi5_cfg_x0 = xilinxmultiregimpl251;
assign roi5_cfg_y0 = xilinxmultiregimpl261;
assign roi5_cfg_x1 = xilinxmultiregimpl271;
assign roi5_cfg_y1 = xilinxmultiregimpl281;
assign roi6_cfg_x0 = xilinxmultiregimpl291;
assign roi6_cfg_y0 = xilinxmultiregimpl301;
assign roi6_cfg_x1 = xilinxmultiregimpl311;
assign roi6_cfg_y1 = xilinxmultiregimpl321;
assign roi7_cfg_x0 = xilinxmultiregimpl331;
assign roi7_cfg_y0 = xilinxmultiregimpl341;
assign roi7_cfg_x1 = xilinxmultiregimpl351;
assign roi7_cfg_y1 = xilinxmultiregimpl361;
assign roi8_cfg_x0 = xilinxmultiregimpl371;
assign roi8_cfg_y0 = xilinxmultiregimpl381;
assign roi8_cfg_x1 = xilinxmultiregimpl391;
assign roi8_cfg_y1 = xilinxmultiregimpl401;
assign roi9_cfg_x0 = xilinxmultiregimpl411;
assign roi9_cfg_y0 = xilinxmultiregimpl421;
assign roi9_cfg_x1 = xilinxmultiregimpl431;
assign roi9_cfg_y1 = xilinxmultiregimpl441;
assign roi10_cfg_x0 = xilinxmultiregimpl451;
assign roi10_cfg_y0 = xilinxmultiregimpl461;
assign roi10_cfg_x1 = xilinxmultiregimpl471;
assign roi10_cfg_y1 = xilinxmultiregimpl481;
assign roi11_cfg_x0 = xilinxmultiregimpl491;
assign roi11_cfg_y0 = xilinxmultiregimpl501;
assign roi11_cfg_x1 = xilinxmultiregimpl511;
assign roi11_cfg_y1 = xilinxmultiregimpl521;
assign roi12_cfg_x0 = xilinxmultiregimpl531;
assign roi12_cfg_y0 = xilinxmultiregimpl541;
assign roi12_cfg_x1 = xilinxmultiregimpl551;
assign roi12_cfg_y1 = xilinxmultiregimpl561;
assign roi13_cfg_x0 = xilinxmultiregimpl571;
assign roi13_cfg_y0 = xilinxmultiregimpl581;
assign roi13_cfg_x1 = xilinxmultiregimpl591;
assign roi13_cfg_y1 = xilinxmultiregimpl601;
assign roi14_cfg_x0 = xilinxmultiregimpl611;
assign roi14_cfg_y0 = xilinxmultiregimpl621;
assign roi14_cfg_x1 = xilinxmultiregimpl631;
assign roi14_cfg_y1 = xilinxmultiregimpl641;
assign roi15_cfg_x0 = xilinxmultiregimpl651;
assign roi15_cfg_y0 = xilinxmultiregimpl661;
assign roi15_cfg_x1 = xilinxmultiregimpl671;
assign roi15_cfg_y1 = xilinxmultiregimpl681;
assign roi16_cfg_x0 = xilinxmultiregimpl691;
assign roi16_cfg_y0 = xilinxmultiregimpl701;
assign roi16_cfg_x1 = xilinxmultiregimpl711;
assign roi16_cfg_y1 = xilinxmultiregimpl721;
assign roi17_cfg_x0 = xilinxmultiregimpl731;
assign roi17_cfg_y0 = xilinxmultiregimpl741;
assign roi17_cfg_x1 = xilinxmultiregimpl751;
assign roi17_cfg_y1 = xilinxmultiregimpl761;
assign roi18_cfg_x0 = xilinxmultiregimpl771;
assign roi18_cfg_y0 = xilinxmultiregimpl781;
assign roi18_cfg_x1 = xilinxmultiregimpl791;
assign roi18_cfg_y1 = xilinxmultiregimpl801;
assign roi19_cfg_x0 = xilinxmultiregimpl811;
assign roi19_cfg_y0 = xilinxmultiregimpl821;
assign roi19_cfg_x1 = xilinxmultiregimpl831;
assign roi19_cfg_y1 = xilinxmultiregimpl841;
assign roi20_cfg_x0 = xilinxmultiregimpl851;
assign roi20_cfg_y0 = xilinxmultiregimpl861;
assign roi20_cfg_x1 = xilinxmultiregimpl871;
assign roi20_cfg_y1 = xilinxmultiregimpl881;
assign roi21_cfg_x0 = xilinxmultiregimpl891;
assign roi21_cfg_y0 = xilinxmultiregimpl901;
assign roi21_cfg_x1 = xilinxmultiregimpl911;
assign roi21_cfg_y1 = xilinxmultiregimpl921;
assign roi22_cfg_x0 = xilinxmultiregimpl931;
assign roi22_cfg_y0 = xilinxmultiregimpl941;
assign roi22_cfg_x1 = xilinxmultiregimpl951;
assign roi22_cfg_y1 = xilinxmultiregimpl961;
assign roi23_cfg_x0 = xilinxmultiregimpl971;
assign roi23_cfg_y0 = xilinxmultiregimpl981;
assign roi23_cfg_x1 = xilinxmultiregimpl991;
assign roi23_cfg_y1 = xilinxmultiregimpl1001;
assign roi24_cfg_x0 = xilinxmultiregimpl1011;
assign roi24_cfg_y0 = xilinxmultiregimpl1021;
assign roi24_cfg_x1 = xilinxmultiregimpl1031;
assign roi24_cfg_y1 = xilinxmultiregimpl1041;
assign roi25_cfg_x0 = xilinxmultiregimpl1051;
assign roi25_cfg_y0 = xilinxmultiregimpl1061;
assign roi25_cfg_x1 = xilinxmultiregimpl1071;
assign roi25_cfg_y1 = xilinxmultiregimpl1081;
assign roi26_cfg_x0 = xilinxmultiregimpl1091;
assign roi26_cfg_y0 = xilinxmultiregimpl1101;
assign roi26_cfg_x1 = xilinxmultiregimpl1111;
assign roi26_cfg_y1 = xilinxmultiregimpl1121;
assign roi27_cfg_x0 = xilinxmultiregimpl1131;
assign roi27_cfg_y0 = xilinxmultiregimpl1141;
assign roi27_cfg_x1 = xilinxmultiregimpl1151;
assign roi27_cfg_y1 = xilinxmultiregimpl1161;
assign roi28_cfg_x0 = xilinxmultiregimpl1171;
assign roi28_cfg_y0 = xilinxmultiregimpl1181;
assign roi28_cfg_x1 = xilinxmultiregimpl1191;
assign roi28_cfg_y1 = xilinxmultiregimpl1201;
assign roi29_cfg_x0 = xilinxmultiregimpl1211;
assign roi29_cfg_y0 = xilinxmultiregimpl1221;
assign roi29_cfg_x1 = xilinxmultiregimpl1231;
assign roi29_cfg_y1 = xilinxmultiregimpl1241;
assign roi30_cfg_x0 = xilinxmultiregimpl1251;
assign roi30_cfg_y0 = xilinxmultiregimpl1261;
assign roi30_cfg_x1 = xilinxmultiregimpl1271;
assign roi30_cfg_y1 = xilinxmultiregimpl1281;
assign roi31_cfg_x0 = xilinxmultiregimpl1291;
assign roi31_cfg_y0 = xilinxmultiregimpl1301;
assign roi31_cfg_x1 = xilinxmultiregimpl1311;
assign roi31_cfg_y1 = xilinxmultiregimpl1321;
assign roi32_cfg_x0 = xilinxmultiregimpl1331;
assign roi32_cfg_y0 = xilinxmultiregimpl1341;
assign roi32_cfg_x1 = xilinxmultiregimpl1351;
assign roi32_cfg_y1 = xilinxmultiregimpl1361;
assign roi33_cfg_x0 = xilinxmultiregimpl1371;
assign roi33_cfg_y0 = xilinxmultiregimpl1381;
assign roi33_cfg_x1 = xilinxmultiregimpl1391;
assign roi33_cfg_y1 = xilinxmultiregimpl1401;
assign roi34_cfg_x0 = xilinxmultiregimpl1411;
assign roi34_cfg_y0 = xilinxmultiregimpl1421;
assign roi34_cfg_x1 = xilinxmultiregimpl1431;
assign roi34_cfg_y1 = xilinxmultiregimpl1441;
assign roi35_cfg_x0 = xilinxmultiregimpl1451;
assign roi35_cfg_y0 = xilinxmultiregimpl1461;
assign roi35_cfg_x1 = xilinxmultiregimpl1471;
assign roi35_cfg_y1 = xilinxmultiregimpl1481;
assign roi36_cfg_x0 = xilinxmultiregimpl1491;
assign roi36_cfg_y0 = xilinxmultiregimpl1501;
assign roi36_cfg_x1 = xilinxmultiregimpl1511;
assign roi36_cfg_y1 = xilinxmultiregimpl1521;
assign roi37_cfg_x0 = xilinxmultiregimpl1531;
assign roi37_cfg_y0 = xilinxmultiregimpl1541;
assign roi37_cfg_x1 = xilinxmultiregimpl1551;
assign roi37_cfg_y1 = xilinxmultiregimpl1561;
assign roi38_cfg_x0 = xilinxmultiregimpl1571;
assign roi38_cfg_y0 = xilinxmultiregimpl1581;
assign roi38_cfg_x1 = xilinxmultiregimpl1591;
assign roi38_cfg_y1 = xilinxmultiregimpl1601;
assign roi39_cfg_x0 = xilinxmultiregimpl1611;
assign roi39_cfg_y0 = xilinxmultiregimpl1621;
assign roi39_cfg_x1 = xilinxmultiregimpl1631;
assign roi39_cfg_y1 = xilinxmultiregimpl1641;
assign roi40_cfg_x0 = xilinxmultiregimpl1651;
assign roi40_cfg_y0 = xilinxmultiregimpl1661;
assign roi40_cfg_x1 = xilinxmultiregimpl1671;
assign roi40_cfg_y1 = xilinxmultiregimpl1681;
assign roi41_cfg_x0 = xilinxmultiregimpl1691;
assign roi41_cfg_y0 = xilinxmultiregimpl1701;
assign roi41_cfg_x1 = xilinxmultiregimpl1711;
assign roi41_cfg_y1 = xilinxmultiregimpl1721;
assign roi42_cfg_x0 = xilinxmultiregimpl1731;
assign roi42_cfg_y0 = xilinxmultiregimpl1741;
assign roi42_cfg_x1 = xilinxmultiregimpl1751;
assign roi42_cfg_y1 = xilinxmultiregimpl1761;
assign roi43_cfg_x0 = xilinxmultiregimpl1771;
assign roi43_cfg_y0 = xilinxmultiregimpl1781;
assign roi43_cfg_x1 = xilinxmultiregimpl1791;
assign roi43_cfg_y1 = xilinxmultiregimpl1801;
assign roi44_cfg_x0 = xilinxmultiregimpl1811;
assign roi44_cfg_y0 = xilinxmultiregimpl1821;
assign roi44_cfg_x1 = xilinxmultiregimpl1831;
assign roi44_cfg_y1 = xilinxmultiregimpl1841;
assign roi45_cfg_x0 = xilinxmultiregimpl1851;
assign roi45_cfg_y0 = xilinxmultiregimpl1861;
assign roi45_cfg_x1 = xilinxmultiregimpl1871;
assign roi45_cfg_y1 = xilinxmultiregimpl1881;
assign roi46_cfg_x0 = xilinxmultiregimpl1891;
assign roi46_cfg_y0 = xilinxmultiregimpl1901;
assign roi46_cfg_x1 = xilinxmultiregimpl1911;
assign roi46_cfg_y1 = xilinxmultiregimpl1921;
assign roi47_cfg_x0 = xilinxmultiregimpl1931;
assign roi47_cfg_y0 = xilinxmultiregimpl1941;
assign roi47_cfg_x1 = xilinxmultiregimpl1951;
assign roi47_cfg_y1 = xilinxmultiregimpl1961;
assign roi48_cfg_x0 = xilinxmultiregimpl1971;
assign roi48_cfg_y0 = xilinxmultiregimpl1981;
assign roi48_cfg_x1 = xilinxmultiregimpl1991;
assign roi48_cfg_y1 = xilinxmultiregimpl2001;
assign roi49_cfg_x0 = xilinxmultiregimpl2011;
assign roi49_cfg_y0 = xilinxmultiregimpl2021;
assign roi49_cfg_x1 = xilinxmultiregimpl2031;
assign roi49_cfg_y1 = xilinxmultiregimpl2041;
assign roi50_cfg_x0 = xilinxmultiregimpl2051;
assign roi50_cfg_y0 = xilinxmultiregimpl2061;
assign roi50_cfg_x1 = xilinxmultiregimpl2071;
assign roi50_cfg_y1 = xilinxmultiregimpl2081;
assign roi51_cfg_x0 = xilinxmultiregimpl2091;
assign roi51_cfg_y0 = xilinxmultiregimpl2101;
assign roi51_cfg_x1 = xilinxmultiregimpl2111;
assign roi51_cfg_y1 = xilinxmultiregimpl2121;
assign roi52_cfg_x0 = xilinxmultiregimpl2131;
assign roi52_cfg_y0 = xilinxmultiregimpl2141;
assign roi52_cfg_x1 = xilinxmultiregimpl2151;
assign roi52_cfg_y1 = xilinxmultiregimpl2161;
assign roi53_cfg_x0 = xilinxmultiregimpl2171;
assign roi53_cfg_y0 = xilinxmultiregimpl2181;
assign roi53_cfg_x1 = xilinxmultiregimpl2191;
assign roi53_cfg_y1 = xilinxmultiregimpl2201;
assign roi54_cfg_x0 = xilinxmultiregimpl2211;
assign roi54_cfg_y0 = xilinxmultiregimpl2221;
assign roi54_cfg_x1 = xilinxmultiregimpl2231;
assign roi54_cfg_y1 = xilinxmultiregimpl2241;
assign roi55_cfg_x0 = xilinxmultiregimpl2251;
assign roi55_cfg_y0 = xilinxmultiregimpl2261;
assign roi55_cfg_x1 = xilinxmultiregimpl2271;
assign roi55_cfg_y1 = xilinxmultiregimpl2281;
assign roi56_cfg_x0 = xilinxmultiregimpl2291;
assign roi56_cfg_y0 = xilinxmultiregimpl2301;
assign roi56_cfg_x1 = xilinxmultiregimpl2311;
assign roi56_cfg_y1 = xilinxmultiregimpl2321;
assign roi57_cfg_x0 = xilinxmultiregimpl2331;
assign roi57_cfg_y0 = xilinxmultiregimpl2341;
assign roi57_cfg_x1 = xilinxmultiregimpl2351;
assign roi57_cfg_y1 = xilinxmultiregimpl2361;
assign roi58_cfg_x0 = xilinxmultiregimpl2371;
assign roi58_cfg_y0 = xilinxmultiregimpl2381;
assign roi58_cfg_x1 = xilinxmultiregimpl2391;
assign roi58_cfg_y1 = xilinxmultiregimpl2401;
assign roi59_cfg_x0 = xilinxmultiregimpl2411;
assign roi59_cfg_y0 = xilinxmultiregimpl2421;
assign roi59_cfg_x1 = xilinxmultiregimpl2431;
assign roi59_cfg_y1 = xilinxmultiregimpl2441;
assign roi60_cfg_x0 = xilinxmultiregimpl2451;
assign roi60_cfg_y0 = xilinxmultiregimpl2461;
assign roi60_cfg_x1 = xilinxmultiregimpl2471;
assign roi60_cfg_y1 = xilinxmultiregimpl2481;
assign roi61_cfg_x0 = xilinxmultiregimpl2491;
assign roi61_cfg_y0 = xilinxmultiregimpl2501;
assign roi61_cfg_x1 = xilinxmultiregimpl2511;
assign roi61_cfg_y1 = xilinxmultiregimpl2521;
assign roi62_cfg_x0 = xilinxmultiregimpl2531;
assign roi62_cfg_y0 = xilinxmultiregimpl2541;
assign roi62_cfg_x1 = xilinxmultiregimpl2551;
assign roi62_cfg_y1 = xilinxmultiregimpl2561;
assign roi63_cfg_x0 = xilinxmultiregimpl2571;
assign roi63_cfg_y0 = xilinxmultiregimpl2581;
assign roi63_cfg_x1 = xilinxmultiregimpl2591;
assign roi63_cfg_y1 = xilinxmultiregimpl2601;

always @(posedge cl_clk) begin
	frequency_counter_toggle <= (~frequency_counter_toggle);
	last_lval <= lval;
	last_fval <= fval;
	if (dval) begin
		pix_x <= (pix_x + 1'd1);
	end
	if ((~lval)) begin
		if (last_lval) begin
			last_x <= pix_x;
			pix_y <= (pix_y + 1'd1);
		end
		pix_x <= 1'd0;
	end
	if ((~fval)) begin
		if (last_fval) begin
			last_y <= pix_y;
		end
		pix_y <= 1'd0;
	end
	if ((pix_y == roi0_cfg_y0)) begin
		roi0_y_good <= 1'd1;
	end
	if ((pix_y == roi0_cfg_y1)) begin
		roi0_y_good <= 1'd0;
	end
	if ((pix_x == roi0_cfg_x0)) begin
		roi0_x_good <= 1'd1;
	end
	if ((pix_x == roi0_cfg_x1)) begin
		roi0_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi0_y_good <= 1'd0;
		roi0_x_good <= 1'd0;
	end
	roi0_gray <= slice_proxy0[15:0];
	roi0_stb <= pix_stb;
	roi0_eop <= pix_eop;
	if (((roi0_stb & roi0_x_good) & roi0_y_good)) begin
		roi0_count <= (roi0_count + roi0_gray);
	end
	roi0_out_update <= 1'd0;
	if (roi0_eop) begin
		roi0_count <= 1'd0;
		roi0_out_update <= 1'd1;
		roi0_out_count <= roi0_count;
	end
	if ((pix_y == roi1_cfg_y0)) begin
		roi1_y_good <= 1'd1;
	end
	if ((pix_y == roi1_cfg_y1)) begin
		roi1_y_good <= 1'd0;
	end
	if ((pix_x == roi1_cfg_x0)) begin
		roi1_x_good <= 1'd1;
	end
	if ((pix_x == roi1_cfg_x1)) begin
		roi1_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi1_y_good <= 1'd0;
		roi1_x_good <= 1'd0;
	end
	roi1_gray <= slice_proxy1[15:0];
	roi1_stb <= pix_stb;
	roi1_eop <= pix_eop;
	if (((roi1_stb & roi1_x_good) & roi1_y_good)) begin
		roi1_count <= (roi1_count + roi1_gray);
	end
	roi1_out_update <= 1'd0;
	if (roi1_eop) begin
		roi1_count <= 1'd0;
		roi1_out_update <= 1'd1;
		roi1_out_count <= roi1_count;
	end
	if ((pix_y == roi2_cfg_y0)) begin
		roi2_y_good <= 1'd1;
	end
	if ((pix_y == roi2_cfg_y1)) begin
		roi2_y_good <= 1'd0;
	end
	if ((pix_x == roi2_cfg_x0)) begin
		roi2_x_good <= 1'd1;
	end
	if ((pix_x == roi2_cfg_x1)) begin
		roi2_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi2_y_good <= 1'd0;
		roi2_x_good <= 1'd0;
	end
	roi2_gray <= slice_proxy2[15:0];
	roi2_stb <= pix_stb;
	roi2_eop <= pix_eop;
	if (((roi2_stb & roi2_x_good) & roi2_y_good)) begin
		roi2_count <= (roi2_count + roi2_gray);
	end
	roi2_out_update <= 1'd0;
	if (roi2_eop) begin
		roi2_count <= 1'd0;
		roi2_out_update <= 1'd1;
		roi2_out_count <= roi2_count;
	end
	if ((pix_y == roi3_cfg_y0)) begin
		roi3_y_good <= 1'd1;
	end
	if ((pix_y == roi3_cfg_y1)) begin
		roi3_y_good <= 1'd0;
	end
	if ((pix_x == roi3_cfg_x0)) begin
		roi3_x_good <= 1'd1;
	end
	if ((pix_x == roi3_cfg_x1)) begin
		roi3_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi3_y_good <= 1'd0;
		roi3_x_good <= 1'd0;
	end
	roi3_gray <= slice_proxy3[15:0];
	roi3_stb <= pix_stb;
	roi3_eop <= pix_eop;
	if (((roi3_stb & roi3_x_good) & roi3_y_good)) begin
		roi3_count <= (roi3_count + roi3_gray);
	end
	roi3_out_update <= 1'd0;
	if (roi3_eop) begin
		roi3_count <= 1'd0;
		roi3_out_update <= 1'd1;
		roi3_out_count <= roi3_count;
	end
	if ((pix_y == roi4_cfg_y0)) begin
		roi4_y_good <= 1'd1;
	end
	if ((pix_y == roi4_cfg_y1)) begin
		roi4_y_good <= 1'd0;
	end
	if ((pix_x == roi4_cfg_x0)) begin
		roi4_x_good <= 1'd1;
	end
	if ((pix_x == roi4_cfg_x1)) begin
		roi4_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi4_y_good <= 1'd0;
		roi4_x_good <= 1'd0;
	end
	roi4_gray <= slice_proxy4[15:0];
	roi4_stb <= pix_stb;
	roi4_eop <= pix_eop;
	if (((roi4_stb & roi4_x_good) & roi4_y_good)) begin
		roi4_count <= (roi4_count + roi4_gray);
	end
	roi4_out_update <= 1'd0;
	if (roi4_eop) begin
		roi4_count <= 1'd0;
		roi4_out_update <= 1'd1;
		roi4_out_count <= roi4_count;
	end
	if ((pix_y == roi5_cfg_y0)) begin
		roi5_y_good <= 1'd1;
	end
	if ((pix_y == roi5_cfg_y1)) begin
		roi5_y_good <= 1'd0;
	end
	if ((pix_x == roi5_cfg_x0)) begin
		roi5_x_good <= 1'd1;
	end
	if ((pix_x == roi5_cfg_x1)) begin
		roi5_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi5_y_good <= 1'd0;
		roi5_x_good <= 1'd0;
	end
	roi5_gray <= slice_proxy5[15:0];
	roi5_stb <= pix_stb;
	roi5_eop <= pix_eop;
	if (((roi5_stb & roi5_x_good) & roi5_y_good)) begin
		roi5_count <= (roi5_count + roi5_gray);
	end
	roi5_out_update <= 1'd0;
	if (roi5_eop) begin
		roi5_count <= 1'd0;
		roi5_out_update <= 1'd1;
		roi5_out_count <= roi5_count;
	end
	if ((pix_y == roi6_cfg_y0)) begin
		roi6_y_good <= 1'd1;
	end
	if ((pix_y == roi6_cfg_y1)) begin
		roi6_y_good <= 1'd0;
	end
	if ((pix_x == roi6_cfg_x0)) begin
		roi6_x_good <= 1'd1;
	end
	if ((pix_x == roi6_cfg_x1)) begin
		roi6_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi6_y_good <= 1'd0;
		roi6_x_good <= 1'd0;
	end
	roi6_gray <= slice_proxy6[15:0];
	roi6_stb <= pix_stb;
	roi6_eop <= pix_eop;
	if (((roi6_stb & roi6_x_good) & roi6_y_good)) begin
		roi6_count <= (roi6_count + roi6_gray);
	end
	roi6_out_update <= 1'd0;
	if (roi6_eop) begin
		roi6_count <= 1'd0;
		roi6_out_update <= 1'd1;
		roi6_out_count <= roi6_count;
	end
	if ((pix_y == roi7_cfg_y0)) begin
		roi7_y_good <= 1'd1;
	end
	if ((pix_y == roi7_cfg_y1)) begin
		roi7_y_good <= 1'd0;
	end
	if ((pix_x == roi7_cfg_x0)) begin
		roi7_x_good <= 1'd1;
	end
	if ((pix_x == roi7_cfg_x1)) begin
		roi7_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi7_y_good <= 1'd0;
		roi7_x_good <= 1'd0;
	end
	roi7_gray <= slice_proxy7[15:0];
	roi7_stb <= pix_stb;
	roi7_eop <= pix_eop;
	if (((roi7_stb & roi7_x_good) & roi7_y_good)) begin
		roi7_count <= (roi7_count + roi7_gray);
	end
	roi7_out_update <= 1'd0;
	if (roi7_eop) begin
		roi7_count <= 1'd0;
		roi7_out_update <= 1'd1;
		roi7_out_count <= roi7_count;
	end
	if ((pix_y == roi8_cfg_y0)) begin
		roi8_y_good <= 1'd1;
	end
	if ((pix_y == roi8_cfg_y1)) begin
		roi8_y_good <= 1'd0;
	end
	if ((pix_x == roi8_cfg_x0)) begin
		roi8_x_good <= 1'd1;
	end
	if ((pix_x == roi8_cfg_x1)) begin
		roi8_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi8_y_good <= 1'd0;
		roi8_x_good <= 1'd0;
	end
	roi8_gray <= slice_proxy8[15:0];
	roi8_stb <= pix_stb;
	roi8_eop <= pix_eop;
	if (((roi8_stb & roi8_x_good) & roi8_y_good)) begin
		roi8_count <= (roi8_count + roi8_gray);
	end
	roi8_out_update <= 1'd0;
	if (roi8_eop) begin
		roi8_count <= 1'd0;
		roi8_out_update <= 1'd1;
		roi8_out_count <= roi8_count;
	end
	if ((pix_y == roi9_cfg_y0)) begin
		roi9_y_good <= 1'd1;
	end
	if ((pix_y == roi9_cfg_y1)) begin
		roi9_y_good <= 1'd0;
	end
	if ((pix_x == roi9_cfg_x0)) begin
		roi9_x_good <= 1'd1;
	end
	if ((pix_x == roi9_cfg_x1)) begin
		roi9_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi9_y_good <= 1'd0;
		roi9_x_good <= 1'd0;
	end
	roi9_gray <= slice_proxy9[15:0];
	roi9_stb <= pix_stb;
	roi9_eop <= pix_eop;
	if (((roi9_stb & roi9_x_good) & roi9_y_good)) begin
		roi9_count <= (roi9_count + roi9_gray);
	end
	roi9_out_update <= 1'd0;
	if (roi9_eop) begin
		roi9_count <= 1'd0;
		roi9_out_update <= 1'd1;
		roi9_out_count <= roi9_count;
	end
	if ((pix_y == roi10_cfg_y0)) begin
		roi10_y_good <= 1'd1;
	end
	if ((pix_y == roi10_cfg_y1)) begin
		roi10_y_good <= 1'd0;
	end
	if ((pix_x == roi10_cfg_x0)) begin
		roi10_x_good <= 1'd1;
	end
	if ((pix_x == roi10_cfg_x1)) begin
		roi10_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi10_y_good <= 1'd0;
		roi10_x_good <= 1'd0;
	end
	roi10_gray <= slice_proxy10[15:0];
	roi10_stb <= pix_stb;
	roi10_eop <= pix_eop;
	if (((roi10_stb & roi10_x_good) & roi10_y_good)) begin
		roi10_count <= (roi10_count + roi10_gray);
	end
	roi10_out_update <= 1'd0;
	if (roi10_eop) begin
		roi10_count <= 1'd0;
		roi10_out_update <= 1'd1;
		roi10_out_count <= roi10_count;
	end
	if ((pix_y == roi11_cfg_y0)) begin
		roi11_y_good <= 1'd1;
	end
	if ((pix_y == roi11_cfg_y1)) begin
		roi11_y_good <= 1'd0;
	end
	if ((pix_x == roi11_cfg_x0)) begin
		roi11_x_good <= 1'd1;
	end
	if ((pix_x == roi11_cfg_x1)) begin
		roi11_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi11_y_good <= 1'd0;
		roi11_x_good <= 1'd0;
	end
	roi11_gray <= slice_proxy11[15:0];
	roi11_stb <= pix_stb;
	roi11_eop <= pix_eop;
	if (((roi11_stb & roi11_x_good) & roi11_y_good)) begin
		roi11_count <= (roi11_count + roi11_gray);
	end
	roi11_out_update <= 1'd0;
	if (roi11_eop) begin
		roi11_count <= 1'd0;
		roi11_out_update <= 1'd1;
		roi11_out_count <= roi11_count;
	end
	if ((pix_y == roi12_cfg_y0)) begin
		roi12_y_good <= 1'd1;
	end
	if ((pix_y == roi12_cfg_y1)) begin
		roi12_y_good <= 1'd0;
	end
	if ((pix_x == roi12_cfg_x0)) begin
		roi12_x_good <= 1'd1;
	end
	if ((pix_x == roi12_cfg_x1)) begin
		roi12_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi12_y_good <= 1'd0;
		roi12_x_good <= 1'd0;
	end
	roi12_gray <= slice_proxy12[15:0];
	roi12_stb <= pix_stb;
	roi12_eop <= pix_eop;
	if (((roi12_stb & roi12_x_good) & roi12_y_good)) begin
		roi12_count <= (roi12_count + roi12_gray);
	end
	roi12_out_update <= 1'd0;
	if (roi12_eop) begin
		roi12_count <= 1'd0;
		roi12_out_update <= 1'd1;
		roi12_out_count <= roi12_count;
	end
	if ((pix_y == roi13_cfg_y0)) begin
		roi13_y_good <= 1'd1;
	end
	if ((pix_y == roi13_cfg_y1)) begin
		roi13_y_good <= 1'd0;
	end
	if ((pix_x == roi13_cfg_x0)) begin
		roi13_x_good <= 1'd1;
	end
	if ((pix_x == roi13_cfg_x1)) begin
		roi13_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi13_y_good <= 1'd0;
		roi13_x_good <= 1'd0;
	end
	roi13_gray <= slice_proxy13[15:0];
	roi13_stb <= pix_stb;
	roi13_eop <= pix_eop;
	if (((roi13_stb & roi13_x_good) & roi13_y_good)) begin
		roi13_count <= (roi13_count + roi13_gray);
	end
	roi13_out_update <= 1'd0;
	if (roi13_eop) begin
		roi13_count <= 1'd0;
		roi13_out_update <= 1'd1;
		roi13_out_count <= roi13_count;
	end
	if ((pix_y == roi14_cfg_y0)) begin
		roi14_y_good <= 1'd1;
	end
	if ((pix_y == roi14_cfg_y1)) begin
		roi14_y_good <= 1'd0;
	end
	if ((pix_x == roi14_cfg_x0)) begin
		roi14_x_good <= 1'd1;
	end
	if ((pix_x == roi14_cfg_x1)) begin
		roi14_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi14_y_good <= 1'd0;
		roi14_x_good <= 1'd0;
	end
	roi14_gray <= slice_proxy14[15:0];
	roi14_stb <= pix_stb;
	roi14_eop <= pix_eop;
	if (((roi14_stb & roi14_x_good) & roi14_y_good)) begin
		roi14_count <= (roi14_count + roi14_gray);
	end
	roi14_out_update <= 1'd0;
	if (roi14_eop) begin
		roi14_count <= 1'd0;
		roi14_out_update <= 1'd1;
		roi14_out_count <= roi14_count;
	end
	if ((pix_y == roi15_cfg_y0)) begin
		roi15_y_good <= 1'd1;
	end
	if ((pix_y == roi15_cfg_y1)) begin
		roi15_y_good <= 1'd0;
	end
	if ((pix_x == roi15_cfg_x0)) begin
		roi15_x_good <= 1'd1;
	end
	if ((pix_x == roi15_cfg_x1)) begin
		roi15_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi15_y_good <= 1'd0;
		roi15_x_good <= 1'd0;
	end
	roi15_gray <= slice_proxy15[15:0];
	roi15_stb <= pix_stb;
	roi15_eop <= pix_eop;
	if (((roi15_stb & roi15_x_good) & roi15_y_good)) begin
		roi15_count <= (roi15_count + roi15_gray);
	end
	roi15_out_update <= 1'd0;
	if (roi15_eop) begin
		roi15_count <= 1'd0;
		roi15_out_update <= 1'd1;
		roi15_out_count <= roi15_count;
	end
	if ((pix_y == roi16_cfg_y0)) begin
		roi16_y_good <= 1'd1;
	end
	if ((pix_y == roi16_cfg_y1)) begin
		roi16_y_good <= 1'd0;
	end
	if ((pix_x == roi16_cfg_x0)) begin
		roi16_x_good <= 1'd1;
	end
	if ((pix_x == roi16_cfg_x1)) begin
		roi16_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi16_y_good <= 1'd0;
		roi16_x_good <= 1'd0;
	end
	roi16_gray <= slice_proxy16[15:0];
	roi16_stb <= pix_stb;
	roi16_eop <= pix_eop;
	if (((roi16_stb & roi16_x_good) & roi16_y_good)) begin
		roi16_count <= (roi16_count + roi16_gray);
	end
	roi16_out_update <= 1'd0;
	if (roi16_eop) begin
		roi16_count <= 1'd0;
		roi16_out_update <= 1'd1;
		roi16_out_count <= roi16_count;
	end
	if ((pix_y == roi17_cfg_y0)) begin
		roi17_y_good <= 1'd1;
	end
	if ((pix_y == roi17_cfg_y1)) begin
		roi17_y_good <= 1'd0;
	end
	if ((pix_x == roi17_cfg_x0)) begin
		roi17_x_good <= 1'd1;
	end
	if ((pix_x == roi17_cfg_x1)) begin
		roi17_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi17_y_good <= 1'd0;
		roi17_x_good <= 1'd0;
	end
	roi17_gray <= slice_proxy17[15:0];
	roi17_stb <= pix_stb;
	roi17_eop <= pix_eop;
	if (((roi17_stb & roi17_x_good) & roi17_y_good)) begin
		roi17_count <= (roi17_count + roi17_gray);
	end
	roi17_out_update <= 1'd0;
	if (roi17_eop) begin
		roi17_count <= 1'd0;
		roi17_out_update <= 1'd1;
		roi17_out_count <= roi17_count;
	end
	if ((pix_y == roi18_cfg_y0)) begin
		roi18_y_good <= 1'd1;
	end
	if ((pix_y == roi18_cfg_y1)) begin
		roi18_y_good <= 1'd0;
	end
	if ((pix_x == roi18_cfg_x0)) begin
		roi18_x_good <= 1'd1;
	end
	if ((pix_x == roi18_cfg_x1)) begin
		roi18_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi18_y_good <= 1'd0;
		roi18_x_good <= 1'd0;
	end
	roi18_gray <= slice_proxy18[15:0];
	roi18_stb <= pix_stb;
	roi18_eop <= pix_eop;
	if (((roi18_stb & roi18_x_good) & roi18_y_good)) begin
		roi18_count <= (roi18_count + roi18_gray);
	end
	roi18_out_update <= 1'd0;
	if (roi18_eop) begin
		roi18_count <= 1'd0;
		roi18_out_update <= 1'd1;
		roi18_out_count <= roi18_count;
	end
	if ((pix_y == roi19_cfg_y0)) begin
		roi19_y_good <= 1'd1;
	end
	if ((pix_y == roi19_cfg_y1)) begin
		roi19_y_good <= 1'd0;
	end
	if ((pix_x == roi19_cfg_x0)) begin
		roi19_x_good <= 1'd1;
	end
	if ((pix_x == roi19_cfg_x1)) begin
		roi19_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi19_y_good <= 1'd0;
		roi19_x_good <= 1'd0;
	end
	roi19_gray <= slice_proxy19[15:0];
	roi19_stb <= pix_stb;
	roi19_eop <= pix_eop;
	if (((roi19_stb & roi19_x_good) & roi19_y_good)) begin
		roi19_count <= (roi19_count + roi19_gray);
	end
	roi19_out_update <= 1'd0;
	if (roi19_eop) begin
		roi19_count <= 1'd0;
		roi19_out_update <= 1'd1;
		roi19_out_count <= roi19_count;
	end
	if ((pix_y == roi20_cfg_y0)) begin
		roi20_y_good <= 1'd1;
	end
	if ((pix_y == roi20_cfg_y1)) begin
		roi20_y_good <= 1'd0;
	end
	if ((pix_x == roi20_cfg_x0)) begin
		roi20_x_good <= 1'd1;
	end
	if ((pix_x == roi20_cfg_x1)) begin
		roi20_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi20_y_good <= 1'd0;
		roi20_x_good <= 1'd0;
	end
	roi20_gray <= slice_proxy20[15:0];
	roi20_stb <= pix_stb;
	roi20_eop <= pix_eop;
	if (((roi20_stb & roi20_x_good) & roi20_y_good)) begin
		roi20_count <= (roi20_count + roi20_gray);
	end
	roi20_out_update <= 1'd0;
	if (roi20_eop) begin
		roi20_count <= 1'd0;
		roi20_out_update <= 1'd1;
		roi20_out_count <= roi20_count;
	end
	if ((pix_y == roi21_cfg_y0)) begin
		roi21_y_good <= 1'd1;
	end
	if ((pix_y == roi21_cfg_y1)) begin
		roi21_y_good <= 1'd0;
	end
	if ((pix_x == roi21_cfg_x0)) begin
		roi21_x_good <= 1'd1;
	end
	if ((pix_x == roi21_cfg_x1)) begin
		roi21_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi21_y_good <= 1'd0;
		roi21_x_good <= 1'd0;
	end
	roi21_gray <= slice_proxy21[15:0];
	roi21_stb <= pix_stb;
	roi21_eop <= pix_eop;
	if (((roi21_stb & roi21_x_good) & roi21_y_good)) begin
		roi21_count <= (roi21_count + roi21_gray);
	end
	roi21_out_update <= 1'd0;
	if (roi21_eop) begin
		roi21_count <= 1'd0;
		roi21_out_update <= 1'd1;
		roi21_out_count <= roi21_count;
	end
	if ((pix_y == roi22_cfg_y0)) begin
		roi22_y_good <= 1'd1;
	end
	if ((pix_y == roi22_cfg_y1)) begin
		roi22_y_good <= 1'd0;
	end
	if ((pix_x == roi22_cfg_x0)) begin
		roi22_x_good <= 1'd1;
	end
	if ((pix_x == roi22_cfg_x1)) begin
		roi22_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi22_y_good <= 1'd0;
		roi22_x_good <= 1'd0;
	end
	roi22_gray <= slice_proxy22[15:0];
	roi22_stb <= pix_stb;
	roi22_eop <= pix_eop;
	if (((roi22_stb & roi22_x_good) & roi22_y_good)) begin
		roi22_count <= (roi22_count + roi22_gray);
	end
	roi22_out_update <= 1'd0;
	if (roi22_eop) begin
		roi22_count <= 1'd0;
		roi22_out_update <= 1'd1;
		roi22_out_count <= roi22_count;
	end
	if ((pix_y == roi23_cfg_y0)) begin
		roi23_y_good <= 1'd1;
	end
	if ((pix_y == roi23_cfg_y1)) begin
		roi23_y_good <= 1'd0;
	end
	if ((pix_x == roi23_cfg_x0)) begin
		roi23_x_good <= 1'd1;
	end
	if ((pix_x == roi23_cfg_x1)) begin
		roi23_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi23_y_good <= 1'd0;
		roi23_x_good <= 1'd0;
	end
	roi23_gray <= slice_proxy23[15:0];
	roi23_stb <= pix_stb;
	roi23_eop <= pix_eop;
	if (((roi23_stb & roi23_x_good) & roi23_y_good)) begin
		roi23_count <= (roi23_count + roi23_gray);
	end
	roi23_out_update <= 1'd0;
	if (roi23_eop) begin
		roi23_count <= 1'd0;
		roi23_out_update <= 1'd1;
		roi23_out_count <= roi23_count;
	end
	if ((pix_y == roi24_cfg_y0)) begin
		roi24_y_good <= 1'd1;
	end
	if ((pix_y == roi24_cfg_y1)) begin
		roi24_y_good <= 1'd0;
	end
	if ((pix_x == roi24_cfg_x0)) begin
		roi24_x_good <= 1'd1;
	end
	if ((pix_x == roi24_cfg_x1)) begin
		roi24_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi24_y_good <= 1'd0;
		roi24_x_good <= 1'd0;
	end
	roi24_gray <= slice_proxy24[15:0];
	roi24_stb <= pix_stb;
	roi24_eop <= pix_eop;
	if (((roi24_stb & roi24_x_good) & roi24_y_good)) begin
		roi24_count <= (roi24_count + roi24_gray);
	end
	roi24_out_update <= 1'd0;
	if (roi24_eop) begin
		roi24_count <= 1'd0;
		roi24_out_update <= 1'd1;
		roi24_out_count <= roi24_count;
	end
	if ((pix_y == roi25_cfg_y0)) begin
		roi25_y_good <= 1'd1;
	end
	if ((pix_y == roi25_cfg_y1)) begin
		roi25_y_good <= 1'd0;
	end
	if ((pix_x == roi25_cfg_x0)) begin
		roi25_x_good <= 1'd1;
	end
	if ((pix_x == roi25_cfg_x1)) begin
		roi25_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi25_y_good <= 1'd0;
		roi25_x_good <= 1'd0;
	end
	roi25_gray <= slice_proxy25[15:0];
	roi25_stb <= pix_stb;
	roi25_eop <= pix_eop;
	if (((roi25_stb & roi25_x_good) & roi25_y_good)) begin
		roi25_count <= (roi25_count + roi25_gray);
	end
	roi25_out_update <= 1'd0;
	if (roi25_eop) begin
		roi25_count <= 1'd0;
		roi25_out_update <= 1'd1;
		roi25_out_count <= roi25_count;
	end
	if ((pix_y == roi26_cfg_y0)) begin
		roi26_y_good <= 1'd1;
	end
	if ((pix_y == roi26_cfg_y1)) begin
		roi26_y_good <= 1'd0;
	end
	if ((pix_x == roi26_cfg_x0)) begin
		roi26_x_good <= 1'd1;
	end
	if ((pix_x == roi26_cfg_x1)) begin
		roi26_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi26_y_good <= 1'd0;
		roi26_x_good <= 1'd0;
	end
	roi26_gray <= slice_proxy26[15:0];
	roi26_stb <= pix_stb;
	roi26_eop <= pix_eop;
	if (((roi26_stb & roi26_x_good) & roi26_y_good)) begin
		roi26_count <= (roi26_count + roi26_gray);
	end
	roi26_out_update <= 1'd0;
	if (roi26_eop) begin
		roi26_count <= 1'd0;
		roi26_out_update <= 1'd1;
		roi26_out_count <= roi26_count;
	end
	if ((pix_y == roi27_cfg_y0)) begin
		roi27_y_good <= 1'd1;
	end
	if ((pix_y == roi27_cfg_y1)) begin
		roi27_y_good <= 1'd0;
	end
	if ((pix_x == roi27_cfg_x0)) begin
		roi27_x_good <= 1'd1;
	end
	if ((pix_x == roi27_cfg_x1)) begin
		roi27_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi27_y_good <= 1'd0;
		roi27_x_good <= 1'd0;
	end
	roi27_gray <= slice_proxy27[15:0];
	roi27_stb <= pix_stb;
	roi27_eop <= pix_eop;
	if (((roi27_stb & roi27_x_good) & roi27_y_good)) begin
		roi27_count <= (roi27_count + roi27_gray);
	end
	roi27_out_update <= 1'd0;
	if (roi27_eop) begin
		roi27_count <= 1'd0;
		roi27_out_update <= 1'd1;
		roi27_out_count <= roi27_count;
	end
	if ((pix_y == roi28_cfg_y0)) begin
		roi28_y_good <= 1'd1;
	end
	if ((pix_y == roi28_cfg_y1)) begin
		roi28_y_good <= 1'd0;
	end
	if ((pix_x == roi28_cfg_x0)) begin
		roi28_x_good <= 1'd1;
	end
	if ((pix_x == roi28_cfg_x1)) begin
		roi28_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi28_y_good <= 1'd0;
		roi28_x_good <= 1'd0;
	end
	roi28_gray <= slice_proxy28[15:0];
	roi28_stb <= pix_stb;
	roi28_eop <= pix_eop;
	if (((roi28_stb & roi28_x_good) & roi28_y_good)) begin
		roi28_count <= (roi28_count + roi28_gray);
	end
	roi28_out_update <= 1'd0;
	if (roi28_eop) begin
		roi28_count <= 1'd0;
		roi28_out_update <= 1'd1;
		roi28_out_count <= roi28_count;
	end
	if ((pix_y == roi29_cfg_y0)) begin
		roi29_y_good <= 1'd1;
	end
	if ((pix_y == roi29_cfg_y1)) begin
		roi29_y_good <= 1'd0;
	end
	if ((pix_x == roi29_cfg_x0)) begin
		roi29_x_good <= 1'd1;
	end
	if ((pix_x == roi29_cfg_x1)) begin
		roi29_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi29_y_good <= 1'd0;
		roi29_x_good <= 1'd0;
	end
	roi29_gray <= slice_proxy29[15:0];
	roi29_stb <= pix_stb;
	roi29_eop <= pix_eop;
	if (((roi29_stb & roi29_x_good) & roi29_y_good)) begin
		roi29_count <= (roi29_count + roi29_gray);
	end
	roi29_out_update <= 1'd0;
	if (roi29_eop) begin
		roi29_count <= 1'd0;
		roi29_out_update <= 1'd1;
		roi29_out_count <= roi29_count;
	end
	if ((pix_y == roi30_cfg_y0)) begin
		roi30_y_good <= 1'd1;
	end
	if ((pix_y == roi30_cfg_y1)) begin
		roi30_y_good <= 1'd0;
	end
	if ((pix_x == roi30_cfg_x0)) begin
		roi30_x_good <= 1'd1;
	end
	if ((pix_x == roi30_cfg_x1)) begin
		roi30_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi30_y_good <= 1'd0;
		roi30_x_good <= 1'd0;
	end
	roi30_gray <= slice_proxy30[15:0];
	roi30_stb <= pix_stb;
	roi30_eop <= pix_eop;
	if (((roi30_stb & roi30_x_good) & roi30_y_good)) begin
		roi30_count <= (roi30_count + roi30_gray);
	end
	roi30_out_update <= 1'd0;
	if (roi30_eop) begin
		roi30_count <= 1'd0;
		roi30_out_update <= 1'd1;
		roi30_out_count <= roi30_count;
	end
	if ((pix_y == roi31_cfg_y0)) begin
		roi31_y_good <= 1'd1;
	end
	if ((pix_y == roi31_cfg_y1)) begin
		roi31_y_good <= 1'd0;
	end
	if ((pix_x == roi31_cfg_x0)) begin
		roi31_x_good <= 1'd1;
	end
	if ((pix_x == roi31_cfg_x1)) begin
		roi31_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi31_y_good <= 1'd0;
		roi31_x_good <= 1'd0;
	end
	roi31_gray <= slice_proxy31[15:0];
	roi31_stb <= pix_stb;
	roi31_eop <= pix_eop;
	if (((roi31_stb & roi31_x_good) & roi31_y_good)) begin
		roi31_count <= (roi31_count + roi31_gray);
	end
	roi31_out_update <= 1'd0;
	if (roi31_eop) begin
		roi31_count <= 1'd0;
		roi31_out_update <= 1'd1;
		roi31_out_count <= roi31_count;
	end
	if ((pix_y == roi32_cfg_y0)) begin
		roi32_y_good <= 1'd1;
	end
	if ((pix_y == roi32_cfg_y1)) begin
		roi32_y_good <= 1'd0;
	end
	if ((pix_x == roi32_cfg_x0)) begin
		roi32_x_good <= 1'd1;
	end
	if ((pix_x == roi32_cfg_x1)) begin
		roi32_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi32_y_good <= 1'd0;
		roi32_x_good <= 1'd0;
	end
	roi32_gray <= slice_proxy32[15:0];
	roi32_stb <= pix_stb;
	roi32_eop <= pix_eop;
	if (((roi32_stb & roi32_x_good) & roi32_y_good)) begin
		roi32_count <= (roi32_count + roi32_gray);
	end
	roi32_out_update <= 1'd0;
	if (roi32_eop) begin
		roi32_count <= 1'd0;
		roi32_out_update <= 1'd1;
		roi32_out_count <= roi32_count;
	end
	if ((pix_y == roi33_cfg_y0)) begin
		roi33_y_good <= 1'd1;
	end
	if ((pix_y == roi33_cfg_y1)) begin
		roi33_y_good <= 1'd0;
	end
	if ((pix_x == roi33_cfg_x0)) begin
		roi33_x_good <= 1'd1;
	end
	if ((pix_x == roi33_cfg_x1)) begin
		roi33_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi33_y_good <= 1'd0;
		roi33_x_good <= 1'd0;
	end
	roi33_gray <= slice_proxy33[15:0];
	roi33_stb <= pix_stb;
	roi33_eop <= pix_eop;
	if (((roi33_stb & roi33_x_good) & roi33_y_good)) begin
		roi33_count <= (roi33_count + roi33_gray);
	end
	roi33_out_update <= 1'd0;
	if (roi33_eop) begin
		roi33_count <= 1'd0;
		roi33_out_update <= 1'd1;
		roi33_out_count <= roi33_count;
	end
	if ((pix_y == roi34_cfg_y0)) begin
		roi34_y_good <= 1'd1;
	end
	if ((pix_y == roi34_cfg_y1)) begin
		roi34_y_good <= 1'd0;
	end
	if ((pix_x == roi34_cfg_x0)) begin
		roi34_x_good <= 1'd1;
	end
	if ((pix_x == roi34_cfg_x1)) begin
		roi34_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi34_y_good <= 1'd0;
		roi34_x_good <= 1'd0;
	end
	roi34_gray <= slice_proxy34[15:0];
	roi34_stb <= pix_stb;
	roi34_eop <= pix_eop;
	if (((roi34_stb & roi34_x_good) & roi34_y_good)) begin
		roi34_count <= (roi34_count + roi34_gray);
	end
	roi34_out_update <= 1'd0;
	if (roi34_eop) begin
		roi34_count <= 1'd0;
		roi34_out_update <= 1'd1;
		roi34_out_count <= roi34_count;
	end
	if ((pix_y == roi35_cfg_y0)) begin
		roi35_y_good <= 1'd1;
	end
	if ((pix_y == roi35_cfg_y1)) begin
		roi35_y_good <= 1'd0;
	end
	if ((pix_x == roi35_cfg_x0)) begin
		roi35_x_good <= 1'd1;
	end
	if ((pix_x == roi35_cfg_x1)) begin
		roi35_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi35_y_good <= 1'd0;
		roi35_x_good <= 1'd0;
	end
	roi35_gray <= slice_proxy35[15:0];
	roi35_stb <= pix_stb;
	roi35_eop <= pix_eop;
	if (((roi35_stb & roi35_x_good) & roi35_y_good)) begin
		roi35_count <= (roi35_count + roi35_gray);
	end
	roi35_out_update <= 1'd0;
	if (roi35_eop) begin
		roi35_count <= 1'd0;
		roi35_out_update <= 1'd1;
		roi35_out_count <= roi35_count;
	end
	if ((pix_y == roi36_cfg_y0)) begin
		roi36_y_good <= 1'd1;
	end
	if ((pix_y == roi36_cfg_y1)) begin
		roi36_y_good <= 1'd0;
	end
	if ((pix_x == roi36_cfg_x0)) begin
		roi36_x_good <= 1'd1;
	end
	if ((pix_x == roi36_cfg_x1)) begin
		roi36_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi36_y_good <= 1'd0;
		roi36_x_good <= 1'd0;
	end
	roi36_gray <= slice_proxy36[15:0];
	roi36_stb <= pix_stb;
	roi36_eop <= pix_eop;
	if (((roi36_stb & roi36_x_good) & roi36_y_good)) begin
		roi36_count <= (roi36_count + roi36_gray);
	end
	roi36_out_update <= 1'd0;
	if (roi36_eop) begin
		roi36_count <= 1'd0;
		roi36_out_update <= 1'd1;
		roi36_out_count <= roi36_count;
	end
	if ((pix_y == roi37_cfg_y0)) begin
		roi37_y_good <= 1'd1;
	end
	if ((pix_y == roi37_cfg_y1)) begin
		roi37_y_good <= 1'd0;
	end
	if ((pix_x == roi37_cfg_x0)) begin
		roi37_x_good <= 1'd1;
	end
	if ((pix_x == roi37_cfg_x1)) begin
		roi37_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi37_y_good <= 1'd0;
		roi37_x_good <= 1'd0;
	end
	roi37_gray <= slice_proxy37[15:0];
	roi37_stb <= pix_stb;
	roi37_eop <= pix_eop;
	if (((roi37_stb & roi37_x_good) & roi37_y_good)) begin
		roi37_count <= (roi37_count + roi37_gray);
	end
	roi37_out_update <= 1'd0;
	if (roi37_eop) begin
		roi37_count <= 1'd0;
		roi37_out_update <= 1'd1;
		roi37_out_count <= roi37_count;
	end
	if ((pix_y == roi38_cfg_y0)) begin
		roi38_y_good <= 1'd1;
	end
	if ((pix_y == roi38_cfg_y1)) begin
		roi38_y_good <= 1'd0;
	end
	if ((pix_x == roi38_cfg_x0)) begin
		roi38_x_good <= 1'd1;
	end
	if ((pix_x == roi38_cfg_x1)) begin
		roi38_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi38_y_good <= 1'd0;
		roi38_x_good <= 1'd0;
	end
	roi38_gray <= slice_proxy38[15:0];
	roi38_stb <= pix_stb;
	roi38_eop <= pix_eop;
	if (((roi38_stb & roi38_x_good) & roi38_y_good)) begin
		roi38_count <= (roi38_count + roi38_gray);
	end
	roi38_out_update <= 1'd0;
	if (roi38_eop) begin
		roi38_count <= 1'd0;
		roi38_out_update <= 1'd1;
		roi38_out_count <= roi38_count;
	end
	if ((pix_y == roi39_cfg_y0)) begin
		roi39_y_good <= 1'd1;
	end
	if ((pix_y == roi39_cfg_y1)) begin
		roi39_y_good <= 1'd0;
	end
	if ((pix_x == roi39_cfg_x0)) begin
		roi39_x_good <= 1'd1;
	end
	if ((pix_x == roi39_cfg_x1)) begin
		roi39_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi39_y_good <= 1'd0;
		roi39_x_good <= 1'd0;
	end
	roi39_gray <= slice_proxy39[15:0];
	roi39_stb <= pix_stb;
	roi39_eop <= pix_eop;
	if (((roi39_stb & roi39_x_good) & roi39_y_good)) begin
		roi39_count <= (roi39_count + roi39_gray);
	end
	roi39_out_update <= 1'd0;
	if (roi39_eop) begin
		roi39_count <= 1'd0;
		roi39_out_update <= 1'd1;
		roi39_out_count <= roi39_count;
	end
	if ((pix_y == roi40_cfg_y0)) begin
		roi40_y_good <= 1'd1;
	end
	if ((pix_y == roi40_cfg_y1)) begin
		roi40_y_good <= 1'd0;
	end
	if ((pix_x == roi40_cfg_x0)) begin
		roi40_x_good <= 1'd1;
	end
	if ((pix_x == roi40_cfg_x1)) begin
		roi40_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi40_y_good <= 1'd0;
		roi40_x_good <= 1'd0;
	end
	roi40_gray <= slice_proxy40[15:0];
	roi40_stb <= pix_stb;
	roi40_eop <= pix_eop;
	if (((roi40_stb & roi40_x_good) & roi40_y_good)) begin
		roi40_count <= (roi40_count + roi40_gray);
	end
	roi40_out_update <= 1'd0;
	if (roi40_eop) begin
		roi40_count <= 1'd0;
		roi40_out_update <= 1'd1;
		roi40_out_count <= roi40_count;
	end
	if ((pix_y == roi41_cfg_y0)) begin
		roi41_y_good <= 1'd1;
	end
	if ((pix_y == roi41_cfg_y1)) begin
		roi41_y_good <= 1'd0;
	end
	if ((pix_x == roi41_cfg_x0)) begin
		roi41_x_good <= 1'd1;
	end
	if ((pix_x == roi41_cfg_x1)) begin
		roi41_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi41_y_good <= 1'd0;
		roi41_x_good <= 1'd0;
	end
	roi41_gray <= slice_proxy41[15:0];
	roi41_stb <= pix_stb;
	roi41_eop <= pix_eop;
	if (((roi41_stb & roi41_x_good) & roi41_y_good)) begin
		roi41_count <= (roi41_count + roi41_gray);
	end
	roi41_out_update <= 1'd0;
	if (roi41_eop) begin
		roi41_count <= 1'd0;
		roi41_out_update <= 1'd1;
		roi41_out_count <= roi41_count;
	end
	if ((pix_y == roi42_cfg_y0)) begin
		roi42_y_good <= 1'd1;
	end
	if ((pix_y == roi42_cfg_y1)) begin
		roi42_y_good <= 1'd0;
	end
	if ((pix_x == roi42_cfg_x0)) begin
		roi42_x_good <= 1'd1;
	end
	if ((pix_x == roi42_cfg_x1)) begin
		roi42_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi42_y_good <= 1'd0;
		roi42_x_good <= 1'd0;
	end
	roi42_gray <= slice_proxy42[15:0];
	roi42_stb <= pix_stb;
	roi42_eop <= pix_eop;
	if (((roi42_stb & roi42_x_good) & roi42_y_good)) begin
		roi42_count <= (roi42_count + roi42_gray);
	end
	roi42_out_update <= 1'd0;
	if (roi42_eop) begin
		roi42_count <= 1'd0;
		roi42_out_update <= 1'd1;
		roi42_out_count <= roi42_count;
	end
	if ((pix_y == roi43_cfg_y0)) begin
		roi43_y_good <= 1'd1;
	end
	if ((pix_y == roi43_cfg_y1)) begin
		roi43_y_good <= 1'd0;
	end
	if ((pix_x == roi43_cfg_x0)) begin
		roi43_x_good <= 1'd1;
	end
	if ((pix_x == roi43_cfg_x1)) begin
		roi43_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi43_y_good <= 1'd0;
		roi43_x_good <= 1'd0;
	end
	roi43_gray <= slice_proxy43[15:0];
	roi43_stb <= pix_stb;
	roi43_eop <= pix_eop;
	if (((roi43_stb & roi43_x_good) & roi43_y_good)) begin
		roi43_count <= (roi43_count + roi43_gray);
	end
	roi43_out_update <= 1'd0;
	if (roi43_eop) begin
		roi43_count <= 1'd0;
		roi43_out_update <= 1'd1;
		roi43_out_count <= roi43_count;
	end
	if ((pix_y == roi44_cfg_y0)) begin
		roi44_y_good <= 1'd1;
	end
	if ((pix_y == roi44_cfg_y1)) begin
		roi44_y_good <= 1'd0;
	end
	if ((pix_x == roi44_cfg_x0)) begin
		roi44_x_good <= 1'd1;
	end
	if ((pix_x == roi44_cfg_x1)) begin
		roi44_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi44_y_good <= 1'd0;
		roi44_x_good <= 1'd0;
	end
	roi44_gray <= slice_proxy44[15:0];
	roi44_stb <= pix_stb;
	roi44_eop <= pix_eop;
	if (((roi44_stb & roi44_x_good) & roi44_y_good)) begin
		roi44_count <= (roi44_count + roi44_gray);
	end
	roi44_out_update <= 1'd0;
	if (roi44_eop) begin
		roi44_count <= 1'd0;
		roi44_out_update <= 1'd1;
		roi44_out_count <= roi44_count;
	end
	if ((pix_y == roi45_cfg_y0)) begin
		roi45_y_good <= 1'd1;
	end
	if ((pix_y == roi45_cfg_y1)) begin
		roi45_y_good <= 1'd0;
	end
	if ((pix_x == roi45_cfg_x0)) begin
		roi45_x_good <= 1'd1;
	end
	if ((pix_x == roi45_cfg_x1)) begin
		roi45_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi45_y_good <= 1'd0;
		roi45_x_good <= 1'd0;
	end
	roi45_gray <= slice_proxy45[15:0];
	roi45_stb <= pix_stb;
	roi45_eop <= pix_eop;
	if (((roi45_stb & roi45_x_good) & roi45_y_good)) begin
		roi45_count <= (roi45_count + roi45_gray);
	end
	roi45_out_update <= 1'd0;
	if (roi45_eop) begin
		roi45_count <= 1'd0;
		roi45_out_update <= 1'd1;
		roi45_out_count <= roi45_count;
	end
	if ((pix_y == roi46_cfg_y0)) begin
		roi46_y_good <= 1'd1;
	end
	if ((pix_y == roi46_cfg_y1)) begin
		roi46_y_good <= 1'd0;
	end
	if ((pix_x == roi46_cfg_x0)) begin
		roi46_x_good <= 1'd1;
	end
	if ((pix_x == roi46_cfg_x1)) begin
		roi46_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi46_y_good <= 1'd0;
		roi46_x_good <= 1'd0;
	end
	roi46_gray <= slice_proxy46[15:0];
	roi46_stb <= pix_stb;
	roi46_eop <= pix_eop;
	if (((roi46_stb & roi46_x_good) & roi46_y_good)) begin
		roi46_count <= (roi46_count + roi46_gray);
	end
	roi46_out_update <= 1'd0;
	if (roi46_eop) begin
		roi46_count <= 1'd0;
		roi46_out_update <= 1'd1;
		roi46_out_count <= roi46_count;
	end
	if ((pix_y == roi47_cfg_y0)) begin
		roi47_y_good <= 1'd1;
	end
	if ((pix_y == roi47_cfg_y1)) begin
		roi47_y_good <= 1'd0;
	end
	if ((pix_x == roi47_cfg_x0)) begin
		roi47_x_good <= 1'd1;
	end
	if ((pix_x == roi47_cfg_x1)) begin
		roi47_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi47_y_good <= 1'd0;
		roi47_x_good <= 1'd0;
	end
	roi47_gray <= slice_proxy47[15:0];
	roi47_stb <= pix_stb;
	roi47_eop <= pix_eop;
	if (((roi47_stb & roi47_x_good) & roi47_y_good)) begin
		roi47_count <= (roi47_count + roi47_gray);
	end
	roi47_out_update <= 1'd0;
	if (roi47_eop) begin
		roi47_count <= 1'd0;
		roi47_out_update <= 1'd1;
		roi47_out_count <= roi47_count;
	end
	if ((pix_y == roi48_cfg_y0)) begin
		roi48_y_good <= 1'd1;
	end
	if ((pix_y == roi48_cfg_y1)) begin
		roi48_y_good <= 1'd0;
	end
	if ((pix_x == roi48_cfg_x0)) begin
		roi48_x_good <= 1'd1;
	end
	if ((pix_x == roi48_cfg_x1)) begin
		roi48_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi48_y_good <= 1'd0;
		roi48_x_good <= 1'd0;
	end
	roi48_gray <= slice_proxy48[15:0];
	roi48_stb <= pix_stb;
	roi48_eop <= pix_eop;
	if (((roi48_stb & roi48_x_good) & roi48_y_good)) begin
		roi48_count <= (roi48_count + roi48_gray);
	end
	roi48_out_update <= 1'd0;
	if (roi48_eop) begin
		roi48_count <= 1'd0;
		roi48_out_update <= 1'd1;
		roi48_out_count <= roi48_count;
	end
	if ((pix_y == roi49_cfg_y0)) begin
		roi49_y_good <= 1'd1;
	end
	if ((pix_y == roi49_cfg_y1)) begin
		roi49_y_good <= 1'd0;
	end
	if ((pix_x == roi49_cfg_x0)) begin
		roi49_x_good <= 1'd1;
	end
	if ((pix_x == roi49_cfg_x1)) begin
		roi49_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi49_y_good <= 1'd0;
		roi49_x_good <= 1'd0;
	end
	roi49_gray <= slice_proxy49[15:0];
	roi49_stb <= pix_stb;
	roi49_eop <= pix_eop;
	if (((roi49_stb & roi49_x_good) & roi49_y_good)) begin
		roi49_count <= (roi49_count + roi49_gray);
	end
	roi49_out_update <= 1'd0;
	if (roi49_eop) begin
		roi49_count <= 1'd0;
		roi49_out_update <= 1'd1;
		roi49_out_count <= roi49_count;
	end
	if ((pix_y == roi50_cfg_y0)) begin
		roi50_y_good <= 1'd1;
	end
	if ((pix_y == roi50_cfg_y1)) begin
		roi50_y_good <= 1'd0;
	end
	if ((pix_x == roi50_cfg_x0)) begin
		roi50_x_good <= 1'd1;
	end
	if ((pix_x == roi50_cfg_x1)) begin
		roi50_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi50_y_good <= 1'd0;
		roi50_x_good <= 1'd0;
	end
	roi50_gray <= slice_proxy50[15:0];
	roi50_stb <= pix_stb;
	roi50_eop <= pix_eop;
	if (((roi50_stb & roi50_x_good) & roi50_y_good)) begin
		roi50_count <= (roi50_count + roi50_gray);
	end
	roi50_out_update <= 1'd0;
	if (roi50_eop) begin
		roi50_count <= 1'd0;
		roi50_out_update <= 1'd1;
		roi50_out_count <= roi50_count;
	end
	if ((pix_y == roi51_cfg_y0)) begin
		roi51_y_good <= 1'd1;
	end
	if ((pix_y == roi51_cfg_y1)) begin
		roi51_y_good <= 1'd0;
	end
	if ((pix_x == roi51_cfg_x0)) begin
		roi51_x_good <= 1'd1;
	end
	if ((pix_x == roi51_cfg_x1)) begin
		roi51_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi51_y_good <= 1'd0;
		roi51_x_good <= 1'd0;
	end
	roi51_gray <= slice_proxy51[15:0];
	roi51_stb <= pix_stb;
	roi51_eop <= pix_eop;
	if (((roi51_stb & roi51_x_good) & roi51_y_good)) begin
		roi51_count <= (roi51_count + roi51_gray);
	end
	roi51_out_update <= 1'd0;
	if (roi51_eop) begin
		roi51_count <= 1'd0;
		roi51_out_update <= 1'd1;
		roi51_out_count <= roi51_count;
	end
	if ((pix_y == roi52_cfg_y0)) begin
		roi52_y_good <= 1'd1;
	end
	if ((pix_y == roi52_cfg_y1)) begin
		roi52_y_good <= 1'd0;
	end
	if ((pix_x == roi52_cfg_x0)) begin
		roi52_x_good <= 1'd1;
	end
	if ((pix_x == roi52_cfg_x1)) begin
		roi52_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi52_y_good <= 1'd0;
		roi52_x_good <= 1'd0;
	end
	roi52_gray <= slice_proxy52[15:0];
	roi52_stb <= pix_stb;
	roi52_eop <= pix_eop;
	if (((roi52_stb & roi52_x_good) & roi52_y_good)) begin
		roi52_count <= (roi52_count + roi52_gray);
	end
	roi52_out_update <= 1'd0;
	if (roi52_eop) begin
		roi52_count <= 1'd0;
		roi52_out_update <= 1'd1;
		roi52_out_count <= roi52_count;
	end
	if ((pix_y == roi53_cfg_y0)) begin
		roi53_y_good <= 1'd1;
	end
	if ((pix_y == roi53_cfg_y1)) begin
		roi53_y_good <= 1'd0;
	end
	if ((pix_x == roi53_cfg_x0)) begin
		roi53_x_good <= 1'd1;
	end
	if ((pix_x == roi53_cfg_x1)) begin
		roi53_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi53_y_good <= 1'd0;
		roi53_x_good <= 1'd0;
	end
	roi53_gray <= slice_proxy53[15:0];
	roi53_stb <= pix_stb;
	roi53_eop <= pix_eop;
	if (((roi53_stb & roi53_x_good) & roi53_y_good)) begin
		roi53_count <= (roi53_count + roi53_gray);
	end
	roi53_out_update <= 1'd0;
	if (roi53_eop) begin
		roi53_count <= 1'd0;
		roi53_out_update <= 1'd1;
		roi53_out_count <= roi53_count;
	end
	if ((pix_y == roi54_cfg_y0)) begin
		roi54_y_good <= 1'd1;
	end
	if ((pix_y == roi54_cfg_y1)) begin
		roi54_y_good <= 1'd0;
	end
	if ((pix_x == roi54_cfg_x0)) begin
		roi54_x_good <= 1'd1;
	end
	if ((pix_x == roi54_cfg_x1)) begin
		roi54_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi54_y_good <= 1'd0;
		roi54_x_good <= 1'd0;
	end
	roi54_gray <= slice_proxy54[15:0];
	roi54_stb <= pix_stb;
	roi54_eop <= pix_eop;
	if (((roi54_stb & roi54_x_good) & roi54_y_good)) begin
		roi54_count <= (roi54_count + roi54_gray);
	end
	roi54_out_update <= 1'd0;
	if (roi54_eop) begin
		roi54_count <= 1'd0;
		roi54_out_update <= 1'd1;
		roi54_out_count <= roi54_count;
	end
	if ((pix_y == roi55_cfg_y0)) begin
		roi55_y_good <= 1'd1;
	end
	if ((pix_y == roi55_cfg_y1)) begin
		roi55_y_good <= 1'd0;
	end
	if ((pix_x == roi55_cfg_x0)) begin
		roi55_x_good <= 1'd1;
	end
	if ((pix_x == roi55_cfg_x1)) begin
		roi55_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi55_y_good <= 1'd0;
		roi55_x_good <= 1'd0;
	end
	roi55_gray <= slice_proxy55[15:0];
	roi55_stb <= pix_stb;
	roi55_eop <= pix_eop;
	if (((roi55_stb & roi55_x_good) & roi55_y_good)) begin
		roi55_count <= (roi55_count + roi55_gray);
	end
	roi55_out_update <= 1'd0;
	if (roi55_eop) begin
		roi55_count <= 1'd0;
		roi55_out_update <= 1'd1;
		roi55_out_count <= roi55_count;
	end
	if ((pix_y == roi56_cfg_y0)) begin
		roi56_y_good <= 1'd1;
	end
	if ((pix_y == roi56_cfg_y1)) begin
		roi56_y_good <= 1'd0;
	end
	if ((pix_x == roi56_cfg_x0)) begin
		roi56_x_good <= 1'd1;
	end
	if ((pix_x == roi56_cfg_x1)) begin
		roi56_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi56_y_good <= 1'd0;
		roi56_x_good <= 1'd0;
	end
	roi56_gray <= slice_proxy56[15:0];
	roi56_stb <= pix_stb;
	roi56_eop <= pix_eop;
	if (((roi56_stb & roi56_x_good) & roi56_y_good)) begin
		roi56_count <= (roi56_count + roi56_gray);
	end
	roi56_out_update <= 1'd0;
	if (roi56_eop) begin
		roi56_count <= 1'd0;
		roi56_out_update <= 1'd1;
		roi56_out_count <= roi56_count;
	end
	if ((pix_y == roi57_cfg_y0)) begin
		roi57_y_good <= 1'd1;
	end
	if ((pix_y == roi57_cfg_y1)) begin
		roi57_y_good <= 1'd0;
	end
	if ((pix_x == roi57_cfg_x0)) begin
		roi57_x_good <= 1'd1;
	end
	if ((pix_x == roi57_cfg_x1)) begin
		roi57_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi57_y_good <= 1'd0;
		roi57_x_good <= 1'd0;
	end
	roi57_gray <= slice_proxy57[15:0];
	roi57_stb <= pix_stb;
	roi57_eop <= pix_eop;
	if (((roi57_stb & roi57_x_good) & roi57_y_good)) begin
		roi57_count <= (roi57_count + roi57_gray);
	end
	roi57_out_update <= 1'd0;
	if (roi57_eop) begin
		roi57_count <= 1'd0;
		roi57_out_update <= 1'd1;
		roi57_out_count <= roi57_count;
	end
	if ((pix_y == roi58_cfg_y0)) begin
		roi58_y_good <= 1'd1;
	end
	if ((pix_y == roi58_cfg_y1)) begin
		roi58_y_good <= 1'd0;
	end
	if ((pix_x == roi58_cfg_x0)) begin
		roi58_x_good <= 1'd1;
	end
	if ((pix_x == roi58_cfg_x1)) begin
		roi58_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi58_y_good <= 1'd0;
		roi58_x_good <= 1'd0;
	end
	roi58_gray <= slice_proxy58[15:0];
	roi58_stb <= pix_stb;
	roi58_eop <= pix_eop;
	if (((roi58_stb & roi58_x_good) & roi58_y_good)) begin
		roi58_count <= (roi58_count + roi58_gray);
	end
	roi58_out_update <= 1'd0;
	if (roi58_eop) begin
		roi58_count <= 1'd0;
		roi58_out_update <= 1'd1;
		roi58_out_count <= roi58_count;
	end
	if ((pix_y == roi59_cfg_y0)) begin
		roi59_y_good <= 1'd1;
	end
	if ((pix_y == roi59_cfg_y1)) begin
		roi59_y_good <= 1'd0;
	end
	if ((pix_x == roi59_cfg_x0)) begin
		roi59_x_good <= 1'd1;
	end
	if ((pix_x == roi59_cfg_x1)) begin
		roi59_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi59_y_good <= 1'd0;
		roi59_x_good <= 1'd0;
	end
	roi59_gray <= slice_proxy59[15:0];
	roi59_stb <= pix_stb;
	roi59_eop <= pix_eop;
	if (((roi59_stb & roi59_x_good) & roi59_y_good)) begin
		roi59_count <= (roi59_count + roi59_gray);
	end
	roi59_out_update <= 1'd0;
	if (roi59_eop) begin
		roi59_count <= 1'd0;
		roi59_out_update <= 1'd1;
		roi59_out_count <= roi59_count;
	end
	if ((pix_y == roi60_cfg_y0)) begin
		roi60_y_good <= 1'd1;
	end
	if ((pix_y == roi60_cfg_y1)) begin
		roi60_y_good <= 1'd0;
	end
	if ((pix_x == roi60_cfg_x0)) begin
		roi60_x_good <= 1'd1;
	end
	if ((pix_x == roi60_cfg_x1)) begin
		roi60_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi60_y_good <= 1'd0;
		roi60_x_good <= 1'd0;
	end
	roi60_gray <= slice_proxy60[15:0];
	roi60_stb <= pix_stb;
	roi60_eop <= pix_eop;
	if (((roi60_stb & roi60_x_good) & roi60_y_good)) begin
		roi60_count <= (roi60_count + roi60_gray);
	end
	roi60_out_update <= 1'd0;
	if (roi60_eop) begin
		roi60_count <= 1'd0;
		roi60_out_update <= 1'd1;
		roi60_out_count <= roi60_count;
	end
	if ((pix_y == roi61_cfg_y0)) begin
		roi61_y_good <= 1'd1;
	end
	if ((pix_y == roi61_cfg_y1)) begin
		roi61_y_good <= 1'd0;
	end
	if ((pix_x == roi61_cfg_x0)) begin
		roi61_x_good <= 1'd1;
	end
	if ((pix_x == roi61_cfg_x1)) begin
		roi61_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi61_y_good <= 1'd0;
		roi61_x_good <= 1'd0;
	end
	roi61_gray <= slice_proxy61[15:0];
	roi61_stb <= pix_stb;
	roi61_eop <= pix_eop;
	if (((roi61_stb & roi61_x_good) & roi61_y_good)) begin
		roi61_count <= (roi61_count + roi61_gray);
	end
	roi61_out_update <= 1'd0;
	if (roi61_eop) begin
		roi61_count <= 1'd0;
		roi61_out_update <= 1'd1;
		roi61_out_count <= roi61_count;
	end
	if ((pix_y == roi62_cfg_y0)) begin
		roi62_y_good <= 1'd1;
	end
	if ((pix_y == roi62_cfg_y1)) begin
		roi62_y_good <= 1'd0;
	end
	if ((pix_x == roi62_cfg_x0)) begin
		roi62_x_good <= 1'd1;
	end
	if ((pix_x == roi62_cfg_x1)) begin
		roi62_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi62_y_good <= 1'd0;
		roi62_x_good <= 1'd0;
	end
	roi62_gray <= slice_proxy62[15:0];
	roi62_stb <= pix_stb;
	roi62_eop <= pix_eop;
	if (((roi62_stb & roi62_x_good) & roi62_y_good)) begin
		roi62_count <= (roi62_count + roi62_gray);
	end
	roi62_out_update <= 1'd0;
	if (roi62_eop) begin
		roi62_count <= 1'd0;
		roi62_out_update <= 1'd1;
		roi62_out_count <= roi62_count;
	end
	if ((pix_y == roi63_cfg_y0)) begin
		roi63_y_good <= 1'd1;
	end
	if ((pix_y == roi63_cfg_y1)) begin
		roi63_y_good <= 1'd0;
	end
	if ((pix_x == roi63_cfg_x0)) begin
		roi63_x_good <= 1'd1;
	end
	if ((pix_x == roi63_cfg_x1)) begin
		roi63_x_good <= 1'd0;
	end
	if (pix_eop) begin
		roi63_y_good <= 1'd0;
		roi63_x_good <= 1'd0;
	end
	roi63_gray <= slice_proxy63[15:0];
	roi63_stb <= pix_stb;
	roi63_eop <= pix_eop;
	if (((roi63_stb & roi63_x_good) & roi63_y_good)) begin
		roi63_count <= (roi63_count + roi63_gray);
	end
	roi63_out_update <= 1'd0;
	if (roi63_eop) begin
		roi63_count <= 1'd0;
		roi63_out_update <= 1'd1;
		roi63_out_count <= roi63_count;
	end
	if (cl_rst) begin
		pix_x <= 12'd0;
		pix_y <= 12'd0;
		last_x <= 12'd0;
		last_y <= 12'd0;
		last_lval <= 1'd0;
		last_fval <= 1'd0;
		roi0_out_update <= 1'd0;
		roi0_out_count <= 31'd0;
		roi0_y_good <= 1'd0;
		roi0_x_good <= 1'd0;
		roi0_stb <= 1'd0;
		roi0_eop <= 1'd0;
		roi0_gray <= 16'd0;
		roi0_count <= 31'd0;
		roi1_out_update <= 1'd0;
		roi1_out_count <= 31'd0;
		roi1_y_good <= 1'd0;
		roi1_x_good <= 1'd0;
		roi1_stb <= 1'd0;
		roi1_eop <= 1'd0;
		roi1_gray <= 16'd0;
		roi1_count <= 31'd0;
		roi2_out_update <= 1'd0;
		roi2_out_count <= 31'd0;
		roi2_y_good <= 1'd0;
		roi2_x_good <= 1'd0;
		roi2_stb <= 1'd0;
		roi2_eop <= 1'd0;
		roi2_gray <= 16'd0;
		roi2_count <= 31'd0;
		roi3_out_update <= 1'd0;
		roi3_out_count <= 31'd0;
		roi3_y_good <= 1'd0;
		roi3_x_good <= 1'd0;
		roi3_stb <= 1'd0;
		roi3_eop <= 1'd0;
		roi3_gray <= 16'd0;
		roi3_count <= 31'd0;
		roi4_out_update <= 1'd0;
		roi4_out_count <= 31'd0;
		roi4_y_good <= 1'd0;
		roi4_x_good <= 1'd0;
		roi4_stb <= 1'd0;
		roi4_eop <= 1'd0;
		roi4_gray <= 16'd0;
		roi4_count <= 31'd0;
		roi5_out_update <= 1'd0;
		roi5_out_count <= 31'd0;
		roi5_y_good <= 1'd0;
		roi5_x_good <= 1'd0;
		roi5_stb <= 1'd0;
		roi5_eop <= 1'd0;
		roi5_gray <= 16'd0;
		roi5_count <= 31'd0;
		roi6_out_update <= 1'd0;
		roi6_out_count <= 31'd0;
		roi6_y_good <= 1'd0;
		roi6_x_good <= 1'd0;
		roi6_stb <= 1'd0;
		roi6_eop <= 1'd0;
		roi6_gray <= 16'd0;
		roi6_count <= 31'd0;
		roi7_out_update <= 1'd0;
		roi7_out_count <= 31'd0;
		roi7_y_good <= 1'd0;
		roi7_x_good <= 1'd0;
		roi7_stb <= 1'd0;
		roi7_eop <= 1'd0;
		roi7_gray <= 16'd0;
		roi7_count <= 31'd0;
		roi8_out_update <= 1'd0;
		roi8_out_count <= 31'd0;
		roi8_y_good <= 1'd0;
		roi8_x_good <= 1'd0;
		roi8_stb <= 1'd0;
		roi8_eop <= 1'd0;
		roi8_gray <= 16'd0;
		roi8_count <= 31'd0;
		roi9_out_update <= 1'd0;
		roi9_out_count <= 31'd0;
		roi9_y_good <= 1'd0;
		roi9_x_good <= 1'd0;
		roi9_stb <= 1'd0;
		roi9_eop <= 1'd0;
		roi9_gray <= 16'd0;
		roi9_count <= 31'd0;
		roi10_out_update <= 1'd0;
		roi10_out_count <= 31'd0;
		roi10_y_good <= 1'd0;
		roi10_x_good <= 1'd0;
		roi10_stb <= 1'd0;
		roi10_eop <= 1'd0;
		roi10_gray <= 16'd0;
		roi10_count <= 31'd0;
		roi11_out_update <= 1'd0;
		roi11_out_count <= 31'd0;
		roi11_y_good <= 1'd0;
		roi11_x_good <= 1'd0;
		roi11_stb <= 1'd0;
		roi11_eop <= 1'd0;
		roi11_gray <= 16'd0;
		roi11_count <= 31'd0;
		roi12_out_update <= 1'd0;
		roi12_out_count <= 31'd0;
		roi12_y_good <= 1'd0;
		roi12_x_good <= 1'd0;
		roi12_stb <= 1'd0;
		roi12_eop <= 1'd0;
		roi12_gray <= 16'd0;
		roi12_count <= 31'd0;
		roi13_out_update <= 1'd0;
		roi13_out_count <= 31'd0;
		roi13_y_good <= 1'd0;
		roi13_x_good <= 1'd0;
		roi13_stb <= 1'd0;
		roi13_eop <= 1'd0;
		roi13_gray <= 16'd0;
		roi13_count <= 31'd0;
		roi14_out_update <= 1'd0;
		roi14_out_count <= 31'd0;
		roi14_y_good <= 1'd0;
		roi14_x_good <= 1'd0;
		roi14_stb <= 1'd0;
		roi14_eop <= 1'd0;
		roi14_gray <= 16'd0;
		roi14_count <= 31'd0;
		roi15_out_update <= 1'd0;
		roi15_out_count <= 31'd0;
		roi15_y_good <= 1'd0;
		roi15_x_good <= 1'd0;
		roi15_stb <= 1'd0;
		roi15_eop <= 1'd0;
		roi15_gray <= 16'd0;
		roi15_count <= 31'd0;
		roi16_out_update <= 1'd0;
		roi16_out_count <= 31'd0;
		roi16_y_good <= 1'd0;
		roi16_x_good <= 1'd0;
		roi16_stb <= 1'd0;
		roi16_eop <= 1'd0;
		roi16_gray <= 16'd0;
		roi16_count <= 31'd0;
		roi17_out_update <= 1'd0;
		roi17_out_count <= 31'd0;
		roi17_y_good <= 1'd0;
		roi17_x_good <= 1'd0;
		roi17_stb <= 1'd0;
		roi17_eop <= 1'd0;
		roi17_gray <= 16'd0;
		roi17_count <= 31'd0;
		roi18_out_update <= 1'd0;
		roi18_out_count <= 31'd0;
		roi18_y_good <= 1'd0;
		roi18_x_good <= 1'd0;
		roi18_stb <= 1'd0;
		roi18_eop <= 1'd0;
		roi18_gray <= 16'd0;
		roi18_count <= 31'd0;
		roi19_out_update <= 1'd0;
		roi19_out_count <= 31'd0;
		roi19_y_good <= 1'd0;
		roi19_x_good <= 1'd0;
		roi19_stb <= 1'd0;
		roi19_eop <= 1'd0;
		roi19_gray <= 16'd0;
		roi19_count <= 31'd0;
		roi20_out_update <= 1'd0;
		roi20_out_count <= 31'd0;
		roi20_y_good <= 1'd0;
		roi20_x_good <= 1'd0;
		roi20_stb <= 1'd0;
		roi20_eop <= 1'd0;
		roi20_gray <= 16'd0;
		roi20_count <= 31'd0;
		roi21_out_update <= 1'd0;
		roi21_out_count <= 31'd0;
		roi21_y_good <= 1'd0;
		roi21_x_good <= 1'd0;
		roi21_stb <= 1'd0;
		roi21_eop <= 1'd0;
		roi21_gray <= 16'd0;
		roi21_count <= 31'd0;
		roi22_out_update <= 1'd0;
		roi22_out_count <= 31'd0;
		roi22_y_good <= 1'd0;
		roi22_x_good <= 1'd0;
		roi22_stb <= 1'd0;
		roi22_eop <= 1'd0;
		roi22_gray <= 16'd0;
		roi22_count <= 31'd0;
		roi23_out_update <= 1'd0;
		roi23_out_count <= 31'd0;
		roi23_y_good <= 1'd0;
		roi23_x_good <= 1'd0;
		roi23_stb <= 1'd0;
		roi23_eop <= 1'd0;
		roi23_gray <= 16'd0;
		roi23_count <= 31'd0;
		roi24_out_update <= 1'd0;
		roi24_out_count <= 31'd0;
		roi24_y_good <= 1'd0;
		roi24_x_good <= 1'd0;
		roi24_stb <= 1'd0;
		roi24_eop <= 1'd0;
		roi24_gray <= 16'd0;
		roi24_count <= 31'd0;
		roi25_out_update <= 1'd0;
		roi25_out_count <= 31'd0;
		roi25_y_good <= 1'd0;
		roi25_x_good <= 1'd0;
		roi25_stb <= 1'd0;
		roi25_eop <= 1'd0;
		roi25_gray <= 16'd0;
		roi25_count <= 31'd0;
		roi26_out_update <= 1'd0;
		roi26_out_count <= 31'd0;
		roi26_y_good <= 1'd0;
		roi26_x_good <= 1'd0;
		roi26_stb <= 1'd0;
		roi26_eop <= 1'd0;
		roi26_gray <= 16'd0;
		roi26_count <= 31'd0;
		roi27_out_update <= 1'd0;
		roi27_out_count <= 31'd0;
		roi27_y_good <= 1'd0;
		roi27_x_good <= 1'd0;
		roi27_stb <= 1'd0;
		roi27_eop <= 1'd0;
		roi27_gray <= 16'd0;
		roi27_count <= 31'd0;
		roi28_out_update <= 1'd0;
		roi28_out_count <= 31'd0;
		roi28_y_good <= 1'd0;
		roi28_x_good <= 1'd0;
		roi28_stb <= 1'd0;
		roi28_eop <= 1'd0;
		roi28_gray <= 16'd0;
		roi28_count <= 31'd0;
		roi29_out_update <= 1'd0;
		roi29_out_count <= 31'd0;
		roi29_y_good <= 1'd0;
		roi29_x_good <= 1'd0;
		roi29_stb <= 1'd0;
		roi29_eop <= 1'd0;
		roi29_gray <= 16'd0;
		roi29_count <= 31'd0;
		roi30_out_update <= 1'd0;
		roi30_out_count <= 31'd0;
		roi30_y_good <= 1'd0;
		roi30_x_good <= 1'd0;
		roi30_stb <= 1'd0;
		roi30_eop <= 1'd0;
		roi30_gray <= 16'd0;
		roi30_count <= 31'd0;
		roi31_out_update <= 1'd0;
		roi31_out_count <= 31'd0;
		roi31_y_good <= 1'd0;
		roi31_x_good <= 1'd0;
		roi31_stb <= 1'd0;
		roi31_eop <= 1'd0;
		roi31_gray <= 16'd0;
		roi31_count <= 31'd0;
		roi32_out_update <= 1'd0;
		roi32_out_count <= 31'd0;
		roi32_y_good <= 1'd0;
		roi32_x_good <= 1'd0;
		roi32_stb <= 1'd0;
		roi32_eop <= 1'd0;
		roi32_gray <= 16'd0;
		roi32_count <= 31'd0;
		roi33_out_update <= 1'd0;
		roi33_out_count <= 31'd0;
		roi33_y_good <= 1'd0;
		roi33_x_good <= 1'd0;
		roi33_stb <= 1'd0;
		roi33_eop <= 1'd0;
		roi33_gray <= 16'd0;
		roi33_count <= 31'd0;
		roi34_out_update <= 1'd0;
		roi34_out_count <= 31'd0;
		roi34_y_good <= 1'd0;
		roi34_x_good <= 1'd0;
		roi34_stb <= 1'd0;
		roi34_eop <= 1'd0;
		roi34_gray <= 16'd0;
		roi34_count <= 31'd0;
		roi35_out_update <= 1'd0;
		roi35_out_count <= 31'd0;
		roi35_y_good <= 1'd0;
		roi35_x_good <= 1'd0;
		roi35_stb <= 1'd0;
		roi35_eop <= 1'd0;
		roi35_gray <= 16'd0;
		roi35_count <= 31'd0;
		roi36_out_update <= 1'd0;
		roi36_out_count <= 31'd0;
		roi36_y_good <= 1'd0;
		roi36_x_good <= 1'd0;
		roi36_stb <= 1'd0;
		roi36_eop <= 1'd0;
		roi36_gray <= 16'd0;
		roi36_count <= 31'd0;
		roi37_out_update <= 1'd0;
		roi37_out_count <= 31'd0;
		roi37_y_good <= 1'd0;
		roi37_x_good <= 1'd0;
		roi37_stb <= 1'd0;
		roi37_eop <= 1'd0;
		roi37_gray <= 16'd0;
		roi37_count <= 31'd0;
		roi38_out_update <= 1'd0;
		roi38_out_count <= 31'd0;
		roi38_y_good <= 1'd0;
		roi38_x_good <= 1'd0;
		roi38_stb <= 1'd0;
		roi38_eop <= 1'd0;
		roi38_gray <= 16'd0;
		roi38_count <= 31'd0;
		roi39_out_update <= 1'd0;
		roi39_out_count <= 31'd0;
		roi39_y_good <= 1'd0;
		roi39_x_good <= 1'd0;
		roi39_stb <= 1'd0;
		roi39_eop <= 1'd0;
		roi39_gray <= 16'd0;
		roi39_count <= 31'd0;
		roi40_out_update <= 1'd0;
		roi40_out_count <= 31'd0;
		roi40_y_good <= 1'd0;
		roi40_x_good <= 1'd0;
		roi40_stb <= 1'd0;
		roi40_eop <= 1'd0;
		roi40_gray <= 16'd0;
		roi40_count <= 31'd0;
		roi41_out_update <= 1'd0;
		roi41_out_count <= 31'd0;
		roi41_y_good <= 1'd0;
		roi41_x_good <= 1'd0;
		roi41_stb <= 1'd0;
		roi41_eop <= 1'd0;
		roi41_gray <= 16'd0;
		roi41_count <= 31'd0;
		roi42_out_update <= 1'd0;
		roi42_out_count <= 31'd0;
		roi42_y_good <= 1'd0;
		roi42_x_good <= 1'd0;
		roi42_stb <= 1'd0;
		roi42_eop <= 1'd0;
		roi42_gray <= 16'd0;
		roi42_count <= 31'd0;
		roi43_out_update <= 1'd0;
		roi43_out_count <= 31'd0;
		roi43_y_good <= 1'd0;
		roi43_x_good <= 1'd0;
		roi43_stb <= 1'd0;
		roi43_eop <= 1'd0;
		roi43_gray <= 16'd0;
		roi43_count <= 31'd0;
		roi44_out_update <= 1'd0;
		roi44_out_count <= 31'd0;
		roi44_y_good <= 1'd0;
		roi44_x_good <= 1'd0;
		roi44_stb <= 1'd0;
		roi44_eop <= 1'd0;
		roi44_gray <= 16'd0;
		roi44_count <= 31'd0;
		roi45_out_update <= 1'd0;
		roi45_out_count <= 31'd0;
		roi45_y_good <= 1'd0;
		roi45_x_good <= 1'd0;
		roi45_stb <= 1'd0;
		roi45_eop <= 1'd0;
		roi45_gray <= 16'd0;
		roi45_count <= 31'd0;
		roi46_out_update <= 1'd0;
		roi46_out_count <= 31'd0;
		roi46_y_good <= 1'd0;
		roi46_x_good <= 1'd0;
		roi46_stb <= 1'd0;
		roi46_eop <= 1'd0;
		roi46_gray <= 16'd0;
		roi46_count <= 31'd0;
		roi47_out_update <= 1'd0;
		roi47_out_count <= 31'd0;
		roi47_y_good <= 1'd0;
		roi47_x_good <= 1'd0;
		roi47_stb <= 1'd0;
		roi47_eop <= 1'd0;
		roi47_gray <= 16'd0;
		roi47_count <= 31'd0;
		roi48_out_update <= 1'd0;
		roi48_out_count <= 31'd0;
		roi48_y_good <= 1'd0;
		roi48_x_good <= 1'd0;
		roi48_stb <= 1'd0;
		roi48_eop <= 1'd0;
		roi48_gray <= 16'd0;
		roi48_count <= 31'd0;
		roi49_out_update <= 1'd0;
		roi49_out_count <= 31'd0;
		roi49_y_good <= 1'd0;
		roi49_x_good <= 1'd0;
		roi49_stb <= 1'd0;
		roi49_eop <= 1'd0;
		roi49_gray <= 16'd0;
		roi49_count <= 31'd0;
		roi50_out_update <= 1'd0;
		roi50_out_count <= 31'd0;
		roi50_y_good <= 1'd0;
		roi50_x_good <= 1'd0;
		roi50_stb <= 1'd0;
		roi50_eop <= 1'd0;
		roi50_gray <= 16'd0;
		roi50_count <= 31'd0;
		roi51_out_update <= 1'd0;
		roi51_out_count <= 31'd0;
		roi51_y_good <= 1'd0;
		roi51_x_good <= 1'd0;
		roi51_stb <= 1'd0;
		roi51_eop <= 1'd0;
		roi51_gray <= 16'd0;
		roi51_count <= 31'd0;
		roi52_out_update <= 1'd0;
		roi52_out_count <= 31'd0;
		roi52_y_good <= 1'd0;
		roi52_x_good <= 1'd0;
		roi52_stb <= 1'd0;
		roi52_eop <= 1'd0;
		roi52_gray <= 16'd0;
		roi52_count <= 31'd0;
		roi53_out_update <= 1'd0;
		roi53_out_count <= 31'd0;
		roi53_y_good <= 1'd0;
		roi53_x_good <= 1'd0;
		roi53_stb <= 1'd0;
		roi53_eop <= 1'd0;
		roi53_gray <= 16'd0;
		roi53_count <= 31'd0;
		roi54_out_update <= 1'd0;
		roi54_out_count <= 31'd0;
		roi54_y_good <= 1'd0;
		roi54_x_good <= 1'd0;
		roi54_stb <= 1'd0;
		roi54_eop <= 1'd0;
		roi54_gray <= 16'd0;
		roi54_count <= 31'd0;
		roi55_out_update <= 1'd0;
		roi55_out_count <= 31'd0;
		roi55_y_good <= 1'd0;
		roi55_x_good <= 1'd0;
		roi55_stb <= 1'd0;
		roi55_eop <= 1'd0;
		roi55_gray <= 16'd0;
		roi55_count <= 31'd0;
		roi56_out_update <= 1'd0;
		roi56_out_count <= 31'd0;
		roi56_y_good <= 1'd0;
		roi56_x_good <= 1'd0;
		roi56_stb <= 1'd0;
		roi56_eop <= 1'd0;
		roi56_gray <= 16'd0;
		roi56_count <= 31'd0;
		roi57_out_update <= 1'd0;
		roi57_out_count <= 31'd0;
		roi57_y_good <= 1'd0;
		roi57_x_good <= 1'd0;
		roi57_stb <= 1'd0;
		roi57_eop <= 1'd0;
		roi57_gray <= 16'd0;
		roi57_count <= 31'd0;
		roi58_out_update <= 1'd0;
		roi58_out_count <= 31'd0;
		roi58_y_good <= 1'd0;
		roi58_x_good <= 1'd0;
		roi58_stb <= 1'd0;
		roi58_eop <= 1'd0;
		roi58_gray <= 16'd0;
		roi58_count <= 31'd0;
		roi59_out_update <= 1'd0;
		roi59_out_count <= 31'd0;
		roi59_y_good <= 1'd0;
		roi59_x_good <= 1'd0;
		roi59_stb <= 1'd0;
		roi59_eop <= 1'd0;
		roi59_gray <= 16'd0;
		roi59_count <= 31'd0;
		roi60_out_update <= 1'd0;
		roi60_out_count <= 31'd0;
		roi60_y_good <= 1'd0;
		roi60_x_good <= 1'd0;
		roi60_stb <= 1'd0;
		roi60_eop <= 1'd0;
		roi60_gray <= 16'd0;
		roi60_count <= 31'd0;
		roi61_out_update <= 1'd0;
		roi61_out_count <= 31'd0;
		roi61_y_good <= 1'd0;
		roi61_x_good <= 1'd0;
		roi61_stb <= 1'd0;
		roi61_eop <= 1'd0;
		roi61_gray <= 16'd0;
		roi61_count <= 31'd0;
		roi62_out_update <= 1'd0;
		roi62_out_count <= 31'd0;
		roi62_y_good <= 1'd0;
		roi62_x_good <= 1'd0;
		roi62_stb <= 1'd0;
		roi62_eop <= 1'd0;
		roi62_gray <= 16'd0;
		roi62_count <= 31'd0;
		roi63_out_update <= 1'd0;
		roi63_out_count <= 31'd0;
		roi63_y_good <= 1'd0;
		roi63_x_good <= 1'd0;
		roi63_stb <= 1'd0;
		roi63_eop <= 1'd0;
		roi63_gray <= 16'd0;
		roi63_count <= 31'd0;
	end
	xilinxmultiregimpl50 <= roi_boundary0;
	xilinxmultiregimpl51 <= xilinxmultiregimpl50;
	xilinxmultiregimpl60 <= roi_boundary1;
	xilinxmultiregimpl61 <= xilinxmultiregimpl60;
	xilinxmultiregimpl70 <= roi_boundary2;
	xilinxmultiregimpl71 <= xilinxmultiregimpl70;
	xilinxmultiregimpl80 <= roi_boundary3;
	xilinxmultiregimpl81 <= xilinxmultiregimpl80;
	xilinxmultiregimpl90 <= roi_boundary4;
	xilinxmultiregimpl91 <= xilinxmultiregimpl90;
	xilinxmultiregimpl100 <= roi_boundary5;
	xilinxmultiregimpl101 <= xilinxmultiregimpl100;
	xilinxmultiregimpl110 <= roi_boundary6;
	xilinxmultiregimpl111 <= xilinxmultiregimpl110;
	xilinxmultiregimpl120 <= roi_boundary7;
	xilinxmultiregimpl121 <= xilinxmultiregimpl120;
	xilinxmultiregimpl130 <= roi_boundary8;
	xilinxmultiregimpl131 <= xilinxmultiregimpl130;
	xilinxmultiregimpl140 <= roi_boundary9;
	xilinxmultiregimpl141 <= xilinxmultiregimpl140;
	xilinxmultiregimpl150 <= roi_boundary10;
	xilinxmultiregimpl151 <= xilinxmultiregimpl150;
	xilinxmultiregimpl160 <= roi_boundary11;
	xilinxmultiregimpl161 <= xilinxmultiregimpl160;
	xilinxmultiregimpl170 <= roi_boundary12;
	xilinxmultiregimpl171 <= xilinxmultiregimpl170;
	xilinxmultiregimpl180 <= roi_boundary13;
	xilinxmultiregimpl181 <= xilinxmultiregimpl180;
	xilinxmultiregimpl190 <= roi_boundary14;
	xilinxmultiregimpl191 <= xilinxmultiregimpl190;
	xilinxmultiregimpl200 <= roi_boundary15;
	xilinxmultiregimpl201 <= xilinxmultiregimpl200;
	xilinxmultiregimpl210 <= roi_boundary16;
	xilinxmultiregimpl211 <= xilinxmultiregimpl210;
	xilinxmultiregimpl220 <= roi_boundary17;
	xilinxmultiregimpl221 <= xilinxmultiregimpl220;
	xilinxmultiregimpl230 <= roi_boundary18;
	xilinxmultiregimpl231 <= xilinxmultiregimpl230;
	xilinxmultiregimpl240 <= roi_boundary19;
	xilinxmultiregimpl241 <= xilinxmultiregimpl240;
	xilinxmultiregimpl250 <= roi_boundary20;
	xilinxmultiregimpl251 <= xilinxmultiregimpl250;
	xilinxmultiregimpl260 <= roi_boundary21;
	xilinxmultiregimpl261 <= xilinxmultiregimpl260;
	xilinxmultiregimpl270 <= roi_boundary22;
	xilinxmultiregimpl271 <= xilinxmultiregimpl270;
	xilinxmultiregimpl280 <= roi_boundary23;
	xilinxmultiregimpl281 <= xilinxmultiregimpl280;
	xilinxmultiregimpl290 <= roi_boundary24;
	xilinxmultiregimpl291 <= xilinxmultiregimpl290;
	xilinxmultiregimpl300 <= roi_boundary25;
	xilinxmultiregimpl301 <= xilinxmultiregimpl300;
	xilinxmultiregimpl310 <= roi_boundary26;
	xilinxmultiregimpl311 <= xilinxmultiregimpl310;
	xilinxmultiregimpl320 <= roi_boundary27;
	xilinxmultiregimpl321 <= xilinxmultiregimpl320;
	xilinxmultiregimpl330 <= roi_boundary28;
	xilinxmultiregimpl331 <= xilinxmultiregimpl330;
	xilinxmultiregimpl340 <= roi_boundary29;
	xilinxmultiregimpl341 <= xilinxmultiregimpl340;
	xilinxmultiregimpl350 <= roi_boundary30;
	xilinxmultiregimpl351 <= xilinxmultiregimpl350;
	xilinxmultiregimpl360 <= roi_boundary31;
	xilinxmultiregimpl361 <= xilinxmultiregimpl360;
	xilinxmultiregimpl370 <= roi_boundary32;
	xilinxmultiregimpl371 <= xilinxmultiregimpl370;
	xilinxmultiregimpl380 <= roi_boundary33;
	xilinxmultiregimpl381 <= xilinxmultiregimpl380;
	xilinxmultiregimpl390 <= roi_boundary34;
	xilinxmultiregimpl391 <= xilinxmultiregimpl390;
	xilinxmultiregimpl400 <= roi_boundary35;
	xilinxmultiregimpl401 <= xilinxmultiregimpl400;
	xilinxmultiregimpl410 <= roi_boundary36;
	xilinxmultiregimpl411 <= xilinxmultiregimpl410;
	xilinxmultiregimpl420 <= roi_boundary37;
	xilinxmultiregimpl421 <= xilinxmultiregimpl420;
	xilinxmultiregimpl430 <= roi_boundary38;
	xilinxmultiregimpl431 <= xilinxmultiregimpl430;
	xilinxmultiregimpl440 <= roi_boundary39;
	xilinxmultiregimpl441 <= xilinxmultiregimpl440;
	xilinxmultiregimpl450 <= roi_boundary40;
	xilinxmultiregimpl451 <= xilinxmultiregimpl450;
	xilinxmultiregimpl460 <= roi_boundary41;
	xilinxmultiregimpl461 <= xilinxmultiregimpl460;
	xilinxmultiregimpl470 <= roi_boundary42;
	xilinxmultiregimpl471 <= xilinxmultiregimpl470;
	xilinxmultiregimpl480 <= roi_boundary43;
	xilinxmultiregimpl481 <= xilinxmultiregimpl480;
	xilinxmultiregimpl490 <= roi_boundary44;
	xilinxmultiregimpl491 <= xilinxmultiregimpl490;
	xilinxmultiregimpl500 <= roi_boundary45;
	xilinxmultiregimpl501 <= xilinxmultiregimpl500;
	xilinxmultiregimpl510 <= roi_boundary46;
	xilinxmultiregimpl511 <= xilinxmultiregimpl510;
	xilinxmultiregimpl520 <= roi_boundary47;
	xilinxmultiregimpl521 <= xilinxmultiregimpl520;
	xilinxmultiregimpl530 <= roi_boundary48;
	xilinxmultiregimpl531 <= xilinxmultiregimpl530;
	xilinxmultiregimpl540 <= roi_boundary49;
	xilinxmultiregimpl541 <= xilinxmultiregimpl540;
	xilinxmultiregimpl550 <= roi_boundary50;
	xilinxmultiregimpl551 <= xilinxmultiregimpl550;
	xilinxmultiregimpl560 <= roi_boundary51;
	xilinxmultiregimpl561 <= xilinxmultiregimpl560;
	xilinxmultiregimpl570 <= roi_boundary52;
	xilinxmultiregimpl571 <= xilinxmultiregimpl570;
	xilinxmultiregimpl580 <= roi_boundary53;
	xilinxmultiregimpl581 <= xilinxmultiregimpl580;
	xilinxmultiregimpl590 <= roi_boundary54;
	xilinxmultiregimpl591 <= xilinxmultiregimpl590;
	xilinxmultiregimpl600 <= roi_boundary55;
	xilinxmultiregimpl601 <= xilinxmultiregimpl600;
	xilinxmultiregimpl610 <= roi_boundary56;
	xilinxmultiregimpl611 <= xilinxmultiregimpl610;
	xilinxmultiregimpl620 <= roi_boundary57;
	xilinxmultiregimpl621 <= xilinxmultiregimpl620;
	xilinxmultiregimpl630 <= roi_boundary58;
	xilinxmultiregimpl631 <= xilinxmultiregimpl630;
	xilinxmultiregimpl640 <= roi_boundary59;
	xilinxmultiregimpl641 <= xilinxmultiregimpl640;
	xilinxmultiregimpl650 <= roi_boundary60;
	xilinxmultiregimpl651 <= xilinxmultiregimpl650;
	xilinxmultiregimpl660 <= roi_boundary61;
	xilinxmultiregimpl661 <= xilinxmultiregimpl660;
	xilinxmultiregimpl670 <= roi_boundary62;
	xilinxmultiregimpl671 <= xilinxmultiregimpl670;
	xilinxmultiregimpl680 <= roi_boundary63;
	xilinxmultiregimpl681 <= xilinxmultiregimpl680;
	xilinxmultiregimpl690 <= roi_boundary64;
	xilinxmultiregimpl691 <= xilinxmultiregimpl690;
	xilinxmultiregimpl700 <= roi_boundary65;
	xilinxmultiregimpl701 <= xilinxmultiregimpl700;
	xilinxmultiregimpl710 <= roi_boundary66;
	xilinxmultiregimpl711 <= xilinxmultiregimpl710;
	xilinxmultiregimpl720 <= roi_boundary67;
	xilinxmultiregimpl721 <= xilinxmultiregimpl720;
	xilinxmultiregimpl730 <= roi_boundary68;
	xilinxmultiregimpl731 <= xilinxmultiregimpl730;
	xilinxmultiregimpl740 <= roi_boundary69;
	xilinxmultiregimpl741 <= xilinxmultiregimpl740;
	xilinxmultiregimpl750 <= roi_boundary70;
	xilinxmultiregimpl751 <= xilinxmultiregimpl750;
	xilinxmultiregimpl760 <= roi_boundary71;
	xilinxmultiregimpl761 <= xilinxmultiregimpl760;
	xilinxmultiregimpl770 <= roi_boundary72;
	xilinxmultiregimpl771 <= xilinxmultiregimpl770;
	xilinxmultiregimpl780 <= roi_boundary73;
	xilinxmultiregimpl781 <= xilinxmultiregimpl780;
	xilinxmultiregimpl790 <= roi_boundary74;
	xilinxmultiregimpl791 <= xilinxmultiregimpl790;
	xilinxmultiregimpl800 <= roi_boundary75;
	xilinxmultiregimpl801 <= xilinxmultiregimpl800;
	xilinxmultiregimpl810 <= roi_boundary76;
	xilinxmultiregimpl811 <= xilinxmultiregimpl810;
	xilinxmultiregimpl820 <= roi_boundary77;
	xilinxmultiregimpl821 <= xilinxmultiregimpl820;
	xilinxmultiregimpl830 <= roi_boundary78;
	xilinxmultiregimpl831 <= xilinxmultiregimpl830;
	xilinxmultiregimpl840 <= roi_boundary79;
	xilinxmultiregimpl841 <= xilinxmultiregimpl840;
	xilinxmultiregimpl850 <= roi_boundary80;
	xilinxmultiregimpl851 <= xilinxmultiregimpl850;
	xilinxmultiregimpl860 <= roi_boundary81;
	xilinxmultiregimpl861 <= xilinxmultiregimpl860;
	xilinxmultiregimpl870 <= roi_boundary82;
	xilinxmultiregimpl871 <= xilinxmultiregimpl870;
	xilinxmultiregimpl880 <= roi_boundary83;
	xilinxmultiregimpl881 <= xilinxmultiregimpl880;
	xilinxmultiregimpl890 <= roi_boundary84;
	xilinxmultiregimpl891 <= xilinxmultiregimpl890;
	xilinxmultiregimpl900 <= roi_boundary85;
	xilinxmultiregimpl901 <= xilinxmultiregimpl900;
	xilinxmultiregimpl910 <= roi_boundary86;
	xilinxmultiregimpl911 <= xilinxmultiregimpl910;
	xilinxmultiregimpl920 <= roi_boundary87;
	xilinxmultiregimpl921 <= xilinxmultiregimpl920;
	xilinxmultiregimpl930 <= roi_boundary88;
	xilinxmultiregimpl931 <= xilinxmultiregimpl930;
	xilinxmultiregimpl940 <= roi_boundary89;
	xilinxmultiregimpl941 <= xilinxmultiregimpl940;
	xilinxmultiregimpl950 <= roi_boundary90;
	xilinxmultiregimpl951 <= xilinxmultiregimpl950;
	xilinxmultiregimpl960 <= roi_boundary91;
	xilinxmultiregimpl961 <= xilinxmultiregimpl960;
	xilinxmultiregimpl970 <= roi_boundary92;
	xilinxmultiregimpl971 <= xilinxmultiregimpl970;
	xilinxmultiregimpl980 <= roi_boundary93;
	xilinxmultiregimpl981 <= xilinxmultiregimpl980;
	xilinxmultiregimpl990 <= roi_boundary94;
	xilinxmultiregimpl991 <= xilinxmultiregimpl990;
	xilinxmultiregimpl1000 <= roi_boundary95;
	xilinxmultiregimpl1001 <= xilinxmultiregimpl1000;
	xilinxmultiregimpl1010 <= roi_boundary96;
	xilinxmultiregimpl1011 <= xilinxmultiregimpl1010;
	xilinxmultiregimpl1020 <= roi_boundary97;
	xilinxmultiregimpl1021 <= xilinxmultiregimpl1020;
	xilinxmultiregimpl1030 <= roi_boundary98;
	xilinxmultiregimpl1031 <= xilinxmultiregimpl1030;
	xilinxmultiregimpl1040 <= roi_boundary99;
	xilinxmultiregimpl1041 <= xilinxmultiregimpl1040;
	xilinxmultiregimpl1050 <= roi_boundary100;
	xilinxmultiregimpl1051 <= xilinxmultiregimpl1050;
	xilinxmultiregimpl1060 <= roi_boundary101;
	xilinxmultiregimpl1061 <= xilinxmultiregimpl1060;
	xilinxmultiregimpl1070 <= roi_boundary102;
	xilinxmultiregimpl1071 <= xilinxmultiregimpl1070;
	xilinxmultiregimpl1080 <= roi_boundary103;
	xilinxmultiregimpl1081 <= xilinxmultiregimpl1080;
	xilinxmultiregimpl1090 <= roi_boundary104;
	xilinxmultiregimpl1091 <= xilinxmultiregimpl1090;
	xilinxmultiregimpl1100 <= roi_boundary105;
	xilinxmultiregimpl1101 <= xilinxmultiregimpl1100;
	xilinxmultiregimpl1110 <= roi_boundary106;
	xilinxmultiregimpl1111 <= xilinxmultiregimpl1110;
	xilinxmultiregimpl1120 <= roi_boundary107;
	xilinxmultiregimpl1121 <= xilinxmultiregimpl1120;
	xilinxmultiregimpl1130 <= roi_boundary108;
	xilinxmultiregimpl1131 <= xilinxmultiregimpl1130;
	xilinxmultiregimpl1140 <= roi_boundary109;
	xilinxmultiregimpl1141 <= xilinxmultiregimpl1140;
	xilinxmultiregimpl1150 <= roi_boundary110;
	xilinxmultiregimpl1151 <= xilinxmultiregimpl1150;
	xilinxmultiregimpl1160 <= roi_boundary111;
	xilinxmultiregimpl1161 <= xilinxmultiregimpl1160;
	xilinxmultiregimpl1170 <= roi_boundary112;
	xilinxmultiregimpl1171 <= xilinxmultiregimpl1170;
	xilinxmultiregimpl1180 <= roi_boundary113;
	xilinxmultiregimpl1181 <= xilinxmultiregimpl1180;
	xilinxmultiregimpl1190 <= roi_boundary114;
	xilinxmultiregimpl1191 <= xilinxmultiregimpl1190;
	xilinxmultiregimpl1200 <= roi_boundary115;
	xilinxmultiregimpl1201 <= xilinxmultiregimpl1200;
	xilinxmultiregimpl1210 <= roi_boundary116;
	xilinxmultiregimpl1211 <= xilinxmultiregimpl1210;
	xilinxmultiregimpl1220 <= roi_boundary117;
	xilinxmultiregimpl1221 <= xilinxmultiregimpl1220;
	xilinxmultiregimpl1230 <= roi_boundary118;
	xilinxmultiregimpl1231 <= xilinxmultiregimpl1230;
	xilinxmultiregimpl1240 <= roi_boundary119;
	xilinxmultiregimpl1241 <= xilinxmultiregimpl1240;
	xilinxmultiregimpl1250 <= roi_boundary120;
	xilinxmultiregimpl1251 <= xilinxmultiregimpl1250;
	xilinxmultiregimpl1260 <= roi_boundary121;
	xilinxmultiregimpl1261 <= xilinxmultiregimpl1260;
	xilinxmultiregimpl1270 <= roi_boundary122;
	xilinxmultiregimpl1271 <= xilinxmultiregimpl1270;
	xilinxmultiregimpl1280 <= roi_boundary123;
	xilinxmultiregimpl1281 <= xilinxmultiregimpl1280;
	xilinxmultiregimpl1290 <= roi_boundary124;
	xilinxmultiregimpl1291 <= xilinxmultiregimpl1290;
	xilinxmultiregimpl1300 <= roi_boundary125;
	xilinxmultiregimpl1301 <= xilinxmultiregimpl1300;
	xilinxmultiregimpl1310 <= roi_boundary126;
	xilinxmultiregimpl1311 <= xilinxmultiregimpl1310;
	xilinxmultiregimpl1320 <= roi_boundary127;
	xilinxmultiregimpl1321 <= xilinxmultiregimpl1320;
	xilinxmultiregimpl1330 <= roi_boundary128;
	xilinxmultiregimpl1331 <= xilinxmultiregimpl1330;
	xilinxmultiregimpl1340 <= roi_boundary129;
	xilinxmultiregimpl1341 <= xilinxmultiregimpl1340;
	xilinxmultiregimpl1350 <= roi_boundary130;
	xilinxmultiregimpl1351 <= xilinxmultiregimpl1350;
	xilinxmultiregimpl1360 <= roi_boundary131;
	xilinxmultiregimpl1361 <= xilinxmultiregimpl1360;
	xilinxmultiregimpl1370 <= roi_boundary132;
	xilinxmultiregimpl1371 <= xilinxmultiregimpl1370;
	xilinxmultiregimpl1380 <= roi_boundary133;
	xilinxmultiregimpl1381 <= xilinxmultiregimpl1380;
	xilinxmultiregimpl1390 <= roi_boundary134;
	xilinxmultiregimpl1391 <= xilinxmultiregimpl1390;
	xilinxmultiregimpl1400 <= roi_boundary135;
	xilinxmultiregimpl1401 <= xilinxmultiregimpl1400;
	xilinxmultiregimpl1410 <= roi_boundary136;
	xilinxmultiregimpl1411 <= xilinxmultiregimpl1410;
	xilinxmultiregimpl1420 <= roi_boundary137;
	xilinxmultiregimpl1421 <= xilinxmultiregimpl1420;
	xilinxmultiregimpl1430 <= roi_boundary138;
	xilinxmultiregimpl1431 <= xilinxmultiregimpl1430;
	xilinxmultiregimpl1440 <= roi_boundary139;
	xilinxmultiregimpl1441 <= xilinxmultiregimpl1440;
	xilinxmultiregimpl1450 <= roi_boundary140;
	xilinxmultiregimpl1451 <= xilinxmultiregimpl1450;
	xilinxmultiregimpl1460 <= roi_boundary141;
	xilinxmultiregimpl1461 <= xilinxmultiregimpl1460;
	xilinxmultiregimpl1470 <= roi_boundary142;
	xilinxmultiregimpl1471 <= xilinxmultiregimpl1470;
	xilinxmultiregimpl1480 <= roi_boundary143;
	xilinxmultiregimpl1481 <= xilinxmultiregimpl1480;
	xilinxmultiregimpl1490 <= roi_boundary144;
	xilinxmultiregimpl1491 <= xilinxmultiregimpl1490;
	xilinxmultiregimpl1500 <= roi_boundary145;
	xilinxmultiregimpl1501 <= xilinxmultiregimpl1500;
	xilinxmultiregimpl1510 <= roi_boundary146;
	xilinxmultiregimpl1511 <= xilinxmultiregimpl1510;
	xilinxmultiregimpl1520 <= roi_boundary147;
	xilinxmultiregimpl1521 <= xilinxmultiregimpl1520;
	xilinxmultiregimpl1530 <= roi_boundary148;
	xilinxmultiregimpl1531 <= xilinxmultiregimpl1530;
	xilinxmultiregimpl1540 <= roi_boundary149;
	xilinxmultiregimpl1541 <= xilinxmultiregimpl1540;
	xilinxmultiregimpl1550 <= roi_boundary150;
	xilinxmultiregimpl1551 <= xilinxmultiregimpl1550;
	xilinxmultiregimpl1560 <= roi_boundary151;
	xilinxmultiregimpl1561 <= xilinxmultiregimpl1560;
	xilinxmultiregimpl1570 <= roi_boundary152;
	xilinxmultiregimpl1571 <= xilinxmultiregimpl1570;
	xilinxmultiregimpl1580 <= roi_boundary153;
	xilinxmultiregimpl1581 <= xilinxmultiregimpl1580;
	xilinxmultiregimpl1590 <= roi_boundary154;
	xilinxmultiregimpl1591 <= xilinxmultiregimpl1590;
	xilinxmultiregimpl1600 <= roi_boundary155;
	xilinxmultiregimpl1601 <= xilinxmultiregimpl1600;
	xilinxmultiregimpl1610 <= roi_boundary156;
	xilinxmultiregimpl1611 <= xilinxmultiregimpl1610;
	xilinxmultiregimpl1620 <= roi_boundary157;
	xilinxmultiregimpl1621 <= xilinxmultiregimpl1620;
	xilinxmultiregimpl1630 <= roi_boundary158;
	xilinxmultiregimpl1631 <= xilinxmultiregimpl1630;
	xilinxmultiregimpl1640 <= roi_boundary159;
	xilinxmultiregimpl1641 <= xilinxmultiregimpl1640;
	xilinxmultiregimpl1650 <= roi_boundary160;
	xilinxmultiregimpl1651 <= xilinxmultiregimpl1650;
	xilinxmultiregimpl1660 <= roi_boundary161;
	xilinxmultiregimpl1661 <= xilinxmultiregimpl1660;
	xilinxmultiregimpl1670 <= roi_boundary162;
	xilinxmultiregimpl1671 <= xilinxmultiregimpl1670;
	xilinxmultiregimpl1680 <= roi_boundary163;
	xilinxmultiregimpl1681 <= xilinxmultiregimpl1680;
	xilinxmultiregimpl1690 <= roi_boundary164;
	xilinxmultiregimpl1691 <= xilinxmultiregimpl1690;
	xilinxmultiregimpl1700 <= roi_boundary165;
	xilinxmultiregimpl1701 <= xilinxmultiregimpl1700;
	xilinxmultiregimpl1710 <= roi_boundary166;
	xilinxmultiregimpl1711 <= xilinxmultiregimpl1710;
	xilinxmultiregimpl1720 <= roi_boundary167;
	xilinxmultiregimpl1721 <= xilinxmultiregimpl1720;
	xilinxmultiregimpl1730 <= roi_boundary168;
	xilinxmultiregimpl1731 <= xilinxmultiregimpl1730;
	xilinxmultiregimpl1740 <= roi_boundary169;
	xilinxmultiregimpl1741 <= xilinxmultiregimpl1740;
	xilinxmultiregimpl1750 <= roi_boundary170;
	xilinxmultiregimpl1751 <= xilinxmultiregimpl1750;
	xilinxmultiregimpl1760 <= roi_boundary171;
	xilinxmultiregimpl1761 <= xilinxmultiregimpl1760;
	xilinxmultiregimpl1770 <= roi_boundary172;
	xilinxmultiregimpl1771 <= xilinxmultiregimpl1770;
	xilinxmultiregimpl1780 <= roi_boundary173;
	xilinxmultiregimpl1781 <= xilinxmultiregimpl1780;
	xilinxmultiregimpl1790 <= roi_boundary174;
	xilinxmultiregimpl1791 <= xilinxmultiregimpl1790;
	xilinxmultiregimpl1800 <= roi_boundary175;
	xilinxmultiregimpl1801 <= xilinxmultiregimpl1800;
	xilinxmultiregimpl1810 <= roi_boundary176;
	xilinxmultiregimpl1811 <= xilinxmultiregimpl1810;
	xilinxmultiregimpl1820 <= roi_boundary177;
	xilinxmultiregimpl1821 <= xilinxmultiregimpl1820;
	xilinxmultiregimpl1830 <= roi_boundary178;
	xilinxmultiregimpl1831 <= xilinxmultiregimpl1830;
	xilinxmultiregimpl1840 <= roi_boundary179;
	xilinxmultiregimpl1841 <= xilinxmultiregimpl1840;
	xilinxmultiregimpl1850 <= roi_boundary180;
	xilinxmultiregimpl1851 <= xilinxmultiregimpl1850;
	xilinxmultiregimpl1860 <= roi_boundary181;
	xilinxmultiregimpl1861 <= xilinxmultiregimpl1860;
	xilinxmultiregimpl1870 <= roi_boundary182;
	xilinxmultiregimpl1871 <= xilinxmultiregimpl1870;
	xilinxmultiregimpl1880 <= roi_boundary183;
	xilinxmultiregimpl1881 <= xilinxmultiregimpl1880;
	xilinxmultiregimpl1890 <= roi_boundary184;
	xilinxmultiregimpl1891 <= xilinxmultiregimpl1890;
	xilinxmultiregimpl1900 <= roi_boundary185;
	xilinxmultiregimpl1901 <= xilinxmultiregimpl1900;
	xilinxmultiregimpl1910 <= roi_boundary186;
	xilinxmultiregimpl1911 <= xilinxmultiregimpl1910;
	xilinxmultiregimpl1920 <= roi_boundary187;
	xilinxmultiregimpl1921 <= xilinxmultiregimpl1920;
	xilinxmultiregimpl1930 <= roi_boundary188;
	xilinxmultiregimpl1931 <= xilinxmultiregimpl1930;
	xilinxmultiregimpl1940 <= roi_boundary189;
	xilinxmultiregimpl1941 <= xilinxmultiregimpl1940;
	xilinxmultiregimpl1950 <= roi_boundary190;
	xilinxmultiregimpl1951 <= xilinxmultiregimpl1950;
	xilinxmultiregimpl1960 <= roi_boundary191;
	xilinxmultiregimpl1961 <= xilinxmultiregimpl1960;
	xilinxmultiregimpl1970 <= roi_boundary192;
	xilinxmultiregimpl1971 <= xilinxmultiregimpl1970;
	xilinxmultiregimpl1980 <= roi_boundary193;
	xilinxmultiregimpl1981 <= xilinxmultiregimpl1980;
	xilinxmultiregimpl1990 <= roi_boundary194;
	xilinxmultiregimpl1991 <= xilinxmultiregimpl1990;
	xilinxmultiregimpl2000 <= roi_boundary195;
	xilinxmultiregimpl2001 <= xilinxmultiregimpl2000;
	xilinxmultiregimpl2010 <= roi_boundary196;
	xilinxmultiregimpl2011 <= xilinxmultiregimpl2010;
	xilinxmultiregimpl2020 <= roi_boundary197;
	xilinxmultiregimpl2021 <= xilinxmultiregimpl2020;
	xilinxmultiregimpl2030 <= roi_boundary198;
	xilinxmultiregimpl2031 <= xilinxmultiregimpl2030;
	xilinxmultiregimpl2040 <= roi_boundary199;
	xilinxmultiregimpl2041 <= xilinxmultiregimpl2040;
	xilinxmultiregimpl2050 <= roi_boundary200;
	xilinxmultiregimpl2051 <= xilinxmultiregimpl2050;
	xilinxmultiregimpl2060 <= roi_boundary201;
	xilinxmultiregimpl2061 <= xilinxmultiregimpl2060;
	xilinxmultiregimpl2070 <= roi_boundary202;
	xilinxmultiregimpl2071 <= xilinxmultiregimpl2070;
	xilinxmultiregimpl2080 <= roi_boundary203;
	xilinxmultiregimpl2081 <= xilinxmultiregimpl2080;
	xilinxmultiregimpl2090 <= roi_boundary204;
	xilinxmultiregimpl2091 <= xilinxmultiregimpl2090;
	xilinxmultiregimpl2100 <= roi_boundary205;
	xilinxmultiregimpl2101 <= xilinxmultiregimpl2100;
	xilinxmultiregimpl2110 <= roi_boundary206;
	xilinxmultiregimpl2111 <= xilinxmultiregimpl2110;
	xilinxmultiregimpl2120 <= roi_boundary207;
	xilinxmultiregimpl2121 <= xilinxmultiregimpl2120;
	xilinxmultiregimpl2130 <= roi_boundary208;
	xilinxmultiregimpl2131 <= xilinxmultiregimpl2130;
	xilinxmultiregimpl2140 <= roi_boundary209;
	xilinxmultiregimpl2141 <= xilinxmultiregimpl2140;
	xilinxmultiregimpl2150 <= roi_boundary210;
	xilinxmultiregimpl2151 <= xilinxmultiregimpl2150;
	xilinxmultiregimpl2160 <= roi_boundary211;
	xilinxmultiregimpl2161 <= xilinxmultiregimpl2160;
	xilinxmultiregimpl2170 <= roi_boundary212;
	xilinxmultiregimpl2171 <= xilinxmultiregimpl2170;
	xilinxmultiregimpl2180 <= roi_boundary213;
	xilinxmultiregimpl2181 <= xilinxmultiregimpl2180;
	xilinxmultiregimpl2190 <= roi_boundary214;
	xilinxmultiregimpl2191 <= xilinxmultiregimpl2190;
	xilinxmultiregimpl2200 <= roi_boundary215;
	xilinxmultiregimpl2201 <= xilinxmultiregimpl2200;
	xilinxmultiregimpl2210 <= roi_boundary216;
	xilinxmultiregimpl2211 <= xilinxmultiregimpl2210;
	xilinxmultiregimpl2220 <= roi_boundary217;
	xilinxmultiregimpl2221 <= xilinxmultiregimpl2220;
	xilinxmultiregimpl2230 <= roi_boundary218;
	xilinxmultiregimpl2231 <= xilinxmultiregimpl2230;
	xilinxmultiregimpl2240 <= roi_boundary219;
	xilinxmultiregimpl2241 <= xilinxmultiregimpl2240;
	xilinxmultiregimpl2250 <= roi_boundary220;
	xilinxmultiregimpl2251 <= xilinxmultiregimpl2250;
	xilinxmultiregimpl2260 <= roi_boundary221;
	xilinxmultiregimpl2261 <= xilinxmultiregimpl2260;
	xilinxmultiregimpl2270 <= roi_boundary222;
	xilinxmultiregimpl2271 <= xilinxmultiregimpl2270;
	xilinxmultiregimpl2280 <= roi_boundary223;
	xilinxmultiregimpl2281 <= xilinxmultiregimpl2280;
	xilinxmultiregimpl2290 <= roi_boundary224;
	xilinxmultiregimpl2291 <= xilinxmultiregimpl2290;
	xilinxmultiregimpl2300 <= roi_boundary225;
	xilinxmultiregimpl2301 <= xilinxmultiregimpl2300;
	xilinxmultiregimpl2310 <= roi_boundary226;
	xilinxmultiregimpl2311 <= xilinxmultiregimpl2310;
	xilinxmultiregimpl2320 <= roi_boundary227;
	xilinxmultiregimpl2321 <= xilinxmultiregimpl2320;
	xilinxmultiregimpl2330 <= roi_boundary228;
	xilinxmultiregimpl2331 <= xilinxmultiregimpl2330;
	xilinxmultiregimpl2340 <= roi_boundary229;
	xilinxmultiregimpl2341 <= xilinxmultiregimpl2340;
	xilinxmultiregimpl2350 <= roi_boundary230;
	xilinxmultiregimpl2351 <= xilinxmultiregimpl2350;
	xilinxmultiregimpl2360 <= roi_boundary231;
	xilinxmultiregimpl2361 <= xilinxmultiregimpl2360;
	xilinxmultiregimpl2370 <= roi_boundary232;
	xilinxmultiregimpl2371 <= xilinxmultiregimpl2370;
	xilinxmultiregimpl2380 <= roi_boundary233;
	xilinxmultiregimpl2381 <= xilinxmultiregimpl2380;
	xilinxmultiregimpl2390 <= roi_boundary234;
	xilinxmultiregimpl2391 <= xilinxmultiregimpl2390;
	xilinxmultiregimpl2400 <= roi_boundary235;
	xilinxmultiregimpl2401 <= xilinxmultiregimpl2400;
	xilinxmultiregimpl2410 <= roi_boundary236;
	xilinxmultiregimpl2411 <= xilinxmultiregimpl2410;
	xilinxmultiregimpl2420 <= roi_boundary237;
	xilinxmultiregimpl2421 <= xilinxmultiregimpl2420;
	xilinxmultiregimpl2430 <= roi_boundary238;
	xilinxmultiregimpl2431 <= xilinxmultiregimpl2430;
	xilinxmultiregimpl2440 <= roi_boundary239;
	xilinxmultiregimpl2441 <= xilinxmultiregimpl2440;
	xilinxmultiregimpl2450 <= roi_boundary240;
	xilinxmultiregimpl2451 <= xilinxmultiregimpl2450;
	xilinxmultiregimpl2460 <= roi_boundary241;
	xilinxmultiregimpl2461 <= xilinxmultiregimpl2460;
	xilinxmultiregimpl2470 <= roi_boundary242;
	xilinxmultiregimpl2471 <= xilinxmultiregimpl2470;
	xilinxmultiregimpl2480 <= roi_boundary243;
	xilinxmultiregimpl2481 <= xilinxmultiregimpl2480;
	xilinxmultiregimpl2490 <= roi_boundary244;
	xilinxmultiregimpl2491 <= xilinxmultiregimpl2490;
	xilinxmultiregimpl2500 <= roi_boundary245;
	xilinxmultiregimpl2501 <= xilinxmultiregimpl2500;
	xilinxmultiregimpl2510 <= roi_boundary246;
	xilinxmultiregimpl2511 <= xilinxmultiregimpl2510;
	xilinxmultiregimpl2520 <= roi_boundary247;
	xilinxmultiregimpl2521 <= xilinxmultiregimpl2520;
	xilinxmultiregimpl2530 <= roi_boundary248;
	xilinxmultiregimpl2531 <= xilinxmultiregimpl2530;
	xilinxmultiregimpl2540 <= roi_boundary249;
	xilinxmultiregimpl2541 <= xilinxmultiregimpl2540;
	xilinxmultiregimpl2550 <= roi_boundary250;
	xilinxmultiregimpl2551 <= xilinxmultiregimpl2550;
	xilinxmultiregimpl2560 <= roi_boundary251;
	xilinxmultiregimpl2561 <= xilinxmultiregimpl2560;
	xilinxmultiregimpl2570 <= roi_boundary252;
	xilinxmultiregimpl2571 <= xilinxmultiregimpl2570;
	xilinxmultiregimpl2580 <= roi_boundary253;
	xilinxmultiregimpl2581 <= xilinxmultiregimpl2580;
	xilinxmultiregimpl2590 <= roi_boundary254;
	xilinxmultiregimpl2591 <= xilinxmultiregimpl2590;
	xilinxmultiregimpl2600 <= roi_boundary255;
	xilinxmultiregimpl2601 <= xilinxmultiregimpl2600;
end

always @(posedge rio_clk) begin
	if (ointerface1_stb) begin
		gate0 <= ointerface1_data;
	end
	state <= next_state;
	if (gate1_next_value_ce) begin
		gate1 <= gate1_next_value;
	end
	if (rio_rst) begin
		gate0 <= 64'd0;
		gate1 <= 64'd0;
		state <= 7'd0;
	end
end

always @(posedge sys_clk) begin
	if ((ointerface0_stb & (ointerface0_address == 1'd0))) begin
		roi_boundary0 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 1'd1))) begin
		roi_boundary1 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 2'd2))) begin
		roi_boundary2 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 2'd3))) begin
		roi_boundary3 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 3'd4))) begin
		roi_boundary4 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 3'd5))) begin
		roi_boundary5 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 3'd6))) begin
		roi_boundary6 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 3'd7))) begin
		roi_boundary7 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd8))) begin
		roi_boundary8 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd9))) begin
		roi_boundary9 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd10))) begin
		roi_boundary10 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd11))) begin
		roi_boundary11 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd12))) begin
		roi_boundary12 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd13))) begin
		roi_boundary13 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd14))) begin
		roi_boundary14 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 4'd15))) begin
		roi_boundary15 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd16))) begin
		roi_boundary16 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd17))) begin
		roi_boundary17 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd18))) begin
		roi_boundary18 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd19))) begin
		roi_boundary19 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd20))) begin
		roi_boundary20 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd21))) begin
		roi_boundary21 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd22))) begin
		roi_boundary22 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd23))) begin
		roi_boundary23 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd24))) begin
		roi_boundary24 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd25))) begin
		roi_boundary25 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd26))) begin
		roi_boundary26 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd27))) begin
		roi_boundary27 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd28))) begin
		roi_boundary28 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd29))) begin
		roi_boundary29 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd30))) begin
		roi_boundary30 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 5'd31))) begin
		roi_boundary31 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd32))) begin
		roi_boundary32 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd33))) begin
		roi_boundary33 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd34))) begin
		roi_boundary34 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd35))) begin
		roi_boundary35 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd36))) begin
		roi_boundary36 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd37))) begin
		roi_boundary37 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd38))) begin
		roi_boundary38 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd39))) begin
		roi_boundary39 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd40))) begin
		roi_boundary40 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd41))) begin
		roi_boundary41 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd42))) begin
		roi_boundary42 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd43))) begin
		roi_boundary43 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd44))) begin
		roi_boundary44 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd45))) begin
		roi_boundary45 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd46))) begin
		roi_boundary46 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd47))) begin
		roi_boundary47 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd48))) begin
		roi_boundary48 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd49))) begin
		roi_boundary49 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd50))) begin
		roi_boundary50 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd51))) begin
		roi_boundary51 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd52))) begin
		roi_boundary52 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd53))) begin
		roi_boundary53 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd54))) begin
		roi_boundary54 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd55))) begin
		roi_boundary55 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd56))) begin
		roi_boundary56 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd57))) begin
		roi_boundary57 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd58))) begin
		roi_boundary58 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd59))) begin
		roi_boundary59 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd60))) begin
		roi_boundary60 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd61))) begin
		roi_boundary61 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd62))) begin
		roi_boundary62 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 6'd63))) begin
		roi_boundary63 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd64))) begin
		roi_boundary64 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd65))) begin
		roi_boundary65 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd66))) begin
		roi_boundary66 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd67))) begin
		roi_boundary67 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd68))) begin
		roi_boundary68 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd69))) begin
		roi_boundary69 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd70))) begin
		roi_boundary70 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd71))) begin
		roi_boundary71 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd72))) begin
		roi_boundary72 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd73))) begin
		roi_boundary73 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd74))) begin
		roi_boundary74 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd75))) begin
		roi_boundary75 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd76))) begin
		roi_boundary76 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd77))) begin
		roi_boundary77 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd78))) begin
		roi_boundary78 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd79))) begin
		roi_boundary79 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd80))) begin
		roi_boundary80 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd81))) begin
		roi_boundary81 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd82))) begin
		roi_boundary82 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd83))) begin
		roi_boundary83 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd84))) begin
		roi_boundary84 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd85))) begin
		roi_boundary85 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd86))) begin
		roi_boundary86 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd87))) begin
		roi_boundary87 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd88))) begin
		roi_boundary88 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd89))) begin
		roi_boundary89 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd90))) begin
		roi_boundary90 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd91))) begin
		roi_boundary91 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd92))) begin
		roi_boundary92 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd93))) begin
		roi_boundary93 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd94))) begin
		roi_boundary94 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd95))) begin
		roi_boundary95 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd96))) begin
		roi_boundary96 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd97))) begin
		roi_boundary97 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd98))) begin
		roi_boundary98 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd99))) begin
		roi_boundary99 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd100))) begin
		roi_boundary100 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd101))) begin
		roi_boundary101 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd102))) begin
		roi_boundary102 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd103))) begin
		roi_boundary103 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd104))) begin
		roi_boundary104 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd105))) begin
		roi_boundary105 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd106))) begin
		roi_boundary106 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd107))) begin
		roi_boundary107 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd108))) begin
		roi_boundary108 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd109))) begin
		roi_boundary109 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd110))) begin
		roi_boundary110 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd111))) begin
		roi_boundary111 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd112))) begin
		roi_boundary112 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd113))) begin
		roi_boundary113 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd114))) begin
		roi_boundary114 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd115))) begin
		roi_boundary115 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd116))) begin
		roi_boundary116 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd117))) begin
		roi_boundary117 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd118))) begin
		roi_boundary118 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd119))) begin
		roi_boundary119 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd120))) begin
		roi_boundary120 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd121))) begin
		roi_boundary121 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd122))) begin
		roi_boundary122 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd123))) begin
		roi_boundary123 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd124))) begin
		roi_boundary124 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd125))) begin
		roi_boundary125 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd126))) begin
		roi_boundary126 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 7'd127))) begin
		roi_boundary127 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd128))) begin
		roi_boundary128 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd129))) begin
		roi_boundary129 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd130))) begin
		roi_boundary130 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd131))) begin
		roi_boundary131 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd132))) begin
		roi_boundary132 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd133))) begin
		roi_boundary133 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd134))) begin
		roi_boundary134 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd135))) begin
		roi_boundary135 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd136))) begin
		roi_boundary136 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd137))) begin
		roi_boundary137 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd138))) begin
		roi_boundary138 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd139))) begin
		roi_boundary139 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd140))) begin
		roi_boundary140 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd141))) begin
		roi_boundary141 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd142))) begin
		roi_boundary142 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd143))) begin
		roi_boundary143 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd144))) begin
		roi_boundary144 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd145))) begin
		roi_boundary145 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd146))) begin
		roi_boundary146 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd147))) begin
		roi_boundary147 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd148))) begin
		roi_boundary148 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd149))) begin
		roi_boundary149 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd150))) begin
		roi_boundary150 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd151))) begin
		roi_boundary151 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd152))) begin
		roi_boundary152 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd153))) begin
		roi_boundary153 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd154))) begin
		roi_boundary154 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd155))) begin
		roi_boundary155 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd156))) begin
		roi_boundary156 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd157))) begin
		roi_boundary157 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd158))) begin
		roi_boundary158 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd159))) begin
		roi_boundary159 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd160))) begin
		roi_boundary160 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd161))) begin
		roi_boundary161 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd162))) begin
		roi_boundary162 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd163))) begin
		roi_boundary163 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd164))) begin
		roi_boundary164 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd165))) begin
		roi_boundary165 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd166))) begin
		roi_boundary166 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd167))) begin
		roi_boundary167 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd168))) begin
		roi_boundary168 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd169))) begin
		roi_boundary169 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd170))) begin
		roi_boundary170 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd171))) begin
		roi_boundary171 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd172))) begin
		roi_boundary172 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd173))) begin
		roi_boundary173 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd174))) begin
		roi_boundary174 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd175))) begin
		roi_boundary175 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd176))) begin
		roi_boundary176 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd177))) begin
		roi_boundary177 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd178))) begin
		roi_boundary178 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd179))) begin
		roi_boundary179 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd180))) begin
		roi_boundary180 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd181))) begin
		roi_boundary181 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd182))) begin
		roi_boundary182 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd183))) begin
		roi_boundary183 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd184))) begin
		roi_boundary184 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd185))) begin
		roi_boundary185 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd186))) begin
		roi_boundary186 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd187))) begin
		roi_boundary187 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd188))) begin
		roi_boundary188 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd189))) begin
		roi_boundary189 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd190))) begin
		roi_boundary190 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd191))) begin
		roi_boundary191 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd192))) begin
		roi_boundary192 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd193))) begin
		roi_boundary193 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd194))) begin
		roi_boundary194 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd195))) begin
		roi_boundary195 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd196))) begin
		roi_boundary196 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd197))) begin
		roi_boundary197 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd198))) begin
		roi_boundary198 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd199))) begin
		roi_boundary199 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd200))) begin
		roi_boundary200 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd201))) begin
		roi_boundary201 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd202))) begin
		roi_boundary202 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd203))) begin
		roi_boundary203 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd204))) begin
		roi_boundary204 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd205))) begin
		roi_boundary205 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd206))) begin
		roi_boundary206 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd207))) begin
		roi_boundary207 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd208))) begin
		roi_boundary208 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd209))) begin
		roi_boundary209 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd210))) begin
		roi_boundary210 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd211))) begin
		roi_boundary211 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd212))) begin
		roi_boundary212 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd213))) begin
		roi_boundary213 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd214))) begin
		roi_boundary214 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd215))) begin
		roi_boundary215 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd216))) begin
		roi_boundary216 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd217))) begin
		roi_boundary217 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd218))) begin
		roi_boundary218 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd219))) begin
		roi_boundary219 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd220))) begin
		roi_boundary220 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd221))) begin
		roi_boundary221 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd222))) begin
		roi_boundary222 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd223))) begin
		roi_boundary223 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd224))) begin
		roi_boundary224 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd225))) begin
		roi_boundary225 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd226))) begin
		roi_boundary226 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd227))) begin
		roi_boundary227 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd228))) begin
		roi_boundary228 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd229))) begin
		roi_boundary229 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd230))) begin
		roi_boundary230 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd231))) begin
		roi_boundary231 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd232))) begin
		roi_boundary232 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd233))) begin
		roi_boundary233 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd234))) begin
		roi_boundary234 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd235))) begin
		roi_boundary235 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd236))) begin
		roi_boundary236 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd237))) begin
		roi_boundary237 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd238))) begin
		roi_boundary238 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd239))) begin
		roi_boundary239 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd240))) begin
		roi_boundary240 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd241))) begin
		roi_boundary241 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd242))) begin
		roi_boundary242 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd243))) begin
		roi_boundary243 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd244))) begin
		roi_boundary244 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd245))) begin
		roi_boundary245 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd246))) begin
		roi_boundary246 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd247))) begin
		roi_boundary247 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd248))) begin
		roi_boundary248 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd249))) begin
		roi_boundary249 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd250))) begin
		roi_boundary250 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd251))) begin
		roi_boundary251 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd252))) begin
		roi_boundary252 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd253))) begin
		roi_boundary253 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd254))) begin
		roi_boundary254 <= ointerface0_data;
	end
	if ((ointerface0_stb & (ointerface0_address == 8'd255))) begin
		roi_boundary255 <= ointerface0_data;
	end
	if (phase_shift_re) begin
		phase_shift_done_status <= 1'd0;
	end
	if (mmcm_ps_psdone) begin
		phase_shift_done_status <= 1'd1;
	end
	pll_reset <= pll_reset_storage;
	{frequency_counter_tick, frequency_counter_timer} <= (frequency_counter_timer + 1'd1);
	frequency_counter_toggle_sys_r <= frequency_counter_toggle_sys;
	if (frequency_counter_tick) begin
		frequency_counter_status <= frequency_counter_count;
		frequency_counter_count <= 1'd0;
	end else begin
		if ((frequency_counter_toggle_sys & (~frequency_counter_toggle_sys_r))) begin
			frequency_counter_count <= (frequency_counter_count + 1'd1);
		end
	end
	synchronizer_update <= roi0_out_update;
	if (sys_rst) begin
		phase_shift_done_status <= 1'd1;
		pll_reset <= 1'd1;
		frequency_counter_status <= 8'd0;
		frequency_counter_timer <= 9'd0;
		frequency_counter_tick <= 1'd1;
		frequency_counter_count <= 8'd0;
		frequency_counter_toggle_sys_r <= 1'd0;
		synchronizer_update <= 1'd0;
		roi_boundary0 <= 12'd0;
		roi_boundary1 <= 12'd0;
		roi_boundary2 <= 12'd0;
		roi_boundary3 <= 12'd0;
		roi_boundary4 <= 12'd0;
		roi_boundary5 <= 12'd0;
		roi_boundary6 <= 12'd0;
		roi_boundary7 <= 12'd0;
		roi_boundary8 <= 12'd0;
		roi_boundary9 <= 12'd0;
		roi_boundary10 <= 12'd0;
		roi_boundary11 <= 12'd0;
		roi_boundary12 <= 12'd0;
		roi_boundary13 <= 12'd0;
		roi_boundary14 <= 12'd0;
		roi_boundary15 <= 12'd0;
		roi_boundary16 <= 12'd0;
		roi_boundary17 <= 12'd0;
		roi_boundary18 <= 12'd0;
		roi_boundary19 <= 12'd0;
		roi_boundary20 <= 12'd0;
		roi_boundary21 <= 12'd0;
		roi_boundary22 <= 12'd0;
		roi_boundary23 <= 12'd0;
		roi_boundary24 <= 12'd0;
		roi_boundary25 <= 12'd0;
		roi_boundary26 <= 12'd0;
		roi_boundary27 <= 12'd0;
		roi_boundary28 <= 12'd0;
		roi_boundary29 <= 12'd0;
		roi_boundary30 <= 12'd0;
		roi_boundary31 <= 12'd0;
		roi_boundary32 <= 12'd0;
		roi_boundary33 <= 12'd0;
		roi_boundary34 <= 12'd0;
		roi_boundary35 <= 12'd0;
		roi_boundary36 <= 12'd0;
		roi_boundary37 <= 12'd0;
		roi_boundary38 <= 12'd0;
		roi_boundary39 <= 12'd0;
		roi_boundary40 <= 12'd0;
		roi_boundary41 <= 12'd0;
		roi_boundary42 <= 12'd0;
		roi_boundary43 <= 12'd0;
		roi_boundary44 <= 12'd0;
		roi_boundary45 <= 12'd0;
		roi_boundary46 <= 12'd0;
		roi_boundary47 <= 12'd0;
		roi_boundary48 <= 12'd0;
		roi_boundary49 <= 12'd0;
		roi_boundary50 <= 12'd0;
		roi_boundary51 <= 12'd0;
		roi_boundary52 <= 12'd0;
		roi_boundary53 <= 12'd0;
		roi_boundary54 <= 12'd0;
		roi_boundary55 <= 12'd0;
		roi_boundary56 <= 12'd0;
		roi_boundary57 <= 12'd0;
		roi_boundary58 <= 12'd0;
		roi_boundary59 <= 12'd0;
		roi_boundary60 <= 12'd0;
		roi_boundary61 <= 12'd0;
		roi_boundary62 <= 12'd0;
		roi_boundary63 <= 12'd0;
		roi_boundary64 <= 12'd0;
		roi_boundary65 <= 12'd0;
		roi_boundary66 <= 12'd0;
		roi_boundary67 <= 12'd0;
		roi_boundary68 <= 12'd0;
		roi_boundary69 <= 12'd0;
		roi_boundary70 <= 12'd0;
		roi_boundary71 <= 12'd0;
		roi_boundary72 <= 12'd0;
		roi_boundary73 <= 12'd0;
		roi_boundary74 <= 12'd0;
		roi_boundary75 <= 12'd0;
		roi_boundary76 <= 12'd0;
		roi_boundary77 <= 12'd0;
		roi_boundary78 <= 12'd0;
		roi_boundary79 <= 12'd0;
		roi_boundary80 <= 12'd0;
		roi_boundary81 <= 12'd0;
		roi_boundary82 <= 12'd0;
		roi_boundary83 <= 12'd0;
		roi_boundary84 <= 12'd0;
		roi_boundary85 <= 12'd0;
		roi_boundary86 <= 12'd0;
		roi_boundary87 <= 12'd0;
		roi_boundary88 <= 12'd0;
		roi_boundary89 <= 12'd0;
		roi_boundary90 <= 12'd0;
		roi_boundary91 <= 12'd0;
		roi_boundary92 <= 12'd0;
		roi_boundary93 <= 12'd0;
		roi_boundary94 <= 12'd0;
		roi_boundary95 <= 12'd0;
		roi_boundary96 <= 12'd0;
		roi_boundary97 <= 12'd0;
		roi_boundary98 <= 12'd0;
		roi_boundary99 <= 12'd0;
		roi_boundary100 <= 12'd0;
		roi_boundary101 <= 12'd0;
		roi_boundary102 <= 12'd0;
		roi_boundary103 <= 12'd0;
		roi_boundary104 <= 12'd0;
		roi_boundary105 <= 12'd0;
		roi_boundary106 <= 12'd0;
		roi_boundary107 <= 12'd0;
		roi_boundary108 <= 12'd0;
		roi_boundary109 <= 12'd0;
		roi_boundary110 <= 12'd0;
		roi_boundary111 <= 12'd0;
		roi_boundary112 <= 12'd0;
		roi_boundary113 <= 12'd0;
		roi_boundary114 <= 12'd0;
		roi_boundary115 <= 12'd0;
		roi_boundary116 <= 12'd0;
		roi_boundary117 <= 12'd0;
		roi_boundary118 <= 12'd0;
		roi_boundary119 <= 12'd0;
		roi_boundary120 <= 12'd0;
		roi_boundary121 <= 12'd0;
		roi_boundary122 <= 12'd0;
		roi_boundary123 <= 12'd0;
		roi_boundary124 <= 12'd0;
		roi_boundary125 <= 12'd0;
		roi_boundary126 <= 12'd0;
		roi_boundary127 <= 12'd0;
		roi_boundary128 <= 12'd0;
		roi_boundary129 <= 12'd0;
		roi_boundary130 <= 12'd0;
		roi_boundary131 <= 12'd0;
		roi_boundary132 <= 12'd0;
		roi_boundary133 <= 12'd0;
		roi_boundary134 <= 12'd0;
		roi_boundary135 <= 12'd0;
		roi_boundary136 <= 12'd0;
		roi_boundary137 <= 12'd0;
		roi_boundary138 <= 12'd0;
		roi_boundary139 <= 12'd0;
		roi_boundary140 <= 12'd0;
		roi_boundary141 <= 12'd0;
		roi_boundary142 <= 12'd0;
		roi_boundary143 <= 12'd0;
		roi_boundary144 <= 12'd0;
		roi_boundary145 <= 12'd0;
		roi_boundary146 <= 12'd0;
		roi_boundary147 <= 12'd0;
		roi_boundary148 <= 12'd0;
		roi_boundary149 <= 12'd0;
		roi_boundary150 <= 12'd0;
		roi_boundary151 <= 12'd0;
		roi_boundary152 <= 12'd0;
		roi_boundary153 <= 12'd0;
		roi_boundary154 <= 12'd0;
		roi_boundary155 <= 12'd0;
		roi_boundary156 <= 12'd0;
		roi_boundary157 <= 12'd0;
		roi_boundary158 <= 12'd0;
		roi_boundary159 <= 12'd0;
		roi_boundary160 <= 12'd0;
		roi_boundary161 <= 12'd0;
		roi_boundary162 <= 12'd0;
		roi_boundary163 <= 12'd0;
		roi_boundary164 <= 12'd0;
		roi_boundary165 <= 12'd0;
		roi_boundary166 <= 12'd0;
		roi_boundary167 <= 12'd0;
		roi_boundary168 <= 12'd0;
		roi_boundary169 <= 12'd0;
		roi_boundary170 <= 12'd0;
		roi_boundary171 <= 12'd0;
		roi_boundary172 <= 12'd0;
		roi_boundary173 <= 12'd0;
		roi_boundary174 <= 12'd0;
		roi_boundary175 <= 12'd0;
		roi_boundary176 <= 12'd0;
		roi_boundary177 <= 12'd0;
		roi_boundary178 <= 12'd0;
		roi_boundary179 <= 12'd0;
		roi_boundary180 <= 12'd0;
		roi_boundary181 <= 12'd0;
		roi_boundary182 <= 12'd0;
		roi_boundary183 <= 12'd0;
		roi_boundary184 <= 12'd0;
		roi_boundary185 <= 12'd0;
		roi_boundary186 <= 12'd0;
		roi_boundary187 <= 12'd0;
		roi_boundary188 <= 12'd0;
		roi_boundary189 <= 12'd0;
		roi_boundary190 <= 12'd0;
		roi_boundary191 <= 12'd0;
		roi_boundary192 <= 12'd0;
		roi_boundary193 <= 12'd0;
		roi_boundary194 <= 12'd0;
		roi_boundary195 <= 12'd0;
		roi_boundary196 <= 12'd0;
		roi_boundary197 <= 12'd0;
		roi_boundary198 <= 12'd0;
		roi_boundary199 <= 12'd0;
		roi_boundary200 <= 12'd0;
		roi_boundary201 <= 12'd0;
		roi_boundary202 <= 12'd0;
		roi_boundary203 <= 12'd0;
		roi_boundary204 <= 12'd0;
		roi_boundary205 <= 12'd0;
		roi_boundary206 <= 12'd0;
		roi_boundary207 <= 12'd0;
		roi_boundary208 <= 12'd0;
		roi_boundary209 <= 12'd0;
		roi_boundary210 <= 12'd0;
		roi_boundary211 <= 12'd0;
		roi_boundary212 <= 12'd0;
		roi_boundary213 <= 12'd0;
		roi_boundary214 <= 12'd0;
		roi_boundary215 <= 12'd0;
		roi_boundary216 <= 12'd0;
		roi_boundary217 <= 12'd0;
		roi_boundary218 <= 12'd0;
		roi_boundary219 <= 12'd0;
		roi_boundary220 <= 12'd0;
		roi_boundary221 <= 12'd0;
		roi_boundary222 <= 12'd0;
		roi_boundary223 <= 12'd0;
		roi_boundary224 <= 12'd0;
		roi_boundary225 <= 12'd0;
		roi_boundary226 <= 12'd0;
		roi_boundary227 <= 12'd0;
		roi_boundary228 <= 12'd0;
		roi_boundary229 <= 12'd0;
		roi_boundary230 <= 12'd0;
		roi_boundary231 <= 12'd0;
		roi_boundary232 <= 12'd0;
		roi_boundary233 <= 12'd0;
		roi_boundary234 <= 12'd0;
		roi_boundary235 <= 12'd0;
		roi_boundary236 <= 12'd0;
		roi_boundary237 <= 12'd0;
		roi_boundary238 <= 12'd0;
		roi_boundary239 <= 12'd0;
		roi_boundary240 <= 12'd0;
		roi_boundary241 <= 12'd0;
		roi_boundary242 <= 12'd0;
		roi_boundary243 <= 12'd0;
		roi_boundary244 <= 12'd0;
		roi_boundary245 <= 12'd0;
		roi_boundary246 <= 12'd0;
		roi_boundary247 <= 12'd0;
		roi_boundary248 <= 12'd0;
		roi_boundary249 <= 12'd0;
		roi_boundary250 <= 12'd0;
		roi_boundary251 <= 12'd0;
		roi_boundary252 <= 12'd0;
		roi_boundary253 <= 12'd0;
		roi_boundary254 <= 12'd0;
		roi_boundary255 <= 12'd0;
	end
	xilinxmultiregimpl00 <= q_clk;
	xilinxmultiregimpl01 <= xilinxmultiregimpl00;
	xilinxmultiregimpl10 <= mmcm_locked;
	xilinxmultiregimpl11 <= xilinxmultiregimpl10;
	xilinxmultiregimpl20 <= frequency_counter_toggle;
	xilinxmultiregimpl21 <= xilinxmultiregimpl20;
	xilinxmultiregimpl30 <= last_x;
	xilinxmultiregimpl31 <= xilinxmultiregimpl30;
	xilinxmultiregimpl40 <= last_y;
	xilinxmultiregimpl41 <= xilinxmultiregimpl40;
end

IBUFDS IBUFDS(
	.I(camera_link_pads_clk_p),
	.IB(camera_link_pads_clk_n),
	.O(clk_se)
);

ISERDESE2 #(
	.DATA_RATE("SDR"),
	.DATA_WIDTH(3'd7),
	.INTERFACE_TYPE("NETWORKING"),
	.NUM_CE(1'd1),
	.SERDES_MODE("MASTER")
) ISERDESE2 (
	.CE1(1'd1),
	.CLK(cl7x_clk),
	.CLKB((~cl7x_clk)),
	.CLKDIV(cl_clk),
	.D(clk_se),
	.RST(cl_rst),
	.O(clk_se_iserdes),
	.Q1(q_clk[6]),
	.Q2(q_clk[5]),
	.Q3(q_clk[4]),
	.Q4(q_clk[3]),
	.Q5(q_clk[2]),
	.Q6(q_clk[1]),
	.Q7(q_clk[0])
);

IBUFDS IBUFDS_1(
	.I(camera_link_pads_sdi_p),
	.IB(camera_link_pads_sdi_n),
	.O(sdi_se)
);

ISERDESE2 #(
	.DATA_RATE("SDR"),
	.DATA_WIDTH(3'd7),
	.INTERFACE_TYPE("NETWORKING"),
	.NUM_CE(1'd1),
	.SERDES_MODE("MASTER")
) ISERDESE2_1 (
	.CE1(1'd1),
	.CLK(cl7x_clk),
	.CLKB((~cl7x_clk)),
	.CLKDIV(cl_clk),
	.D(sdi_se),
	.RST(cl_rst),
	.Q1(q[6]),
	.Q2(q[5]),
	.Q3(q[4]),
	.Q4(q[3]),
	.Q5(q[2]),
	.Q6(q[1]),
	.Q7(q[0])
);

MMCME2_ADV #(
	.CLKFBOUT_MULT_F(21.0),
	.CLKIN1_PERIOD(18.0),
	.CLKOUT1_DIVIDE(2'd3),
	.CLKOUT1_PHASE(0.0),
	.CLKOUT1_USE_FINE_PS("TRUE"),
	.DIVCLK_DIVIDE(1'd1)
) MMCME2_ADV (
	.CLKFBIN(mmcm_fb),
	.CLKIN1(clk_se_iserdes),
	.CLKINSEL(1'd1),
	.PSCLK(sys_clk),
	.PSEN(phase_shift_re),
	.PSINCDEC(phase_shift_r),
	.RST(pll_reset),
	.CLKFBOUT(mmcm_fb),
	.CLKOUT1(cl7x_clk_1),
	.LOCKED(mmcm_locked),
	.PSDONE(mmcm_ps_psdone)
);

BUFR #(
	.BUFR_DIVIDE("7")
) BUFR (
	.CLR((~mmcm_locked)),
	.I(cl7x_clk_1),
	.O(cl_clk)
);

BUFIO BUFIO(
	.I(cl7x_clk_1),
	.O(cl7x_clk)
);

(* ars_ff1 = "true", async_reg = "true" *) FDPE #(
	.INIT(1'd1)
) FDPE (
	.C(cl_clk),
	.CE(1'd1),
	.D(1'd0),
	.PRE(async_reset),
	.Q(rst_meta)
);

(* ars_ff2 = "true", async_reg = "true" *) FDPE #(
	.INIT(1'd1)
) FDPE_1 (
	.C(cl_clk),
	.CE(1'd1),
	.D(rst_meta),
	.PRE(async_reset),
	.Q(cl_rst)
);

endmodule


