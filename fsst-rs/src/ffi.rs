use std::ffi::c_void;

pub const FSST_MAXHEADER: usize = 8 + 1 + 8 + 2048 + 1;

#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub(super) struct fsst_decoder_t {
    version: u64,
    zero_terminated: u8,
    len: [u8; 255],
    symbol: [u64; 255],
}

impl Default for fsst_decoder_t {
    fn default() -> Self {
        Self {
            version: Default::default(),
            zero_terminated: Default::default(),
            len: [0; 255],
            symbol: [0; 255],
        }
    }
}

#[link(name = "fsst")]
extern "C" {
    /// Calibrate a FSST symboltable from a batch of strings (it is best to provide at least 16KB of data).
    ///
    /// `n` - number of strings in batch to sample from.
    /// `len_in` - byte-lengths of the inputs
    /// `str_in` - string start pointers.
    /// `zero_terminated` - whether input strings are zero-terminated. If so, encoded strings are as well (i.e. symbol[0]="").
    pub(super) fn fsst_create(
        n: usize,
        len_in: *const usize,
        str_in: *const *const u8,
        zero_terminated: i32,
    ) -> *mut c_void;

    /// Create another encoder instance, necessary to do multi-threaded encoding using the same symbol table.
    ///
    /// `encoder` - the symbol table to duplicate.
    pub(super) fn fsst_duplicate(encoder: *const c_void) -> *mut c_void;

    /// Space-efficient symbol table serialization (smaller than sizeof(fsst_decoder_t) - by saving on the unused bytes in symbols of len < 8).
    ///
    /// `encoder` - the symbol table to dump.
    /// `buf` - pointer to a byte-buffer where to serialize this symbol table.
    ///
    /// # Returns
    /// number of bytes written in buf, at most sizeof(fsst_decoder_t)
    pub(super) fn fsst_export(encoder: *const c_void, buf: *mut u8) -> u32;

    /// Deallocate encoder.
    pub(super) fn fsst_destroy(encoder: *mut c_void);

    /// Return a decoder structure from serialized format (typically used in a block-, file- or row-group header).
    ///
    /// `decoder` - this symbol table will be overwritten.
    /// `buf` - pointer to a byte-buffer where fsst_export() serialized this symbol table.
    ///
    /// # Returns
    /// number of bytes consumed in buf (0 on failure).
    pub(super) fn fsst_import(decoder: *mut fsst_decoder_t, buf: *const u8) -> u32;

    /// Return a decoder structure from an encoder.
    pub(super) fn fsst_decoder(encoder: *const c_void) -> fsst_decoder_t;

    /// Compress a batch of strings (on AVX512 machines best performance is obtained by compressing more than 32KB of string volume).
    /// The output buffer must be large; at least "conservative space" (7+2*inputlength) for the first string for something to happen.
    ///
    /// `encoder` - encoder obtained from fsst_create().
    /// `nstrings` - number of strings in batch to compress.
    /// `len_in` - byte-lengths of the inputs
    /// `str_in` - input string start pointers.
    /// `outsize` - byte-length of output buffer.
    /// `output` - memory buffer to put the compressed strings in (one after the other).
    /// `len_out` - byte-lengths of the compressed strings.
    /// `str_out` - output string start pointers. Will all point into [output,output+size).
    ///
    /// # Returns
    /// the number of compressed strings (<=n) that fit the output buffer.
    pub(super) fn fsst_compress(
        encoder: *const c_void,
        nstrings: usize,
        len_in: *const usize,
        str_in: *const *const u8,
        outsize: usize,
        output: *mut u8,
        len_out: *mut usize,
        str_out: *mut *mut u8,
    ) -> usize;

    /// Decompress a single string, inlined for speed.
    ///
    /// `decoder` - use this symbol table for compression.
    /// `len_in` - byte-length of compressed string.
    /// `str_in` - compressed string.
    /// `size` - byte-length of output buffer.
    /// `output` - memory buffer to put the decompressed string in.
    ///
    /// # Returns
    /// bytesize of the decompressed string. If > size, the decoded output is truncated to size.
    pub(super) fn fsst_decompress_ext(
        decoder: *const fsst_decoder_t,
        len_in: usize,
        str_in: *const u8,
        size: usize,
        output: *mut u8,
    ) -> usize;
}
