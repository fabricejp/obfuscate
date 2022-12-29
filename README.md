# Obfuscate

Obfuscate an e-mail address by only leaving two asterisk characters before and after the @ symbol.

An example:

```
$ obfuscate john.doe@megacorp.com
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
$ cargo obfuscate
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

