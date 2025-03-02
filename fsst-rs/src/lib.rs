use std::ffi::c_void;

mod ffi;

pub const MAX_EXPORT_LEN: usize = ffi::FSST_MAXHEADER;

pub struct FsstEncoder(*mut c_void);
pub struct FsstDecoder(ffi::fsst_decoder_t);

impl FsstEncoder {
    pub fn new(data: &[&[u8]]) -> Self {
        Self::new_impl(data, false)
    }

    pub fn new_zero_terminated(data: &[&[u8]]) -> Self {
        Self::new_impl(data, true)
    }

    fn new_impl(data: &[&[u8]], zero_terminated: bool) -> Self {
        let lens = data.iter().map(|buf| buf.len()).collect::<Vec<_>>();
        let data_ptrs = data.iter().map(|buf| buf.as_ptr()).collect::<Vec<_>>();
        let ptr = unsafe {
            ffi::fsst_create(
                lens.len(),
                lens.as_ptr(),
                data_ptrs.as_ptr(),
                if zero_terminated { 1 } else { 0 },
            )
        };
        Self(ptr)
    }

    pub fn export(&self) -> Vec<u8> {
        let mut vec = vec![0; MAX_EXPORT_LEN];
        let len = self.export_into(&mut vec);
        vec.truncate(len);
        vec
    }

    pub fn export_into(&self, buf: &mut [u8]) -> usize {
        assert!(buf.len() >= MAX_EXPORT_LEN);
        let len = unsafe { ffi::fsst_export(self.0, buf.as_mut_ptr()) };
        usize::try_from(len).unwrap()
    }

    pub fn compress_into<'a>(&self, data: &[&[u8]], output_buf: &'a mut [u8], output_slices: &mut [&'a [u8]]) -> usize {
        let lens = data.iter().map(|buf| buf.len()).collect::<Vec<_>>();
        let data_ptrs = data.iter().map(|buf| buf.as_ptr()).collect::<Vec<_>>();
        let mut output_lens = vec![0_usize; data.len()];
        let mut output_ptrs = vec![std::ptr::null_mut::<u8>(); data.len()];

        let len = unsafe {
            ffi::fsst_compress(
                self.0,
                data.len(),
                lens.as_ptr(),
                data_ptrs.as_ptr(),
                output_buf.len(),
                output_buf.as_mut_ptr(),
                output_lens.as_mut_ptr(),
                output_ptrs.as_mut_ptr(),
            )
        };
        for i in 0..len {
            output_slices[i] = unsafe { std::slice::from_raw_parts(output_ptrs[i], output_lens[i]) };
        }

        len
    }
}

impl Clone for FsstEncoder {
    fn clone(&self) -> Self {
        let ptr = unsafe { ffi::fsst_duplicate(self.0) };
        Self(ptr)
    }
}

impl Drop for FsstEncoder {
    fn drop(&mut self) {
        unsafe { ffi::fsst_destroy(self.0) };
    }
}

impl FsstDecoder {
    pub fn new(encoder: &FsstEncoder) -> Self {
        let r#struct = unsafe { ffi::fsst_decoder(encoder.0) };
        Self(r#struct)
    }

    pub fn import(buf: &[u8]) -> (Self, usize) {
        unsafe { Self::import_from_ptr(buf.as_ptr()) }
    }

    pub unsafe fn import_from_ptr(buf: *const u8) -> (Self, usize) {
        let mut decoder = ffi::fsst_decoder_t::default();
        let len = unsafe { ffi::fsst_import(&raw mut decoder, buf) };
        (Self(decoder), usize::try_from(len).unwrap())
    }

    pub fn decompress_one(&self, compressed_string: &[u8]) -> Vec<u8> {
        let mut vec = vec![0; compressed_string.len() * 8];
        let len = self.decompress_one_into(compressed_string, &mut vec);
        vec.truncate(len);
        vec
    }

    pub fn decompress_one_into(&self, compressed_string: &[u8], buf: &mut [u8]) -> usize {
        unsafe {
            self.decompress_one_from_ptr(
                compressed_string.as_ptr(),
                compressed_string.len(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        }
    }

    pub unsafe fn decompress_one_from_ptr(
        &self,
        compressed_ptr: *const u8,
        compressed_len: usize,
        buf_ptr: *mut u8,
        buf_len: usize,
    ) -> usize {
        unsafe { ffi::fsst_decompress_ext(&raw const self.0, compressed_len, compressed_ptr, buf_len, buf_ptr) }
    }
}

#[cfg(test)]
mod tests {
    use crate::FsstDecoder;

    use super::FsstEncoder;

    #[test]
    fn small() {
        let strings = [
            r"http://in.tum.de",
            r"http://cwi.nl",
            r"www.uni-jena.de",
            r"www.wikipedia.org",
            r"http://www.vldb.org",
        ];
        let data = strings.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

        let mut encoding_buf = vec![0; 1024];
        let mut encoded_slices: Vec<&[u8]> = vec![&[]; 5];

        let encoder = FsstEncoder::new(&data);
        let len = encoder.compress_into(&data, &mut encoding_buf, &mut encoded_slices);

        assert_eq!(len, strings.len());

        let decoder = FsstDecoder::new(&encoder);

        for (encoded, expected) in encoded_slices.iter().zip(strings) {
            let mut output_buf = vec![0; 64];
            let len = decoder.decompress_one_into(encoded, &mut output_buf);

            assert_eq!(len, expected.len());
            let str = std::str::from_utf8(&output_buf[..len]).unwrap();
            assert_eq!(str, expected);
        }
    }

    #[test]
    fn big() {
        let tokens = vec![
            "amorphous",
            "bowling",
            "cumulative",
            "destructive",
            "enormous",
            "frolicking",
            "great",
            "highlander",
            "incongruous",
            "jarring",
            "killer",
            "luminous",
            "maximal",
            "nihilist",
            "operating",
            "partial",
            "queue",
            "robust",
            "string",
            "total",
            "undulating",
            "vicarious",
            "xenon",
            "yelling",
            "zero",
        ];

        const N: usize = 1_000_000;
        let mut total_len = 0;
        let mut strings = Vec::with_capacity(N);
        for code in 0..N {
            let mut string = String::new();
            let mut c = code;
            while c > 0 {
                let i = c % tokens.len();
                string += tokens[i];
                c /= tokens.len();
            }
            total_len += string.len();
            strings.push(string);
        }

        let data = strings.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

        let mut encoding_buf = vec![0; total_len];
        let mut encoded_slices: Vec<&[u8]> = vec![&[]; N];

        let encoder = FsstEncoder::new(&data);
        let len = encoder.compress_into(&data, &mut encoding_buf, &mut encoded_slices);

        assert_eq!(len, strings.len());

        let decoder = FsstDecoder::new(&encoder);

        for (encoded, expected) in encoded_slices.iter().zip(strings) {
            let mut output_buf = vec![0; expected.len() + 64];
            let len = decoder.decompress_one_into(encoded, &mut output_buf);

            assert_eq!(len, expected.len());
            let str = std::str::from_utf8(&output_buf[..len]).unwrap();
            assert_eq!(str, expected);
        }
    }
}
