# PNGme

Command line utility that lets you hide secret messages in PNG files.

Below on the left is original image and on the right modified image with embedded secret message:

![original](/images/dice.png?raw=true)
![with secret](/images/secret.png?raw=true)

Program has 4 commands:
1. Encode a message into a PNG file;
1. Decode a message stored in a PNG file;
1. Remove a message from a PNG file;
1. Print a list of PNG chunks that can be searched for messages.

## Idea

I made this program just for fun and Rust learning purposes. Idea come from
[PNGme resource](https://picklenerd.github.io/pngme_book/) by picklenerd.  
Great about this project is:
> Unlike many of the other tutorials you may have worked through, I will not be providing any
> completed code. The first three chapters come with comprehensive unit tests that will ensure your
> code has the features it needs. You'll use your unit tested code to complete the remaining
> chapters.
>
> This is supposed to be your project, not mine. You can use as much or as little of this material
> as you want.

I definitely can recommend completing it if you want some more advanced project than 'hello world'.
Also, it's not so time-consuming and fun to program.

## Usage

Command line arguments are parsed using [clap](https://github.com/clap-rs/clap):
```
Utility that lets you hide secret messages in PNG files

USAGE:
    pngme <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    decode    Searches for a message hidden in a PNG file and prints the message if one is found
    encode    Encodes a message into a PNG file and saves the result
    help      Prints this message or the help of the given subcommand(s)
    print     Prints all of the chunks in a PNG file
    remove    Removes a chunk from a PNG file and saves the result
```
Firstly you can print all chunks from image:
```bash
$ ./target/release/pngme print images/dice.png
[1] Chunk "IHDR" len:13
[2] Chunk "sRGB" len:1
[3] Chunk "gAMA" len:4
[4] Chunk "pHYs" len:9
[5] Chunk "IDAT" len:65445
[6] Chunk "IDAT" len:45941
[7] Chunk "IEND" len:0
```
Then you can encode secret data:
```bash
$ ./target/release/pngme encode images/dice.png RuSt 'secret message'

# make sure data is written
$ ./target/release/pngme print images/dice.png
[1] Chunk "IHDR" len:13
[2] Chunk "sRGB" len:1
[3] Chunk "gAMA" len:4
[4] Chunk "pHYs" len:9
[5] Chunk "IDAT" len:65445
[6] Chunk "IDAT" len:45941
[7] Chunk "IEND" len:0
[8] Chunk "RuSt" len:14
```
View encoded secret data:
```bash
$ ./target/release/pngme decode images/dice.png RuSt
Chunk "RuSt" len:14
Data: secret message
```
Remove message:
```bash
$ ./target/release/pngme remove images/dice.png RuSt
Chunk "RuSt" len:14
Data: secret message

# check chunk was removed
$ ./target/release/pngme print images/dice.png
[1] Chunk "IHDR" len:13
[2] Chunk "sRGB" len:1
[3] Chunk "gAMA" len:4
[4] Chunk "pHYs" len:9
[5] Chunk "IDAT" len:65445
[6] Chunk "IDAT" len:45941
[7] Chunk "IEND" len:0
```
