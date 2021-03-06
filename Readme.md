# FE3 SPC Code Separator

A simple program I wrote to separate the sections of FE3's SPC data.

## Usage

You need to copy the SPC data from FE3 to a separate bin file.

The data is located at $B08000 and is 17682 bytes long.

After building, run via: `fe3spcinitdumper spcdata.bin`. Or just `cargo run spcdata.bin`.

The program will output separated binary files containing only the SPC code.

It also generates an asm file with the lengths, offsets, and includes for the disassembled binaries.

You will need to disassemble the files with a tool like [this](https://github.com/RobertTheSable/spcdas).

Run the disassembler like so to produce an asar compatible result:

```
offset=$(basename -s .bin $x) && spcdas $x  -hex off -addr off -pc $offset -load $offset  "${offset}.s"
```
