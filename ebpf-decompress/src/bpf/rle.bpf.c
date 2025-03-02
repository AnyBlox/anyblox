#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/usdt.bpf.h>

const u32 IN_SIZE = 8 * 1024 * 1024;
const u32 OUT_SIZE = 16 * IN_SIZE;
const u32 LOOP_FACTOR = 256;

struct
{
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __uint(value_size, IN_SIZE);
    __uint(max_entries, 1);
    __uint(map_flags, BPF_F_MMAPABLE | BPF_F_RDONLY_PROG);

} in_bytes SEC(".maps");

struct
{
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32);
    __uint(value_size, OUT_SIZE);
    __uint(max_entries, 1);
    __uint(map_flags, BPF_F_MMAPABLE);
} out_bytes SEC(".maps");

struct decode_ctx
{
    void *in_ptr;
    void *out_ptr;
    u32 read_i;
    u32 write_i;
};

static int decode_one(u32 i, struct decode_ctx *ctx);

SEC("usdt")
int bpf_prog(struct pt_regs *ctx)
{
    long unsigned int len = BPF_CORE_READ(ctx, ax);

    u32 zero = 0;
    void *in_ptr = bpf_map_lookup_elem(&in_bytes, &zero);
    void *out_ptr = bpf_map_lookup_elem(&out_bytes, &zero);

    if (!in_ptr || !out_ptr)
    {
        return 1;
    }

    struct decode_ctx loop_ctx;
    loop_ctx.in_ptr = in_ptr;
    loop_ctx.out_ptr = out_ptr;
    loop_ctx.read_i = 0;
    loop_ctx.write_i = 0;

    bpf_loop(IN_SIZE / LOOP_FACTOR, decode_one, &loop_ctx, 0);

    return 0;
}

static int decode_one(u32 i, struct decode_ctx *ctx)
{
    for (u32 inner_i = 0; inner_i < LOOP_FACTOR; inner_i += 1)
    {
        if (ctx->read_i > IN_SIZE - 4)
        {
            return 1;
        }

        u32 *val_ptr = ctx->in_ptr + ctx->read_i;
        u32 val = *val_ptr;
        ctx->read_i += 4;

        if (val > 0x7FFFFFFF)
        {
            u32 run_len = val & 0x7FFFFFFF;
            if (ctx->read_i > IN_SIZE - 4)
            {
                return 1;
            }
            u32 *elem_ptr = ctx->in_ptr + ctx->read_i;
            u32 elem = *elem_ptr;
            ctx->read_i += 4;

            for (u32 i = 0; i < 32; i += 1) // artificial limit
            {
                if (i >= run_len)
                    break;

                if (ctx->write_i > OUT_SIZE - 4)
                    return 1;

                u32 *target = ctx->out_ptr + ctx->write_i;
                *target = elem;
                ctx->write_i += 4;
            }
        }
        else
        {
            // singleton
            if (ctx->write_i > OUT_SIZE - 4)
            {
                return 1;
            }
            u32 *target = ctx->out_ptr + ctx->write_i;
            *target = val;
            ctx->write_i += 4;
        }
    }

    return 0;
}

char _license[] SEC("license") = "GPL";