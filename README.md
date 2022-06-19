# sckrd
Search for krypto-keys in ram dumps

## Description
This Tool is able to analyse Byte dumps like ram images to find possible crypto keys.
The criteria for beeing a possible key is its high entropy.
Is such a byte sequence with high entropy found it can be printed to a file or to stdout to show it to the user.
The possible keys can be tried to decrypt encrypted files at the hard disk where the ram image is taken from.


## Installation
1. You have to install Rust and its components (https://www.rust-lang.org/tools/install).

2. Download the source code of sckrd from this github page.

3. Change Directory to the root directory of sckrd.

4. Call cargo build to build the project or use cargo run to build and run the programm.

For further options check out the manual of the cargo tool