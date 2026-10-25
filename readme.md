# Rust Identicon
[<img src="https://badges.ws/badge/Built_with_Rust-000000?logo=rust" alt="Built with Rust" />](https://www.rust-lang.org)
<img src="https://badges.ws/badge/testing-experimental-ff0000">

Linux tool for identicon generation. The tool takes an input buffer from which it generates a unique identicon based off the input. Useful for generating identicons for files, folders or general purpose.
## Installation
Requirements:
```
Rust
Cargo
Git
Feh (Optional)
```
```sh
git clone https://github.com/xxxdddxxx/identicon-rs.git
cd identicon-rs
cargo build --release
cd ./target/release
```
## Usage
```sh
user@laptop:~/identicon-rs/target/release$ ./identicon-rs -h
Usage: identicon-rs [OPTIONS]

Options:
  -s, --size <SIZE>    [default: 5]
  -x, --scale <SCALE>  [default: 64]
  -i, --input <INPUT>  [default: 1024]
  -a, --alpha <ALPHA>  [default: 0.3]
  -h, --help           Print help
  -V, --version        Print version
```
`Example`
-
```sh
user@laptop:~/identicon-rs/target/release$ head -n 64 /dev/urandom | ./identicon-rs -s 9 | feh -
dbg;b=1024;h=0x9a92aa3c;hue=260
```
Where the information printed is the debugging information being `b` the maximum buffer input, `h` the generated hash and `hue` the corresponding color.

`Result`
---
<img width="33%" alt="image" src="https://github.com/user-attachments/assets/bc087c81-e8bc-4a59-9c7a-ad6a5a39c88a" />
<img width="33%" alt="image" src="https://github.com/user-attachments/assets/695e0c1f-8f9a-4d92-855d-b26efe0795eb" />
<img width="33%" alt="image" src="https://github.com/user-attachments/assets/83cc9a7f-0df2-4f39-8f70-2ddfa125b06a" />

---

# TO DO
- [ ] Reduce collision changing `u32` hash to a `u64`
- [ ] Add JPG and SVG support
