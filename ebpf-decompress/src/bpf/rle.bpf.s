	.text
	.file	"rle.bpf.c"
	.file	0 "/home/mat/src/portable-decompress" "ebpf-decompress/src/bpf/rle.bpf.c" md5 0xe62dfaa0f8e3da44093045b940730533
	.file	1 "./ebpf-decompress/src/bpf" "vmlinux.h" md5 0xd609f28e272dd8c860d6fdddd8b15647
	.file	2 "./ebpf-decompress/src/bpf" "rle.bpf.c" md5 0xe62dfaa0f8e3da44093045b940730533
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
	.loc	2 41 0                          # ./ebpf-decompress/src/bpf/rle.bpf.c:41:0
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
	.loc	2 0 0 is_stmt 0                 # ./ebpf-decompress/src/bpf/rle.bpf.c:0:0
.Ltmp248:
.Ltmp249:
	r1 += -32
.Ltmp250:
	.loc	2 42 29 prologue_end is_stmt 1  # ./ebpf-decompress/src/bpf/rle.bpf.c:42:29
.Ltmp251:
.Ltmp252:
	r2 = 8
	call 113
.Ltmp253:
.Ltmp254:
	#DEBUG_VALUE: bpf_prog:len <- undef
	.loc	2 0 29 is_stmt 0                # ./ebpf-decompress/src/bpf/rle.bpf.c:0:29
	r1 = 0
.Ltmp255:
.Ltmp256:
	#DEBUG_VALUE: bpf_prog:zero <- 0
	.loc	2 44 9 is_stmt 1                # ./ebpf-decompress/src/bpf/rle.bpf.c:44:9
.Ltmp257:
	*(u32 *)(r10 - 4) = r1
.Ltmp258:
.Ltmp259:
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 28, DW_OP_deref] $r10
	.loc	2 0 9 is_stmt 0                 # ./ebpf-decompress/src/bpf/rle.bpf.c:0:9
	r6 = r10
.Ltmp260:
	r6 += -4
	.loc	2 45 20 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:45:20
.Ltmp261:
	r1 = in_bytes ll
	r2 = r6
	call 1
.Ltmp262:
	r7 = r0
.Ltmp263:
.Ltmp264:
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r7
	.loc	2 46 21                         # ./ebpf-decompress/src/bpf/rle.bpf.c:46:21
.Ltmp265:
	r1 = out_bytes ll
	r2 = r6
	call 1
.Ltmp266:
.Ltmp267:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	.loc	2 0 21 is_stmt 0                # ./ebpf-decompress/src/bpf/rle.bpf.c:0:21
	r6 = 1
.Ltmp268:
	.loc	2 48 17 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:48:17
.Ltmp269:
.Ltmp270:
	if r7 == 0 goto LBB3_3
.Ltmp271:
.Ltmp272:
# %bb.1:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r7
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 28, DW_OP_deref] $r10
	if r0 == 0 goto LBB3_3
.Ltmp273:
.Ltmp274:
# %bb.2:
	#DEBUG_VALUE: bpf_prog:out_ptr <- $r0
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r7
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 28, DW_OP_deref] $r10
	.loc	2 55 22                         # ./ebpf-decompress/src/bpf/rle.bpf.c:55:22
.Ltmp275:
	*(u64 *)(r10 - 24) = r0
	.loc	2 54 21                         # ./ebpf-decompress/src/bpf/rle.bpf.c:54:21
.Ltmp276:
	*(u64 *)(r10 - 32) = r7
	r6 = 0
	.loc	2 56 21                         # ./ebpf-decompress/src/bpf/rle.bpf.c:56:21
.Ltmp277:
	*(u64 *)(r10 - 16) = r6
	r3 = r10
	.loc	2 55 22                         # ./ebpf-decompress/src/bpf/rle.bpf.c:55:22
.Ltmp278:
	r3 += -32
	.loc	2 59 5                          # ./ebpf-decompress/src/bpf/rle.bpf.c:59:5
.Ltmp279:
	r1 = 32768
	r2 = decode_one ll
	r4 = 0
	call 181
.Ltmp280:
.Ltmp281:
LBB3_3:
	#DEBUG_VALUE: bpf_prog:in_ptr <- $r7
	#DEBUG_VALUE: bpf_prog:zero <- [DW_OP_plus_uconst 28, DW_OP_deref] $r10
	.loc	2 62 1                          # ./ebpf-decompress/src/bpf/rle.bpf.c:62:1
.Ltmp282:
	r0 = r6
	exit
.Ltmp283:
.Ltmp284:
.Lfunc_end3:
	.size	bpf_prog, .Lfunc_end3-bpf_prog
	.cfi_endproc
                                        # -- End function
	.text
	.p2align	3                               # -- Begin function decode_one
	.type	decode_one,@function
decode_one:                             # @decode_one
.Lfunc_begin4:
	.loc	2 65 0                          # ./ebpf-decompress/src/bpf/rle.bpf.c:65:0
	.cfi_startproc
# %bb.0:
	#DEBUG_VALUE: decode_one:ctx <- $r2
	r1 = 0
	#DEBUG_VALUE: inner_i <- 0
	#DEBUG_VALUE: decode_one:i <- undef
	r3 = 0
	goto LBB4_1
.Ltmp285:
.Ltmp286:
LBB4_10:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 104 22 prologue_end           # ./ebpf-decompress/src/bpf/rle.bpf.c:104:22
.Ltmp287:
	r5 = *(u32 *)(r2 + 20)
.Ltmp288:
.Ltmp289:
	.loc	2 104 17 is_stmt 0              # ./ebpf-decompress/src/bpf/rle.bpf.c:104:17
.Ltmp290:
	if r5 > 134217724 goto LBB4_13
.Ltmp291:
.Ltmp292:
# %bb.11:                               #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 108 32 is_stmt 1              # ./ebpf-decompress/src/bpf/rle.bpf.c:108:32
.Ltmp293:
	r0 = *(u64 *)(r2 + 8)
	.loc	2 108 40 is_stmt 0              # ./ebpf-decompress/src/bpf/rle.bpf.c:108:40
.Ltmp294:
	r0 += r5
.Ltmp295:
.Ltmp296:
	#DEBUG_VALUE: target <- $r0
	.loc	2 109 21 is_stmt 1              # ./ebpf-decompress/src/bpf/rle.bpf.c:109:21
.Ltmp297:
	*(u32 *)(r0 + 0) = r4
	.loc	2 110 26                        # ./ebpf-decompress/src/bpf/rle.bpf.c:110:26
.Ltmp298:
	r4 = *(u32 *)(r2 + 20)
	r4 += 4
	*(u32 *)(r2 + 20) = r4
.Ltmp299:
.Ltmp300:
LBB4_12:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 0 26 is_stmt 0                # ./ebpf-decompress/src/bpf/rle.bpf.c:0:26
	r0 = 0
	.loc	2 66 58 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:66:58
.Ltmp301:
	r3 += 1
.Ltmp302:
.Ltmp303:
	#DEBUG_VALUE: inner_i <- $r3
	r4 = r3
	r4 <<= 32
	r4 >>= 32
.Ltmp304:
.Ltmp305:
	.loc	2 66 5 is_stmt 0                # ./ebpf-decompress/src/bpf/rle.bpf.c:66:5
.Ltmp306:
	if r4 != 256 goto LBB4_1
.Ltmp307:
.Ltmp308:
LBB4_13:
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 115 1 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:115:1
.Ltmp309:
	exit
.Ltmp310:
.Ltmp311:
LBB4_1:                                 # =>This Loop Header: Depth=1
                                        #     Child Loop BB4_7 Depth 2
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: inner_i <- $r3
	.loc	2 0 1 is_stmt 0                 # ./ebpf-decompress/src/bpf/rle.bpf.c:0:1
	r0 = 1
.Ltmp312:
	.loc	2 68 18 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:68:18
.Ltmp313:
.Ltmp314:
	r6 = *(u32 *)(r2 + 16)
.Ltmp315:
.Ltmp316:
	.loc	2 68 13 is_stmt 0               # ./ebpf-decompress/src/bpf/rle.bpf.c:68:13
.Ltmp317:
	if r6 > 8388604 goto LBB4_13
.Ltmp318:
.Ltmp319:
# %bb.2:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 73 29 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:73:29
.Ltmp320:
	r5 = *(u64 *)(r2 + 0)
	.loc	2 73 36 is_stmt 0               # ./ebpf-decompress/src/bpf/rle.bpf.c:73:36
.Ltmp321:
	r4 = r5
	r4 += r6
.Ltmp322:
.Ltmp323:
	#DEBUG_VALUE: val_ptr <- $r4
	.loc	2 74 19 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:74:19
.Ltmp324:
	r4 = *(u32 *)(r4 + 0)
.Ltmp325:
.Ltmp326:
	#DEBUG_VALUE: val <- undef
	.loc	2 75 21                         # ./ebpf-decompress/src/bpf/rle.bpf.c:75:21
.Ltmp327:
	r7 = r6
	r7 += 4
	*(u32 *)(r2 + 16) = r7
	.loc	2 74 19                         # ./ebpf-decompress/src/bpf/rle.bpf.c:74:19
.Ltmp328:
	r4 <<= 32
	r4 s>>= 32
	.loc	2 77 13                         # ./ebpf-decompress/src/bpf/rle.bpf.c:77:13
.Ltmp329:
	if r4 s> -1 goto LBB4_10
.Ltmp330:
.Ltmp331:
# %bb.3:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: run_len <- undef
	.loc	2 80 17                         # ./ebpf-decompress/src/bpf/rle.bpf.c:80:17
.Ltmp332:
	if r6 > 8388600 goto LBB4_13
.Ltmp333:
.Ltmp334:
# %bb.4:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: run_len <- undef
	.loc	2 84 41                         # ./ebpf-decompress/src/bpf/rle.bpf.c:84:41
.Ltmp335:
	r7 <<= 32
	r7 >>= 32
	r5 += r7
.Ltmp336:
.Ltmp337:
	#DEBUG_VALUE: elem_ptr <- $r5
	.loc	2 85 24                         # ./ebpf-decompress/src/bpf/rle.bpf.c:85:24
.Ltmp338:
	r5 = *(u32 *)(r5 + 0)
.Ltmp339:
.Ltmp340:
	#DEBUG_VALUE: elem <- undef
	.loc	2 86 25                         # ./ebpf-decompress/src/bpf/rle.bpf.c:86:25
.Ltmp341:
	r6 += 8
	*(u32 *)(r2 + 16) = r6
.Ltmp342:
.Ltmp343:
	#DEBUG_VALUE: i <- 0
	.loc	2 88 13                         # ./ebpf-decompress/src/bpf/rle.bpf.c:88:13
.Ltmp344:
	if r1 != 0 goto LBB4_12
.Ltmp345:
.Ltmp346:
# %bb.5:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: i <- 0
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 0 0 is_stmt 0                 # ./ebpf-decompress/src/bpf/rle.bpf.c:0:0
.Ltmp347:
	r4 &= 2147483647
.Ltmp348:
.Ltmp349:
	#DEBUG_VALUE: run_len <- $r4
	.loc	2 88 13                         # ./ebpf-decompress/src/bpf/rle.bpf.c:88:13
.Ltmp350:
	if r4 == 0 goto LBB4_12
.Ltmp351:
.Ltmp352:
# %bb.6:                                #   in Loop: Header=BB4_1 Depth=1
	#DEBUG_VALUE: run_len <- $r4
	#DEBUG_VALUE: i <- 0
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 0 13                          # ./ebpf-decompress/src/bpf/rle.bpf.c:0:13
	r6 = 0
.Ltmp353:
	.loc	2 93 26 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:93:26
.Ltmp354:
.Ltmp355:
	r7 = *(u32 *)(r2 + 20)
.Ltmp356:
.Ltmp357:
LBB4_7:                                 #   Parent Loop BB4_1 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	#DEBUG_VALUE: run_len <- $r4
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	#DEBUG_VALUE: i <- $r6
	.loc	2 93 34 is_stmt 0               # ./ebpf-decompress/src/bpf/rle.bpf.c:93:34
.Ltmp358:
	r7 <<= 32
	r7 >>= 32
.Ltmp359:
.Ltmp360:
	.loc	2 93 21                         # ./ebpf-decompress/src/bpf/rle.bpf.c:93:21
.Ltmp361:
	if r7 > 134217724 goto LBB4_13
.Ltmp362:
.Ltmp363:
# %bb.8:                                #   in Loop: Header=BB4_7 Depth=2
	#DEBUG_VALUE: i <- $r6
	#DEBUG_VALUE: run_len <- $r4
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 96 36 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:96:36
.Ltmp364:
	r8 = *(u64 *)(r2 + 8)
	.loc	2 96 44 is_stmt 0               # ./ebpf-decompress/src/bpf/rle.bpf.c:96:44
.Ltmp365:
	r8 += r7
.Ltmp366:
.Ltmp367:
	#DEBUG_VALUE: target <- $r8
	.loc	2 97 25 is_stmt 1               # ./ebpf-decompress/src/bpf/rle.bpf.c:97:25
.Ltmp368:
	*(u32 *)(r8 + 0) = r5
	.loc	2 98 30                         # ./ebpf-decompress/src/bpf/rle.bpf.c:98:30
.Ltmp369:
	r7 = *(u32 *)(r2 + 20)
	r7 += 4
	*(u32 *)(r2 + 20) = r7
.Ltmp370:
.Ltmp371:
	#DEBUG_VALUE: i <- undef
	.loc	2 88 39                         # ./ebpf-decompress/src/bpf/rle.bpf.c:88:39
.Ltmp372:
	r8 = r6
.Ltmp373:
.Ltmp374:
	r8 <<= 32
	r8 >>= 32
.Ltmp375:
.Ltmp376:
	.loc	2 88 13 is_stmt 0               # ./ebpf-decompress/src/bpf/rle.bpf.c:88:13
.Ltmp377:
	if r8 > 30 goto LBB4_12
.Ltmp378:
.Ltmp379:
# %bb.9:                                #   in Loop: Header=BB4_7 Depth=2
	#DEBUG_VALUE: run_len <- $r4
	#DEBUG_VALUE: inner_i <- $r3
	#DEBUG_VALUE: decode_one:ctx <- $r2
	.loc	2 0 13                          # ./ebpf-decompress/src/bpf/rle.bpf.c:0:13
	r6 += 1
.Ltmp380:
.Ltmp381:
	#DEBUG_VALUE: i <- $r6
	r8 = r6
	r8 <<= 32
	r8 >>= 32
	.loc	2 88 13                         # ./ebpf-decompress/src/bpf/rle.bpf.c:88:13
	if r4 > r8 goto LBB4_7
	goto LBB4_12
.Ltmp382:
.Ltmp383:
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
	.long	256                             # 0x100
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
	.long	31                              # Offset entry count
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
	.byte	28                              # 28
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc22:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp263-.Lfunc_begin3         #   starting offset
	.uleb128 .Lfunc_end3-.Lfunc_begin3      #   ending offset
	.byte	1                               # Loc expr size
	.byte	87                              # DW_OP_reg7
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc23:
	.byte	1                               # DW_LLE_base_addressx
	.byte	16                              #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp266-.Lfunc_begin3         #   starting offset
	.uleb128 .Ltmp280-.Lfunc_begin3         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc24:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp285-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp307-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp310-.Lfunc_begin0         #   starting offset
	.uleb128 .Lfunc_end4-.Lfunc_begin0      #   ending offset
	.byte	1                               # Loc expr size
	.byte	83                              # DW_OP_reg3
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc25:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp295-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp299-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	80                              # DW_OP_reg0
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc26:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp322-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp325-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	84                              # DW_OP_reg4
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc27:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp348-.Lfunc_begin0         #   starting offset
	.uleb128 .Lfunc_end4-.Lfunc_begin0      #   ending offset
	.byte	1                               # Loc expr size
	.byte	84                              # DW_OP_reg4
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc28:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp336-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp339-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	85                              # DW_OP_reg5
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc29:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp342-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp356-.Lfunc_begin0         #   ending offset
	.byte	2                               # Loc expr size
	.byte	48                              # DW_OP_lit0
	.byte	159                             # DW_OP_stack_value
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp356-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp370-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp380-.Lfunc_begin0         #   starting offset
	.uleb128 .Lfunc_end4-.Lfunc_begin0      #   ending offset
	.byte	1                               # Loc expr size
	.byte	86                              # DW_OP_reg6
	.byte	0                               # DW_LLE_end_of_list
.Ldebug_loc30:
	.byte	1                               # DW_LLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_LLE_offset_pair
	.uleb128 .Ltmp366-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp373-.Lfunc_begin0         #   ending offset
	.byte	1                               # Loc expr size
	.byte	88                              # DW_OP_reg8
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
	.byte	3                               # Abbreviation Code
	.byte	38                              # DW_TAG_const_type
	.byte	0                               # DW_CHILDREN_no
	.byte	73                              # DW_AT_type
	.byte	19                              # DW_FORM_ref4
	.byte	0                               # EOM(1)
	.byte	0                               # EOM(2)
	.byte	4                               # Abbreviation Code
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
	.byte	5                               # Abbreviation Code
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
	.byte	1                               # Abbrev [1] 0xc:0x662 DW_TAG_compile_unit
	.byte	0                               # DW_AT_producer
	.short	29                              # DW_AT_language
	.byte	1                               # DW_AT_name
	.long	.Lstr_offsets_base0             # DW_AT_str_offsets_base
	.long	.Lline_table_start0             # DW_AT_stmt_list
	.byte	2                               # DW_AT_comp_dir
	.quad	0                               # DW_AT_low_pc
	.byte	3                               # DW_AT_ranges
	.long	.Laddr_table_base0              # DW_AT_addr_base
	.long	.Lrnglists_table_base0          # DW_AT_rnglists_base
	.long	.Lloclists_table_base0          # DW_AT_loclists_base
	.byte	2                               # Abbrev [2] 0x2f:0xb DW_TAG_variable
	.byte	3                               # DW_AT_name
	.long	58                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	6                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	0
	.byte	3                               # Abbrev [3] 0x3a:0x5 DW_TAG_const_type
	.long	63                              # DW_AT_type
	.byte	4                               # Abbrev [4] 0x3f:0x8 DW_TAG_typedef
	.long	71                              # DW_AT_type
	.byte	6                               # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	26                              # DW_AT_decl_line
	.byte	4                               # Abbrev [4] 0x47:0x8 DW_TAG_typedef
	.long	79                              # DW_AT_type
	.byte	5                               # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	14                              # DW_AT_decl_line
	.byte	5                               # Abbrev [5] 0x4f:0x4 DW_TAG_base_type
	.byte	4                               # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	4                               # DW_AT_byte_size
	.byte	2                               # Abbrev [2] 0x53:0xb DW_TAG_variable
	.byte	7                               # DW_AT_name
	.long	58                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	7                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	1
	.byte	2                               # Abbrev [2] 0x5e:0xb DW_TAG_variable
	.byte	8                               # DW_AT_name
	.long	58                              # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	8                               # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	2
	.byte	2                               # Abbrev [2] 0x69:0xb DW_TAG_variable
	.byte	9                               # DW_AT_name
	.long	116                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	117                             # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	3
	.byte	6                               # Abbrev [6] 0x74:0xc DW_TAG_array_type
	.long	128                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x79:0x6 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.byte	4                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	5                               # Abbrev [5] 0x80:0x4 DW_TAG_base_type
	.byte	10                              # DW_AT_name
	.byte	6                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	8                               # Abbrev [8] 0x84:0x4 DW_TAG_base_type
	.byte	11                              # DW_AT_name
	.byte	8                               # DW_AT_byte_size
	.byte	7                               # DW_AT_encoding
	.byte	2                               # Abbrev [2] 0x88:0xb DW_TAG_variable
	.byte	12                              # DW_AT_name
	.long	147                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	68                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	4
	.byte	9                               # Abbrev [9] 0x93:0x29 DW_TAG_structure_type
	.byte	32                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	63                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x97:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	188                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xa0:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	209                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	65                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xa9:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	227                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	66                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xb2:0x9 DW_TAG_member
	.byte	17                              # DW_AT_name
	.long	232                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	67                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0xbc:0x5 DW_TAG_pointer_type
	.long	193                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0xc1:0xc DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0xc6:0x6 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.byte	2                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	5                               # Abbrev [5] 0xcd:0x4 DW_TAG_base_type
	.byte	14                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	4                               # DW_AT_byte_size
	.byte	11                              # Abbrev [11] 0xd1:0x5 DW_TAG_pointer_type
	.long	214                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0xd6:0xd DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0xdb:0x7 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.short	256                             # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0xe3:0x5 DW_TAG_pointer_type
	.long	205                             # DW_AT_type
	.byte	11                              # Abbrev [11] 0xe8:0x5 DW_TAG_pointer_type
	.long	237                             # DW_AT_type
	.byte	13                              # Abbrev [13] 0xed:0x21 DW_TAG_structure_type
	.byte	36                              # DW_AT_name
	.byte	208                             # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	57                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0xf2:0x9 DW_TAG_member
	.byte	18                              # DW_AT_name
	.long	270                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	58                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0xfb:0x9 DW_TAG_member
	.byte	34                              # DW_AT_name
	.long	333                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	59                              # DW_AT_decl_line
	.byte	192                             # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x104:0x9 DW_TAG_member
	.byte	35                              # DW_AT_name
	.long	364                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	60                              # DW_AT_decl_line
	.byte	200                             # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	6                               # Abbrev [6] 0x10e:0xc DW_TAG_array_type
	.long	282                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x113:0x6 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.byte	12                              # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	13                              # Abbrev [13] 0x11a:0x33 DW_TAG_structure_type
	.byte	33                              # DW_AT_name
	.byte	16                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	39                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x11f:0x9 DW_TAG_member
	.byte	19                              # DW_AT_name
	.long	333                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	41                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x128:0x9 DW_TAG_member
	.byte	22                              # DW_AT_name
	.long	345                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	43                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x131:0x9 DW_TAG_member
	.byte	27                              # DW_AT_name
	.long	364                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	45                              # DW_AT_decl_line
	.byte	12                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x13a:0x9 DW_TAG_member
	.byte	29                              # DW_AT_name
	.long	368                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	47                              # DW_AT_decl_line
	.byte	14                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x143:0x9 DW_TAG_member
	.byte	32                              # DW_AT_name
	.long	128                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	52                              # DW_AT_decl_line
	.byte	15                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	4                               # Abbrev [4] 0x14d:0x8 DW_TAG_typedef
	.long	341                             # DW_AT_type
	.byte	21                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	18                              # DW_AT_decl_line
	.byte	5                               # Abbrev [5] 0x155:0x4 DW_TAG_base_type
	.byte	20                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	14                              # Abbrev [14] 0x159:0x13 DW_TAG_enumeration_type
	.long	79                              # DW_AT_type
	.byte	26                              # DW_AT_name
	.byte	4                               # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	33                              # DW_AT_decl_line
	.byte	15                              # Abbrev [15] 0x162:0x3 DW_TAG_enumerator
	.byte	23                              # DW_AT_name
	.byte	0                               # DW_AT_const_value
	.byte	15                              # Abbrev [15] 0x165:0x3 DW_TAG_enumerator
	.byte	24                              # DW_AT_name
	.byte	1                               # DW_AT_const_value
	.byte	15                              # Abbrev [15] 0x168:0x3 DW_TAG_enumerator
	.byte	25                              # DW_AT_name
	.byte	2                               # DW_AT_const_value
	.byte	0                               # End Of Children Mark
	.byte	5                               # Abbrev [5] 0x16c:0x4 DW_TAG_base_type
	.byte	28                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	2                               # DW_AT_byte_size
	.byte	4                               # Abbrev [4] 0x170:0x8 DW_TAG_typedef
	.long	376                             # DW_AT_type
	.byte	31                              # DW_AT_name
	.byte	1                               # DW_AT_decl_file
	.byte	54                              # DW_AT_decl_line
	.byte	5                               # Abbrev [5] 0x178:0x4 DW_TAG_base_type
	.byte	30                              # DW_AT_name
	.byte	2                               # DW_AT_encoding
	.byte	1                               # DW_AT_byte_size
	.byte	2                               # Abbrev [2] 0x17c:0xb DW_TAG_variable
	.byte	37                              # DW_AT_name
	.long	391                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	75                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	5
	.byte	9                               # Abbrev [9] 0x187:0x29 DW_TAG_structure_type
	.byte	32                              # DW_AT_byte_size
	.byte	3                               # DW_AT_decl_file
	.byte	70                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x18b:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	432                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	71                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x194:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	449                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	72                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x19d:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	467                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	73                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x1a6:0x9 DW_TAG_member
	.byte	17                              # DW_AT_name
	.long	476                             # DW_AT_type
	.byte	3                               # DW_AT_decl_file
	.byte	74                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1b0:0x5 DW_TAG_pointer_type
	.long	437                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x1b5:0xc DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	7                               # Abbrev [7] 0x1ba:0x6 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.byte	1                               # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1c1:0x5 DW_TAG_pointer_type
	.long	454                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x1c6:0xd DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0x1cb:0x7 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.short	1024                            # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x1d3:0x5 DW_TAG_pointer_type
	.long	472                             # DW_AT_type
	.byte	5                               # Abbrev [5] 0x1d8:0x4 DW_TAG_base_type
	.byte	38                              # DW_AT_name
	.byte	5                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	11                              # Abbrev [11] 0x1dc:0x5 DW_TAG_pointer_type
	.long	71                              # DW_AT_type
	.byte	2                               # Abbrev [2] 0x1e1:0xb DW_TAG_variable
	.byte	39                              # DW_AT_name
	.long	492                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	18                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	6
	.byte	9                               # Abbrev [9] 0x1ec:0x32 DW_TAG_structure_type
	.byte	40                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	10                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x1f0:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	188                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	12                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x1f9:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	542                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	13                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x202:0x9 DW_TAG_member
	.byte	40                              # DW_AT_name
	.long	547                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	14                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x20b:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	432                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	15                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x214:0x9 DW_TAG_member
	.byte	41                              # DW_AT_name
	.long	567                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	16                              # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x21e:0x5 DW_TAG_pointer_type
	.long	63                              # DW_AT_type
	.byte	11                              # Abbrev [11] 0x223:0x5 DW_TAG_pointer_type
	.long	552                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x228:0xf DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	16                              # Abbrev [16] 0x22d:0x9 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.long	8388608                         # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x237:0x5 DW_TAG_pointer_type
	.long	572                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x23c:0xd DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	12                              # Abbrev [12] 0x241:0x7 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.short	1152                            # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	2                               # Abbrev [2] 0x249:0xb DW_TAG_variable
	.byte	42                              # DW_AT_name
	.long	596                             # DW_AT_type
                                        # DW_AT_external
	.byte	2                               # DW_AT_decl_file
	.byte	27                              # DW_AT_decl_line
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	7
	.byte	9                               # Abbrev [9] 0x254:0x32 DW_TAG_structure_type
	.byte	40                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	20                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x258:0x9 DW_TAG_member
	.byte	13                              # DW_AT_name
	.long	188                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	22                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x261:0x9 DW_TAG_member
	.byte	16                              # DW_AT_name
	.long	542                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	23                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x26a:0x9 DW_TAG_member
	.byte	40                              # DW_AT_name
	.long	646                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	24                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x273:0x9 DW_TAG_member
	.byte	15                              # DW_AT_name
	.long	432                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	25                              # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x27c:0x9 DW_TAG_member
	.byte	41                              # DW_AT_name
	.long	449                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	26                              # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x286:0x5 DW_TAG_pointer_type
	.long	651                             # DW_AT_type
	.byte	6                               # Abbrev [6] 0x28b:0xf DW_TAG_array_type
	.long	205                             # DW_AT_type
	.byte	16                              # Abbrev [16] 0x290:0x9 DW_TAG_subrange_type
	.long	132                             # DW_AT_type
	.long	134217728                       # DW_AT_count
	.byte	0                               # End Of Children Mark
	.byte	17                              # Abbrev [17] 0x29a:0xb DW_TAG_variable
	.byte	43                              # DW_AT_name
	.long	677                             # DW_AT_type
                                        # DW_AT_external
	.byte	3                               # DW_AT_decl_file
	.byte	77                              # DW_AT_decl_line
                                        # DW_AT_declaration
	.byte	2                               # DW_AT_location
	.byte	161
	.byte	8
	.byte	3                               # Abbrev [3] 0x2a5:0x5 DW_TAG_const_type
	.long	376                             # DW_AT_type
	.byte	18                              # Abbrev [18] 0x2aa:0x9 DW_TAG_variable
	.byte	44                              # DW_AT_name
	.long	691                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	4087                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2b3:0x5 DW_TAG_pointer_type
	.long	696                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2b8:0xb DW_TAG_subroutine_type
	.long	333                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x2bd:0x5 DW_TAG_formal_parameter
	.long	707                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	21                              # Abbrev [21] 0x2c3:0x1 DW_TAG_pointer_type
	.byte	22                              # Abbrev [22] 0x2c4:0x8 DW_TAG_variable
	.byte	45                              # DW_AT_name
	.long	716                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.byte	56                              # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2cc:0x5 DW_TAG_pointer_type
	.long	721                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2d1:0x10 DW_TAG_subroutine_type
	.long	707                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x2d6:0x5 DW_TAG_formal_parameter
	.long	707                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x2db:0x5 DW_TAG_formal_parameter
	.long	737                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x2e1:0x5 DW_TAG_pointer_type
	.long	742                             # DW_AT_type
	.byte	23                              # Abbrev [23] 0x2e6:0x1 DW_TAG_const_type
	.byte	18                              # Abbrev [18] 0x2e7:0x9 DW_TAG_variable
	.byte	46                              # DW_AT_name
	.long	752                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	2796                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x2f0:0x5 DW_TAG_pointer_type
	.long	757                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x2f5:0x15 DW_TAG_subroutine_type
	.long	472                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x2fa:0x5 DW_TAG_formal_parameter
	.long	707                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x2ff:0x5 DW_TAG_formal_parameter
	.long	71                              # DW_AT_type
	.byte	20                              # Abbrev [20] 0x304:0x5 DW_TAG_formal_parameter
	.long	737                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	18                              # Abbrev [18] 0x30a:0x9 DW_TAG_variable
	.byte	47                              # DW_AT_name
	.long	752                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	2785                            # DW_AT_decl_line
	.byte	18                              # Abbrev [18] 0x313:0x9 DW_TAG_variable
	.byte	48                              # DW_AT_name
	.long	796                             # DW_AT_type
	.byte	4                               # DW_AT_decl_file
	.short	4216                            # DW_AT_decl_line
	.byte	11                              # Abbrev [11] 0x31c:0x5 DW_TAG_pointer_type
	.long	801                             # DW_AT_type
	.byte	19                              # Abbrev [19] 0x321:0x1a DW_TAG_subroutine_type
	.long	472                             # DW_AT_type
                                        # DW_AT_prototyped
	.byte	20                              # Abbrev [20] 0x326:0x5 DW_TAG_formal_parameter
	.long	71                              # DW_AT_type
	.byte	20                              # Abbrev [20] 0x32b:0x5 DW_TAG_formal_parameter
	.long	707                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x330:0x5 DW_TAG_formal_parameter
	.long	707                             # DW_AT_type
	.byte	20                              # Abbrev [20] 0x335:0x5 DW_TAG_formal_parameter
	.long	333                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x33b:0x5 DW_TAG_pointer_type
	.long	832                             # DW_AT_type
	.byte	24                              # Abbrev [24] 0x340:0xd9 DW_TAG_structure_type
	.byte	71                              # DW_AT_name
	.byte	168                             # DW_AT_byte_size
	.byte	1                               # DW_AT_decl_file
	.short	1070                            # DW_AT_decl_line
	.byte	25                              # Abbrev [25] 0x346:0xa DW_TAG_member
	.byte	49                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1071                            # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x350:0xa DW_TAG_member
	.byte	51                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1072                            # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x35a:0xa DW_TAG_member
	.byte	52                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1073                            # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x364:0xa DW_TAG_member
	.byte	53                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1074                            # DW_AT_decl_line
	.byte	24                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x36e:0xa DW_TAG_member
	.byte	54                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1075                            # DW_AT_decl_line
	.byte	32                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x378:0xa DW_TAG_member
	.byte	55                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1076                            # DW_AT_decl_line
	.byte	40                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x382:0xa DW_TAG_member
	.byte	56                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1077                            # DW_AT_decl_line
	.byte	48                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x38c:0xa DW_TAG_member
	.byte	57                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1078                            # DW_AT_decl_line
	.byte	56                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x396:0xa DW_TAG_member
	.byte	58                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1079                            # DW_AT_decl_line
	.byte	64                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3a0:0xa DW_TAG_member
	.byte	59                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1080                            # DW_AT_decl_line
	.byte	72                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3aa:0xa DW_TAG_member
	.byte	60                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1081                            # DW_AT_decl_line
	.byte	80                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3b4:0xa DW_TAG_member
	.byte	61                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1082                            # DW_AT_decl_line
	.byte	88                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3be:0xa DW_TAG_member
	.byte	62                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1083                            # DW_AT_decl_line
	.byte	96                              # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3c8:0xa DW_TAG_member
	.byte	63                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1084                            # DW_AT_decl_line
	.byte	104                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3d2:0xa DW_TAG_member
	.byte	64                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1085                            # DW_AT_decl_line
	.byte	112                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3dc:0xa DW_TAG_member
	.byte	65                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1086                            # DW_AT_decl_line
	.byte	120                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3e6:0xa DW_TAG_member
	.byte	66                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1087                            # DW_AT_decl_line
	.byte	128                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3f0:0xa DW_TAG_member
	.byte	67                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1088                            # DW_AT_decl_line
	.byte	136                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x3fa:0xa DW_TAG_member
	.byte	68                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1089                            # DW_AT_decl_line
	.byte	144                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x404:0xa DW_TAG_member
	.byte	69                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1090                            # DW_AT_decl_line
	.byte	152                             # DW_AT_data_member_location
	.byte	25                              # Abbrev [25] 0x40e:0xa DW_TAG_member
	.byte	70                              # DW_AT_name
	.long	1049                            # DW_AT_type
	.byte	1                               # DW_AT_decl_file
	.short	1091                            # DW_AT_decl_line
	.byte	160                             # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	5                               # Abbrev [5] 0x419:0x4 DW_TAG_base_type
	.byte	50                              # DW_AT_name
	.byte	7                               # DW_AT_encoding
	.byte	8                               # DW_AT_byte_size
	.byte	26                              # Abbrev [26] 0x41d:0x23 DW_TAG_subprogram
	.byte	72                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	80                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	205                             # DW_AT_type
                                        # DW_AT_inline
	.byte	27                              # Abbrev [27] 0x425:0x8 DW_TAG_formal_parameter
	.byte	73                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	80                              # DW_AT_decl_line
	.long	827                             # DW_AT_type
	.byte	28                              # Abbrev [28] 0x42d:0x12 DW_TAG_lexical_block
	.byte	29                              # Abbrev [29] 0x42e:0x8 DW_TAG_variable
	.byte	66                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	83                              # DW_AT_decl_line
	.long	472                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x436:0x8 DW_TAG_variable
	.byte	74                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	84                              # DW_AT_decl_line
	.long	227                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x440:0x4c DW_TAG_subprogram
	.byte	9                               # DW_AT_low_pc
	.long	.Lfunc_end0-.Lfunc_begin0       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	75                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	95                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	205                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x44f:0x9 DW_TAG_formal_parameter
	.byte	0                               # DW_AT_location
	.byte	73                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	95                              # DW_AT_decl_line
	.long	827                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x458:0x9 DW_TAG_variable
	.byte	3                               # DW_AT_location
	.byte	80                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	98                              # DW_AT_decl_line
	.long	205                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x461:0x9 DW_TAG_variable
	.byte	4                               # DW_AT_location
	.byte	81                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	97                              # DW_AT_decl_line
	.long	232                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x46a:0x21 DW_TAG_inlined_subroutine
	.long	1053                            # DW_AT_abstract_origin
	.byte	9                               # DW_AT_low_pc
	.long	.Ltmp25-.Lfunc_begin0           # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	100                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x477:0x13 DW_TAG_lexical_block
	.byte	10                              # DW_AT_low_pc
	.long	.Ltmp22-.Ltmp4                  # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x47d:0x6 DW_TAG_variable
	.byte	1                               # DW_AT_location
	.long	1070                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x483:0x6 DW_TAG_variable
	.byte	2                               # DW_AT_location
	.long	1078                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x48c:0x79 DW_TAG_subprogram
	.byte	11                              # DW_AT_low_pc
	.long	.Lfunc_end1-.Lfunc_begin1       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	76                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	205                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x49b:0x9 DW_TAG_formal_parameter
	.byte	5                               # DW_AT_location
	.byte	73                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	827                             # DW_AT_type
	.byte	31                              # Abbrev [31] 0x4a4:0x9 DW_TAG_formal_parameter
	.byte	6                               # DW_AT_location
	.byte	82                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	333                             # DW_AT_type
	.byte	31                              # Abbrev [31] 0x4ad:0x9 DW_TAG_formal_parameter
	.byte	7                               # DW_AT_location
	.byte	83                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	116                             # DW_AT_decl_line
	.long	467                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4b6:0x9 DW_TAG_variable
	.byte	10                              # DW_AT_location
	.byte	80                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	121                             # DW_AT_decl_line
	.long	205                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4bf:0x9 DW_TAG_variable
	.byte	11                              # DW_AT_location
	.byte	81                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	118                             # DW_AT_decl_line
	.long	232                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4c8:0x9 DW_TAG_variable
	.byte	12                              # DW_AT_location
	.byte	84                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	119                             # DW_AT_decl_line
	.long	1593                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4d1:0x9 DW_TAG_variable
	.byte	13                              # DW_AT_location
	.byte	85                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	120                             # DW_AT_decl_line
	.long	1049                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x4da:0x9 DW_TAG_variable
	.byte	14                              # DW_AT_location
	.byte	86                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	121                             # DW_AT_decl_line
	.long	205                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x4e3:0x21 DW_TAG_inlined_subroutine
	.long	1053                            # DW_AT_abstract_origin
	.byte	12                              # DW_AT_low_pc
	.long	.Ltmp80-.Ltmp53                 # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	125                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x4f0:0x13 DW_TAG_lexical_block
	.byte	13                              # DW_AT_low_pc
	.long	.Ltmp77-.Ltmp59                 # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x4f6:0x6 DW_TAG_variable
	.byte	8                               # DW_AT_location
	.long	1070                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x4fc:0x6 DW_TAG_variable
	.byte	9                               # DW_AT_location
	.long	1078                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x505:0x4c DW_TAG_subprogram
	.byte	14                              # DW_AT_low_pc
	.long	.Lfunc_end2-.Lfunc_begin2       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	77                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	199                             # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	472                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x514:0x9 DW_TAG_formal_parameter
	.byte	15                              # DW_AT_location
	.byte	73                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	199                             # DW_AT_decl_line
	.long	827                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x51d:0x9 DW_TAG_variable
	.byte	18                              # DW_AT_location
	.byte	80                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	202                             # DW_AT_decl_line
	.long	205                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x526:0x9 DW_TAG_variable
	.byte	19                              # DW_AT_location
	.byte	81                              # DW_AT_name
	.byte	3                               # DW_AT_decl_file
	.byte	201                             # DW_AT_decl_line
	.long	232                             # DW_AT_type
	.byte	33                              # Abbrev [33] 0x52f:0x21 DW_TAG_inlined_subroutine
	.long	1053                            # DW_AT_abstract_origin
	.byte	14                              # DW_AT_low_pc
	.long	.Ltmp221-.Lfunc_begin2          # DW_AT_high_pc
	.byte	3                               # DW_AT_call_file
	.byte	204                             # DW_AT_call_line
	.byte	12                              # DW_AT_call_column
	.byte	34                              # Abbrev [34] 0x53c:0x13 DW_TAG_lexical_block
	.byte	15                              # DW_AT_low_pc
	.long	.Ltmp218-.Ltmp200               # DW_AT_high_pc
	.byte	35                              # Abbrev [35] 0x542:0x6 DW_TAG_variable
	.byte	16                              # DW_AT_location
	.long	1070                            # DW_AT_abstract_origin
	.byte	35                              # Abbrev [35] 0x548:0x6 DW_TAG_variable
	.byte	17                              # DW_AT_location
	.long	1078                            # DW_AT_abstract_origin
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	30                              # Abbrev [30] 0x551:0x59 DW_TAG_subprogram
	.byte	16                              # DW_AT_low_pc
	.long	.Lfunc_end3-.Lfunc_begin3       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	78                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	40                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	205                             # DW_AT_type
                                        # DW_AT_external
	.byte	31                              # Abbrev [31] 0x560:0x9 DW_TAG_formal_parameter
	.byte	20                              # DW_AT_location
	.byte	73                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	40                              # DW_AT_decl_line
	.long	827                             # DW_AT_type
	.byte	36                              # Abbrev [36] 0x569:0xb DW_TAG_variable
	.byte	2                               # DW_AT_location
	.byte	145
	.byte	0
	.byte	87                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	53                              # DW_AT_decl_line
	.long	1598                            # DW_AT_type
	.byte	32                              # Abbrev [32] 0x574:0x9 DW_TAG_variable
	.byte	21                              # DW_AT_location
	.byte	94                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	44                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x57d:0x9 DW_TAG_variable
	.byte	22                              # DW_AT_location
	.byte	88                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	45                              # DW_AT_decl_line
	.long	707                             # DW_AT_type
	.byte	32                              # Abbrev [32] 0x586:0x9 DW_TAG_variable
	.byte	23                              # DW_AT_location
	.byte	89                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	46                              # DW_AT_decl_line
	.long	707                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x58f:0x8 DW_TAG_variable
	.byte	95                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	42                              # DW_AT_decl_line
	.long	1049                            # DW_AT_type
	.byte	34                              # Abbrev [34] 0x597:0x12 DW_TAG_lexical_block
	.byte	17                              # DW_AT_low_pc
	.long	.Ltmp255-.Ltmp250               # DW_AT_high_pc
	.byte	36                              # Abbrev [36] 0x59d:0xb DW_TAG_variable
	.byte	2                               # DW_AT_location
	.byte	145
	.byte	0
	.byte	93                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	42                              # DW_AT_decl_line
	.long	1049                            # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	37                              # Abbrev [37] 0x5aa:0x8f DW_TAG_subprogram
	.byte	18                              # DW_AT_low_pc
	.long	.Lfunc_end4-.Lfunc_begin4       # DW_AT_high_pc
	.byte	1                               # DW_AT_frame_base
	.byte	90
                                        # DW_AT_call_all_calls
	.byte	79                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
                                        # DW_AT_prototyped
	.long	205                             # DW_AT_type
	.byte	27                              # Abbrev [27] 0x5b9:0x8 DW_TAG_formal_parameter
	.byte	101                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	38                              # Abbrev [38] 0x5c1:0xa DW_TAG_formal_parameter
	.byte	1                               # DW_AT_location
	.byte	82
	.byte	73                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	64                              # DW_AT_decl_line
	.long	1640                            # DW_AT_type
	.byte	39                              # Abbrev [39] 0x5cb:0x6d DW_TAG_lexical_block
	.byte	0                               # DW_AT_ranges
	.byte	32                              # Abbrev [32] 0x5cd:0x9 DW_TAG_variable
	.byte	24                              # DW_AT_location
	.byte	96                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	66                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	39                              # Abbrev [39] 0x5d6:0x61 DW_TAG_lexical_block
	.byte	1                               # DW_AT_ranges
	.byte	32                              # Abbrev [32] 0x5d8:0x9 DW_TAG_variable
	.byte	26                              # DW_AT_location
	.byte	98                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	73                              # DW_AT_decl_line
	.long	542                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x5e1:0x8 DW_TAG_variable
	.byte	85                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	74                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	34                              # Abbrev [34] 0x5e9:0x10 DW_TAG_lexical_block
	.byte	19                              # DW_AT_low_pc
	.long	.Ltmp299-.Ltmp285               # DW_AT_high_pc
	.byte	32                              # Abbrev [32] 0x5ef:0x9 DW_TAG_variable
	.byte	25                              # DW_AT_location
	.byte	97                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	108                             # DW_AT_decl_line
	.long	542                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	34                              # Abbrev [34] 0x5f9:0x3d DW_TAG_lexical_block
	.byte	20                              # DW_AT_low_pc
	.long	.Ltmp382-.Ltmp330               # DW_AT_high_pc
	.byte	32                              # Abbrev [32] 0x5ff:0x9 DW_TAG_variable
	.byte	27                              # DW_AT_location
	.byte	99                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	79                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	32                              # Abbrev [32] 0x608:0x9 DW_TAG_variable
	.byte	28                              # DW_AT_location
	.byte	100                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	84                              # DW_AT_decl_line
	.long	542                             # DW_AT_type
	.byte	29                              # Abbrev [29] 0x611:0x8 DW_TAG_variable
	.byte	102                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	85                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	39                              # Abbrev [39] 0x619:0x1c DW_TAG_lexical_block
	.byte	2                               # DW_AT_ranges
	.byte	32                              # Abbrev [32] 0x61b:0x9 DW_TAG_variable
	.byte	29                              # DW_AT_location
	.byte	101                             # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	88                              # DW_AT_decl_line
	.long	63                              # DW_AT_type
	.byte	34                              # Abbrev [34] 0x624:0x10 DW_TAG_lexical_block
	.byte	21                              # DW_AT_low_pc
	.long	.Ltmp370-.Ltmp353               # DW_AT_high_pc
	.byte	32                              # Abbrev [32] 0x62a:0x9 DW_TAG_variable
	.byte	30                              # DW_AT_location
	.byte	97                              # DW_AT_name
	.byte	2                               # DW_AT_decl_file
	.byte	96                              # DW_AT_decl_line
	.long	542                             # DW_AT_type
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x639:0x5 DW_TAG_pointer_type
	.long	282                             # DW_AT_type
	.byte	13                              # Abbrev [13] 0x63e:0x2a DW_TAG_structure_type
	.byte	92                              # DW_AT_name
	.byte	24                              # DW_AT_byte_size
	.byte	2                               # DW_AT_decl_file
	.byte	29                              # DW_AT_decl_line
	.byte	10                              # Abbrev [10] 0x643:0x9 DW_TAG_member
	.byte	88                              # DW_AT_name
	.long	707                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	31                              # DW_AT_decl_line
	.byte	0                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x64c:0x9 DW_TAG_member
	.byte	89                              # DW_AT_name
	.long	707                             # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	32                              # DW_AT_decl_line
	.byte	8                               # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x655:0x9 DW_TAG_member
	.byte	90                              # DW_AT_name
	.long	63                              # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	33                              # DW_AT_decl_line
	.byte	16                              # DW_AT_data_member_location
	.byte	10                              # Abbrev [10] 0x65e:0x9 DW_TAG_member
	.byte	91                              # DW_AT_name
	.long	63                              # DW_AT_type
	.byte	2                               # DW_AT_decl_file
	.byte	34                              # DW_AT_decl_line
	.byte	20                              # DW_AT_data_member_location
	.byte	0                               # End Of Children Mark
	.byte	11                              # Abbrev [11] 0x668:0x5 DW_TAG_pointer_type
	.long	1598                            # DW_AT_type
	.byte	0                               # End Of Children Mark
.Ldebug_info_end0:
	.section	.debug_rnglists,"",@progbits
	.long	.Ldebug_list_header_end1-.Ldebug_list_header_start1 # Length
.Ldebug_list_header_start1:
	.short	5                               # Version
	.byte	8                               # Address size
	.byte	0                               # Segment selector size
	.long	4                               # Offset entry count
.Lrnglists_table_base0:
	.long	.Ldebug_ranges0-.Lrnglists_table_base0
	.long	.Ldebug_ranges1-.Lrnglists_table_base0
	.long	.Ldebug_ranges2-.Lrnglists_table_base0
	.long	.Ldebug_ranges3-.Lrnglists_table_base0
.Ldebug_ranges0:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp285-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp307-.Lfunc_begin0         #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp312-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp382-.Lfunc_begin0         #   ending offset
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_ranges1:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp285-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp299-.Lfunc_begin0         #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp312-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp382-.Lfunc_begin0         #   ending offset
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_ranges2:
	.byte	1                               # DW_RLE_base_addressx
	.byte	9                               #   base address index
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp342-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp345-.Lfunc_begin0         #   ending offset
	.byte	4                               # DW_RLE_offset_pair
	.uleb128 .Ltmp348-.Lfunc_begin0         #   starting offset
	.uleb128 .Ltmp382-.Lfunc_begin0         #   ending offset
	.byte	0                               # DW_RLE_end_of_list
.Ldebug_ranges3:
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
	.long	416                             # Length of String Offsets Set
	.short	5
	.short	0
.Lstr_offsets_base0:
	.section	.debug_str,"MS",@progbits,1
.Linfo_string0:
	.asciz	"Ubuntu clang version 16.0.6 (15)" # string offset=0
.Linfo_string1:
	.asciz	"ebpf-decompress/src/bpf/rle.bpf.c" # string offset=33
.Linfo_string2:
	.asciz	"/home/mat/src/portable-decompress" # string offset=67
.Linfo_string3:
	.asciz	"IN_SIZE"                       # string offset=101
.Linfo_string4:
	.asciz	"unsigned int"                  # string offset=109
.Linfo_string5:
	.asciz	"__u32"                         # string offset=122
.Linfo_string6:
	.asciz	"u32"                           # string offset=128
.Linfo_string7:
	.asciz	"OUT_SIZE"                      # string offset=132
.Linfo_string8:
	.asciz	"LOOP_FACTOR"                   # string offset=141
.Linfo_string9:
	.asciz	"_license"                      # string offset=153
.Linfo_string10:
	.asciz	"char"                          # string offset=162
.Linfo_string11:
	.asciz	"__ARRAY_SIZE_TYPE__"           # string offset=167
.Linfo_string12:
	.asciz	"__bpf_usdt_specs"              # string offset=187
.Linfo_string13:
	.asciz	"type"                          # string offset=204
.Linfo_string14:
	.asciz	"int"                           # string offset=209
.Linfo_string15:
	.asciz	"max_entries"                   # string offset=213
.Linfo_string16:
	.asciz	"key"                           # string offset=225
.Linfo_string17:
	.asciz	"value"                         # string offset=229
.Linfo_string18:
	.asciz	"args"                          # string offset=235
.Linfo_string19:
	.asciz	"val_off"                       # string offset=240
.Linfo_string20:
	.asciz	"unsigned long long"            # string offset=248
.Linfo_string21:
	.asciz	"__u64"                         # string offset=267
.Linfo_string22:
	.asciz	"arg_type"                      # string offset=273
.Linfo_string23:
	.asciz	"BPF_USDT_ARG_CONST"            # string offset=282
.Linfo_string24:
	.asciz	"BPF_USDT_ARG_REG"              # string offset=301
.Linfo_string25:
	.asciz	"BPF_USDT_ARG_REG_DEREF"        # string offset=318
.Linfo_string26:
	.asciz	"__bpf_usdt_arg_type"           # string offset=341
.Linfo_string27:
	.asciz	"reg_off"                       # string offset=361
.Linfo_string28:
	.asciz	"short"                         # string offset=369
.Linfo_string29:
	.asciz	"arg_signed"                    # string offset=375
.Linfo_string30:
	.asciz	"_Bool"                         # string offset=386
.Linfo_string31:
	.asciz	"bool"                          # string offset=392
.Linfo_string32:
	.asciz	"arg_bitshift"                  # string offset=397
.Linfo_string33:
	.asciz	"__bpf_usdt_arg_spec"           # string offset=410
.Linfo_string34:
	.asciz	"usdt_cookie"                   # string offset=430
.Linfo_string35:
	.asciz	"arg_cnt"                       # string offset=442
.Linfo_string36:
	.asciz	"__bpf_usdt_spec"               # string offset=450
.Linfo_string37:
	.asciz	"__bpf_usdt_ip_to_spec_id"      # string offset=466
.Linfo_string38:
	.asciz	"long"                          # string offset=491
.Linfo_string39:
	.asciz	"in_bytes"                      # string offset=496
.Linfo_string40:
	.asciz	"value_size"                    # string offset=505
.Linfo_string41:
	.asciz	"map_flags"                     # string offset=516
.Linfo_string42:
	.asciz	"out_bytes"                     # string offset=526
.Linfo_string43:
	.asciz	"LINUX_HAS_BPF_COOKIE"          # string offset=536
.Linfo_string44:
	.asciz	"bpf_get_attach_cookie"         # string offset=557
.Linfo_string45:
	.asciz	"bpf_map_lookup_elem"           # string offset=579
.Linfo_string46:
	.asciz	"bpf_probe_read_kernel"         # string offset=599
.Linfo_string47:
	.asciz	"bpf_probe_read_user"           # string offset=621
.Linfo_string48:
	.asciz	"bpf_loop"                      # string offset=641
.Linfo_string49:
	.asciz	"r15"                           # string offset=650
.Linfo_string50:
	.asciz	"unsigned long"                 # string offset=654
.Linfo_string51:
	.asciz	"r14"                           # string offset=668
.Linfo_string52:
	.asciz	"r13"                           # string offset=672
.Linfo_string53:
	.asciz	"r12"                           # string offset=676
.Linfo_string54:
	.asciz	"bp"                            # string offset=680
.Linfo_string55:
	.asciz	"bx"                            # string offset=683
.Linfo_string56:
	.asciz	"r11"                           # string offset=686
.Linfo_string57:
	.asciz	"r10"                           # string offset=690
.Linfo_string58:
	.asciz	"r9"                            # string offset=694
.Linfo_string59:
	.asciz	"r8"                            # string offset=697
.Linfo_string60:
	.asciz	"ax"                            # string offset=700
.Linfo_string61:
	.asciz	"cx"                            # string offset=703
.Linfo_string62:
	.asciz	"dx"                            # string offset=706
.Linfo_string63:
	.asciz	"si"                            # string offset=709
.Linfo_string64:
	.asciz	"di"                            # string offset=712
.Linfo_string65:
	.asciz	"orig_ax"                       # string offset=715
.Linfo_string66:
	.asciz	"ip"                            # string offset=723
.Linfo_string67:
	.asciz	"cs"                            # string offset=726
.Linfo_string68:
	.asciz	"flags"                         # string offset=729
.Linfo_string69:
	.asciz	"sp"                            # string offset=735
.Linfo_string70:
	.asciz	"ss"                            # string offset=738
.Linfo_string71:
	.asciz	"pt_regs"                       # string offset=741
.Linfo_string72:
	.asciz	"__bpf_usdt_spec_id"            # string offset=749
.Linfo_string73:
	.asciz	"ctx"                           # string offset=768
.Linfo_string74:
	.asciz	"spec_id_ptr"                   # string offset=772
.Linfo_string75:
	.asciz	"bpf_usdt_arg_cnt"              # string offset=784
.Linfo_string76:
	.asciz	"bpf_usdt_arg"                  # string offset=801
.Linfo_string77:
	.asciz	"bpf_usdt_cookie"               # string offset=814
.Linfo_string78:
	.asciz	"bpf_prog"                      # string offset=830
.Linfo_string79:
	.asciz	"decode_one"                    # string offset=839
.Linfo_string80:
	.asciz	"spec_id"                       # string offset=850
.Linfo_string81:
	.asciz	"spec"                          # string offset=858
.Linfo_string82:
	.asciz	"arg_num"                       # string offset=863
.Linfo_string83:
	.asciz	"res"                           # string offset=871
.Linfo_string84:
	.asciz	"arg_spec"                      # string offset=875
.Linfo_string85:
	.asciz	"val"                           # string offset=884
.Linfo_string86:
	.asciz	"err"                           # string offset=888
.Linfo_string87:
	.asciz	"loop_ctx"                      # string offset=892
.Linfo_string88:
	.asciz	"in_ptr"                        # string offset=901
.Linfo_string89:
	.asciz	"out_ptr"                       # string offset=908
.Linfo_string90:
	.asciz	"read_i"                        # string offset=916
.Linfo_string91:
	.asciz	"write_i"                       # string offset=923
.Linfo_string92:
	.asciz	"decode_ctx"                    # string offset=931
.Linfo_string93:
	.asciz	"__r"                           # string offset=942
.Linfo_string94:
	.asciz	"zero"                          # string offset=946
.Linfo_string95:
	.asciz	"len"                           # string offset=951
.Linfo_string96:
	.asciz	"inner_i"                       # string offset=955
.Linfo_string97:
	.asciz	"target"                        # string offset=963
.Linfo_string98:
	.asciz	"val_ptr"                       # string offset=970
.Linfo_string99:
	.asciz	"run_len"                       # string offset=978
.Linfo_string100:
	.asciz	"elem_ptr"                      # string offset=986
.Linfo_string101:
	.asciz	"i"                             # string offset=995
.Linfo_string102:
	.asciz	"elem"                          # string offset=997
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
	.quad	.Ltmp285
	.quad	.Ltmp330
	.quad	.Ltmp353
.Ldebug_addr_end0:
	.section	.BTF,"",@progbits
	.short	60319                           # 0xeb9f
	.byte	1
	.byte	0
	.long	24
	.long	0
	.long	1840
	.long	1840
	.long	2950
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
	.long	1958                            # BTF_KIND_STRUCT(id = 56)
	.long	67108868                        # 0x4000004
	.long	24
	.long	1969
	.long	57
	.long	0                               # 0x0
	.long	1976
	.long	57
	.long	64                              # 0x40
	.long	1984
	.long	22
	.long	128                             # 0x80
	.long	1991
	.long	22
	.long	160                             # 0xa0
	.long	0                               # BTF_KIND_PTR(id = 57)
	.long	33554432                        # 0x2000000
	.long	0
	.long	0                               # BTF_KIND_FUNC_PROTO(id = 58)
	.long	218103810                       # 0xd000002
	.long	2
	.long	1999
	.long	22
	.long	497
	.long	55
	.long	2001                            # BTF_KIND_FUNC(id = 59)
	.long	201326592                       # 0xc000000
	.long	58
	.long	0                               # BTF_KIND_CONST(id = 60)
	.long	167772160                       # 0xa000000
	.long	22
	.long	2860                            # BTF_KIND_VAR(id = 61)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	2868                            # BTF_KIND_VAR(id = 62)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	2877                            # BTF_KIND_VAR(id = 63)
	.long	234881024                       # 0xe000000
	.long	60
	.long	1
	.long	0                               # BTF_KIND_ARRAY(id = 64)
	.long	50331648                        # 0x3000000
	.long	0
	.long	17
	.long	4
	.long	4
	.long	2889                            # BTF_KIND_VAR(id = 65)
	.long	234881024                       # 0xe000000
	.long	64
	.long	1
	.long	0                               # BTF_KIND_CONST(id = 66)
	.long	167772160                       # 0xa000000
	.long	16
	.long	2898                            # BTF_KIND_VAR(id = 67)
	.long	234881024                       # 0xe000000
	.long	66
	.long	2
	.long	2919                            # BTF_KIND_DATASEC(id = 68)
	.long	251658241                       # 0xf000001
	.long	0
	.long	67
	.long	LINUX_HAS_BPF_COOKIE
	.long	1
	.long	2928                            # BTF_KIND_DATASEC(id = 69)
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
	.long	2934                            # BTF_KIND_DATASEC(id = 70)
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
	.long	2942                            # BTF_KIND_DATASEC(id = 71)
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
	.ascii	"/home/mat/src/portable-decompress/./ebpf-decompress/src/bpf/rle.bpf.c" # string offset=1482
	.byte	0
	.ascii	"int bpf_prog(struct pt_regs *ctx)" # string offset=1552
	.byte	0
	.ascii	"0:10"                          # string offset=1586
	.byte	0
	.ascii	"    long unsigned int len = BPF_CORE_READ(ctx, ax);" # string offset=1591
	.byte	0
	.ascii	"    u32 zero = 0;"             # string offset=1643
	.byte	0
	.ascii	"    void *in_ptr = bpf_map_lookup_elem(&in_bytes, &zero);" # string offset=1661
	.byte	0
	.ascii	"    void *out_ptr = bpf_map_lookup_elem(&out_bytes, &zero);" # string offset=1719
	.byte	0
	.ascii	"    if (!in_ptr || !out_ptr)"  # string offset=1779
	.byte	0
	.ascii	"    loop_ctx.out_ptr = out_ptr;" # string offset=1808
	.byte	0
	.ascii	"    loop_ctx.in_ptr = in_ptr;" # string offset=1840
	.byte	0
	.ascii	"    loop_ctx.read_i = 0;"      # string offset=1870
	.byte	0
	.ascii	"    bpf_loop(IN_SIZE / LOOP_FACTOR, decode_one, &loop_ctx, 0);" # string offset=1895
	.byte	0
	.ascii	"decode_ctx"                    # string offset=1958
	.byte	0
	.ascii	"in_ptr"                        # string offset=1969
	.byte	0
	.ascii	"out_ptr"                       # string offset=1976
	.byte	0
	.ascii	"read_i"                        # string offset=1984
	.byte	0
	.ascii	"write_i"                       # string offset=1991
	.byte	0
	.byte	105                             # string offset=1999
	.byte	0
	.ascii	"decode_one"                    # string offset=2001
	.byte	0
	.ascii	"static int decode_one(u32 i, struct decode_ctx *ctx)" # string offset=2012
	.byte	0
	.ascii	"            if (ctx->write_i > OUT_SIZE - 4)" # string offset=2065
	.byte	0
	.ascii	"            u32 *target = ctx->out_ptr + ctx->write_i;" # string offset=2110
	.byte	0
	.ascii	"            *target = val;"    # string offset=2165
	.byte	0
	.ascii	"            ctx->write_i += 4;" # string offset=2192
	.byte	0
	.ascii	"    for (u32 inner_i = 0; inner_i < LOOP_FACTOR; inner_i += 1)" # string offset=2223
	.byte	0
	.ascii	"        if (ctx->read_i > IN_SIZE - 4)" # string offset=2286
	.byte	0
	.ascii	"        u32 *val_ptr = ctx->in_ptr + ctx->read_i;" # string offset=2325
	.byte	0
	.ascii	"        u32 val = *val_ptr;"   # string offset=2375
	.byte	0
	.ascii	"        ctx->read_i += 4;"     # string offset=2403
	.byte	0
	.ascii	"        if (val > 0x7FFFFFFF)" # string offset=2429
	.byte	0
	.ascii	"            if (ctx->read_i > IN_SIZE - 4)" # string offset=2459
	.byte	0
	.ascii	"            u32 *elem_ptr = ctx->in_ptr + ctx->read_i;" # string offset=2502
	.byte	0
	.ascii	"            u32 elem = *elem_ptr;" # string offset=2557
	.byte	0
	.ascii	"            ctx->read_i += 4;" # string offset=2591
	.byte	0
	.ascii	"            for (u32 i = 0; i < 32; i += 1) // artificial limit" # string offset=2621
	.byte	0
	.ascii	"                if (ctx->write_i > OUT_SIZE - 4)" # string offset=2685
	.byte	0
	.ascii	"                u32 *target = ctx->out_ptr + ctx->write_i;" # string offset=2734
	.byte	0
	.ascii	"                *target = elem;" # string offset=2793
	.byte	0
	.ascii	"                ctx->write_i += 4;" # string offset=2825
	.byte	0
	.ascii	"IN_SIZE"                       # string offset=2860
	.byte	0
	.ascii	"OUT_SIZE"                      # string offset=2868
	.byte	0
	.ascii	"LOOP_FACTOR"                   # string offset=2877
	.byte	0
	.ascii	"_license"                      # string offset=2889
	.byte	0
	.ascii	"LINUX_HAS_BPF_COOKIE"          # string offset=2898
	.byte	0
	.ascii	".kconfig"                      # string offset=2919
	.byte	0
	.ascii	".maps"                         # string offset=2928
	.byte	0
	.ascii	".rodata"                       # string offset=2934
	.byte	0
	.ascii	"license"                       # string offset=2942
	.byte	0
	.section	.BTF.ext,"",@progbits
	.short	60319                           # 0xeb9f
	.byte	1
	.byte	0
	.long	32
	.long	0
	.long	60
	.long	60
	.long	2052
	.long	2112
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
	.long	113
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
	.long	2012
	.long	65536                           # Line 64 Col 0
	.long	.Ltmp287
	.long	1482
	.long	2065
	.long	106518                          # Line 104 Col 22
	.long	.Ltmp290
	.long	1482
	.long	2065
	.long	106513                          # Line 104 Col 17
	.long	.Ltmp293
	.long	1482
	.long	2110
	.long	110624                          # Line 108 Col 32
	.long	.Ltmp294
	.long	1482
	.long	2110
	.long	110632                          # Line 108 Col 40
	.long	.Ltmp297
	.long	1482
	.long	2165
	.long	111637                          # Line 109 Col 21
	.long	.Ltmp298
	.long	1482
	.long	2192
	.long	112666                          # Line 110 Col 26
	.long	.Ltmp301
	.long	1482
	.long	2223
	.long	67642                           # Line 66 Col 58
	.long	.Ltmp306
	.long	1482
	.long	2223
	.long	67589                           # Line 66 Col 5
	.long	.Ltmp309
	.long	1482
	.long	914
	.long	117761                          # Line 115 Col 1
	.long	.Ltmp314
	.long	1482
	.long	2286
	.long	69650                           # Line 68 Col 18
	.long	.Ltmp317
	.long	1482
	.long	2286
	.long	69645                           # Line 68 Col 13
	.long	.Ltmp320
	.long	1482
	.long	2325
	.long	74781                           # Line 73 Col 29
	.long	.Ltmp321
	.long	1482
	.long	2325
	.long	74788                           # Line 73 Col 36
	.long	.Ltmp324
	.long	1482
	.long	2375
	.long	75795                           # Line 74 Col 19
	.long	.Ltmp327
	.long	1482
	.long	2403
	.long	76821                           # Line 75 Col 21
	.long	.Ltmp328
	.long	1482
	.long	2375
	.long	75795                           # Line 74 Col 19
	.long	.Ltmp329
	.long	1482
	.long	2429
	.long	78861                           # Line 77 Col 13
	.long	.Ltmp332
	.long	1482
	.long	2459
	.long	81937                           # Line 80 Col 17
	.long	.Ltmp335
	.long	1482
	.long	2502
	.long	86057                           # Line 84 Col 41
	.long	.Ltmp338
	.long	1482
	.long	2557
	.long	87064                           # Line 85 Col 24
	.long	.Ltmp341
	.long	1482
	.long	2591
	.long	88089                           # Line 86 Col 25
	.long	.Ltmp344
	.long	1482
	.long	2621
	.long	90125                           # Line 88 Col 13
	.long	.Ltmp347
	.long	1482
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp350
	.long	1482
	.long	2621
	.long	90125                           # Line 88 Col 13
	.long	.Ltmp355
	.long	1482
	.long	2685
	.long	95258                           # Line 93 Col 26
	.long	.Ltmp358
	.long	1482
	.long	2685
	.long	95266                           # Line 93 Col 34
	.long	.Ltmp361
	.long	1482
	.long	2685
	.long	95253                           # Line 93 Col 21
	.long	.Ltmp364
	.long	1482
	.long	2734
	.long	98340                           # Line 96 Col 36
	.long	.Ltmp365
	.long	1482
	.long	2734
	.long	98348                           # Line 96 Col 44
	.long	.Ltmp368
	.long	1482
	.long	2793
	.long	99353                           # Line 97 Col 25
	.long	.Ltmp369
	.long	1482
	.long	2825
	.long	100382                          # Line 98 Col 30
	.long	.Ltmp372
	.long	1482
	.long	2621
	.long	90151                           # Line 88 Col 39
	.long	.Ltmp377
	.long	1482
	.long	2621
	.long	90125                           # Line 88 Col 13
	.long	1477                            # LineInfo section string offset=1477
	.long	14
	.long	.Lfunc_begin3
	.long	1482
	.long	1552
	.long	40960                           # Line 40 Col 0
	.long	.Ltmp249
	.long	1482
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp252
	.long	1482
	.long	1591
	.long	43037                           # Line 42 Col 29
	.long	.Ltmp257
	.long	1482
	.long	1643
	.long	45065                           # Line 44 Col 9
	.long	.Ltmp260
	.long	1482
	.long	0
	.long	0                               # Line 0 Col 0
	.long	.Ltmp261
	.long	1482
	.long	1661
	.long	46100                           # Line 45 Col 20
	.long	.Ltmp265
	.long	1482
	.long	1719
	.long	47125                           # Line 46 Col 21
	.long	.Ltmp270
	.long	1482
	.long	1779
	.long	49169                           # Line 48 Col 17
	.long	.Ltmp275
	.long	1482
	.long	1808
	.long	56342                           # Line 55 Col 22
	.long	.Ltmp276
	.long	1482
	.long	1840
	.long	55317                           # Line 54 Col 21
	.long	.Ltmp277
	.long	1482
	.long	1870
	.long	57365                           # Line 56 Col 21
	.long	.Ltmp278
	.long	1482
	.long	1808
	.long	56342                           # Line 55 Col 22
	.long	.Ltmp279
	.long	1482
	.long	1895
	.long	60421                           # Line 59 Col 5
	.long	.Ltmp282
	.long	1482
	.long	914
	.long	63489                           # Line 62 Col 1
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
	.long	1586
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
