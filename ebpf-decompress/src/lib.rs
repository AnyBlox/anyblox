use libbpf_rs::MapFlags;

pub mod bpf;

pub fn write_map(bpf_map: &mut libbpf_rs::Map, data: &[u8]) {
    bpf_map.update(&[0, 0, 0, 0], data, MapFlags::ANY).expect("update")
}

pub fn read_map(bpf_map: &libbpf_rs::Map, dest: &mut [u32]) {
    let bytes = bpf_map
        .lookup(&[0, 0, 0, 0], MapFlags::ANY)
        .expect("lookup")
        .expect("lookup not None");

    assert_eq!(bytes.len(), dest.len() * 4);

    let raw_dest = unsafe { std::slice::from_raw_parts_mut(dest.as_mut_ptr().cast::<u8>(), dest.len() * 4) };
    raw_dest.copy_from_slice(&bytes);
}
