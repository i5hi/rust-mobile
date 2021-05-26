// ignore_for_file: omit_local_variable_types
import 'dart:ffi';
import 'package:ffi/ffi.dart';

typedef ffi_func = Pointer<Utf8> Function(
  Pointer<Utf8> length,
);

typedef dart_func = Pointer<Utf8> Function(
  Pointer<Utf8> length,
);

void main() {
  DynamicLibrary dl = DynamicLibrary.open(
    'rust/target/release/libditto.so',
  );
  var mneu = dl.lookupFunction<ffi_func, dart_func>('mnemonic');
  final twelve = '12'.toNativeUtf8();
  var vals = mneu(twelve);
  final str = vals.toDartString();
  print('words:::' + str);
}
