# Greenriver

A command line tool to work with
[greenfield images](https://github.com/Tomcat-42/greenfield).

## Installation

This command line tool is available on
[crates.io/greenriver](https://crates.io/greenriver). So, just install it with
cargo:

```bash
cargo install greenriver
```

## Usage

```text
A command line tool to work with greenfield images

Usage: greenriver <COMMAND>

Commands:
  quantize  Quantizes an image and saves it to a file
  inspect   Inspet an image
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### quantize

To convert an image on a common file format to [greenfield](docs.rs/greenfield)
with a quantization level, and saving it to [greenfield](docs.rs/greenfield):

```bash
greenriver quantize 5 6 5 Lenna.png Lenna_565.gfd
```

To convert an image on a common file format to [greenfield](docs.rs/greenfield)
with a quantization level, and saving it to a common file format (the formats
are inferred by file name):

```bash
greenriver quantize 5 6 5 Lenna.png Lenna_565.png
```
         Original          |  Quantized to (5, 6, 5)
:-------------------------:|:-------------------------:
![](./assets/Lenna.png)  |  ![](./assets/Lenna_565.png)


### convert

To convert between file formats (the formats are inferred by extension, so for [greenfield](docs.rs/greenfield) use `.gfd`):

```bash
greenriver convert ./assets/Lenna_565.gfd ./assets/Lenna_565.png
```

### inspect

To get file information about a [greenfield](docs.rs/greenfield) image, use the `inspect` command:

```bash
greenriver inspect ./assets/Lenna_565.gfd
```

```text
┌───────┬─────────────┬────────────┬────────────┬──────────────┬────────────────────────┐
│       │ Magic       │ Width      │ Height     │ Quantization │ Data                   │
├───────┼─────────────┼────────────┼────────────┼──────────────┼────────────────────────┤
│ Value │ b'grnfld42' │ 512        │ 512        │ (5, 6, 5)    │ 512x512 [RGB]          │
├───────┼─────────────┼────────────┼────────────┼──────────────┼────────────────────────┤
│ Size  │ 64 b (8 B)  │ 32 b (4 B) │ 32 b (4 B) │ 9 b (~1 B)   │ 4194304 b (512.00 KiB) │
└───────┴─────────────┴────────────┴────────────┴──────────────┴────────────────────────┘
```

You can also inspect files in other formats. They will be converted to [greenfield](docs.rs/greenfield) (8, 8, 8):

```bash
greenriver inspect ./assets/Lenna.png
```

```text
┌───────┬─────────────┬────────────┬────────────┬──────────────┬────────────────────────┐
│       │ Magic       │ Width      │ Height     │ Quantization │ Data                   │
├───────┼─────────────┼────────────┼────────────┼──────────────┼────────────────────────┤
│ Value │ b'grnfld42' │ 512        │ 512        │ (8, 8, 8)    │ 512x512 [RGB]          │
├───────┼─────────────┼────────────┼────────────┼──────────────┼────────────────────────┤
│ Size  │ 64 b (8 B)  │ 32 b (4 B) │ 32 b (4 B) │ 9 b (~1 B)   │ 6291456 b (768.00 KiB) │
└───────┴─────────────┴────────────┴────────────┴──────────────┴────────────────────────┘
```
