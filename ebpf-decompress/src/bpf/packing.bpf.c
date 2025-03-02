#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_core_read.h>
#include <bpf/usdt.bpf.h>

const u32 IN_SIZE = 8 * 1024 * 1024;
const u32 OUT_SIZE = 16 * 8 * 1024 * 1024;
const u32 LOOP_FACTOR = 1 * 1024;

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

    bpf_loop(IN_SIZE / LOOP_FACTOR, decode_one, &loop_ctx, 0);

    return 0;
}

static int decode_one(u32 i, struct decode_ctx *ctx)
{
    for (u32 inner_i = 0; inner_i < LOOP_FACTOR; inner_i += 1)
    {
        u32 read_i = i * LOOP_FACTOR + inner_i;

        if (read_i >= IN_SIZE)
        {
            return 1;
        }

        u8 *elem = ctx->in_ptr + read_i;
        u8 byte = *(u8 *)elem;

        u8 a1 = byte & 0b11000000;
        u8 a2 = byte & 0b00110000;
        u8 a3 = byte & 0b00001100;
        u8 a4 = byte & 0b00000011;
        u32 b1 = (u32)(a1 >> 6);
        u32 b2 = (u32)(a2 >> 4);
        u32 b3 = (u32)(a3 >> 2);
        u32 b4 = (u32)(a4);

        u32 write_i = 16 * read_i;

        if (write_i + 16 <= OUT_SIZE)
        {
            u32 *elem_1 = ctx->out_ptr + write_i;
            *elem_1 = b1;
            u32 *elem_2 = ctx->out_ptr + write_i + 4;
            *elem_2 = b2;
            u32 *elem_3 = ctx->out_ptr + write_i + 8;
            *elem_3 = b3;
            u32 *elem_4 = ctx->out_ptr + write_i + 12;
            *elem_4 = b4;
        }
    }

    return 0;
}

char _license[] SEC("license") = "GPL";