fn byte_to_file_to_byte_to_string() {
    #[repr(C)]
    #[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
    struct OurData {
        number: u16,
        tag: [u8; 8],
    }

    let some_data = vec![
        OurData {
            number: 1,
            tag: *b"hello   ",
        },
        OurData {
            number: 2,
            tag: *b"world   ",
        },
    ];

    let bytes: &[u8] = bytemuck::cast_slice(&some_data);
    std::fs::write("data.bin", bytes).unwrap();

    // Read the data back
    let bytes = std::fs::read("data.bin").unwrap();
    let data: &[OurData] = bytemuck::cast_slice(&bytes);

    // Debug print the data to show the round-trip worked
    println!("{data:?}");

    // Print the first record's tag as a string
    println!(
        "{}, {}",
        std::str::from_utf8(&data[0].tag).unwrap(),
        std::str::from_utf8(&data[1].tag).unwrap()
    );
}

use std::{fs::File, io::Write};

#[derive(Debug)]
struct OurData {
    number: u16,
    tag: String,
}

fn main() {
    byte_to_file_to_byte_to_string();

    // working with byte representing a protocol (network, file, anything...)
    let a = OurData {
        number: 12,
        tag: "Hello World".to_string(),
    };

    // Write the record in parts
    let mut file = File::create("bytes.bin").unwrap();

    // Write the number and check that 2 bytes were written
    assert_eq!(file.write(&a.number.to_le_bytes()).unwrap(), 2);

    // Write the string length IN BYTES and check that 8 bytes were written
    let len = a.tag.as_bytes().len();
    assert_eq!(file.write(&(len as u64).to_le_bytes()).unwrap(), 8); // little-endian byte order

    // Write the string and check that the correct number of bytes were written
    assert_eq!(file.write(a.tag.as_bytes()).unwrap(), len);

    // example of creating a protocol for streaming this data in binary out weithout having to use serde or extra layers;
    // might be good for embedded, no wasted space
    // so needs to read the data back (below)

    ///// READ THE DATA BACK
    // Read the whole file as bytes.
    let bytes = std::fs::read("bytes.bin").unwrap();

    // Read the number
    let number = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

    // Read the string length
    let length = u64::from_le_bytes(bytes[2..10].try_into().unwrap());

    // Decode the string
    let tag = std::str::from_utf8(&bytes[10..(10 + length as usize)]).unwrap();

    let a = OurData {
        number,
        tag: tag.to_string(),
    };

    println!("{a:?}");
}

// cargo add bytemuck -F derive
