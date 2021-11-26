use std::env;
use std::fs;
use std::io::{
    Read, Write, ErrorKind, Seek
};
use byteorder::{LittleEndian, ReadBytesExt};

struct SPCSection {
    cpu_offset: u64,
    spc_offset: u16,
    size: u16,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let mut file = fs::File::open(filename)
            .expect("Couldn't open the provided file.");
            
        let mut sections: Vec<SPCSection> = Vec::new();
        loop {
            let section_length: u16;
            let offset: u64 = file.stream_position()
                            .expect("Failed reading desination address.");
            match file.read_u16::<LittleEndian>() {
                Ok(v) => section_length = v,
                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => panic!("Can't read from file: {}, err {}", filename, e),
            }
            
            let destination = file.read_u16::<LittleEndian>()
                .expect("Failed reading desination address.");
            let out_file_name = format!("{:04x}.bin", destination);
            
            
            if section_length != 0 {
                let buffer_size = section_length;
                let mut buffer = vec![0; buffer_size.into()];
                
                file.read_exact(&mut buffer)
                    .expect("Failed reading desination address.");
                let mut out_file = fs::File::create(out_file_name)
                    .expect("Failed opening output file");
                let bytes_copied = out_file.write(&buffer)
                    .expect("Failed writing output file");
                if bytes_copied != section_length.into() {
                    panic!("Didn't copy the expected number of bytes.");
                }
            }
            
            sections.push(SPCSection {
                cpu_offset: offset + 0xB08000,
                spc_offset: destination,
                size: section_length,
            });
            if section_length == 0 {
                println!("Parsed {} sections.", sections.len() -1);
                break;
            }
        }
        if sections.len() != 0 {
            let mut out_file = fs::File::create("main.asm")
                    .expect("Failed opening output file");
            match write!(out_file, "includeonce\n\n") {
                Err(_e) => panic!("Failed writing output file."),
                _ => (), 
            }
            for section in sections {
                // write!()? doesn't work even though the docs say it should ¯\_(ツ)_/¯
                let _ = write!(out_file, "ORG ${:06x}\n", section.cpu_offset);
                let _ = write!(out_file, "arch spc700-inline\nORG ${:04x}\n", section.spc_offset);
                let _ = write!(out_file, "; dw ${:04x}\n", section.size);
                let _ = write!(out_file, "; dw ${:04x}\n", section.spc_offset);
                if section.size != 0 {
                    let _ = write!(out_file, "incsrc {:04x}.s\narch 65816\n\n", section.spc_offset);
                }
            }
        }
    } else {
        println!("please specify a filename.");
    }

}
