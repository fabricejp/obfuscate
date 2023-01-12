# Obfuscate

Obfuscate an e-mail address by only leaving two asterisk characters before and after the @ symbol.

An example:

```
$ obfuscate john.doe@mailprovider.org
******o*@****p**v********
```

# Compile
Assuming Rust is installed

```
$ git clone git@github.com:fabricejp/obfuscate.git
```
```
$ cd obfuscate
```
```
$ cargo build --release
```
```
$ sudo cp target/debug/obfuscate /usr/local/bin/
```
```
$ cd
```
```
$ source .zshrc
(assuming zsh is your shell)
```

