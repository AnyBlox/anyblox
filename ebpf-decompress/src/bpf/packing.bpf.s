	.text
	.file	"packing.bpf.c"
	.file	0 "/home/mat/src/portable-decompress" "ebpf-decompress/src/bpf/packing.bpf.c" md5 0xb5ef7bd598f0de0e3bc499cea306dce7
	.file	1 "./ebpf-decompress/src/bpf" "vmlinux.h" md5 0xd609f28e272dd8c860d6fdddd8b15647
	.file	2 "./ebpf-decompress/src/bpf" "packing.bpf.c" md5 0xb5ef7bd598f0de0e3bc499cea306dce7
	.file	3 "/usr/include/bpf" "usdt.bpf.h" md5 0xfc27739d8e670f669d7a758532319777
	.file	4 "/usr/include/bpf" "bpf_helper_defs.h" md5 0x7422ca06c9dc86eba2f268a57d8acf2f
	.hidden	bpf_usdt_arg_cnt                # -- Begin function bpf_usdt_arg_cnt
	.weak	bpf_usdt_arg_cnt
	.p2align	3
	.type	bpf_usdt_arg_cnt,@function
bpf_usdt_arg_cnt:                       # @bpf_usdt_arg_cnt
.Lfunc_begin0:
	.loc	3 96 0                          # /usr/include/bpf/usdt.bpf.h:96:0
	.cfi_sections .debug_frame
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:ctx <- $r1
	#DEBUG_VALUE: __bpf_usdt_spec_id:ctx <- undef
	.loc	3 82 7 prologue_end             # /usr/include/bpf/usdt.bpf.h:82:7
.Ltmp0:
	r2 = LINUX_HAS_BPF_COOKIE ll
	r2 = *(u8 *)(r2 + 0)
.Ltmp1:
.Ltmp2:
	.loc	3 82 6 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:82:6
.Ltmp3:
	if r2 != 0 goto LBB0_3
.Ltmp4:
.Ltmp5:
# %bb.1:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:ctx <- $r1
	.loc	3 83 13 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp6:
.Ltmp7:
	r1 = *(u64 *)(r1 + 128)
.Ltmp8:
.Ltmp9:
	#DEBUG_VALUE: ip <- $r1
	.loc	3 83 8 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:83:8
.Ltmp10:
	*(u64 *)(r10 - 8) = r1
.Ltmp11:
.Ltmp12:
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 8, DW_OP_deref] $r10
	.loc	3 0 8                           # /usr/include/bpf/usdt.bpf.h:0:8
	r2 = r10
	.loc	3 83 13                         # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp13:
	r2 += -8
	.loc	3 86 17 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:86:17
.Ltmp14:
	r1 = __bpf_usdt_ip_to_spec_id ll
	call 1
.Ltmp15:
	r1 = r0
.Ltmp16:
.Ltmp17:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	.loc	3 0 17 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:0:17
	r0 = 4294967293 ll
	.loc	3 87 10 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:87:10
.Ltmp18:
	if r1 == 0 goto LBB0_4
.Ltmp19:
.Ltmp20:
# %bb.2:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 8, DW_OP_deref] $r10
	.loc	3 87 24 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:87:24
.Ltmp21:
	r0 = *(u32 *)(r1 + 0)
	goto LBB0_4
.Ltmp22:
.Ltmp23:
LBB0_3:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:ctx <- $r1
	.loc	3 90 9 is_stmt 1                # /usr/include/bpf/usdt.bpf.h:90:9
.Ltmp24:
	call 174
.Ltmp25:
.Ltmp26:
LBB0_4:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:spec_id <- $r0
	.loc	3 0 9 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:9
	r6 = 4294967293 ll
	.loc	3 100 10 is_stmt 1              # /usr/include/bpf/usdt.bpf.h:100:10
.Ltmp27:
	*(u32 *)(r10 - 12) = r0
	r0 <<= 32
.Ltmp28:
.Ltmp29:
	r0 s>>= 32
	r1 = 0
	.loc	3 101 6                         # /usr/include/bpf/usdt.bpf.h:101:6
.Ltmp30:
	if r1 s> r0 goto LBB0_7
# %bb.5:
.Ltmp31:
.Ltmp32:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:spec_id <- [DW_OP_plus_uconst 4, DW_OP_deref] $r10
	.loc	3 0 6 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:6
	r2 = r10
.Ltmp33:
	r2 += -12
	.loc	3 104 9 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:104:9
.Ltmp34:
	r1 = __bpf_usdt_specs ll
	call 1
.Ltmp35:
.Ltmp36:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:spec <- $r0
	.loc	3 105 6                         # /usr/include/bpf/usdt.bpf.h:105:6
.Ltmp37:
	if r0 == 0 goto LBB0_7
.Ltmp38:
.Ltmp39:
# %bb.6:
	#DEBUG_VALUE: bpf_usdt_arg_cnt:spec <- $r0
	#DEBUG_VALUE: bpf_usdt_arg_cnt:spec_id <- [DW_OP_plus_uconst 4, DW_OP_deref] $r10
	.loc	3 108 15                        # /usr/include/bpf/usdt.bpf.h:108:15
.Ltmp40:
	r6 = *(u16 *)(r0 + 200)
	r6 <<= 48
	r6 s>>= 48
.Ltmp41:
.Ltmp42:
LBB0_7:
	.loc	3 109 1                         # /usr/include/bpf/usdt.bpf.h:109:1
.Ltmp43:
	r0 = r6
	exit
.Ltmp44:
.Ltmp45:
.Lfunc_end0:
	.size	bpf_usdt_arg_cnt, .Lfunc_end0-bpf_usdt_arg_cnt
	.cfi_endproc
                                        # -- End function
	.hidden	bpf_usdt_arg                    # -- Begin function bpf_usdt_arg
	.weak	bpf_usdt_arg
	.p2align	3
	.type	bpf_usdt_arg,@function
bpf_usdt_arg:                           # @bpf_usdt_arg
.Lfunc_begin1:
	.loc	3 117 0                         # /usr/include/bpf/usdt.bpf.h:117:0
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r1
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:res <- $r3
	r7 = r2
.Ltmp46:
.Ltmp47:
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	r9 = r1
.Ltmp48:
.Ltmp49:
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	r1 = 0
	*(u64 *)(r10 - 24) = r3
.Ltmp50:
.Ltmp51:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	.loc	3 123 7 prologue_end            # /usr/include/bpf/usdt.bpf.h:123:7
.Ltmp52:
	*(u64 *)(r3 + 0) = r1
.Ltmp53:
.Ltmp54:
	#DEBUG_VALUE: __bpf_usdt_spec_id:ctx <- undef
	.loc	3 82 7                          # /usr/include/bpf/usdt.bpf.h:82:7
.Ltmp55:
	r1 = LINUX_HAS_BPF_COOKIE ll
	r1 = *(u8 *)(r1 + 0)
.Ltmp56:
.Ltmp57:
	.loc	3 82 6 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:82:6
.Ltmp58:
	if r1 != 0 goto LBB1_3
.Ltmp59:
.Ltmp60:
# %bb.1:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 83 13 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp61:
.Ltmp62:
	r1 = *(u64 *)(r9 + 128)
.Ltmp63:
.Ltmp64:
	#DEBUG_VALUE: ip <- $r1
	.loc	3 83 8 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:83:8
.Ltmp65:
	*(u64 *)(r10 - 8) = r1
.Ltmp66:
.Ltmp67:
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	.loc	3 0 8                           # /usr/include/bpf/usdt.bpf.h:0:8
	r2 = r10
	.loc	3 83 13                         # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp68:
	r2 += -8
	.loc	3 86 17 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:86:17
.Ltmp69:
	r1 = __bpf_usdt_ip_to_spec_id ll
	call 1
.Ltmp70:
	r1 = r0
.Ltmp71:
.Ltmp72:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	.loc	3 0 17 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:0:17
	r0 = 4294967293 ll
	.loc	3 87 10 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:87:10
.Ltmp73:
	if r1 == 0 goto LBB1_4
.Ltmp74:
.Ltmp75:
# %bb.2:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 87 24 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:87:24
.Ltmp76:
	r0 = *(u32 *)(r1 + 0)
	goto LBB1_4
.Ltmp77:
.Ltmp78:
LBB1_3:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 90 9 is_stmt 1                # /usr/include/bpf/usdt.bpf.h:90:9
.Ltmp79:
	r1 = r9
	call 174
.Ltmp80:
.Ltmp81:
LBB1_4:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- $r0
	.loc	3 0 9 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:9
	r6 = 4294967293 ll
	.loc	3 125 10 is_stmt 1              # /usr/include/bpf/usdt.bpf.h:125:10
.Ltmp82:
	*(u32 *)(r10 - 12) = r0
	r0 <<= 32
.Ltmp83:
.Ltmp84:
	r0 s>>= 32
	r1 = 0
	.loc	3 126 6                         # /usr/include/bpf/usdt.bpf.h:126:6
.Ltmp85:
	if r1 s> r0 goto LBB1_18
.Ltmp86:
.Ltmp87:
# %bb.5:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	.loc	3 0 6 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:6
	r2 = r10
.Ltmp88:
	r2 += -12
	.loc	3 129 9 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:129:9
.Ltmp89:
	r1 = __bpf_usdt_specs ll
	call 1
.Ltmp90:
	r8 = r0
.Ltmp91:
.Ltmp92:
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	.loc	3 130 6                         # /usr/include/bpf/usdt.bpf.h:130:6
.Ltmp93:
	if r8 == 0 goto LBB1_18
.Ltmp94:
.Ltmp95:
# %bb.6:
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 0 6 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:6
	r6 = 4294967294 ll
	.loc	3 133 6 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:133:6
.Ltmp96:
	if r7 > 11 goto LBB1_18
.Ltmp97:
.Ltmp98:
# %bb.7:
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 135 2                         # /usr/include/bpf/usdt.bpf.h:135:2
	#APP
	#NO_APP
.Ltmp99:
.Ltmp100:
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 136 23                        # /usr/include/bpf/usdt.bpf.h:136:23
.Ltmp101:
	r1 = *(u16 *)(r8 + 200)
	r1 <<= 48
	r1 s>>= 48
.Ltmp102:
.Ltmp103:
	.loc	3 136 6 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:136:6
.Ltmp104:
	if r7 >= r1 goto LBB1_18
.Ltmp105:
.Ltmp106:
# %bb.8:
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 139 14 is_stmt 1              # /usr/include/bpf/usdt.bpf.h:139:14
.Ltmp107:
	r1 = r7
	r1 <<= 4
	r2 = r8
	r2 += r1
.Ltmp108:
.Ltmp109:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	.loc	3 140 20                        # /usr/include/bpf/usdt.bpf.h:140:20
.Ltmp110:
	r1 = *(u32 *)(r2 + 8)
	.loc	3 140 2 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:140:2
.Ltmp111:
	if r1 == 2 goto LBB1_13
.Ltmp112:
.Ltmp113:
# %bb.9:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	if r1 == 1 goto LBB1_12
.Ltmp114:
.Ltmp115:
# %bb.10:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 0 2                           # /usr/include/bpf/usdt.bpf.h:0:2
	r6 = 4294967274 ll
	.loc	3 140 2                         # /usr/include/bpf/usdt.bpf.h:140:2
	if r1 != 0 goto LBB1_18
.Ltmp116:
.Ltmp117:
# %bb.11:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 145 19 is_stmt 1              # /usr/include/bpf/usdt.bpf.h:145:19
.Ltmp118:
	r1 = *(u64 *)(r2 + 0)
.Ltmp119:
.Ltmp120:
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r1
	.loc	3 145 7 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:145:7
.Ltmp121:
	*(u64 *)(r10 - 8) = r1
	goto LBB1_15
.Ltmp122:
.Ltmp123:
LBB1_12:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 153 74 is_stmt 1              # /usr/include/bpf/usdt.bpf.h:153:74
.Ltmp124:
	r1 = r7
	r1 <<= 4
	r2 = r8
.Ltmp125:
.Ltmp126:
	r2 += r1
	r1 = *(u16 *)(r2 + 12)
	r1 <<= 48
	r1 s>>= 48
	.loc	3 153 62 is_stmt 0              # /usr/include/bpf/usdt.bpf.h:153:62
.Ltmp127:
	r9 += r1
.Ltmp128:
.Ltmp129:
	#DEBUG_VALUE: bpf_usdt_arg:val <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	.loc	3 0 62                          # /usr/include/bpf/usdt.bpf.h:0:62
	r1 = r10
	.loc	3 153 74                        # /usr/include/bpf/usdt.bpf.h:153:74
.Ltmp130:
	r1 += -8
	.loc	3 153 9                         # /usr/include/bpf/usdt.bpf.h:153:9
.Ltmp131:
	r2 = 8
	r3 = r9
	call 113
.Ltmp132:
	r6 = r0
.Ltmp133:
.Ltmp134:
	#DEBUG_VALUE: bpf_usdt_arg:err <- $r6
	r1 = r6
	r1 <<= 32
	r1 >>= 32
	.loc	3 154 7 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:154:7
.Ltmp135:
	if r1 == 0 goto LBB1_15
	goto LBB1_18
.Ltmp136:
.Ltmp137:
LBB1_13:
	#DEBUG_VALUE: bpf_usdt_arg:arg_spec <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:ctx <- $r9
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 165 74                        # /usr/include/bpf/usdt.bpf.h:165:74
.Ltmp138:
	r1 = *(u16 *)(r2 + 12)
	r1 <<= 48
	r1 s>>= 48
	.loc	3 165 62 is_stmt 0              # /usr/include/bpf/usdt.bpf.h:165:62
.Ltmp139:
	r9 += r1
.Ltmp140:
.Ltmp141:
	#DEBUG_VALUE: bpf_usdt_arg:val <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	.loc	3 0 62                          # /usr/include/bpf/usdt.bpf.h:0:62
	r1 = r10
	.loc	3 165 74                        # /usr/include/bpf/usdt.bpf.h:165:74
.Ltmp142:
	r1 += -8
	r6 = r2
	.loc	3 165 9                         # /usr/include/bpf/usdt.bpf.h:165:9
.Ltmp143:
	r2 = 8
.Ltmp144:
.Ltmp145:
	r3 = r9
	call 113
.Ltmp146:
	.loc	3 0 9                           # /usr/include/bpf/usdt.bpf.h:0:9
	r2 = r6
	.loc	3 165 9                         # /usr/include/bpf/usdt.bpf.h:165:9
	r6 = r0
.Ltmp147:
.Ltmp148:
	#DEBUG_VALUE: bpf_usdt_arg:err <- $r6
	r1 = r6
	r1 <<= 32
	r1 >>= 32
	.loc	3 166 7 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:166:7
.Ltmp149:
	if r1 != 0 goto LBB1_18
.Ltmp150:
.Ltmp151:
# %bb.14:
	#DEBUG_VALUE: bpf_usdt_arg:err <- $r6
	#DEBUG_VALUE: bpf_usdt_arg:val <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 168 72                        # /usr/include/bpf/usdt.bpf.h:168:72
.Ltmp152:
	r1 = *(u64 *)(r2 + 0)
	.loc	3 168 56 is_stmt 0              # /usr/include/bpf/usdt.bpf.h:168:56
.Ltmp153:
	r3 = *(u64 *)(r10 - 8)
.Ltmp154:
.Ltmp155:
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r3
	.loc	3 168 60                        # /usr/include/bpf/usdt.bpf.h:168:60
.Ltmp156:
	r3 += r1
.Ltmp157:
.Ltmp158:
	#DEBUG_VALUE: bpf_usdt_arg:val <- [DW_OP_plus_uconst 16, DW_OP_deref] $r10
	.loc	3 0 60                          # /usr/include/bpf/usdt.bpf.h:0:60
	r1 = r10
	.loc	3 168 72                        # /usr/include/bpf/usdt.bpf.h:168:72
.Ltmp159:
	r1 += -8
	.loc	3 168 9                         # /usr/include/bpf/usdt.bpf.h:168:9
.Ltmp160:
	r2 = 8
	call 112
.Ltmp161:
	r6 = r0
.Ltmp162:
.Ltmp163:
	#DEBUG_VALUE: bpf_usdt_arg:err <- $r6
	r1 = r6
	r1 <<= 32
	r1 >>= 32
	.loc	3 169 7 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:169:7
.Ltmp164:
	if r1 != 0 goto LBB1_18
.Ltmp165:
.Ltmp166:
LBB1_15:
	#DEBUG_VALUE: bpf_usdt_arg:spec <- $r8
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:arg_num <- $r7
	.loc	3 183 20                        # /usr/include/bpf/usdt.bpf.h:183:20
.Ltmp167:
	r7 <<= 4
.Ltmp168:
.Ltmp169:
	r8 += r7
.Ltmp170:
.Ltmp171:
	r1 = *(u8 *)(r8 + 15)
	r1 <<= 56
	r1 s>>= 56
	.loc	3 183 6 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:183:6
.Ltmp172:
	r1 <<= 32
	r1 >>= 32
	r2 = *(u64 *)(r10 - 8)
.Ltmp173:
.Ltmp174:
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r2
	r2 <<= r1
.Ltmp175:
.Ltmp176:
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r2
	.loc	3 184 6 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:184:6
.Ltmp177:
	r3 = r2
	r3 >>= r1
.Ltmp178:
	.loc	3 184 16 is_stmt 0              # /usr/include/bpf/usdt.bpf.h:184:16
.Ltmp179:
.Ltmp180:
	r4 = *(u8 *)(r8 + 14)
.Ltmp181:
.Ltmp182:
	.loc	3 184 6                         # /usr/include/bpf/usdt.bpf.h:184:6
.Ltmp183:
	if r4 == 0 goto LBB1_17
.Ltmp184:
.Ltmp185:
# %bb.16:
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r2
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	.loc	3 0 6                           # /usr/include/bpf/usdt.bpf.h:0:6
	r2 s>>= r1
.Ltmp186:
.Ltmp187:
	r3 = r2
.Ltmp188:
.Ltmp189:
LBB1_17:
	#DEBUG_VALUE: bpf_usdt_arg:spec_id <- [DW_OP_plus_uconst 12, DW_OP_deref] $r10
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	#DEBUG_VALUE: bpf_usdt_arg:val <- $r3
	.loc	3 188 7 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:188:7
.Ltmp190:
	r1 = *(u64 *)(r10 - 24)
	*(u64 *)(r1 + 0) = r3
	r6 = 0
.Ltmp191:
.Ltmp192:
LBB1_18:
	#DEBUG_VALUE: bpf_usdt_arg:res <- [$r10+0]
	.loc	3 190 1                         # /usr/include/bpf/usdt.bpf.h:190:1
.Ltmp193:
	r0 = r6
	exit
.Ltmp194:
.Ltmp195:
.Lfunc_end1:
	.size	bpf_usdt_arg, .Lfunc_end1-bpf_usdt_arg
	.cfi_endproc
                                        # -- End function
	.hidden	bpf_usdt_cookie                 # -- Begin function bpf_usdt_cookie
	.weak	bpf_usdt_cookie
	.p2align	3
	.type	bpf_usdt_cookie,@function
bpf_usdt_cookie:                        # @bpf_usdt_cookie
.Lfunc_begin2:
	.loc	3 200 0                         # /usr/include/bpf/usdt.bpf.h:200:0
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: bpf_usdt_cookie:ctx <- $r1
	#DEBUG_VALUE: __bpf_usdt_spec_id:ctx <- undef
	.loc	3 82 7 prologue_end             # /usr/include/bpf/usdt.bpf.h:82:7
.Ltmp196:
	r2 = LINUX_HAS_BPF_COOKIE ll
	r2 = *(u8 *)(r2 + 0)
.Ltmp197:
.Ltmp198:
	.loc	3 82 6 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:82:6
.Ltmp199:
	if r2 != 0 goto LBB2_3
.Ltmp200:
.Ltmp201:
# %bb.1:
	#DEBUG_VALUE: bpf_usdt_cookie:ctx <- $r1
	.loc	3 83 13 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp202:
.Ltmp203:
	r1 = *(u64 *)(r1 + 128)
.Ltmp204:
.Ltmp205:
	#DEBUG_VALUE: ip <- $r1
	.loc	3 83 8 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:83:8
.Ltmp206:
	*(u64 *)(r10 - 8) = r1
.Ltmp207:
.Ltmp208:
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 8, DW_OP_deref] $r10
	.loc	3 0 8                           # /usr/include/bpf/usdt.bpf.h:0:8
	r2 = r10
	.loc	3 83 13                         # /usr/include/bpf/usdt.bpf.h:83:13
.Ltmp209:
	r2 += -8
	.loc	3 86 17 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:86:17
.Ltmp210:
	r1 = __bpf_usdt_ip_to_spec_id ll
	call 1
.Ltmp211:
	r1 = r0
.Ltmp212:
.Ltmp213:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	.loc	3 0 17 is_stmt 0                # /usr/include/bpf/usdt.bpf.h:0:17
	r0 = 4294967293 ll
	.loc	3 87 10 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:87:10
.Ltmp214:
	if r1 == 0 goto LBB2_4
.Ltmp215:
.Ltmp216:
# %bb.2:
	#DEBUG_VALUE: spec_id_ptr <- $r1
	#DEBUG_VALUE: ip <- [DW_OP_plus_uconst 8, DW_OP_deref] $r10
	.loc	3 87 24 is_stmt 0               # /usr/include/bpf/usdt.bpf.h:87:24
.Ltmp217:
	r0 = *(u32 *)(r1 + 0)
	goto LBB2_4
.Ltmp218:
.Ltmp219:
LBB2_3:
	#DEBUG_VALUE: bpf_usdt_cookie:ctx <- $r1
	.loc	3 90 9 is_stmt 1                # /usr/include/bpf/usdt.bpf.h:90:9
.Ltmp220:
	call 174
.Ltmp221:
.Ltmp222:
LBB2_4:
	#DEBUG_VALUE: bpf_usdt_cookie:spec_id <- $r0
	.loc	3 204 10                        # /usr/include/bpf/usdt.bpf.h:204:10
.Ltmp223:
	*(u32 *)(r10 - 12) = r0
	r0 <<= 32
.Ltmp224:
.Ltmp225:
	r0 s>>= 32
	r6 = 0
	.loc	3 205 6                         # /usr/include/bpf/usdt.bpf.h:205:6
.Ltmp226:
	if r6 s> r0 goto LBB2_7
# %bb.5:
.Ltmp227:
.Ltmp228:
	#DEBUG_VALUE: bpf_usdt_cookie:spec_id <- [DW_OP_plus_uconst 4, DW_OP_deref] $r10
	.loc	3 0 6 is_stmt 0                 # /usr/include/bpf/usdt.bpf.h:0:6
	r2 = r10
.Ltmp229:
	r2 += -12
	.loc	3 208 9 is_stmt 1               # /usr/include/bpf/usdt.bpf.h:208:9
.Ltmp230:
	r1 = __bpf_usdt_specs ll
	call 1
.Ltmp231:
.Ltmp232:
	#DEBUG_VALUE: bpf_usdt_cookie:spec <- $r0
	.loc	3 209 6                         # /usr/include/bpf/usdt.bpf.h:209:6
.Ltmp233:
	if r0 == 0 goto LBB2_7
.Ltmp234:
.Ltmp235:
# %bb.6:
	#DEBUG_VALUE: bpf_usdt_cookie:spec <- $r0
	#DEBUG_VALUE: bpf_usdt_cookie:spec_id <- [DW_OP_plus_uconst 4, DW_OP_deref] $r10
	.loc	3 212 15                        # /usr/include/bpf/usdt.bpf.h:212:15
.Ltmp236:
	r6 = *(u64 *)(r0 + 192)
.Ltmp237:
.Ltmp238:
LBB2_7:
	.loc	3 213 1                         # /usr/include/bpf/usdt.bpf.h:213:1
.Ltmp239:
	r0 = r6
	exit
.Ltmp240:
.Ltmp241:
.Lfunc_end2:
	.size	bpf_usdt_cookie, .Lfunc_end2-bpf_usdt_cookie
	.cfi_endproc
                                        # -- End function
	.section	usdt,"ax",@progbits
	.globl	bpf_prog                        # -- Begin function bpf_prog
	.p2align	3
	.type	bpf_prog,@function
bpf_prog:                               # @bpf_prog
.Lfunc_begin3:
	.loc	2 39 0                          # ./ebpf-decompress/src/bpf/packing.bpf.c:39:0
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: bpf_prog:ctx <- $r1
	r3 = r1
.Ltmp242:
.Ltmp243:
	#DEBUG_VALUE: bpf_prog:ctx <- $r3
.Ltmp244:
	r1 = 80
	r3 += r1
.Ltmp245:
.Ltmp246:
	#DEBUG_VALUE: __r <- [DW_OP_deref] $r10
	r1 = r10
.Ltmp247:
	.loc	2 0 0 is_stmt 0                 # ./ebpf-decompress/src/bpf/packing.bpf.c:0:0
.Ltmp248:
.Ltmp249:
	r1 += -24
.Ltmp250:
	.loc	2 40 29 prologue_end is_stmt 1  # ./ebpf-decompress/src/bpf/packing.bpf.c:40:29
.Ltmp251:
.Ltmp252:
	r2 = 8
	call 113
.Ltmp253:
.Ltmp254:
	#DEBUG_VALUE: bpf_prog:len <- undef
	.loc	2 0 29 is_stmt 0                # ./ebpf-decompress/src/bpf/packing.bpf.c:0:29
	r1 = 0
.Ltmp255:
.Ltmp256:
	#DEBUG_VALUE: bpf_prog:zero <- 0
	.loc	2 42 9 is_stmt 1                # ./ebpf-decompress/src/bpf/packing.bpf.c:42:9
.Ltmp257:
	*(u32 *)(r10 - 4) = r1
.Ltmp258:
.Ltmp259:
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 20, DW_OP_deref] $r10
	.loc	2 0 9 is_stmt 0                 # ./ebpf-decompress/src/bpf/packing.bpf.c:0:9
	r7 = r10
.Ltmp260:
	r7 += -4
	.loc	2 43 20 is_stmt 1               # ./ebpf-decompress/src/bpf/packing.bpf.c:43:20
.Ltmp261:
	r1 = in_bytes ll
	r2 = r7
	call 1
.Ltmp262:
	r6 = r0
.Ltmp263:
.Ltmp264:
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r6
	.loc	2 44 21                         # ./ebpf-decompress/src/bpf/packing.bpf.c:44:21
.Ltmp265:
	r1 = out_bytes ll
	r2 = r7
	call 1
.Ltmp266:
.Ltmp267:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	.loc	2 0 21 is_stmt 0                # ./ebpf-decompress/src/bpf/packing.bpf.c:0:21
	r7 = 1
.Ltmp268:
	.loc	2 46 17 is_stmt 1               # ./ebpf-decompress/src/bpf/packing.bpf.c:46:17
.Ltmp269:
.Ltmp270:
	if r6 == 0 goto LBB3_3
.Ltmp271:
.Ltmp272:
# %bb.1:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r6
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 20, DW_OP_deref] $r10
	if r0 == 0 goto LBB3_3
.Ltmp273:
.Ltmp274:
# %bb.2:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r6
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 20, DW_OP_deref] $r10
	.loc	2 53 22                         # ./ebpf-decompress/src/bpf/packing.bpf.c:53:22
.Ltmp275:
	*(u64 *)(r10 - 16) = r0
	.loc	2 52 21                         # ./ebpf-decompress/src/bpf/packing.bpf.c:52:21
.Ltmp276:
	*(u64 *)(r10 - 24) = r6
	r3 = r10
	.loc	2 53 22                         # ./ebpf-decompress/src/bpf/packing.bpf.c:53:22
.Ltmp277:
	r3 += -24
	r7 = 0
	.loc	2 55 5                          # ./ebpf-decompress/src/bpf/packing.bpf.c:55:5
.Ltmp278:
	r1 = 8192
	r2 = decode_one ll
	r4 = 0
	call 181
.Ltmp279:
.Ltmp280:
LBB3_3:
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r6
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 20, DW_OP_deref] $r10
	.loc	2 58 1                          # ./ebpf-decompress/src/bpf/packing.bpf.c:58:1
.Ltmp281:
	r0 = r7
	exit
.Ltmp282:
.Ltmp283:
.Lfunc_end3:
	.size	bpf_prog, .Lfunc_end3-bpf_prog
	.cfi_endproc
                                        # -- End function
	.text
	.p2align	3                               # -- Begin function decode_one
	.type	decode_one,@function
decode_one:                             # @decode_one
.Lfunc_begin4:
	.loc	2 61 0                          # ./ebpf-decompress/src/bpf/packing.bpf.c:61:0
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: decode_one:i <- $r1
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: inner_i <- 0
	r3 = 0
.Ltmp284:
	.loc	2 62 5 prologue_end             # ./ebpf-decompress/src/bpf/packing.bpf.c:62:5
.Ltmp285:
.Ltmp286:
	r4 = r1
	r4 <<= 14
	r4 <<= 32
	r4 >>= 32
.Ltmp287:
	.loc	2 64 24                         # ./ebpf-decompress/src/bpf/packing.bpf.c:64:24
.Ltmp288:
.Ltmp289:
	r1 <<= 10
.Ltmp290:
.Ltmp291:
	.loc	2 62 5                          # ./ebpf-decompress/src/bpf/packing.bpf.c:62:5
.Ltmp292:
	r1 <<= 32
	r1 >>= 32
.Ltmp293:
.Ltmp294:
LBB4_1:                                 # =>This Inner Loop Header: Depth=1
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: read_i <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 1, DW_OP_plus, DW_OP_stack_value] $r3, $r1
	.loc	2 66 20                         # ./ebpf-decompress/src/bpf/packing.bpf.c:66:20
.Ltmp295:
	r5 = r1
	r5 += r3
	r0 = 1
.Ltmp296:
.Ltmp297:
	.loc	2 66 13 is_stmt 0               # ./ebpf-decompress/src/bpf/packing.bpf.c:66:13
.Ltmp298:
	if r5 > 8388607 goto LBB4_3
.Ltmp299:
.Ltmp300:
# %bb.2:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: read_i <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 1, DW_OP_plus, DW_OP_stack_value] $r3, $r1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: elem <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 32 7, DW_OP_LLVM_convert 64 7, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_stack_value] undef
	#DEBUG_VALUE: byte <- undef
	#DEBUG_VALUE: a1 <- [DW_OP_constu 18446744073709551552, DW_OP_and, DW_OP_stack_value] undef
	#DEBUG_VALUE: a2 <- [DW_OP_constu 48, DW_OP_and, DW_OP_stack_value] undef
	#DEBUG_VALUE: a3 <- [DW_OP_constu 12, DW_OP_and, DW_OP_stack_value] undef
	#DEBUG_VALUE: a4 <- [DW_OP_constu 3, DW_OP_and, DW_OP_stack_value] undef
	#DEBUG_VALUE: b1 <- [DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 6, DW_OP_shr, DW_OP_stack_value] undef
	#DEBUG_VALUE: b2 <- [DW_OP_constu 48, DW_OP_and, DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 4, DW_OP_shr, DW_OP_stack_value] undef
	#DEBUG_VALUE: b3 <- [DW_OP_constu 12, DW_OP_and, DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 2, DW_OP_shr, DW_OP_stack_value] undef
	#DEBUG_VALUE: b4 <- [DW_OP_constu 3, DW_OP_and, DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_stack_value] undef
	#DEBUG_VALUE: write_i <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 64 7, DW_OP_LLVM_convert 32 7, DW_OP_plus, DW_OP_constu 4, DW_OP_shl, DW_OP_stack_value] undef
	.loc	2 71 25 is_stmt 1               # ./ebpf-decompress/src/bpf/packing.bpf.c:71:25
.Ltmp301:
	r0 = *(u64 *)(r2 + 0)
.Ltmp302:
.Ltmp303:
	#DEBUG_VALUE: elem <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_stack_value] undef
	.loc	2 72 19                         # ./ebpf-decompress/src/bpf/packing.bpf.c:72:19
.Ltmp304:
	r0 += r1
.Ltmp305:
.Ltmp306:
	#DEBUG_VALUE: b1 <- undef
	.loc	2 87 32                         # ./ebpf-decompress/src/bpf/packing.bpf.c:87:32
.Ltmp307:
	r5 = *(u64 *)(r2 + 8)
.Ltmp308:
.Ltmp309:
	#DEBUG_VALUE: elem_1 <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 64 7, DW_OP_LLVM_convert 32 7, DW_OP_plus, DW_OP_constu 4, DW_OP_shl, DW_OP_LLVM_convert 32 7, DW_OP_LLVM_convert 64 7, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_stack_value] undef
	.loc	2 88 21                         # ./ebpf-decompress/src/bpf/packing.bpf.c:88:21
.Ltmp310:
	r5 += r4
.Ltmp311:
.Ltmp312:
	.loc	2 72 19                         # ./ebpf-decompress/src/bpf/packing.bpf.c:72:19
.Ltmp313:
	r0 += r3
	r0 = *(u8 *)(r0 + 0)
.Ltmp314:
.Ltmp315:
	#DEBUG_VALUE: b4 <- undef
	#DEBUG_VALUE: b3 <- [DW_OP_constu 12, DW_OP_and, DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 2, DW_OP_shr, DW_OP_stack_value] $r0
	#DEBUG_VALUE: b2 <- [DW_OP_constu 48, DW_OP_and, DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 4, DW_OP_shr, DW_OP_stack_value] $r0
	#DEBUG_VALUE: b1 <- [DW_OP_LLVM_convert 8 7, DW_OP_LLVM_convert 32 7, DW_OP_constu 6, DW_OP_shr, DW_OP_stack_value] $r0
	#DEBUG_VALUE: a4 <- undef
	#DEBUG_VALUE: a3 <- [DW_OP_constu 12, DW_OP_and, DW_OP_stack_value] $r0
	#DEBUG_VALUE: a2 <- [DW_OP_constu 48, DW_OP_and, DW_OP_stack_value] $r0
	#DEBUG_VALUE: a1 <- [DW_OP_constu 18446744073709551552, DW_OP_and, DW_OP_stack_value] $r0
	#DEBUG_VALUE: byte <- $r0
	.loc	2 81 18                         # ./ebpf-decompress/src/bpf/packing.bpf.c:81:18
.Ltmp316:
	r6 = r0
	r6 &= 3
.Ltmp317:
.Ltmp318:
	#DEBUG_VALUE: elem_4 <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 64 7, DW_OP_LLVM_convert 32 7, DW_OP_plus, DW_OP_constu 4, DW_OP_shl, DW_OP_LLVM_convert 32 7, DW_OP_LLVM_convert 64 7, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_plus_uconst 12, DW_OP_stack_value] undef
	#DEBUG_VALUE: b4 <- $r6
	.loc	2 94 21                         # ./ebpf-decompress/src/bpf/packing.bpf.c:94:21
.Ltmp319:
	*(u32 *)(r5 + 12) = r6
.Ltmp320:
.Ltmp321:
	#DEBUG_VALUE: b2 <- $r0
	.loc	2 0 21 is_stmt 0                # ./ebpf-decompress/src/bpf/packing.bpf.c:0:21
	r6 = r0
.Ltmp322:
.Ltmp323:
	r6 >>= 6
	.loc	2 88 21 is_stmt 1               # ./ebpf-decompress/src/bpf/packing.bpf.c:88:21
.Ltmp324:
	*(u32 *)(r5 + 0) = r6
	r6 = r0
	r6 >>= 2
	r6 &= 3
.Ltmp325:
.Ltmp326:
	#DEBUG_VALUE: elem_3 <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 64 7, DW_OP_LLVM_convert 32 7, DW_OP_plus, DW_OP_constu 4, DW_OP_shl, DW_OP_LLVM_convert 32 7, DW_OP_LLVM_convert 64 7, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_plus_uconst 8, DW_OP_stack_value] undef
	.loc	2 92 21                         # ./ebpf-decompress/src/bpf/packing.bpf.c:92:21
.Ltmp327:
	*(u32 *)(r5 + 8) = r6
.Ltmp328:
.Ltmp329:
	#DEBUG_VALUE: b3 <- undef
	.loc	2 0 21 is_stmt 0                # ./ebpf-decompress/src/bpf/packing.bpf.c:0:21
	r0 >>= 4
.Ltmp330:
.Ltmp331:
	r0 &= 3
.Ltmp332:
.Ltmp333:
	#DEBUG_VALUE: elem_2 <- [DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_arg 0, DW_OP_LLVM_convert 64 7, DW_OP_LLVM_convert 32 7, DW_OP_plus, DW_OP_constu 4, DW_OP_shl, DW_OP_LLVM_convert 32 7, DW_OP_LLVM_convert 64 7, DW_OP_constu 1, DW_OP_mul, DW_OP_plus, DW_OP_plus_uconst 4, DW_OP_stack_value] undef
	.loc	2 90 21 is_stmt 1               # ./ebpf-decompress/src/bpf/packing.bpf.c:90:21
.Ltmp334:
	*(u32 *)(r5 + 4) = r0
	r0 = 0
.Ltmp335:
.Ltmp336:
	.loc	2 62 35                         # ./ebpf-decompress/src/bpf/packing.bpf.c:62:35
.Ltmp337:
	r4 += 16
	.loc	2 62 58 is_stmt 0               # ./ebpf-decompress/src/bpf/packing.bpf.c:62:58
.Ltmp338:
	r3 += 1
.Ltmp339:
.Ltmp340:
	#DEBUG_VALUE: inner_i <- $r3
	.loc	2 62 5                          # ./ebpf-decompress/src/bpf/packing.bpf.c:62:5
.Ltmp341:
	if r3 != 1024 goto LBB4_1
.Ltmp342:
.Ltmp343:
LBB4_3:
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 99 1 is_stmt 1                # ./ebpf-decompress/src/bpf/packing.bpf.c:99:1
.Ltmp344:
	exit
.Ltmp345:
.Ltmp346:
.Lfunc_end4:
	.size	decode_one, .Lfunc_end4-decode_one
	.cfi_endproc
                                        # -- End function
	.type	__bpf_usdt_specs,@object        # @__bpf_usdt_specs
	.section	.maps,"aw",@progbits
	.weak	__bpf_usdt_specs
	.p2align	3
__bpf_usdt_specs:
	.zero	32
	.size	__bpf_usdt_specs, 32

	.type	IN_SIZE,@object                 # @IN_SIZE
	.section	.rodata,"a",@progbits
	.globl	IN_SIZE
	.p2align	2, 0x0
IN_SIZE:
	.long	8388608                         # 0x800000
	.size	IN_SIZE, 4

	.type	OUT_SIZE,@object                # @OUT_SIZE
	.globl	OUT_SIZE
	.p2align	2, 0x0
OUT_SIZE:
	.long	134217728                       # 0x8000000
	.size	OUT_SIZE, 4

	.type	LOOP_FACTOR,@object             # @LOOP_FACTOR
	.globl	LOOP_FACTOR
	.p2align	2, 0x0
LOOP_FACTOR:
	.long	1024                            # 0x400
	.size	LOOP_FACTOR, 4

	.type	in_bytes,@object                # @in_bytes
	.section	.maps,"aw",@progbits
	.globl	in_bytes
	.p2align	3
in_bytes:
	.zero	40
	.size	in_bytes, 40

	.type	out_bytes,@object               # @out_bytes
	.globl	out_bytes
	.p2align	3
out_bytes:
	.zero	40
	.size	out_bytes, 40

	.type	_license,@object                # @_license
	.section	license,"aw",@progbits
	.globl	_license
_license:
	.asciz	"GPL"
	.size	_license, 4

	.type	__bpf_usdt_ip_to_spec_id,@object # @__bpf_usdt_ip_to_spec_id
	.section	.maps,"aw",@progbits
	.weak	__bpf_usdt_ip_to_spec_id
	.p2align	3
__bpf_usdt_ip_to_spec_id:
	.zero	32
	.size	__bpf_usdt_ip_to_spec_id, 32

	.section	.debug_loclists,"",@progbits
	.long	.Ldebug_list_header_end0-.Ldebug_list_header_start0 # Length
.Ldebug_list_header_start0:
	.short	5                               # Version
	.byte	8                               # Address size
	.byte	0                               # Segment selector size
	.long	35                              # Offset entry count
.Lloclists_table_base0:
	.long	.Ldebug_loc0-.Lloclists_table_base0
	.long	.Ldebug_loc1-.Lloclists_table_base0
	.long	.Ldebug_loc2-.Lloclists_table_base0
	.long	.Ldebug_loc3-.Lloclists_table_base0
	.long	.Ldebug_loc4-.Lloclists_table_base0
	.long	.Ldebug_loc5-.Lloclists_table_base0
	.long	.Ldebug_loc6-.Lloclists_table_base0
	.long	.Ldebug_loc7-.Lloclists_table_base0
	.long	.Ldebug_loc8-.Lloclists_table_base0
	.long	.Ldebug_loc9-.Lloclists_table_base0
	.long	.Ldebug_loc10-.Lloclists_table_base0
	.long	.Ldebug_loc11-.Lloclists_table_base0
	.long	.Ldebug_loc12-.Lloclists_table_base0
	.long	.Ldebug_loc13-.Lloclists_table_base0
	.long	.Ldebug_loc14-.Lloclists_table_base0
	.long	.Ldebug_loc15-.Lloclists_table_base0
	.long	.Ldebug_loc16-.Lloclists_table_base0
	.long	.Ldebug_loc17-.Lloclists_table_base0
	.long	.Ldebug_loc18-.Lloclists_table_base0
	.long	.Ldebug_loc19-.Lloclists_table_base0
	.long	.Ldebug_loc20-.Lloclists_table_base0
	.long	.Ldebug_loc21-.Lloclists_table_base0
	.long	.Ldebug_loc22-.Lloclists_table_base0
	.long	.Ldebug_loc23-.Lloclists_table_base0
	.long	.Ldebug_loc24-.Lloclists_table_base0
	.long	.Ldebug_loc25-.Lloclists_table_base0
	.long	.Ldebug_loc26-.Lloclists_table_base0
	.long	.Ldebug_loc27-.Lloclists_table_base0
	.long	.Ldebug_loc28-.Lloclists_table_base0
	.long	.Ldebug_loc29-.Lloclists_table_base0
	.long	.Ldebug_loc30-.Lloclists_table_base0
	.long	.Ldebug_loc31-.Lloclists_table_base0
	.long	.Ldebug_loc32-.Lloclists_table_base0
	.long	.Ldebug_loc33-.Lloclists_table_base0
	.long	.Ldebug_loc34-.Lloclists_table_base0
.Ldebug_loc0:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin0-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp8-.Lfunc_begin0           #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp22-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp25-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc1:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp8-.Lfunc_begin0           #   starting offset
	.uleb128 .Ltmp11-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp11-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp22-.Lfunc_begin0          #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	8                               # 8
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc2:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp16-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp22-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc3:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp25-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp28-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp31-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp41-.Lfunc_begin0          #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	4                               # 4
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc4:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp35-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp41-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc5:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin1-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp48-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp48-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp128-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	89                              # DW_OP_reg9
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp136-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp140-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	89                              # DW_OP_reg9
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc6:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin1-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp46-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	82                              # DW_OP_reg2
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp46-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp168-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	87                              # DW_OP_reg7
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc7:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin1-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp50-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp50-.Lfunc_begin0          #   starting offset
	.uleb128 .Lfunc_end1-.Lfunc_begin0      #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	0                               # 0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc8:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp63-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp66-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp66-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp77-.Lfunc_begin0          #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	16                              # 16
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc9:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp71-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp77-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc10:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp80-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp83-.Lfunc_begin0          #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp86-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp191-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	12                              # 12
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc11:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp91-.Lfunc_begin0          #   starting offset
	.uleb128 .Ltmp170-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	88                              # DW_OP_reg8
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc12:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp108-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp125-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	82                              # DW_OP_reg2
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp136-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp144-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	82                              # DW_OP_reg2
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc13:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp119-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp122-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp128-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp136-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	16                              # 16
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp140-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp154-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	16                              # 16
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp154-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp157-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp157-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp165-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	16                              # 16
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp173-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp186-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	82                              # DW_OP_reg2
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp188-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp191-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc14:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp133-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp136-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp147-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp165-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc15:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin2-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp204-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp218-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp221-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc16:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp204-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp207-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp207-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp218-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	8                               # 8
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc17:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp212-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp218-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc18:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp221-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp224-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp227-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp237-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	4                               # 4
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc19:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp231-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp237-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc20:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin3-.Lfunc_begin3    #   starting offset
	.uleb128 .Ltmp242-.Lfunc_begin3         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp242-.Lfunc_begin3         #   starting offset
	.uleb128 .Ltmp245-.Lfunc_begin3         #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc21:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp255-.Lfunc_begin3         #   starting offset
	.uleb128 .Ltmp258-.Lfunc_begin3         #   ending offset
	.byte	2                               # Loc expr size
	.byte	48                              # DW_OP_lit0
	.byte	159                             # DW_OP_stack_value
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp258-.Lfunc_begin3         #   starting offset
	.uleb128 .Lfunc_end3-.Lfunc_begin3      #   ending offset
	.byte	2                               # Loc expr size
	.byte	122                             # DW_OP_breg10
	.byte	20                              # 20
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc22:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp263-.Lfunc_begin3         #   starting offset
	.uleb128 .Lfunc_end3-.Lfunc_begin3      #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc23:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp266-.Lfunc_begin3         #   starting offset
	.uleb128 .Ltmp279-.Lfunc_begin3         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc24:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin4-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp290-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	81                              # DW_OP_reg1
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc25:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Lfunc_begin4-.Lfunc_begin0    #   starting offset
	.uleb128 .Ltmp293-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	48                              # DW_OP_lit0
	.byte	159                             # DW_OP_stack_value
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp293-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp342-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc26:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp293-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp339-.Lfunc_begin0         #   ending offset
	.byte	6                               # Loc expr size
	.byte	115                             # DW_OP_breg3
	.byte	0                               # 0
	.byte	113                             # DW_OP_breg1
	.byte	0                               # 0
	.byte	34                              # DW_OP_plus
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc27:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc28:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	15                              # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	16                              # DW_OP_constu
	.byte	192                             # 18446744073709551552
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	255                             # 
	.byte	1                               # 
	.byte	26                              # DW_OP_and
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc29:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	6                               # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	16                              # DW_OP_constu
	.byte	48                              # 48
	.byte	26                              # DW_OP_and
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc30:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	5                               # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	60                              # DW_OP_lit12
	.byte	26                              # DW_OP_and
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc31:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	15                              # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	168                             # DW_OP_convert
	.asciz	"\257\200\200"                  # 
	.byte	168                             # DW_OP_convert
	.asciz	"\263\200\200"                  # 
	.byte	54                              # DW_OP_lit6
	.byte	37                              # DW_OP_shr
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc32:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp320-.Lfunc_begin0         #   ending offset
	.byte	18                              # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	16                              # DW_OP_constu
	.byte	48                              # 48
	.byte	26                              # DW_OP_and
	.byte	168                             # DW_OP_convert
	.asciz	"\257\200\200"                  # 
	.byte	168                             # DW_OP_convert
	.asciz	"\263\200\200"                  # 
	.byte	52                              # DW_OP_lit4
	.byte	37                              # DW_OP_shr
	.byte	159                             # DW_OP_stack_value
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp320-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp330-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc33:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp314-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp328-.Lfunc_begin0         #   ending offset
	.byte	17                              # Loc expr size
	.byte	112                             # DW_OP_breg0
	.byte	0                               # 0
	.byte	60                              # DW_OP_lit12
	.byte	26                              # DW_OP_and
	.byte	168                             # DW_OP_convert
	.asciz	"\257\200\200"                  # 
	.byte	168                             # DW_OP_convert
	.asciz	"\263\200\200"                  # 
	.byte	50                              # DW_OP_lit2
	.byte	37                              # DW_OP_shr
	.byte	159                             # DW_OP_stack_value
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc34:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp317-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp322-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_list_header_end0:
	.section	.debug_abbrev,"",@progbits
	.byte	1                               # Abbreviation Code
	.byte	17                              # DW_TAG_compile_unit
	.byte	1                               # DW_CHILDREN_yes
	.byte	37                              # DW_AT_producer
	.byte	37                              # DW_FORM_strx1
	.byte	19                              # DW_AT_language
	.byte	5                               # DW_FORM_data2
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	114                             # DW_AT_str_offsets_base
	.byte	23                              # DW_FORM_sec_offset
	.byte	16                              # DW_AT_stmt_list
	.byte	23                              # DW_FORM_sec_offset
	.byte	27                              # DW_AT_comp_dir
	.byte	37                              # DW_FORM_strx1
	.byte	17                              # DW_AT_low_pc
	.byte	1                               # DW_FORM_addr
	.byte	85                              # DW_AT_ranges
	.byte	35                              # DW_FORM_rnglistx
	.byte	115                             # DW_AT_addr_base
	.byte	23                              # DW_FORM_sec_offset
	.byte	116                             # DW_AT_rnglists_base
	.byte	23                              # DW_FORM_sec_offset
	.ascii	"\214\001"                      # DW_AT_loclists_base
	.byte	23                              # DW_FORM_sec_offset
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	2                               # Abbreviation Code
	.byte	36                              # DW_TAG_base_type
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	62                              # DW_AT_encoding
	.byte	11                              # DW_FORM_data1
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	3                               # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	63                              # DW_AT_external
	.byte	25                              # DW_FORM_flag_present
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	2                               # DW_AT_location
	.byte	24                              # DW_FORM_exprloc
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	4                               # Abbreviation Code
	.byte	38                              # DW_TAG_const_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	5                               # Abbreviation Code
	.byte	22                              # DW_TAG_typedef
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	6                               # Abbreviation Code
	.byte	1                               # DW_TAG_array_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	7                               # Abbreviation Code
	.byte	33                              # DW_TAG_subrange_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	55                              # DW_AT_count
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	8                               # Abbreviation Code
	.byte	36                              # DW_TAG_base_type
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	62                              # DW_AT_encoding
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	9                               # Abbreviation Code
	.byte	19                              # DW_TAG_structure_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	10                              # Abbreviation Code
	.byte	13                              # DW_TAG_member
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	56                              # DW_AT_data_member_location
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	11                              # Abbreviation Code
	.byte	15                              # DW_TAG_pointer_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	12                              # Abbreviation Code
	.byte	33                              # DW_TAG_subrange_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	55                              # DW_AT_count
	.byte	5                               # DW_FORM_data2
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	13                              # Abbreviation Code
	.byte	19                              # DW_TAG_structure_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	14                              # Abbreviation Code
	.byte	4                               # DW_TAG_enumeration_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	15                              # Abbreviation Code
	.byte	40                              # DW_TAG_enumerator
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	28                              # DW_AT_const_value
	.byte	15                              # DW_FORM_udata
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	16                              # Abbreviation Code
	.byte	33                              # DW_TAG_subrange_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	55                              # DW_AT_count
	.byte	6                               # DW_FORM_data4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	17                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	63                              # DW_AT_external
	.byte	25                              # DW_FORM_flag_present
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	60                              # DW_AT_declaration
	.byte	25                              # DW_FORM_flag_present
	.byte	2                               # DW_AT_location
	.byte	24                              # DW_FORM_exprloc
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	18                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	5                               # DW_FORM_data2
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	19                              # Abbreviation Code
	.byte	21                              # DW_TAG_subroutine_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	39                              # DW_AT_prototyped
	.byte	25                              # DW_FORM_flag_present
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	20                              # Abbreviation Code
	.byte	5                               # DW_TAG_formal_parameter
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	21                              # Abbreviation Code
	.byte	15                              # DW_TAG_pointer_type
	.byte	0                               # DW_CHILDREN_no
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	22                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	23                              # Abbreviation Code
	.byte	38                              # DW_TAG_const_type
	.byte	0                               # DW_CHILDREN_no
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	24                              # Abbreviation Code
	.byte	19                              # DW_TAG_structure_type
	.byte	1                               # DW_CHILDREN_yes
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	11                              # DW_AT_byte_size
	.byte	11                              # DW_FORM_data1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	5                               # DW_FORM_data2
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	25                              # Abbreviation Code
	.byte	13                              # DW_TAG_member
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	5                               # DW_FORM_data2
	.byte	56                              # DW_AT_data_member_location
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	26                              # Abbreviation Code
	.byte	46                              # DW_TAG_subprogram
	.byte	1                               # DW_CHILDREN_yes
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	39                              # DW_AT_prototyped
	.byte	25                              # DW_FORM_flag_present
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	32                              # DW_AT_inline
	.byte	33                              # DW_FORM_implicit_const
	.byte	1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	27                              # Abbreviation Code
	.byte	5                               # DW_TAG_formal_parameter
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	28                              # Abbreviation Code
	.byte	11                              # DW_TAG_lexical_block
	.byte	1                               # DW_CHILDREN_yes
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	29                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	30                              # Abbreviation Code
	.byte	46                              # DW_TAG_subprogram
	.byte	1                               # DW_CHILDREN_yes
	.byte	17                              # DW_AT_low_pc
	.byte	27                              # DW_FORM_addrx
	.byte	18                              # DW_AT_high_pc
	.byte	6                               # DW_FORM_data4
	.byte	64                              # DW_AT_frame_base
	.byte	24                              # DW_FORM_exprloc
	.byte	122                             # DW_AT_call_all_calls
	.byte	25                              # DW_FORM_flag_present
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	39                              # DW_AT_prototyped
	.byte	25                              # DW_FORM_flag_present
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	63                              # DW_AT_external
	.byte	25                              # DW_FORM_flag_present
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	31                              # Abbreviation Code
	.byte	5                               # DW_TAG_formal_parameter
	.byte	0                               # DW_CHILDREN_no
	.byte	2                               # DW_AT_location
	.byte	34                              # DW_FORM_loclistx
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	32                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	2                               # DW_AT_location
	.byte	34                              # DW_FORM_loclistx
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	33                              # Abbreviation Code
	.byte	29                              # DW_TAG_inlined_subroutine
	.byte	1                               # DW_CHILDREN_yes
	.byte	49                              # DW_AT_abstract_origin
	.byte	19                              # DW_FORM_ref4
	.byte	17                              # DW_AT_low_pc
	.byte	27                              # DW_FORM_addrx
	.byte	18                              # DW_AT_high_pc
	.byte	6                               # DW_FORM_data4
	.byte	88                              # DW_AT_call_file
	.byte	11                              # DW_FORM_data1
	.byte	89                              # DW_AT_call_line
	.byte	11                              # DW_FORM_data1
	.byte	87                              # DW_AT_call_column
	.byte	11                              # DW_FORM_data1
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	34                              # Abbreviation Code
	.byte	11                              # DW_TAG_lexical_block
	.byte	1                               # DW_CHILDREN_yes
	.byte	17                              # DW_AT_low_pc
	.byte	27                              # DW_FORM_addrx
	.byte	18                              # DW_AT_high_pc
	.byte	6                               # DW_FORM_data4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	35                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	2                               # DW_AT_location
	.byte	34                              # DW_FORM_loclistx
	.byte	49                              # DW_AT_abstract_origin
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	36                              # Abbreviation Code
	.byte	52                              # DW_TAG_variable
	.byte	0                               # DW_CHILDREN_no
	.byte	2                               # DW_AT_location
	.byte	24                              # DW_FORM_exprloc
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	37                              # Abbreviation Code
	.byte	46                              # DW_TAG_subprogram
	.byte	1                               # DW_CHILDREN_yes
	.byte	17                              # DW_AT_low_pc
	.byte	27                              # DW_FORM_addrx
	.byte	18                              # DW_AT_high_pc
	.byte	6                               # DW_FORM_data4
	.byte	64                              # DW_AT_frame_base
	.byte	24                              # DW_FORM_exprloc
	.byte	122                             # DW_AT_call_all_calls
	.byte	25                              # DW_FORM_flag_present
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	39                              # DW_AT_prototyped
	.byte	25                              # DW_FORM_flag_present
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	38                              # Abbreviation Code
	.byte	5                               # DW_TAG_formal_parameter
	.byte	0                               # DW_CHILDREN_no
	.byte	2                               # DW_AT_location
	.byte	24                              # DW_FORM_exprloc
	.byte	3                               # DW_AT_name
	.byte	37                              # DW_FORM_strx1
	.byte	58                              # DW_AT_decl_file
	.byte	11                              # DW_FORM_data1
	.byte	59                              # DW_AT_decl_line
	.byte	11                              # DW_FORM_data1
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	39                              # Abbreviation Code
	.byte	11                              # DW_TAG_lexical_block
	.byte	1                               # DW_CHILDREN_yes
	.byte	85                              # DW_AT_ranges
	.byte	35                              # DW_FORM_rnglistx
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	0                               # EOM(3)
	.section	.debug_info,"",@progbits
.Lcu_begin0:
	.long	.Ldebug_info_end0-.Ldebug_info_start0 # Length of Unit
.Ldebug_info_start0:
	.short	5                               # DWARF version number
	.byte	1                               # DWARF Unit Type
	.byte	8                               # Address Size (in bytes)
	.long	.debug_abbrev                   # Offset Into Abbrev. Section
	.byte	1                               # Abbrev [1] 0xc:0x6a4 DW_TAG_compile_unit
	.byte	0                               # DW_AT_producer
	.short	29                              # DW_AT_language
	.byte	1                               # DW_AT_name
	.long	.Lstr_offsets_base0             # DW_AT_str_offsets_base
	.long	.Lline_table_start0             # DW_AT_stmt_list
	.byte	2                               # DW_AT_comp_dir
	.quad	0                               # DW_AT_low_pc
	.byte	2                               # DW_AT_ranges
	.long	.Laddr_table_base0              # DW_AT_addr_base
	.long	.Lrnglists_table_base0          # DW_AT_rnglists_base
	.long	.Lloclists_table_base0          # DW_AT_loclists_base
	.byte	2                               # Abbrev [2] 0x2f:0x4 DW_TAG_base_type
	.byte	79                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	2                               # Abbrev [2] 0x33:0x4 DW_TAG_base_type
	.byte	78                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	4                               # DW_AT_byte_size
	.byte	3                               # Abbrev [3] 0x37:0xb DW_TAG_variable
	.byte	3                               # DW_AT_name
	.long	66                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	6                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	0
	.byte	4                               # Abbrev [4] 0x42:0x5 DW_TAG_const_type
	.long	71                              # DW_AT_type
	.byte	5                               # Abbrev [5] 0x47:0x8 DW_TAG_typedef
	.long	79                              # DW_AT_type
	.byte	6                               # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	26                              # DW_AT_decl_line
	.byte	5                               # Abbrev [5] 0x4f:0x8 DW_TAG_typedef
	.long	87                              # DW_AT_type
	.byte	5                               # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	14                              # DW_AT_decl_line
	.byte	2                               # Abbrev [2] 0x57:0x4 DW_TAG_base_type
	.byte	4                               # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	4                               # DW_AT_byte_size
	.byte	3                               # Abbrev [3] 0x5b:0xb DW_TAG_variable
	.byte	7                               # DW_AT_name
	.long	66                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	7                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	1
	.byte	3                               # Abbrev [3] 0x66:0xb DW_TAG_variable
	.byte	8                               # DW_AT_name
	.long	66                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	8                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	2
	.byte	3                               # Abbrev [3] 0x71:0xb DW_TAG_variable
	.byte	9                               # DW_AT_name
	.long	124                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	101                             # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	3
	.byte	6                               # Abbrev [6] 0x7c:0xc DW_TAG_array_type
	.long	136                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x81:0x6 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.byte	4                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	2                               # Abbrev [2] 0x88:0x4 DW_TAG_base_type
	.byte	10                              # DW_AT_name
	.byte	6                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	8                               # Abbrev [8] 0x8c:0x4 DW_TAG_base_type
	.byte	11                              # DW_AT_name
	.byte	8                               # DW_AT_byte_size
	.byte	7                               # DW_AT_encoding
	.byte	3                               # Abbrev [3] 0x90:0xb DW_TAG_variable
	.byte	12                              # DW_AT_name
	.long	155                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	68                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	4
	.byte	9                               # Abbrev [9] 0x9b:0x29 DW_TAG_structure_type
	.byte	32                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	63                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x9f:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	196                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xa8:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	217                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	65                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xb1:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	235                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	66                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xba:0x9 DW_TAG_member
	.byte	17                              # DW_AT_name
	.long	240                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	67                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0xc4:0x5 DW_TAG_pointer_type
	.long	201                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0xc9:0xc DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0xce:0x6 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.byte	2                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	2                               # Abbrev [2] 0xd5:0x4 DW_TAG_base_type
	.byte	14                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	4                               # DW_AT_byte_size
	.byte	11                              # Abbrev [11] 0xd9:0x5 DW_TAG_pointer_type
	.long	222                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0xde:0xd DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0xe3:0x7 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.short	256                             # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0xeb:0x5 DW_TAG_pointer_type
	.long	213                             # DW_AT_type
	.byte	11                              # Abbrev [11] 0xf0:0x5 DW_TAG_pointer_type
	.long	245                             # DW_AT_type
	.byte	13                              # Abbrev [13] 0xf5:0x21 DW_TAG_structure_type
	.byte	36                              # DW_AT_name
	.byte	208                             # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	57                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0xfa:0x9 DW_TAG_member
	.byte	18                              # DW_AT_name
	.long	278                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	58                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x103:0x9 DW_TAG_member
	.byte	34                              # DW_AT_name
	.long	341                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	59                              # DW_AT_decl_line
	.byte	192                             # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x10c:0x9 DW_TAG_member
	.byte	35                              # DW_AT_name
	.long	372                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	60                              # DW_AT_decl_line
	.byte	200                             # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	6                               # Abbrev [6] 0x116:0xc DW_TAG_array_type
	.long	290                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x11b:0x6 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.byte	12                              # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	13                              # Abbrev [13] 0x122:0x33 DW_TAG_structure_type
	.byte	33                              # DW_AT_name
	.byte	16                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	39                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x127:0x9 DW_TAG_member
	.byte	19                              # DW_AT_name
	.long	341                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	41                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x130:0x9 DW_TAG_member
	.byte	22                              # DW_AT_name
	.long	353                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	43                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x139:0x9 DW_TAG_member
	.byte	27                              # DW_AT_name
	.long	372                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	45                              # DW_AT_decl_line
	.byte	12                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x142:0x9 DW_TAG_member
	.byte	29                              # DW_AT_name
	.long	376                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	47                              # DW_AT_decl_line
	.byte	14                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x14b:0x9 DW_TAG_member
	.byte	32                              # DW_AT_name
	.long	136                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	52                              # DW_AT_decl_line
	.byte	15                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	5                               # Abbrev [5] 0x155:0x8 DW_TAG_typedef
	.long	349                             # DW_AT_type
	.byte	21                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	18                              # DW_AT_decl_line
	.byte	2                               # Abbrev [2] 0x15d:0x4 DW_TAG_base_type
	.byte	20                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	14                              # Abbrev [14] 0x161:0x13 DW_TAG_enumeration_type
	.long	87                              # DW_AT_type
	.byte	26                              # DW_AT_name
	.byte	4                               # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	33                              # DW_AT_decl_line
	.byte	15                              # Abbrev [15] 0x16a:0x3 DW_TAG_enumerator
	.byte	23                              # DW_AT_name
	.byte	0                               # DW_AT_const_value
	.byte	15                              # Abbrev [15] 0x16d:0x3 DW_TAG_enumerator
	.byte	24                              # DW_AT_name
	.byte	1                               # DW_AT_const_value
	.byte	15                              # Abbrev [15] 0x170:0x3 DW_TAG_enumerator
	.byte	25                              # DW_AT_name
	.byte	2                               # DW_AT_const_value
	.byte	0                               # End Of Children Mark
	.byte	2                               # Abbrev [2] 0x174:0x4 DW_TAG_base_type
	.byte	28                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	2                               # DW_AT_byte_size
	.byte	5                               # Abbrev [5] 0x178:0x8 DW_TAG_typedef
	.long	384                             # DW_AT_type
	.byte	31                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	54                              # DW_AT_decl_line
	.byte	2                               # Abbrev [2] 0x180:0x4 DW_TAG_base_type
	.byte	30                              # DW_AT_name
	.byte	2                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	3                               # Abbrev [3] 0x184:0xb DW_TAG_variable
	.byte	37                              # DW_AT_name
	.long	399                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	75                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	5
	.byte	9                               # Abbrev [9] 0x18f:0x29 DW_TAG_structure_type
	.byte	32                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	70                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x193:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	440                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	71                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x19c:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	457                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	72                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x1a5:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	475                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	73                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x1ae:0x9 DW_TAG_member
	.byte	17                              # DW_AT_name
	.long	484                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	74                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1b8:0x5 DW_TAG_pointer_type
	.long	445                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x1bd:0xc DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x1c2:0x6 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.byte	1                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1c9:0x5 DW_TAG_pointer_type
	.long	462                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x1ce:0xd DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0x1d3:0x7 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.short	1024                            # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1db:0x5 DW_TAG_pointer_type
	.long	480                             # DW_AT_type
	.byte	2                               # Abbrev [2] 0x1e0:0x4 DW_TAG_base_type
	.byte	38                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	11                              # Abbrev [11] 0x1e4:0x5 DW_TAG_pointer_type
	.long	79                              # DW_AT_type
	.byte	3                               # Abbrev [3] 0x1e9:0xb DW_TAG_variable
	.byte	39                              # DW_AT_name
	.long	500                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	18                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	6
	.byte	9                               # Abbrev [9] 0x1f4:0x32 DW_TAG_structure_type
	.byte	40                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	10                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x1f8:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	196                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	12                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x201:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	550                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	13                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x20a:0x9 DW_TAG_member
	.byte	40                              # DW_AT_name
	.long	555                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	14                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x213:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	440                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	15                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x21c:0x9 DW_TAG_member
	.byte	41                              # DW_AT_name
	.long	575                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	16                              # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x226:0x5 DW_TAG_pointer_type
	.long	71                              # DW_AT_type
	.byte	11                              # Abbrev [11] 0x22b:0x5 DW_TAG_pointer_type
	.long	560                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x230:0xf DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	16                              # Abbrev [16] 0x235:0x9 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.long	8388608                         # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x23f:0x5 DW_TAG_pointer_type
	.long	580                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x244:0xd DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0x249:0x7 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.short	1152                            # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	3                               # Abbrev [3] 0x251:0xb DW_TAG_variable
	.byte	42                              # DW_AT_name
	.long	604                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	27                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	7
	.byte	9                               # Abbrev [9] 0x25c:0x32 DW_TAG_structure_type
	.byte	40                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	20                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x260:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	196                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	22                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x269:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	550                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	23                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x272:0x9 DW_TAG_member
	.byte	40                              # DW_AT_name
	.long	654                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	24                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x27b:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	440                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	25                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x284:0x9 DW_TAG_member
	.byte	41                              # DW_AT_name
	.long	457                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	26                              # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x28e:0x5 DW_TAG_pointer_type
	.long	659                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x293:0xf DW_TAG_array_type
	.long	213                             # DW_AT_type
	.byte	16                              # Abbrev [16] 0x298:0x9 DW_TAG_subrange_type
	.long	140                             # DW_AT_type
	.long	134217728                       # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	17                              # Abbrev [17] 0x2a2:0xb DW_TAG_variable
	.byte	43                              # DW_AT_name
	.long	685                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	77                              # DW_AT_decl_line
                                        # DW_AT_declaration
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	8
	.byte	4                               # Abbrev [4] 0x2ad:0x5 DW_TAG_const_type
	.long	384                             # DW_AT_type
	.byte	18                              # Abbrev [18] 0x2b2:0x9 DW_TAG_variable
	.byte	44                              # DW_AT_name
	.long	699                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	4087                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2bb:0x5 DW_TAG_pointer_type
	.long	704                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2c0:0xb DW_TAG_subroutine_type
	.long	341                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x2c5:0x5 DW_TAG_formal_parameter
	.long	715                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	21                              # Abbrev [21] 0x2cb:0x1 DW_TAG_pointer_type
	.byte	22                              # Abbrev [22] 0x2cc:0x8 DW_TAG_variable
	.byte	45                              # DW_AT_name
	.long	724                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.byte	56                              # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2d4:0x5 DW_TAG_pointer_type
	.long	729                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2d9:0x10 DW_TAG_subroutine_type
	.long	715                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x2de:0x5 DW_TAG_formal_parameter
	.long	715                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x2e3:0x5 DW_TAG_formal_parameter
	.long	745                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x2e9:0x5 DW_TAG_pointer_type
	.long	750                             # DW_AT_type
	.byte	23                              # Abbrev [23] 0x2ee:0x1 DW_TAG_const_type
	.byte	18                              # Abbrev [18] 0x2ef:0x9 DW_TAG_variable
	.byte	46                              # DW_AT_name
	.long	760                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	2796                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2f8:0x5 DW_TAG_pointer_type
	.long	765                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2fd:0x15 DW_TAG_subroutine_type
	.long	480                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x302:0x5 DW_TAG_formal_parameter
	.long	715                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x307:0x5 DW_TAG_formal_parameter
	.long	79                              # DW_AT_type
	.byte	20                              # Abbrev [20] 0x30c:0x5 DW_TAG_formal_parameter
	.long	745                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	18                              # Abbrev [18] 0x312:0x9 DW_TAG_variable
	.byte	47                              # DW_AT_name
	.long	760                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	2785                            # DW_AT_decl_line
	.byte	18                              # Abbrev [18] 0x31b:0x9 DW_TAG_variable
	.byte	48                              # DW_AT_name
	.long	804                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	4216                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x324:0x5 DW_TAG_pointer_type
	.long	809                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x329:0x1a DW_TAG_subroutine_type
	.long	480                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x32e:0x5 DW_TAG_formal_parameter
	.long	79                              # DW_AT_type
	.byte	20                              # Abbrev [20] 0x333:0x5 DW_TAG_formal_parameter
	.long	715                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x338:0x5 DW_TAG_formal_parameter
	.long	715                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x33d:0x5 DW_TAG_formal_parameter
	.long	341                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x343:0x5 DW_TAG_pointer_type
	.long	840                             # DW_AT_type
	.byte	24                              # Abbrev [24] 0x348:0xd9 DW_TAG_structure_type
	.byte	71                              # DW_AT_name
	.byte	168                             # DW_AT_byte_size
	.byte	1                               # DW_AT_decl_file
	.short	1070                            # DW_AT_decl_line
	.byte	25                              # Abbrev [25] 0x34e:0xa DW_TAG_member
	.byte	49                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1071                            # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x358:0xa DW_TAG_member
	.byte	51                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1072                            # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x362:0xa DW_TAG_member
	.byte	52                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1073                            # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x36c:0xa DW_TAG_member
	.byte	53                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1074                            # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x376:0xa DW_TAG_member
	.byte	54                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1075                            # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x380:0xa DW_TAG_member
	.byte	55                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1076                            # DW_AT_decl_line
	.byte	40                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x38a:0xa DW_TAG_member
	.byte	56                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1077                            # DW_AT_decl_line
	.byte	48                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x394:0xa DW_TAG_member
	.byte	57                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1078                            # DW_AT_decl_line
	.byte	56                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x39e:0xa DW_TAG_member
	.byte	58                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1079                            # DW_AT_decl_line
	.byte	64                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3a8:0xa DW_TAG_member
	.byte	59                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1080                            # DW_AT_decl_line
	.byte	72                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3b2:0xa DW_TAG_member
	.byte	60                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1081                            # DW_AT_decl_line
	.byte	80                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3bc:0xa DW_TAG_member
	.byte	61                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1082                            # DW_AT_decl_line
	.byte	88                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3c6:0xa DW_TAG_member
	.byte	62                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1083                            # DW_AT_decl_line
	.byte	96                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3d0:0xa DW_TAG_member
	.byte	63                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1084                            # DW_AT_decl_line
	.byte	104                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3da:0xa DW_TAG_member
	.byte	64                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1085                            # DW_AT_decl_line
	.byte	112                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3e4:0xa DW_TAG_member
	.byte	65                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1086                            # DW_AT_decl_line
	.byte	120                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3ee:0xa DW_TAG_member
	.byte	66                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1087                            # DW_AT_decl_line
	.byte	128                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3f8:0xa DW_TAG_member
	.byte	67                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1088                            # DW_AT_decl_line
	.byte	136                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x402:0xa DW_TAG_member
	.byte	68                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1089                            # DW_AT_decl_line
	.byte	144                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x40c:0xa DW_TAG_member
	.byte	69                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1090                            # DW_AT_decl_line
	.byte	152                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x416:0xa DW_TAG_member
	.byte	70                              # DW_AT_name
	.long	1057                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1091                            # DW_AT_decl_line
	.byte	160                             # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	2                               # Abbrev [2] 0x421:0x4 DW_TAG_base_type
	.byte	50                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	11                              # Abbrev [11] 0x425:0x5 DW_TAG_pointer_type
	.long	1066                            # DW_AT_type
	.byte	5                               # Abbrev [5] 0x42a:0x8 DW_TAG_typedef
	.long	1074                            # DW_AT_type
	.byte	74                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	20                              # DW_AT_decl_line
	.byte	5                               # Abbrev [5] 0x432:0x8 DW_TAG_typedef
	.long	1082                            # DW_AT_type
	.byte	73                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	8                               # DW_AT_decl_line
	.byte	2                               # Abbrev [2] 0x43a:0x4 DW_TAG_base_type
	.byte	72                              # DW_AT_name
	.byte	8                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	26                              # Abbrev [26] 0x43e:0x23 DW_TAG_subprogram
	.byte	75                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	80                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	213                             # DW_AT_type
                                        # DW_AT_inline
	.byte	27                              # Abbrev [27] 0x446:0x8 DW_TAG_formal_parameter
	.byte	76                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	80                              # DW_AT_decl_line
	.long	835                             # DW_AT_type
	.byte	28                              # Abbrev [28] 0x44e:0x12 DW_TAG_lexical_block
	.byte	29                              # Abbrev [29] 0x44f:0x8 DW_TAG_variable
	.byte	66                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	83                              # DW_AT_decl_line
	.long	480                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x457:0x8 DW_TAG_variable
	.byte	77                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	84                              # DW_AT_decl_line
	.long	235                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x461:0x4c DW_TAG_subprogram
	.byte	9                               # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	80                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	95                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	213                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x470:0x9 DW_TAG_formal_parameter
	.byte	0                               # DW_AT_location
	.byte	76                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	95                              # DW_AT_decl_line
	.long	835                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x479:0x9 DW_TAG_variable
	.byte	3                               # DW_AT_location
	.byte	85                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	98                              # DW_AT_decl_line
	.long	213                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x482:0x9 DW_TAG_variable
	.byte	4                               # DW_AT_location
	.byte	86                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	97                              # DW_AT_decl_line
	.long	240                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x48b:0x21 DW_TAG_inlined_subroutine
	.long	1086                            # DW_AT_abstract_origin
	.byte	9                               # DW_AT_low_pc
	.long	.Ltmp25-.Lfunc_begin0           # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	100                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x498:0x13 DW_TAG_lexical_block
	.byte	10                              # DW_AT_low_pc
	.long	.Ltmp22-.Ltmp4                  # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x49e:0x6 DW_TAG_variable
	.byte	1                               # DW_AT_location
	.long	1103                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x4a4:0x6 DW_TAG_variable
	.byte	2                               # DW_AT_location
	.long	1111                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x4ad:0x79 DW_TAG_subprogram
	.byte	11                              # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin1       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	81                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	213                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x4bc:0x9 DW_TAG_formal_parameter
	.byte	5                               # DW_AT_location
	.byte	76                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	835                             # DW_AT_type
	.byte	31                              # Abbrev [31] 0x4c5:0x9 DW_TAG_formal_parameter
	.byte	6                               # DW_AT_location
	.byte	87                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	341                             # DW_AT_type
	.byte	31                              # Abbrev [31] 0x4ce:0x9 DW_TAG_formal_parameter
	.byte	7                               # DW_AT_location
	.byte	88                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	475                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4d7:0x9 DW_TAG_variable
	.byte	10                              # DW_AT_location
	.byte	85                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	121                             # DW_AT_decl_line
	.long	213                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4e0:0x9 DW_TAG_variable
	.byte	11                              # DW_AT_location
	.byte	86                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	118                             # DW_AT_decl_line
	.long	240                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4e9:0x9 DW_TAG_variable
	.byte	12                              # DW_AT_location
	.byte	89                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	119                             # DW_AT_decl_line
	.long	1677                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4f2:0x9 DW_TAG_variable
	.byte	13                              # DW_AT_location
	.byte	90                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	120                             # DW_AT_decl_line
	.long	1057                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4fb:0x9 DW_TAG_variable
	.byte	14                              # DW_AT_location
	.byte	91                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	121                             # DW_AT_decl_line
	.long	213                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x504:0x21 DW_TAG_inlined_subroutine
	.long	1086                            # DW_AT_abstract_origin
	.byte	12                              # DW_AT_low_pc
	.long	.Ltmp80-.Ltmp53                 # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	125                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x511:0x13 DW_TAG_lexical_block
	.byte	13                              # DW_AT_low_pc
	.long	.Ltmp77-.Ltmp59                 # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x517:0x6 DW_TAG_variable
	.byte	8                               # DW_AT_location
	.long	1103                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x51d:0x6 DW_TAG_variable
	.byte	9                               # DW_AT_location
	.long	1111                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x526:0x4c DW_TAG_subprogram
	.byte	14                              # DW_AT_low_pc
	.long	.Lfunc_end2-.Lfunc_begin2       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	82                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	199                             # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	480                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x535:0x9 DW_TAG_formal_parameter
	.byte	15                              # DW_AT_location
	.byte	76                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	199                             # DW_AT_decl_line
	.long	835                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x53e:0x9 DW_TAG_variable
	.byte	18                              # DW_AT_location
	.byte	85                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	202                             # DW_AT_decl_line
	.long	213                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x547:0x9 DW_TAG_variable
	.byte	19                              # DW_AT_location
	.byte	86                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	201                             # DW_AT_decl_line
	.long	240                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x550:0x21 DW_TAG_inlined_subroutine
	.long	1086                            # DW_AT_abstract_origin
	.byte	14                              # DW_AT_low_pc
	.long	.Ltmp221-.Lfunc_begin2          # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	204                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x55d:0x13 DW_TAG_lexical_block
	.byte	15                              # DW_AT_low_pc
	.long	.Ltmp218-.Ltmp200               # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x563:0x6 DW_TAG_variable
	.byte	16                              # DW_AT_location
	.long	1103                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x569:0x6 DW_TAG_variable
	.byte	17                              # DW_AT_location
	.long	1111                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x572:0x59 DW_TAG_subprogram
	.byte	16                              # DW_AT_low_pc
	.long	.Lfunc_end3-.Lfunc_begin3       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	83                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	38                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	213                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x581:0x9 DW_TAG_formal_parameter
	.byte	20                              # DW_AT_location
	.byte	76                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	38                              # DW_AT_decl_line
	.long	835                             # DW_AT_type
	.byte	36                              # Abbrev [36] 0x58a:0xb DW_TAG_variable
	.byte	2                               # DW_AT_location
	.byte	145
	.byte	0
	.byte	92                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	51                              # DW_AT_decl_line
	.long	1682                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x595:0x9 DW_TAG_variable
	.byte	21                              # DW_AT_location
	.byte	97                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	42                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x59e:0x9 DW_TAG_variable
	.byte	22                              # DW_AT_location
	.byte	93                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	43                              # DW_AT_decl_line
	.long	715                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x5a7:0x9 DW_TAG_variable
	.byte	23                              # DW_AT_location
	.byte	94                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	44                              # DW_AT_decl_line
	.long	715                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x5b0:0x8 DW_TAG_variable
	.byte	98                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	40                              # DW_AT_decl_line
	.long	1057                            # DW_AT_type
	.byte	34                              # Abbrev [34] 0x5b8:0x12 DW_TAG_lexical_block
	.byte	17                              # DW_AT_low_pc
	.long	.Ltmp255-.Ltmp250               # DW_AT_high_pc
	.byte	36                              # Abbrev [36] 0x5be:0xb DW_TAG_variable
	.byte	2                               # DW_AT_location
	.byte	145
	.byte	0
	.byte	96                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	40                              # DW_AT_decl_line
	.long	1057                            # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	37                              # Abbrev [37] 0x5cb:0xc2 DW_TAG_subprogram
	.byte	18                              # DW_AT_low_pc
	.long	.Lfunc_end4-.Lfunc_begin4       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	84                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	60                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	213                             # DW_AT_type
	.byte	31                              # Abbrev [31] 0x5da:0x9 DW_TAG_formal_parameter
	.byte	24                              # DW_AT_location
	.byte	99                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	60                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	38                              # Abbrev [38] 0x5e3:0xa DW_TAG_formal_parameter
	.byte	1                               # DW_AT_location
	.byte	82
	.byte	76                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	60                              # DW_AT_decl_line
	.long	1706                            # DW_AT_type
	.byte	34                              # Abbrev [34] 0x5ed:0x9f DW_TAG_lexical_block
	.byte	19                              # DW_AT_low_pc
	.long	.Ltmp342-.Ltmp284               # DW_AT_high_pc
	.byte	32                              # Abbrev [32] 0x5f3:0x9 DW_TAG_variable
	.byte	25                              # DW_AT_location
	.byte	100                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	62                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	39                              # Abbrev [39] 0x5fc:0x8f DW_TAG_lexical_block
	.byte	0                               # DW_AT_ranges
	.byte	32                              # Abbrev [32] 0x5fe:0x9 DW_TAG_variable
	.byte	26                              # DW_AT_location
	.byte	101                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x607:0x9 DW_TAG_variable
	.byte	27                              # DW_AT_location
	.byte	102                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	72                              # DW_AT_decl_line
	.long	1066                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x610:0x9 DW_TAG_variable
	.byte	28                              # DW_AT_location
	.byte	103                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	74                              # DW_AT_decl_line
	.long	1066                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x619:0x9 DW_TAG_variable
	.byte	29                              # DW_AT_location
	.byte	104                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	75                              # DW_AT_decl_line
	.long	1066                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x622:0x9 DW_TAG_variable
	.byte	30                              # DW_AT_location
	.byte	105                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	76                              # DW_AT_decl_line
	.long	1066                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x62b:0x9 DW_TAG_variable
	.byte	31                              # DW_AT_location
	.byte	106                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	78                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x634:0x9 DW_TAG_variable
	.byte	32                              # DW_AT_location
	.byte	107                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	79                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x63d:0x9 DW_TAG_variable
	.byte	33                              # DW_AT_location
	.byte	108                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	80                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x646:0x9 DW_TAG_variable
	.byte	34                              # DW_AT_location
	.byte	109                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	81                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	29                              # Abbrev [29] 0x64f:0x8 DW_TAG_variable
	.byte	110                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	71                              # DW_AT_decl_line
	.long	1061                            # DW_AT_type
	.byte	29                              # Abbrev [29] 0x657:0x8 DW_TAG_variable
	.byte	111                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	77                              # DW_AT_decl_line
	.long	1066                            # DW_AT_type
	.byte	29                              # Abbrev [29] 0x65f:0x8 DW_TAG_variable
	.byte	112                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	83                              # DW_AT_decl_line
	.long	71                              # DW_AT_type
	.byte	39                              # Abbrev [39] 0x667:0x23 DW_TAG_lexical_block
	.byte	1                               # DW_AT_ranges
	.byte	29                              # Abbrev [29] 0x669:0x8 DW_TAG_variable
	.byte	113                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	87                              # DW_AT_decl_line
	.long	550                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x671:0x8 DW_TAG_variable
	.byte	114                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	89                              # DW_AT_decl_line
	.long	550                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x679:0x8 DW_TAG_variable
	.byte	115                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	91                              # DW_AT_decl_line
	.long	550                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x681:0x8 DW_TAG_variable
	.byte	116                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	93                              # DW_AT_decl_line
	.long	550                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x68d:0x5 DW_TAG_pointer_type
	.long	290                             # DW_AT_type
	.byte	13                              # Abbrev [13] 0x692:0x18 DW_TAG_structure_type
	.byte	95                              # DW_AT_name
	.byte	16                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	29                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x697:0x9 DW_TAG_member
	.byte	93                              # DW_AT_name
	.long	715                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	31                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x6a0:0x9 DW_TAG_member
	.byte	94                              # DW_AT_name
	.long	715                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	32                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x6aa:0x5 DW_TAG_pointer_type
	.long	1682                            # DW_AT_type
	.byte	0                               # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_rnglists,"",@progbits
	.long	.Ldebug_list_header_end1-.Ldebug_list_header_start1 # Length
.Ldebug_list_header_start1:
	.short	5                               # Version
	.byte	8                               # Address size
	.byte	0                               # Segment selector size
	.long	3                               # Offset entry count
.Lrnglists_table_base0:
	.long	.Ldebug_ranges0-.Lrnglists_table_base0
	.long	.Ldebug_ranges1-.Lrnglists_table_base0
	.long	.Ldebug_ranges2-.Lrnglists_table_base0
.Ldebug_ranges0:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp287-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp290-.Lfunc_begin0         #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp293-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp335-.Lfunc_begin0         #   ending offset
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_ranges1:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp305-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp311-.Lfunc_begin0         #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp317-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp335-.Lfunc_begin0         #   ending offset
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_ranges2:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Lfunc_begin0-.Lfunc_begin0    #   starting offset
	.uleb128 .Lfunc_end2-.Lfunc_begin0      #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Lfunc_begin4-.Lfunc_begin0    #   starting offset
	.uleb128 .Lfunc_end4-.Lfunc_begin0      #   ending offset
	.byte	3                               # DW_RLE_startx_length
	.byte	16                              #   start index
	.uleb128 .Lfunc_end3-.Lfunc_begin3      #   length
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_list_header_end1:
	.section	.debug_str_offsets,"",@progbits
	.long	472                             # Length of String Offsets Set
	.short	5
	.short	0
.Lstr_offsets_base0:
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Ubuntu clang version 16.0.6 (15)" # string offset=0
.Linfo_string1:
	.asciz	"ebpf-decompress/src/bpf/packing.bpf.c" # string offset=33
.Linfo_string2:
	.asciz	"/home/mat/src/portable-decompress" # string offset=71
.Linfo_string3:
	.asciz	"IN_SIZE"                       # string offset=105
.Linfo_string4:
	.asciz	"unsigned int"                  # string offset=113
.Linfo_string5:
	.asciz	"__u32"                         # string offset=126
.Linfo_string6:
	.asciz	"u32"                           # string offset=132
.Linfo_string7:
	.asciz	"OUT_SIZE"                      # string offset=136
.Linfo_string8:
	.asciz	"LOOP_FACTOR"                   # string offset=145
.Linfo_string9:
	.asciz	"_license"                      # string offset=157
.Linfo_string10:
	.asciz	"char"                          # string offset=166
.Linfo_string11:
	.asciz	"__ARRAY_SIZE_TYPE__"           # string offset=171
.Linfo_string12:
	.asciz	"__bpf_usdt_specs"              # string offset=191
.Linfo_string13:
	.asciz	"type"                          # string offset=208
.Linfo_string14:
	.asciz	"int"                           # string offset=213
.Linfo_string15:
	.asciz	"max_entries"                   # string offset=217
.Linfo_string16:
	.asciz	"key"                           # string offset=229
.Linfo_string17:
	.asciz	"value"                         # string offset=233
.Linfo_string18:
	.asciz	"args"                          # string offset=239
.Linfo_string19:
	.asciz	"val_off"                       # string offset=244
.Linfo_string20:
	.asciz	"unsigned long long"            # string offset=252
.Linfo_string21:
	.asciz	"__u64"                         # string offset=271
.Linfo_string22:
	.asciz	"arg_type"                      # string offset=277
.Linfo_string23:
	.asciz	"BPF_USDT_ARG_CONST"            # string offset=286
.Linfo_string24:
	.asciz	"BPF_USDT_ARG_REG"              # string offset=305
.Linfo_string25:
	.asciz	"BPF_USDT_ARG_REG_DEREF"        # string offset=322
.Linfo_string26:
	.asciz	"__bpf_usdt_arg_type"           # string offset=345
.Linfo_string27:
	.asciz	"reg_off"                       # string offset=365
.Linfo_string28:
	.asciz	"short"                         # string offset=373
.Linfo_string29:
	.asciz	"arg_signed"                    # string offset=379
.Linfo_string30:
	.asciz	"_Bool"                         # string offset=390
.Linfo_string31:
	.asciz	"bool"                          # string offset=396
.Linfo_string32:
	.asciz	"arg_bitshift"                  # string offset=401
.Linfo_string33:
	.asciz	"__bpf_usdt_arg_spec"           # string offset=414
.Linfo_string34:
	.asciz	"usdt_cookie"                   # string offset=434
.Linfo_string35:
	.asciz	"arg_cnt"                       # string offset=446
.Linfo_string36:
	.asciz	"__bpf_usdt_spec"               # string offset=454
.Linfo_string37:
	.asciz	"__bpf_usdt_ip_to_spec_id"      # string offset=470
.Linfo_string38:
	.asciz	"long"                          # string offset=495
.Linfo_string39:
	.asciz	"in_bytes"                      # string offset=500
.Linfo_string40:
	.asciz	"value_size"                    # string offset=509
.Linfo_string41:
	.asciz	"map_flags"                     # string offset=520
.Linfo_string42:
	.asciz	"out_bytes"                     # string offset=530
.Linfo_string43:
	.asciz	"LINUX_HAS_BPF_COOKIE"          # string offset=540
.Linfo_string44:
	.asciz	"bpf_get_attach_cookie"         # string offset=561
.Linfo_string45:
	.asciz	"bpf_map_lookup_elem"           # string offset=583
.Linfo_string46:
	.asciz	"bpf_probe_read_kernel"         # string offset=603
.Linfo_string47:
	.asciz	"bpf_probe_read_user"           # string offset=625
.Linfo_string48:
	.asciz	"bpf_loop"                      # string offset=645
.Linfo_string49:
	.asciz	"r15"                           # string offset=654
.Linfo_string50:
	.asciz	"unsigned long"                 # string offset=658
.Linfo_string51:
	.asciz	"r14"                           # string offset=672
.Linfo_string52:
	.asciz	"r13"                           # string offset=676
.Linfo_string53:
	.asciz	"r12"                           # string offset=680
.Linfo_string54:
	.asciz	"bp"                            # string offset=684
.Linfo_string55:
	.asciz	"bx"                            # string offset=687
.Linfo_string56:
	.asciz	"r11"                           # string offset=690
.Linfo_string57:
	.asciz	"r10"                           # string offset=694
.Linfo_string58:
	.asciz	"r9"                            # string offset=698
.Linfo_string59:
	.asciz	"r8"                            # string offset=701
.Linfo_string60:
	.asciz	"ax"                            # string offset=704
.Linfo_string61:
	.asciz	"cx"                            # string offset=707
.Linfo_string62:
	.asciz	"dx"                            # string offset=710
.Linfo_string63:
	.asciz	"si"                            # string offset=713
.Linfo_string64:
	.asciz	"di"                            # string offset=716
.Linfo_string65:
	.asciz	"orig_ax"                       # string offset=719
.Linfo_string66:
	.asciz	"ip"                            # string offset=727
.Linfo_string67:
	.asciz	"cs"                            # string offset=730
.Linfo_string68:
	.asciz	"flags"                         # string offset=733
.Linfo_string69:
	.asciz	"sp"                            # string offset=739
.Linfo_string70:
	.asciz	"ss"                            # string offset=742
.Linfo_string71:
	.asciz	"pt_regs"                       # string offset=745
.Linfo_string72:
	.asciz	"unsigned char"                 # string offset=753
.Linfo_string73:
	.asciz	"__u8"                          # string offset=767
.Linfo_string74:
	.asciz	"u8"                            # string offset=772
.Linfo_string75:
	.asciz	"__bpf_usdt_spec_id"            # string offset=775
.Linfo_string76:
	.asciz	"ctx"                           # string offset=794
.Linfo_string77:
	.asciz	"spec_id_ptr"                   # string offset=798
.Linfo_string78:
	.asciz	"DW_ATE_unsigned_32"            # string offset=810
.Linfo_string79:
	.asciz	"DW_ATE_unsigned_8"             # string offset=829
.Linfo_string80:
	.asciz	"bpf_usdt_arg_cnt"              # string offset=847
.Linfo_string81:
	.asciz	"bpf_usdt_arg"                  # string offset=864
.Linfo_string82:
	.asciz	"bpf_usdt_cookie"               # string offset=877
.Linfo_string83:
	.asciz	"bpf_prog"                      # string offset=893
.Linfo_string84:
	.asciz	"decode_one"                    # string offset=902
.Linfo_string85:
	.asciz	"spec_id"                       # string offset=913
.Linfo_string86:
	.asciz	"spec"                          # string offset=921
.Linfo_string87:
	.asciz	"arg_num"                       # string offset=926
.Linfo_string88:
	.asciz	"res"                           # string offset=934
.Linfo_string89:
	.asciz	"arg_spec"                      # string offset=938
.Linfo_string90:
	.asciz	"val"                           # string offset=947
.Linfo_string91:
	.asciz	"err"                           # string offset=951
.Linfo_string92:
	.asciz	"loop_ctx"                      # string offset=955
.Linfo_string93:
	.asciz	"in_ptr"                        # string offset=964
.Linfo_string94:
	.asciz	"out_ptr"                       # string offset=971
.Linfo_string95:
	.asciz	"decode_ctx"                    # string offset=979
.Linfo_string96:
	.asciz	"__r"                           # string offset=990
.Linfo_string97:
	.asciz	"zero"                          # string offset=994
.Linfo_string98:
	.asciz	"len"                           # string offset=999
.Linfo_string99:
	.asciz	"i"                             # string offset=1003
.Linfo_string100:
	.asciz	"inner_i"                       # string offset=1005
.Linfo_string101:
	.asciz	"read_i"                        # string offset=1013
.Linfo_string102:
	.asciz	"byte"                          # string offset=1020
.Linfo_string103:
	.asciz	"a1"                            # string offset=1025
.Linfo_string104:
	.asciz	"a2"                            # string offset=1028
.Linfo_string105:
	.asciz	"a3"                            # string offset=1031
.Linfo_string106:
	.asciz	"b1"                            # string offset=1034
.Linfo_string107:
	.asciz	"b2"                            # string offset=1037
.Linfo_string108:
	.asciz	"b3"                            # string offset=1040
.Linfo_string109:
	.asciz	"b4"                            # string offset=1043
.Linfo_string110:
	.asciz	"elem"                          # string offset=1046
.Linfo_string111:
	.asciz	"a4"                            # string offset=1051
.Linfo_string112:
	.asciz	"write_i"                       # string offset=1054
.Linfo_string113:
	.asciz	"elem_1"                        # string offset=1062
.Linfo_string114:
	.asciz	"elem_2"                        # string offset=1069
.Linfo_string115:
	.asciz	"elem_3"                        # string offset=1076
.Linfo_string116:
	.asciz	"elem_4"                        # string offset=1083
	.section	.debug_str_offsets,"",@progbits
	.long	.Linfo_string0
	.long	.Linfo_string1
	.long	.Linfo_string2
	.long	.Linfo_string3
	.long	.Linfo_string4
	.long	.Linfo_string5
	.long	.Linfo_string6
	.long	.Linfo_string7
	.long	.Linfo_string8
	.long	.Linfo_string9
	.long	.Linfo_string10
	.long	.Linfo_string11
	.long	.Linfo_string12
	.long	.Linfo_string13
	.long	.Linfo_string14
	.long	.Linfo_string15
	.long	.Linfo_string16
	.long	.Linfo_string17
	.long	.Linfo_string18
	.long	.Linfo_string19
	.long	.Linfo_string20
	.long	.Linfo_string21
	.long	.Linfo_string22
	.long	.Linfo_string23
	.long	.Linfo_string24
	.long	.Linfo_string25
	.long	.Linfo_string26
	.long	.Linfo_string27
	.long	.Linfo_string28
	.long	.Linfo_string29
	.long	.Linfo_string30
	.long	.Linfo_string31
	.long	.Linfo_string32
	.long	.Linfo_string33
	.long	.Linfo_string34
	.long	.Linfo_string35
	.long	.Linfo_string36
	.long	.Linfo_string37
	.long	.Linfo_string38
	.long	.Linfo_string39
	.long	.Linfo_string40
	.long	.Linfo_string41
	.long	.Linfo_string42
	.long	.Linfo_string43
	.long	.Linfo_string44
	.long	.Linfo_string45
	.long	.Linfo_string46
	.long	.Linfo_string47
	.long	.Linfo_string48
	.long	.Linfo_string49
	.long	.Linfo_string50
	.long	.Linfo_string51
	.long	.Linfo_string52
	.long	.Linfo_string53
	.long	.Linfo_string54
	.long	.Linfo_string55
	.long	.Linfo_string56
	.long	.Linfo_string57
	.long	.Linfo_string58
	.long	.Linfo_string59
	.long	.Linfo_string60
	.long	.Linfo_string61
	.long	.Linfo_string62
	.long	.Linfo_string63
	.long	.Linfo_string64
	.long	.Linfo_string65
	.long	.Linfo_string66
	.long	.Linfo_string67
	.long	.Linfo_string68
	.long	.Linfo_string69
	.long	.Linfo_string70
	.long	.Linfo_string71
	.long	.Linfo_string72
	.long	.Linfo_string73
	.long	.Linfo_string74
	.long	.Linfo_string75
	.long	.Linfo_string76
	.long	.Linfo_string77
	.long	.Linfo_string78
	.long	.Linfo_string79
	.long	.Linfo_string80
	.long	.Linfo_string81
	.long	.Linfo_string82
	.long	.Linfo_string83
	.long	.Linfo_string84
	.long	.Linfo_string85
	.long	.Linfo_string86
	.long	.Linfo_string87
	.long	.Linfo_string88
	.long	.Linfo_string89
	.long	.Linfo_string90
	.long	.Linfo_string91
	.long	.Linfo_string92
	.long	.Linfo_string93
	.long	.Linfo_string94
	.long	.Linfo_string95
	.long	.Linfo_string96
	.long	.Linfo_string97
	.long	.Linfo_string98
	.long	.Linfo_string99
	.long	.Linfo_string100
	.long	.Linfo_string101
	.long	.Linfo_string102
	.long	.Linfo_string103
	.long	.Linfo_string104
	.long	.Linfo_string105
	.long	.Linfo_string106
	.long	.Linfo_string107
	.long	.Linfo_string108
	.long	.Linfo_string109
	.long	.Linfo_string110
	.long	.Linfo_string111
	.long	.Linfo_string112
	.long	.Linfo_string113
	.long	.Linfo_string114
	.long	.Linfo_string115
	.long	.Linfo_string116
	.section	.debug_addr,"",@progbits
	.long	.Ldebug_addr_end0-.Ldebug_addr_start0 # Length of contribution
.Ldebug_addr_start0:
	.short	5                               # DWARF version number
	.byte	8                               # Address size
	.byte	0                               # Segment selector size
.Laddr_table_base0:
	.quad	IN_SIZE
	.quad	OUT_SIZE
	.quad	LOOP_FACTOR
	.quad	_license
	.quad	__bpf_usdt_specs
	.quad	__bpf_usdt_ip_to_spec_id
	.quad	in_bytes
	.quad	out_bytes
	.quad	LINUX_HAS_BPF_COOKIE
	.quad	.Lfunc_begin0
	.quad	.Ltmp4
	.quad	.Lfunc_begin1
	.quad	.Ltmp53
	.quad	.Ltmp59
	.quad	.Lfunc_begin2
	.quad	.Ltmp200
	.quad	.Lfunc_begin3
	.quad	.Ltmp250
	.quad	.Lfunc_begin4
	.quad	.Ltmp284
.Ldebug_addr_end0:
	.section	.BTF,"",@progbits
	.short	60319                           # 0xeb9f
	.byte	1
	.byte	0
	.long	24
	.long	0
	.long	1816
	.long	1816
	.long	2515
	.long	0                               # BTF_KIND_PTR(id = 1)
	.long	33554432                        # 0x2000000
	.long	3
	.long	1                               # BTF_KIND_INT(id = 2)
	.long	16777216                        # 0x1000000
	.long	4
	.long	16777248                        # 0x1000020
	.long	0                               # BTF_KIND_ARRAY(id = 3)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	2
	.long	5                               # BTF_KIND_INT(id = 4)
	.long	16777216                        # 0x1000000
	.long	4
	.long	32                              # 0x20
	.long	0                               # BTF_KIND_PTR(id = 5)
	.long	33554432                        # 0x2000000
	.long	6
	.long	0                               # BTF_KIND_ARRAY(id = 6)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	256
	.long	0                               # BTF_KIND_PTR(id = 7)
	.long	33554432                        # 0x2000000
	.long	2
	.long	0                               # BTF_KIND_PTR(id = 8)
	.long	33554432                        # 0x2000000
	.long	9
	.long	25                              # BTF_KIND_STRUCT(id = 9)
	.long	67108867                        # 0x4000003
	.long	208
	.long	41
	.long	18
	.long	0                               # 0x0
	.long	46
	.long	11
	.long	1536                            # 0x600
	.long	58
	.long	14
	.long	1600                            # 0x640
	.long	66                              # BTF_KIND_STRUCT(id = 10)
	.long	67108869                        # 0x4000005
	.long	16
	.long	86
	.long	11
	.long	0                               # 0x0
	.long	94
	.long	13
	.long	64                              # 0x40
	.long	103
	.long	14
	.long	96                              # 0x60
	.long	111
	.long	15
	.long	112                             # 0x70
	.long	122
	.long	17
	.long	120                             # 0x78
	.long	135                             # BTF_KIND_TYPEDEF(id = 11)
	.long	134217728                       # 0x8000000
	.long	12
	.long	141                             # BTF_KIND_INT(id = 12)
	.long	16777216                        # 0x1000000
	.long	8
	.long	64                              # 0x40
	.long	160                             # BTF_KIND_ENUM(id = 13)
	.long	100663299                       # 0x6000003
	.long	4
	.long	180
	.long	0
	.long	199
	.long	1
	.long	216
	.long	2
	.long	239                             # BTF_KIND_INT(id = 14)
	.long	16777216                        # 0x1000000
	.long	2
	.long	16777232                        # 0x1000010
	.long	245                             # BTF_KIND_TYPEDEF(id = 15)
	.long	134217728                       # 0x8000000
	.long	16
	.long	250                             # BTF_KIND_INT(id = 16)
	.long	16777216                        # 0x1000000
	.long	1
	.long	67108872                        # 0x4000008
	.long	256                             # BTF_KIND_INT(id = 17)
	.long	16777216                        # 0x1000000
	.long	1
	.long	16777224                        # 0x1000008
	.long	0                               # BTF_KIND_ARRAY(id = 18)
	.long	50331648                        # 0x3000000
	.long	0
	.long	10
	.long	4
	.long	12
	.long	0                               # BTF_KIND_STRUCT(id = 19)
	.long	67108868                        # 0x4000004
	.long	32
	.long	261
	.long	1
	.long	0                               # 0x0
	.long	266
	.long	5
	.long	64                              # 0x40
	.long	278
	.long	7
	.long	128                             # 0x80
	.long	282
	.long	8
	.long	192                             # 0xc0
	.long	288                             # BTF_KIND_VAR(id = 20)
	.long	234881024                       # 0xe000000
	.long	19
	.long	1
	.long	0                               # BTF_KIND_PTR(id = 21)
	.long	33554432                        # 0x2000000
	.long	22
	.long	305                             # BTF_KIND_TYPEDEF(id = 22)
	.long	134217728                       # 0x8000000
	.long	23
	.long	309                             # BTF_KIND_TYPEDEF(id = 23)
	.long	134217728                       # 0x8000000
	.long	24
	.long	315                             # BTF_KIND_INT(id = 24)
	.long	16777216                        # 0x1000000
	.long	4
	.long	32                              # 0x20
	.long	0                               # BTF_KIND_PTR(id = 25)
	.long	33554432                        # 0x2000000
	.long	26
	.long	0                               # BTF_KIND_ARRAY(id = 26)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	8388608
	.long	0                               # BTF_KIND_PTR(id = 27)
	.long	33554432                        # 0x2000000
	.long	28
	.long	0                               # BTF_KIND_ARRAY(id = 28)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	1
	.long	0                               # BTF_KIND_PTR(id = 29)
	.long	33554432                        # 0x2000000
	.long	30
	.long	0                               # BTF_KIND_ARRAY(id = 30)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	1152
	.long	0                               # BTF_KIND_STRUCT(id = 31)
	.long	67108869                        # 0x4000005
	.long	40
	.long	261
	.long	1
	.long	0                               # 0x0
	.long	278
	.long	21
	.long	64                              # 0x40
	.long	328
	.long	25
	.long	128                             # 0x80
	.long	266
	.long	27
	.long	192                             # 0xc0
	.long	339
	.long	29
	.long	256                             # 0x100
	.long	349                             # BTF_KIND_VAR(id = 32)
	.long	234881024                       # 0xe000000
	.long	31
	.long	1
	.long	0                               # BTF_KIND_PTR(id = 33)
	.long	33554432                        # 0x2000000
	.long	34
	.long	0                               # BTF_KIND_ARRAY(id = 34)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	134217728
	.long	0                               # BTF_KIND_PTR(id = 35)
	.long	33554432                        # 0x2000000
	.long	36
	.long	0                               # BTF_KIND_ARRAY(id = 36)
	.long	50331648                        # 0x3000000
	.long	0
	.long	2
	.long	4
	.long	1024
	.long	0                               # BTF_KIND_STRUCT(id = 37)
	.long	67108869                        # 0x4000005
	.long	40
	.long	261
	.long	1
	.long	0                               # 0x0
	.long	278
	.long	21
	.long	64                              # 0x40
	.long	328
	.long	33
	.long	128                             # 0x80
	.long	266
	.long	27
	.long	192                             # 0xc0
	.long	339
	.long	35
	.long	256                             # 0x100
	.long	358                             # BTF_KIND_VAR(id = 38)
	.long	234881024                       # 0xe000000
	.long	37
	.long	1
	.long	0                               # BTF_KIND_PTR(id = 39)
	.long	33554432                        # 0x2000000
	.long	40
	.long	368                             # BTF_KIND_INT(id = 40)
	.long	16777216                        # 0x1000000
	.long	8
	.long	16777280                        # 0x1000040
	.long	0                               # BTF_KIND_PTR(id = 41)
	.long	33554432                        # 0x2000000
	.long	23
	.long	0                               # BTF_KIND_STRUCT(id = 42)
	.long	67108868                        # 0x4000004
	.long	32
	.long	261
	.long	27
	.long	0                               # 0x0
	.long	266
	.long	35
	.long	64                              # 0x40
	.long	278
	.long	39
	.long	128                             # 0x80
	.long	282
	.long	41
	.long	192                             # 0xc0
	.long	373                             # BTF_KIND_VAR(id = 43)
	.long	234881024                       # 0xe000000
	.long	42
	.long	1
	.long	0                               # BTF_KIND_PTR(id = 44)
	.long	33554432                        # 0x2000000
	.long	45
	.long	398                             # BTF_KIND_STRUCT(id = 45)
	.long	67108885                        # 0x4000015
	.long	168
	.long	406
	.long	46
	.long	0                               # 0x0
	.long	410
	.long	46
	.long	64                              # 0x40
	.long	414
	.long	46
	.long	128                             # 0x80
	.long	418
	.long	46
	.long	192                             # 0xc0
	.long	422
	.long	46
	.long	256                             # 0x100
	.long	425
	.long	46
	.long	320                             # 0x140
	.long	428
	.long	46
	.long	384                             # 0x180
	.long	432
	.long	46
	.long	448                             # 0x1c0
	.long	436
	.long	46
	.long	512                             # 0x200
	.long	439
	.long	46
	.long	576                             # 0x240
	.long	442
	.long	46
	.long	640                             # 0x280
	.long	445
	.long	46
	.long	704                             # 0x2c0
	.long	448
	.long	46
	.long	768                             # 0x300
	.long	451
	.long	46
	.long	832                             # 0x340
	.long	454
	.long	46
	.long	896                             # 0x380
	.long	457
	.long	46
	.long	960                             # 0x3c0
	.long	465
	.long	46
	.long	1024                            # 0x400
	.long	468
	.long	46
	.long	1088                            # 0x440
	.long	471
	.long	46
	.long	1152                            # 0x480
	.long	477
	.long	46
	.long	1216                            # 0x4c0
	.long	480
	.long	46
	.long	1280                            # 0x500
	.long	483                             # BTF_KIND_INT(id = 46)
	.long	16777216                        # 0x1000000
	.long	8
	.long	64                              # 0x40
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 47)
	.long	218103809                       # 0xd000001
	.long	2
	.long	497
	.long	44
	.long	501                             # BTF_KIND_FUNC(id = 48)
	.long	201326593                       # 0xc000001
	.long	47
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 49)
	.long	218103811                       # 0xd000003
	.long	2
	.long	497
	.long	44
	.long	916
	.long	11
	.long	924
	.long	39
	.long	928                             # BTF_KIND_FUNC(id = 50)
	.long	201326593                       # 0xc000001
	.long	49
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 51)
	.long	218103809                       # 0xd000001
	.long	40
	.long	497
	.long	44
	.long	1425                            # BTF_KIND_FUNC(id = 52)
	.long	201326593                       # 0xc000001
	.long	51
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 53)
	.long	218103809                       # 0xd000001
	.long	2
	.long	497
	.long	44
	.long	1468                            # BTF_KIND_FUNC(id = 54)
	.long	201326593                       # 0xc000001
	.long	53
	.long	0                               # BTF_KIND_PTR(id = 55)
	.long	33554432                        # 0x2000000
	.long	56
	.long	1937                            # BTF_KIND_STRUCT(id = 56)
	.long	67108866                        # 0x4000002
	.long	16
	.long	1948
	.long	57
	.long	0                               # 0x0
	.long	1955
	.long	57
	.long	64                              # 0x40
	.long	0                               # BTF_KIND_PTR(id = 57)
	.long	33554432                        # 0x2000000
	.long	0
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 58)
	.long	218103810                       # 0xd000002
	.long	2
	.long	1963
	.long	22
	.long	497
	.long	55
	.long	1965                            # BTF_KIND_FUNC(id = 59)
	.long	201326592                       # 0xc000000
	.long	58
	.long	0                               # BTF_KIND_CONST(id = 60)
	.long	167772160                       # 0xa000000
	.long	22
	.long	2425                            # BTF_KIND_VAR(id = 61)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	2433                            # BTF_KIND_VAR(id = 62)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	2442                            # BTF_KIND_VAR(id = 63)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	0                               # BTF_KIND_ARRAY(id = 64)
	.long	50331648                        # 0x3000000
	.long	0
	.long	17
	.long	4
	.long	4
	.long	2454                            # BTF_KIND_VAR(id = 65)
	.long	234881024                       # 0xe000000
	.long	64
	.long	1
	.long	0                               # BTF_KIND_CONST(id = 66)
	.long	167772160                       # 0xa000000
	.long	16
	.long	2463                            # BTF_KIND_VAR(id = 67)
	.long	234881024                       # 0xe000000
	.long	66
	.long	2
	.long	2484                            # BTF_KIND_DATASEC(id = 68)
	.long	251658241                       # 0xf000001
	.long	0
	.long	67
	.long	LINUX_HAS_BPF_COOKIE
	.long	1
	.long	2493                            # BTF_KIND_DATASEC(id = 69)
	.long	251658244                       # 0xf000004
	.long	0
	.long	20
	.long	__bpf_usdt_specs
	.long	32
	.long	32
	.long	in_bytes
	.long	40
	.long	38
	.long	out_bytes
	.long	40
	.long	43
	.long	__bpf_usdt_ip_to_spec_id
	.long	32
	.long	2499                            # BTF_KIND_DATASEC(id = 70)
	.long	251658243                       # 0xf000003
	.long	0
	.long	61
	.long	IN_SIZE
	.long	4
	.long	62
	.long	OUT_SIZE
	.long	4
	.long	63
	.long	LOOP_FACTOR
	.long	4
	.long	2507                            # BTF_KIND_DATASEC(id = 71)
	.long	251658241                       # 0xf000001
	.long	0
	.long	65
	.long	_license
	.long	4
	.byte	0                               # string offset=0
	.ascii	"int"                           # string offset=1
	.byte	0
	.ascii	"__ARRAY_SIZE_TYPE__"           # string offset=5
	.byte	0
	.ascii	"__bpf_usdt_spec"               # string offset=25
	.byte	0
	.ascii	"args"                          # string offset=41
	.byte	0
	.ascii	"usdt_cookie"                   # string offset=46
	.byte	0
	.ascii	"arg_cnt"                       # string offset=58
	.byte	0
	.ascii	"__bpf_usdt_arg_spec"           # string offset=66
	.byte	0
	.ascii	"val_off"                       # string offset=86
	.byte	0
	.ascii	"arg_type"                      # string offset=94
	.byte	0
	.ascii	"reg_off"                       # string offset=103
	.byte	0
	.ascii	"arg_signed"                    # string offset=111
	.byte	0
	.ascii	"arg_bitshift"                  # string offset=122
	.byte	0
	.ascii	"__u64"                         # string offset=135
	.byte	0
	.ascii	"unsigned long long"            # string offset=141
	.byte	0
	.ascii	"__bpf_usdt_arg_type"           # string offset=160
	.byte	0
	.ascii	"BPF_USDT_ARG_CONST"            # string offset=180
	.byte	0
	.ascii	"BPF_USDT_ARG_REG"              # string offset=199
	.byte	0
	.ascii	"BPF_USDT_ARG_REG_DEREF"        # string offset=216
	.byte	0
	.ascii	"short"                         # string offset=239
	.byte	0
	.ascii	"bool"                          # string offset=245
	.byte	0
	.ascii	"_Bool"                         # string offset=250
	.byte	0
	.ascii	"char"                          # string offset=256
	.byte	0
	.ascii	"type"                          # string offset=261
	.byte	0
	.ascii	"max_entries"                   # string offset=266
	.byte	0
	.ascii	"key"                           # string offset=278
	.byte	0
	.ascii	"value"                         # string offset=282
	.byte	0
	.ascii	"__bpf_usdt_specs"              # string offset=288
	.byte	0
	.ascii	"u32"                           # string offset=305
	.byte	0
	.ascii	"__u32"                         # string offset=309
	.byte	0
	.ascii	"unsigned int"                  # string offset=315
	.byte	0
	.ascii	"value_size"                    # string offset=328
	.byte	0
	.ascii	"map_flags"                     # string offset=339
	.byte	0
	.ascii	"in_bytes"                      # string offset=349
	.byte	0
	.ascii	"out_bytes"                     # string offset=358
	.byte	0
	.ascii	"long"                          # string offset=368
	.byte	0
	.ascii	"__bpf_usdt_ip_to_spec_id"      # string offset=373
	.byte	0
	.ascii	"pt_regs"                       # string offset=398
	.byte	0
	.ascii	"r15"                           # string offset=406
	.byte	0
	.ascii	"r14"                           # string offset=410
	.byte	0
	.ascii	"r13"                           # string offset=414
	.byte	0
	.ascii	"r12"                           # string offset=418
	.byte	0
	.ascii	"bp"                            # string offset=422
	.byte	0
	.ascii	"bx"                            # string offset=425
	.byte	0
	.ascii	"r11"                           # string offset=428
	.byte	0
	.ascii	"r10"                           # string offset=432
	.byte	0
	.ascii	"r9"                            # string offset=436
	.byte	0
	.ascii	"r8"                            # string offset=439
	.byte	0
	.ascii	"ax"                            # string offset=442
	.byte	0
	.ascii	"cx"                            # string offset=445
	.byte	0
	.ascii	"dx"                            # string offset=448
	.byte	0
	.ascii	"si"                            # string offset=451
	.byte	0
	.ascii	"di"                            # string offset=454
	.byte	0
	.ascii	"orig_ax"                       # string offset=457
	.byte	0
	.ascii	"ip"                            # string offset=465
	.byte	0
	.ascii	"cs"                            # string offset=468
	.byte	0
	.ascii	"flags"                         # string offset=471
	.byte	0
	.ascii	"sp"                            # string offset=477
	.byte	0
	.ascii	"ss"                            # string offset=480
	.byte	0
	.ascii	"unsigned long"                 # string offset=483
	.byte	0
	.ascii	"ctx"                           # string offset=497
	.byte	0
	.ascii	"bpf_usdt_arg_cnt"              # string offset=501
	.byte	0
	.ascii	".text"                         # string offset=518
	.byte	0
	.ascii	"/usr/include/bpf/usdt.bpf.h"   # string offset=524
	.byte	0
	.ascii	"\tif (!LINUX_HAS_BPF_COOKIE) {" # string offset=552
	.byte	0
	.ascii	"0:16"                          # string offset=582
	.byte	0
	.ascii	"\t\tlong ip = PT_REGS_IP(ctx);" # string offset=587
	.byte	0
	.ascii	"\t\tspec_id_ptr = bpf_map_lookup_elem(&__bpf_usdt_ip_to_spec_id, &ip);" # string offset=616
	.byte	0
	.ascii	"\t\treturn spec_id_ptr ? *spec_id_ptr : -ESRCH;" # string offset=685
	.byte	0
	.ascii	"\treturn bpf_get_attach_cookie(ctx);" # string offset=731
	.byte	0
	.ascii	"\tspec_id = __bpf_usdt_spec_id(ctx);" # string offset=767
	.byte	0
	.ascii	"\tif (spec_id < 0)"            # string offset=803
	.byte	0
	.ascii	"\tspec = bpf_map_lookup_elem(&__bpf_usdt_specs, &spec_id);" # string offset=821
	.byte	0
	.ascii	"\tif (!spec)"                  # string offset=879
	.byte	0
	.ascii	"\treturn spec->arg_cnt;"       # string offset=891
	.byte	0
	.byte	125                             # string offset=914
	.byte	0
	.ascii	"arg_num"                       # string offset=916
	.byte	0
	.ascii	"res"                           # string offset=924
	.byte	0
	.ascii	"bpf_usdt_arg"                  # string offset=928
	.byte	0
	.ascii	"int bpf_usdt_arg(struct pt_regs *ctx, __u64 arg_num, long *res)" # string offset=941
	.byte	0
	.ascii	"\t*res = 0;"                   # string offset=1005
	.byte	0
	.ascii	"\tif (arg_num >= BPF_USDT_MAX_ARG_CNT)" # string offset=1016
	.byte	0
	.ascii	"\tif (arg_num >= spec->arg_cnt)" # string offset=1054
	.byte	0
	.ascii	"\targ_spec = &spec->args[arg_num];" # string offset=1085
	.byte	0
	.ascii	"\tswitch (arg_spec->arg_type) {" # string offset=1119
	.byte	0
	.ascii	"\t\tval = arg_spec->val_off;"  # string offset=1150
	.byte	0
	.ascii	"\t\terr = bpf_probe_read_kernel(&val, sizeof(val), (void *)ctx + arg_spec->reg_off);" # string offset=1177
	.byte	0
	.ascii	"\t\tif (err)"                  # string offset=1260
	.byte	0
	.ascii	"\t\terr = bpf_probe_read_user(&val, sizeof(val), (void *)val + arg_spec->val_off);" # string offset=1271
	.byte	0
	.ascii	"\tval <<= arg_spec->arg_bitshift;" # string offset=1352
	.byte	0
	.ascii	"\tif (arg_spec->arg_signed)"   # string offset=1385
	.byte	0
	.ascii	"\t*res = val;"                 # string offset=1412
	.byte	0
	.ascii	"bpf_usdt_cookie"               # string offset=1425
	.byte	0
	.ascii	"\treturn spec->usdt_cookie;"   # string offset=1441
	.byte	0
	.ascii	"bpf_prog"                      # string offset=1468
	.byte	0
	.ascii	"usdt"                          # string offset=1477
	.byte	0
	.ascii	"/home/mat/src/portable-decompress/./ebpf-decompress/src/bpf/packing.bpf.c" # string offset=1482
	.byte	0
	.ascii	"int bpf_prog(struct pt_regs *ctx)" # string offset=1556
	.byte	0
	.ascii	"0:10"                          # string offset=1590
	.byte	0
	.ascii	"    long unsigned int len = BPF_CORE_READ(ctx, ax);" # string offset=1595
	.byte	0
	.ascii	"    u32 zero = 0;"             # string offset=1647
	.byte	0
	.ascii	"    void *in_ptr = bpf_map_lookup_elem(&in_bytes, &zero);" # string offset=1665
	.byte	0
	.ascii	"    void *out_ptr = bpf_map_lookup_elem(&out_bytes, &zero);" # string offset=1723
	.byte	0
	.ascii	"    if (!in_ptr || !out_ptr)"  # string offset=1783
	.byte	0
	.ascii	"    loop_ctx.out_ptr = out_ptr;" # string offset=1812
	.byte	0
	.ascii	"    loop_ctx.in_ptr = in_ptr;" # string offset=1844
	.byte	0
	.ascii	"    bpf_loop(IN_SIZE / LOOP_FACTOR, decode_one, &loop_ctx, 0);" # string offset=1874
	.byte	0
	.ascii	"decode_ctx"                    # string offset=1937
	.byte	0
	.ascii	"in_ptr"                        # string offset=1948
	.byte	0
	.ascii	"out_ptr"                       # string offset=1955
	.byte	0
	.byte	105                             # string offset=1963
	.byte	0
	.ascii	"decode_one"                    # string offset=1965
	.byte	0
	.ascii	"static int decode_one(u32 i, struct decode_ctx *ctx)" # string offset=1976
	.byte	0
	.ascii	"    for (u32 inner_i = 0; inner_i < LOOP_FACTOR; inner_i += 1)" # string offset=2029
	.byte	0
	.ascii	"        u32 read_i = i * LOOP_FACTOR + inner_i;" # string offset=2092
	.byte	0
	.ascii	"        if (read_i >= IN_SIZE)" # string offset=2140
	.byte	0
	.ascii	"        u8 *elem = ctx->in_ptr + read_i;" # string offset=2171
	.byte	0
	.ascii	"        u8 byte = *(u8 *)elem;" # string offset=2212
	.byte	0
	.ascii	"            u32 *elem_1 = ctx->out_ptr + write_i;" # string offset=2243
	.byte	0
	.ascii	"            *elem_1 = b1;"     # string offset=2293
	.byte	0
	.ascii	"        u32 b4 = (u32)(a4);"   # string offset=2319
	.byte	0
	.ascii	"            *elem_4 = b4;"     # string offset=2347
	.byte	0
	.ascii	"            *elem_3 = b3;"     # string offset=2373
	.byte	0
	.ascii	"            *elem_2 = b2;"     # string offset=2399
	.byte	0
	.ascii	"IN_SIZE"                       # string offset=2425
	.byte	0
	.ascii	"OUT_SIZE"                      # string offset=2433
	.byte	0
	.ascii	"LOOP_FACTOR"                   # string offset=2442
	.byte	0
	.ascii	"_license"                      # string offset=2454
	.byte	0
	.ascii	"LINUX_HAS_BPF_COOKIE"          # string offset=2463
	.byte	0
	.ascii	".kconfig"                      # string offset=2484
	.byte	0
	.ascii	".maps"                         # string offset=2493
	.byte	0
	.ascii	".rodata"                       # string offset=2499
	.byte	0
	.ascii	"license"                       # string offset=2507
	.byte	0
	.section	.BTF.ext,"",@progbits
	.short	60319                           # 0xeb9f
	.byte	1
	.byte	0
	.long	32
	.long	0
	.long	60
	.long	60
	.long	1812
	.long	1872
	.long	84
	.long	8                               # FuncInfo
	.long	518                             # FuncInfo section string offset=518
	.long	4
	.long	.Lfunc_begin0
	.long	48
	.long	.Lfunc_begin1
	.long	50
	.long	.Lfunc_begin2
	.long	52
	.long	.Lfunc_begin4
	.long	59
	.long	1477                            # FuncInfo section string offset=1477
	.long	1
	.long	.Lfunc_begin3
	.long	54
	.long	16                              # LineInfo
	.long	518                             # LineInfo section string offset=518
	.long	99
	.long	.Ltmp0
	.long	524
	.long	552
	.long	83975                           # Line 82 Col 7
	.long	.Ltmp3
	.long	524
	.long	552
	.long	83974                           # Line 82 Col 6
	.long	.Ltmp7
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp10
	.long	524
	.long	587
	.long	85000                           # Line 83 Col 8
	.long	.Ltmp13
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp14
	.long	524
	.long	616
	.long	88081                           # Line 86 Col 17
	.long	.Ltmp18
	.long	524
	.long	685
	.long	89098                           # Line 87 Col 10
	.long	.Ltmp21
	.long	524
	.long	685
	.long	89112                           # Line 87 Col 24
	.long	.Ltmp24
	.long	524
	.long	731
	.long	92169                           # Line 90 Col 9
	.long	.Ltmp27
	.long	524
	.long	767
	.long	102410                          # Line 100 Col 10
	.long	.Ltmp30
	.long	524
	.long	803
	.long	103430                          # Line 101 Col 6
	.long	.Ltmp33
	.long	524
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp34
	.long	524
	.long	821
	.long	106505                          # Line 104 Col 9
	.long	.Ltmp37
	.long	524
	.long	879
	.long	107526                          # Line 105 Col 6
	.long	.Ltmp40
	.long	524
	.long	891
	.long	110607                          # Line 108 Col 15
	.long	.Ltmp43
	.long	524
	.long	914
	.long	111617                          # Line 109 Col 1
	.long	.Lfunc_begin1
	.long	524
	.long	941
	.long	118784                          # Line 116 Col 0
	.long	.Ltmp52
	.long	524
	.long	1005
	.long	125959                          # Line 123 Col 7
	.long	.Ltmp55
	.long	524
	.long	552
	.long	83975                           # Line 82 Col 7
	.long	.Ltmp58
	.long	524
	.long	552
	.long	83974                           # Line 82 Col 6
	.long	.Ltmp62
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp65
	.long	524
	.long	587
	.long	85000                           # Line 83 Col 8
	.long	.Ltmp68
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp69
	.long	524
	.long	616
	.long	88081                           # Line 86 Col 17
	.long	.Ltmp73
	.long	524
	.long	685
	.long	89098                           # Line 87 Col 10
	.long	.Ltmp76
	.long	524
	.long	685
	.long	89112                           # Line 87 Col 24
	.long	.Ltmp79
	.long	524
	.long	731
	.long	92169                           # Line 90 Col 9
	.long	.Ltmp82
	.long	524
	.long	767
	.long	128010                          # Line 125 Col 10
	.long	.Ltmp85
	.long	524
	.long	803
	.long	129030                          # Line 126 Col 6
	.long	.Ltmp88
	.long	524
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp89
	.long	524
	.long	821
	.long	132105                          # Line 129 Col 9
	.long	.Ltmp93
	.long	524
	.long	879
	.long	133126                          # Line 130 Col 6
	.long	.Ltmp96
	.long	524
	.long	1016
	.long	136198                          # Line 133 Col 6
	.long	.Ltmp101
	.long	524
	.long	1054
	.long	139287                          # Line 136 Col 23
	.long	.Ltmp104
	.long	524
	.long	1054
	.long	139270                          # Line 136 Col 6
	.long	.Ltmp107
	.long	524
	.long	1085
	.long	142350                          # Line 139 Col 14
	.long	.Ltmp110
	.long	524
	.long	1119
	.long	143380                          # Line 140 Col 20
	.long	.Ltmp111
	.long	524
	.long	1119
	.long	143362                          # Line 140 Col 2
	.long	.Ltmp118
	.long	524
	.long	1150
	.long	148499                          # Line 145 Col 19
	.long	.Ltmp121
	.long	524
	.long	1150
	.long	148487                          # Line 145 Col 7
	.long	.Ltmp124
	.long	524
	.long	1177
	.long	156746                          # Line 153 Col 74
	.long	.Ltmp127
	.long	524
	.long	1177
	.long	156734                          # Line 153 Col 62
	.long	.Ltmp130
	.long	524
	.long	1177
	.long	156746                          # Line 153 Col 74
	.long	.Ltmp131
	.long	524
	.long	1177
	.long	156681                          # Line 153 Col 9
	.long	.Ltmp135
	.long	524
	.long	1260
	.long	157703                          # Line 154 Col 7
	.long	.Ltmp138
	.long	524
	.long	1177
	.long	169034                          # Line 165 Col 74
	.long	.Ltmp139
	.long	524
	.long	1177
	.long	169022                          # Line 165 Col 62
	.long	.Ltmp142
	.long	524
	.long	1177
	.long	169034                          # Line 165 Col 74
	.long	.Ltmp143
	.long	524
	.long	1177
	.long	168969                          # Line 165 Col 9
	.long	.Ltmp149
	.long	524
	.long	1260
	.long	169991                          # Line 166 Col 7
	.long	.Ltmp152
	.long	524
	.long	1271
	.long	172104                          # Line 168 Col 72
	.long	.Ltmp153
	.long	524
	.long	1271
	.long	172088                          # Line 168 Col 56
	.long	.Ltmp156
	.long	524
	.long	1271
	.long	172092                          # Line 168 Col 60
	.long	.Ltmp159
	.long	524
	.long	1271
	.long	172104                          # Line 168 Col 72
	.long	.Ltmp160
	.long	524
	.long	1271
	.long	172041                          # Line 168 Col 9
	.long	.Ltmp164
	.long	524
	.long	1260
	.long	173063                          # Line 169 Col 7
	.long	.Ltmp167
	.long	524
	.long	1352
	.long	187412                          # Line 183 Col 20
	.long	.Ltmp172
	.long	524
	.long	1352
	.long	187398                          # Line 183 Col 6
	.long	.Ltmp177
	.long	524
	.long	1385
	.long	188422                          # Line 184 Col 6
	.long	.Ltmp180
	.long	524
	.long	1385
	.long	188432                          # Line 184 Col 16
	.long	.Ltmp183
	.long	524
	.long	1385
	.long	188422                          # Line 184 Col 6
	.long	.Ltmp190
	.long	524
	.long	1412
	.long	192519                          # Line 188 Col 7
	.long	.Ltmp193
	.long	524
	.long	914
	.long	194561                          # Line 190 Col 1
	.long	.Ltmp196
	.long	524
	.long	552
	.long	83975                           # Line 82 Col 7
	.long	.Ltmp199
	.long	524
	.long	552
	.long	83974                           # Line 82 Col 6
	.long	.Ltmp203
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp206
	.long	524
	.long	587
	.long	85000                           # Line 83 Col 8
	.long	.Ltmp209
	.long	524
	.long	587
	.long	85005                           # Line 83 Col 13
	.long	.Ltmp210
	.long	524
	.long	616
	.long	88081                           # Line 86 Col 17
	.long	.Ltmp214
	.long	524
	.long	685
	.long	89098                           # Line 87 Col 10
	.long	.Ltmp217
	.long	524
	.long	685
	.long	89112                           # Line 87 Col 24
	.long	.Ltmp220
	.long	524
	.long	731
	.long	92169                           # Line 90 Col 9
	.long	.Ltmp223
	.long	524
	.long	767
	.long	208906                          # Line 204 Col 10
	.long	.Ltmp226
	.long	524
	.long	803
	.long	209926                          # Line 205 Col 6
	.long	.Ltmp229
	.long	524
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp230
	.long	524
	.long	821
	.long	213001                          # Line 208 Col 9
	.long	.Ltmp233
	.long	524
	.long	879
	.long	214022                          # Line 209 Col 6
	.long	.Ltmp236
	.long	524
	.long	1441
	.long	217103                          # Line 212 Col 15
	.long	.Ltmp239
	.long	524
	.long	914
	.long	218113                          # Line 213 Col 1
	.long	.Lfunc_begin4
	.long	1482
	.long	1976
	.long	61440                           # Line 60 Col 0
	.long	.Ltmp286
	.long	1482
	.long	2029
	.long	63493                           # Line 62 Col 5
	.long	.Ltmp289
	.long	1482
	.long	2092
	.long	65560                           # Line 64 Col 24
	.long	.Ltmp292
	.long	1482
	.long	2029
	.long	63493                           # Line 62 Col 5
	.long	.Ltmp295
	.long	1482
	.long	2140
	.long	67604                           # Line 66 Col 20
	.long	.Ltmp298
	.long	1482
	.long	2140
	.long	67597                           # Line 66 Col 13
	.long	.Ltmp301
	.long	1482
	.long	2171
	.long	72729                           # Line 71 Col 25
	.long	.Ltmp304
	.long	1482
	.long	2212
	.long	73747                           # Line 72 Col 19
	.long	.Ltmp307
	.long	1482
	.long	2243
	.long	89120                           # Line 87 Col 32
	.long	.Ltmp310
	.long	1482
	.long	2293
	.long	90133                           # Line 88 Col 21
	.long	.Ltmp313
	.long	1482
	.long	2212
	.long	73747                           # Line 72 Col 19
	.long	.Ltmp316
	.long	1482
	.long	2319
	.long	82962                           # Line 81 Col 18
	.long	.Ltmp319
	.long	1482
	.long	2347
	.long	96277                           # Line 94 Col 21
	.long	.Ltmp324
	.long	1482
	.long	2293
	.long	90133                           # Line 88 Col 21
	.long	.Ltmp327
	.long	1482
	.long	2373
	.long	94229                           # Line 92 Col 21
	.long	.Ltmp334
	.long	1482
	.long	2399
	.long	92181                           # Line 90 Col 21
	.long	.Ltmp337
	.long	1482
	.long	2029
	.long	63523                           # Line 62 Col 35
	.long	.Ltmp338
	.long	1482
	.long	2029
	.long	63546                           # Line 62 Col 58
	.long	.Ltmp341
	.long	1482
	.long	2029
	.long	63493                           # Line 62 Col 5
	.long	.Ltmp344
	.long	1482
	.long	914
	.long	101377                          # Line 99 Col 1
	.long	1477                            # LineInfo section string offset=1477
	.long	13
	.long	.Lfunc_begin3
	.long	1482
	.long	1556
	.long	38912                           # Line 38 Col 0
	.long	.Ltmp249
	.long	1482
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp252
	.long	1482
	.long	1595
	.long	40989                           # Line 40 Col 29
	.long	.Ltmp257
	.long	1482
	.long	1647
	.long	43017                           # Line 42 Col 9
	.long	.Ltmp260
	.long	1482
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp261
	.long	1482
	.long	1665
	.long	44052                           # Line 43 Col 20
	.long	.Ltmp265
	.long	1482
	.long	1723
	.long	45077                           # Line 44 Col 21
	.long	.Ltmp270
	.long	1482
	.long	1783
	.long	47121                           # Line 46 Col 17
	.long	.Ltmp275
	.long	1482
	.long	1812
	.long	54294                           # Line 53 Col 22
	.long	.Ltmp276
	.long	1482
	.long	1844
	.long	53269                           # Line 52 Col 21
	.long	.Ltmp277
	.long	1482
	.long	1812
	.long	54294                           # Line 53 Col 22
	.long	.Ltmp278
	.long	1482
	.long	1874
	.long	56325                           # Line 55 Col 5
	.long	.Ltmp281
	.long	1482
	.long	914
	.long	59393                           # Line 58 Col 1
	.long	16                              # FieldReloc
	.long	518                             # Field reloc section string offset=518
	.long	3
	.long	.Ltmp6
	.long	45
	.long	582
	.long	0
	.long	.Ltmp61
	.long	45
	.long	582
	.long	0
	.long	.Ltmp202
	.long	45
	.long	582
	.long	0
	.long	1477                            # Field reloc section string offset=1477
	.long	1
	.long	.Ltmp244
	.long	45
	.long	1590
	.long	0
	.addrsig
	.addrsig_sym bpf_prog
	.addrsig_sym decode_one
	.addrsig_sym __bpf_usdt_specs
	.addrsig_sym in_bytes
	.addrsig_sym out_bytes
	.addrsig_sym _license
	.addrsig_sym __bpf_usdt_ip_to_spec_id
	.section	.debug_line,"",@progbits
.Lline_table_start0:
