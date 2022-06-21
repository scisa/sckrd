# sckrd
Search for krypto-keys in ram dumps

## Description
This tool is able to analyse byte dumps like ram images to find possible crypto keys.
The criteria for beeing a possible key is its high entropy.
Bytes with a high entropy are more likely to be a real key. 
Based on an entropy-delta, it filters out only byte sequences within this entropy range.
Is a byte sequence with a high entropy found, it can be printed to a file or to stdout.
The found possible keys can be used for trying to decrypt encrypted files on the hard disk, where the ram image is taken from.


## Installation
1. You have to install Rust and its components (https://www.rust-lang.org/tools/install).

2. Download the source code of sckrd from this github page.

3. Change directory to the root directory of sckrd.

4. Call cargo build to build the project or use cargo run to build and run the programm.

For further options check out the manual of the cargo tool