// ignore_for_file: omit_local_variable_types
import 'dart:ffi';

import 'package:ffi/ffi.dart';

typedef mnemonic = Pointer<Utf8> Function(Pointer<Utf8> length);
typedef Words = Pointer<Utf8> Function(Pointer<Utf8> length);

typedef seed_to_master_xprv = Pointer<Utf8> Function(Pointer<Utf8> mnemonic, Pointer<Utf8> passphrase, Pointer<Utf8> network);
typedef MasterKey = Pointer<Utf8> Function(Pointer<Utf8> mnemonic, Pointer<Utf8> passphrase, Pointer<Utf8> network);

typedef derive_hardened = Pointer<Utf8> Function(Pointer<Utf8> master_xprv, Pointer<Utf8>account);
typedef ChildKeys = Pointer<Utf8> Function(Pointer<Utf8> master_xprv, Pointer<Utf8>account);

typedef sign_solo_psbt = Pointer<Utf8> Function(Pointer<Utf8> fingerprint,Pointer<Utf8> account_index,Pointer<Utf8> account_xprv,Pointer<Utf8> psbt);
typedef PSBT = Pointer<Utf8> Function(Pointer<Utf8> fingerprint, Pointer<Utf8> account_index, Pointer<Utf8> account_xprv, Pointer<Utf8> psbt);

void main() {
  DynamicLibrary dl = DynamicLibrary.open(
    'rust/target/release/libditto.so',
  );
  var mneu = dl.lookupFunction<mnemonic, Words>('mnemonic');
  final twelve = '12'.toNativeUtf8();
  var words = mneu(twelve);
  final str = words.toDartString();
  print('words:::' + str + "\n");

  var s2xprv = dl.lookupFunction<seed_to_master_xprv, MasterKey>('seed_to_master_xprv');
  final passphrase = "".toNativeUtf8();
  final network = "test".toNativeUtf8();
  final master_ptr = s2xprv(words,passphrase,network);

  final master_str = master_ptr.toDartString();
  print('master:::' + master_str + "\n");

  var derive = dl.lookupFunction<derive_hardened,ChildKeys>('derive_hardened');
  final account = '0'.toNativeUtf8();
  final child_ptr = derive(master_ptr,account);
  final child_str = child_ptr.toDartString();
  print('child:::' + child_str + "\n");


  var sign = dl.lookupFunction<sign_solo_psbt,PSBT>('sign_solo_psbt');
  final fingerprint = "ecf2c469".toNativeUtf8();
  final account_index = "0".toNativeUtf8();
  final account_xprv = "tprv8fycmZ5gxLRvYG84dkGsa9Uks45SNTPZxJcWx5hY3owexdKwzRnqPkVoqb3s4iTcKgiMcoXQB9tJjWM5WSodZspH3j3xZeefsoyfuUX1bp8".toNativeUtf8();
  final psbt = "cHNidP8BAKkCAAAAAoqqeP2daf1RU9VL9CaR7S4UJLcotwU9OOHv1J/I5JqPAAAAAAD9////VoiAqZfCMID69ugXnGUX0+Ij8yZ/BJfYnBoUquSWIpsBAAAAAP3///8CmjIKAAAAAAAiACBYBDj7ZrV3CNuMoebownkS83v+6sQ/5DjKm89A4g5KDGdLHQAAAAAAGXapFJ+aer1gDAyqA5g6d8jD344GLLL6iKwAAAAAAAEAcgIAAAABJnTOtLxBKeA1mqhGpu8rR7/YLPwMJyhuEw2fOzE8XoEBAAAAAP3///8C2lMOAAAAAAAWABQeuC3YiYhHLznMuAUJLpk6BQ6iwMAnCQAAAAAAF6kUdrhRPQ26tEnOcvKFz1Ztz8dHnPWHAAAAAAEBH9pTDgAAAAAAFgAUHrgt2ImIRy85zLgFCS6ZOgUOosAiBgMC0anznoOtyODTLDSJPpOwY5iyC0KhnuoQ0ktiZhCL/hjs8sRpVAAAgAEAAIAAAACAAQAAABAAAAAAAQBxAgAAAAEdfP0pBSgaeHczMNMwcpGIpoUriRrTakNgS5V8qgPF2QEAAAAA/v///wL6XrQcAQAAABYAFHuvCJ0yQugRPmqFIffpMz2f9gPFBysZAAAAAAAWABTlADAmT4AfSx7f0O0k9AOeWhZQ4fgmHgABAR8HKxkAAAAAABYAFOUAMCZPgB9LHt/Q7ST0A55aFlDhIgYCEqWzXFmYG7GPgsQYoVBPql8FcxahjtrzCq03sdD00/QY7PLEaVQAAIABAACAAAAAgAAAAAAtAAAAAAAA".toNativeUtf8();
  final psbt_ptr = sign(fingerprint,account_index,account_xprv,psbt);
  final psbt_str = psbt_ptr.toDartString();
  print('psbt:::' + psbt_str + "\n");
  // final xprv = s2xprv(vals,"test".toNativeUtf8(),"test".toNativeUtf8());


}
