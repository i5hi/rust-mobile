# dart

This root directory contains the dart project files, with `rust` as a subdirectory being used as a library.

Dart uses `bin` as the conventional directory to store `.dart` files. 

## Installation on Linux

```bash
 sudo apt-get update
 sudo apt-get install apt-transport-https
 sudo sh -c 'wget -qO- https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add -'
 sudo sh -c 'wget -qO- https://storage.googleapis.com/download.dartlang.org/linux/debian/dart_stable.list > /etc/apt/sources.list.d/dart_stable.list'

 sudo apt-get update
 sudo apt-get install dart

 echo 'export PATH="$PATH:/usr/lib/dart/bin"' >> ~/.profile 
 source ~/.profile

```

# Test

Ensure that you have correctly built binaries in the `rust/target/release` directory based on the tutorial in `../docs/part1.md`

In this root directory containing `pubspec.yaml` run 

```bash
pub get
pub run main.dart
```

