# hexie

Yet another hex viewer. Does everything you would expect from a hex viewer, and maybe a little bit more.

- Adapts to the size of your terminal.
- Pages by default so you don't accidentally a 5GB file.
- Allows printing arbitrary regions of files.
- Can be piped in to.
- Supports two coloring modes:
  - "absolute", which colors different byte ranges, similar to [hexyl](https://github.com/sharkdp/hexyl).
  - "entropy", which colors bytes based on the standard deviation of neighboring bytes.

## Usage

Not currently packaged and distributed anywhere, to use hexie you'll need to clone the repository and build from source:

    $ git clone https://github.com/samwho/hexie
    $ cd hexie
    $ cargo build --release
    $ target/release/hexie src/main.rs

From now on in the README we're going to assume you've built the binary and it's on your PATH.

## Flags

Reading from an arbitrary point:

    $ echo "hello, world!" | hexie -s 10
    0x0000000A │ 6C 64 21 0A │ ld!.

Reading to an arbitrary point:

    $ echo "hello, world!" | hexie -e 10
    0x00000000 │ 68 65 6C 6C 6F 2C 20 77 6F 72 │ hello, wor

Reading an arbitrary number of bytes:

    $ echo "hello, world!" | hexie -n 2
    0x00000000 │ 68 65 │ he

Combining the above:

    $ echo "hello, world!" | hexie -s 3 -n 2
    0x00000003 │ 6C 6F │ lo
    $ echo "hello, world!" | hexie -e 3 -n 2
    0x00000001 │ 65 6C │ el
    $ echo "hello, world!" | hexie -s 3 -e 5
    0x00000003 │ 6C 6F 2C 20 77 │ lo, w

All indexes can be given in hex as well as decimal:

    $ echo "hello, world!" | hexie -s 0x0A
    0x0000000A │ 6C 64 21 0A │ ld!.

And if you're sick of the pager:

    $ echo "hello, world!" | hexie --nopager
    0x00000000 │ 68 65 6C 6C 6F 2C 20 77 6F 72 6C 64 21 0A │ hello, world!.