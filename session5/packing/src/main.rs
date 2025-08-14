struct OneByte {
    a: u8,
}

struct TwoBytes {
    a: u16,
}

// #[repr(packed)] // with the representation packed it'll make 3 bytes; don't do that unless you are interacting with something (network) exaclty 3 bytes
// #[unsafe(no_mangle)] // mangling -> your function or type names may not be the same stored in binary
#[repr(C)]
struct ThreeBytes {
    a: u16,
    b: u8,
    // rust compiler might not order the variables at this order in memory (a, b, c, d...)
}

struct FourBytes {
    a: u32,
}

fn main() {
    println!("{}", std::mem::size_of::<OneByte>());
    println!("{}", std::mem::size_of::<TwoBytes>());
    println!("{}", std::mem::size_of::<ThreeBytes>()); // alignment makes it 4 bytes, rust quietly insert 1 more byte and waste that space
    
    println!("{}", std::mem::size_of::<FourBytes>());    
}