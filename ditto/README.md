# dart

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

Ensure that you have correctly built binaries in the `rust` folders based on the tutorial in `../docs/part1.md`

Dart uses /bin as the conventional folder to store `.dart` files. 

In this root directory containing `pubspec.yaml` run 

```bash
pub get
pub run main.dart
```

