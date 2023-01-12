```    _      __                     _       
      | |    / _|                   | |      
  ___ | |__ | |_ _   _ ___  ___ __ _| |_ ___ 
 / _ \| '_ \|  _| | | / __|/ __/ _` | __/ _ \
| (_) | |_) | | | |_| \__ \ (_| (_| | ||  __/
 \___/|_.__/|_|  \__,_|___/\___\__,_|\__\___|

```                                                   


Obfuscate an e-mail address by only leaving two asterisk characters before and after the @ symbol.

An example:

```
$ obfuscate john.doe@mailprovider.org
******o*@****p**v********
```

# Compile and install
Assuming Rust is installed

```
git clone git@github.com:fabricejp/obfuscate.git \
cd obfuscate \
cargo build --release \
sudo cp target/debug/obfuscate /usr/local/bin/
```
