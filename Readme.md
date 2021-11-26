# FE3 SPC Code Separator

A simple program I wrote to separate the sections of FE3's SPC data.

## Usage

You need to copy the SPC data from FE3 to a separate bin file.

The data is located at $B08000 and is 17682 bytes long.

After building, run via: `fe3spcinitdumper spcdata.bin`. Or just `cargo run`.
