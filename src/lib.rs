use std::io::{self, Read, Write};

use gmodx::{
    gmod13_close, gmod13_open,
    lua::{self, State},
};

fn l_brotli_compress(_: &State, data: lua::String, level: u32) -> lua::Result<lua::String> {
    let mut output = Vec::new();

    {
        let mut encoder = brotli::CompressorWriter::new(&mut output, 4096, level, 22);

        encoder
            .write_all(&data)
            .map_err(|_| lua::Error::Runtime("failed to write data into compressor".into()))?;
    }

    Ok(output.into())
}

fn l_brotli_decompress(_: &State, data: lua::String) -> lua::Result<lua::String> {
    let mut decoder = brotli::Decompressor::new(
        io::Cursor::new(<lua::String as AsRef<[u8]>>::as_ref(&data)),
        4096,
    );
    let mut output = Vec::new();

    decoder
        .read_to_end(&mut output)
        .map_err(|_| lua::Error::Runtime("failed to decompress data".into()))?;

    Ok(output.into())
}

#[gmod13_open]
fn gmod13_open(l: State) {
    let brotli = l.create_table();
    brotli.raw_set(&l, "compress", l.create_function(l_brotli_compress));
    brotli.raw_set(&l, "decompress", l.create_function(l_brotli_decompress));

    l.set_global("brotli", brotli).unwrap();
}

#[gmod13_close]
fn gmod13_close(l: State) {}
