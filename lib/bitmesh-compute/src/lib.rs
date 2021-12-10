#![allow(dead_code)]

struct BitLocation {
    page: i32,
    index: i32,
}

struct Bit {
    group_id: i32,
    stats: BitStats,
}

#[allow(non_snake_case)]
struct BitStats {
    // R_Cx : reward when consencus at x
    R_C0: f32,
    R_C1: f32,
    // P_Cx : penalty when consencus at x
    P_C0: f32,
    P_C1: f32,
}

struct BitLink {
    input: BitLocation,  // location of the input bit
    weight: f32,         // weight factor
    drift: f32,          // mutation asymetry factor (~ around 1.0)
    stats: BitLinkStats, // probabilistics data
}

#[allow(non_snake_case)]
struct BitLinkStats {
    // R_Cx_Iy : reward when consencus at x and input at y
    R_C0_I0: f32,
    R_C0_I1: f32,
    R_C1_I0: f32,
    R_C1_I1: f32,
    // P_Cx_Iy : penalty when consencus at x and input at y
    P_C0_I0: f32,
    P_C0_I1: f32,
    P_C1_I0: f32,
    P_C1_I1: f32,
}

struct BitPage {
    num_bits: i32,
    link_per_bit: i32,
    bits: Vec<Bit>,
    links: Vec<BitLink>,
}

struct Mesh {
    pages: Vec<BitPage>,
}

impl BitLinkStats {
    fn new() -> BitLinkStats {
        BitLinkStats {
            R_C0_I0: 0.0,
            R_C0_I1: 0.0,
            R_C1_I0: 0.0,
            R_C1_I1: 0.0,
            P_C0_I0: 0.0,
            P_C0_I1: 0.0,
            P_C1_I0: 0.0,
            P_C1_I1: 0.0,
        }
    }
}

impl BitLink {
    fn new() -> BitLink {
        BitLink {
            input: BitLocation { index: 0, page: 0 },
            stats: BitLinkStats::new(),
            weight: 0.0,
            drift: 0.0,
        }
    }
}

impl BitPage {
    fn new() -> BitPage {
        BitPage {
            num_bits: 0,
            link_per_bit: 0,
            bits: vec![],
            links: vec![],
        }
    }
}
