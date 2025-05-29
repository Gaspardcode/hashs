pub const iv:[u32;8] = [
            0x6a09e667,
            0xbb67ae85,
            0x3c6ef372,
            0xa54ff53a,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
            0x5be0cd19
        ];
pub const enum flags {
    CHUNK_START(u32),
    CHUNK_END(u32),
    PARENT(u32),
    ROOT(u32),
    KEYED_HASH(u32),
    DERIVE_KEY_CONTEXT(u32),
    DERIVE_KEY_MATERIAL(u32)
};
