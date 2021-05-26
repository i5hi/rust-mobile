// ignore_for_file: omit_local_variable_types
import 'dart:ffi';

import 'package:ffi/ffi.dart';

typedef mnemonic = Pointer<Utf8> Function(Pointer<Utf8> length);
typedef Words = Pointer<Utf8> Function(Pointer<Utf8> length);

typedef seed_to_master_xprv = Pointer<Utf8> Function(Pointer<Utf8> mnemonic, Pointer<Utf8> passphrase, Pointer<Utf8> network);
typedef MasterKey = Pointer<Utf8> Function(Pointer<Utf8> mnemonic, Pointer<Utf8> passphrase, Pointer<Utf8> network);

typedef derive_hardened = Pointer<Utf8> Function(Pointer<Utf8> master_xprv, Pointer<Utf8>account);
typedef ChildKeys = Pointer<Utf8> Function(Pointer<Utf8> master_xprv, Pointer<Utf8>account);

void main() {
  DynamicLibrary dl = DynamicLibrary.open(
    'rust/target/release/libditto.so',
  );
  var mneu = dl.lookupFunction<mnemonic, Words>('mnemonic');
  final twelve = '12'.toNativeUtf8();
  var words = mneu(twelve);
  final str = words.toDartString();
  print('words:::' + str);

  var s2xprv = dl.lookupFunction<seed_to_master_xprv, MasterKey>('seed_to_master_xprv');
  final passphrase = "".toNativeUtf8();
  final network = "test".toNativeUtf8();
  final master_ptr = s2xprv(words,passphrase,network);

  final master_str = master_ptr.toDartString();
  print('master:::' + master_str);

  var derive = dl.lookupFunction<derive_hardened,ChildKeys>('derive_hardened');
  final account = '0'.toNativeUtf8();
  final child_ptr = derive(master_ptr,account);
  final child_str = child_ptr.toDartString();
  print('child:::' + child_str);
  // final xprv = s2xprv(vals,"test".toNativeUtf8(),"test".toNativeUtf8());


}
