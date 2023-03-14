Obfuscate an email address using asterisk characters, and by only leaving two randomly chosen characters visible before and after the @ symbol.

An example

```
$ obfuscate the@dude.org
**e@***e.***
```

# Compile and install
Assuming Rust is installed

```
git clone git@github.com:fabricejp/obfuscate.git \
cd obfuscate \
cargo build --release \
sudo cp target/release/obfuscate /usr/local/bin/
```
