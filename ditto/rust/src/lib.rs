use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::str::FromStr;

use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;

use bip39::{Language, Mnemonic};
use bitcoin::network::constants::Network;

use bitcoin::consensus::deserialize;
use bitcoin::consensus::serialize;

use bitcoin::util::bip32::DerivationPath;
use bitcoin::util::bip32::ExtendedPrivKey;
use bitcoin::util::bip32::ExtendedPubKey;
use bitcoin::util::psbt::PartiallySignedTransaction;

use bdk::database::MemoryDatabase;
use bdk::descriptor::checksum;
use bdk::Wallet;

#[no_mangle]
pub extern "C" fn mnemonic(length: *const c_char) -> *mut c_char {
    let input_cstr = unsafe { CStr::from_ptr(length) };
    let len: usize = match input_cstr.to_str() {
        Err(_) => 12,
        Ok(string) => string.parse::<usize>().unwrap(),
    };

    let mut rng = OsRng::new().expect("!!OsRng Error!!");
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, len)
        .unwrap()
        .to_string();

    CString::new(mnemonic).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn seed_to_master_xprv(
    mnemonic: *const c_char,
    passphrase: *const c_char,
    network: *const c_char,
) -> *mut c_char {
    let mnemonic_cstr = unsafe { CStr::from_ptr(mnemonic) };
    let mnemonic: &str = match mnemonic_cstr.to_str() {
        Err(_) => return CString::new("Mnemonic Error").unwrap().into_raw(),
        Ok(string) => &string,
    };
    let passphrase_cstr = unsafe { CStr::from_ptr(passphrase) };
    let passphrase: &str = match passphrase_cstr.to_str() {
        Err(_) => "",
        Ok(string) => &string,
    };

    let network_cstr = unsafe { CStr::from_ptr(network) };
    let network: &str = match network_cstr.to_str() {
        Err(_) => "test",
        Ok(string) => &string,
    };
    let network_enum = match network {
        "main" => Network::Bitcoin,
        "test" => Network::Testnet,
        _ => Network::Testnet,
    };
    let seed = Mnemonic::parse_in(Language::English, mnemonic)
        .unwrap()
        .to_seed(passphrase);
    let master_xprv = ExtendedPrivKey::new_master(network_enum, &seed).unwrap();
    CString::new(master_xprv.to_string()).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn derive_hardened(
    master_xprv: *const c_char,
    account: *const c_char,
) -> *mut c_char {
    let master_xprv_cstr = unsafe { CStr::from_ptr(master_xprv) };
    let master_xprv: &str = match master_xprv_cstr.to_str() {
        Err(_) => return CString::new("MasterXprv Error").unwrap().into_raw(),
        Ok(string) => &string,
    };
    let account_cstr = unsafe { CStr::from_ptr(account) };
    let account: &str = match account_cstr.to_str() {
        Err(_) => "0",
        Ok(string) => &string,
    };

    let secp = Secp256k1::new();
    // // calculate root key from seed
    let root = ExtendedPrivKey::from_str(master_xprv).unwrap();
    let fingerprint = root.fingerprint(&secp);
    let network = root.network;
    let first = "m/84h";

    let second = match network {
        Network::Bitcoin => "/0h",
        Network::Testnet => "/1h",
        _ => "/1h",
    };
    let third = String::from("/") + account + "h";

    let hardened_path = String::from(first) + &second + &third;
    let path = DerivationPath::from_str(&hardened_path).unwrap();
    let child_xprv = root.derive_priv(&secp, &path).unwrap();

    let child_xpub = ExtendedPubKey::from_private(&secp, &child_xprv);
    let result = fingerprint.to_string()
        + ":"
        + &hardened_path
        + ":"
        + &child_xprv.to_string()
        + ":"
        + &child_xpub.to_string();
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn sign_solo_psbt(
    fingerprint: *const c_char,
    account_index: *const c_char,
    account_xprv: *const c_char,
    psbt: *const c_char,
) -> *mut c_char {
    let fingerprint_cstr = unsafe { CStr::from_ptr(fingerprint) };
    let fingerprint: &str = match fingerprint_cstr.to_str() {
        Err(_) => return CString::new("Fingerprint Error").unwrap().into_raw(),
        Ok(string) => &string,
    };

    let account_index_cstr = unsafe { CStr::from_ptr(account_index) };
    let account_index: &str = match account_index_cstr.to_str() {
        Err(_) => return CString::new("Account Index Error").unwrap().into_raw(),
        Ok(string) => &string,
    };

    let account_xprv_cstr = unsafe { CStr::from_ptr(account_xprv) };
    let account_xprv: &str = match account_xprv_cstr.to_str() {
        Err(_) => return CString::new("Account XPrv Error").unwrap().into_raw(),
        Ok(string) => &string,
    };

    let psbt_cstr = unsafe { CStr::from_ptr(psbt) };
    let psbt: &str = match psbt_cstr.to_str() {
        Err(_) => return CString::new("PSBT Error").unwrap().into_raw(),
        Ok(string) => &string,
    };

    let account_xprv = ExtendedPrivKey::from_str(account_xprv).unwrap();
    let network = account_xprv.network;

    let network_path = match network {
        Network::Bitcoin => "/0h",
        Network::Testnet => "/1h",
        _ => "/1h",
    };

    let key_descriptor = String::from("[")
        + fingerprint
        + "/84h"
        + network_path
        + "/"
        + account_index
        + "h]"
        + &account_xprv.to_string();

    let mut desc_deposit = String::from("wpkh(") + &key_descriptor + "/0/*)";
    let mut desc_change = String::from("wpkh(") + &key_descriptor + "/1/*)";
    desc_deposit =
        desc_deposit.clone() + "#" + &checksum::get_checksum(&desc_deposit.clone()).unwrap();
    desc_change =
        desc_change.clone() + "#" + &checksum::get_checksum(&desc_change.clone()).unwrap();

    let wallet = Wallet::new_offline(
        &desc_deposit,
        Some(&desc_change),
        network,
        MemoryDatabase::default(),
    )
    .unwrap();

    let psbt_struct: PartiallySignedTransaction =
        deserialize(&base64::decode(psbt).unwrap()).unwrap();

    let (signed_psbt, finalized) = wallet.sign(psbt_struct, None).unwrap();

    let result = finalized.to_string() + ":" + &base64::encode(&serialize(&signed_psbt));
    CString::new(result).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitcoin_composite() {
        let mnemonic = "cabbage love belt believe coil nut parent leisure sister display novel garlic lawsuit have water pyramid derive bench carry during quick wide arena battle";
        let mnemonic_cstr = CString::new(mnemonic).unwrap().into_raw();

        let passphrase = "";
        let passphrase_cstr = CString::new(passphrase).unwrap().into_raw();

        let network_cstr = CString::new("test").unwrap().into_raw();

        let master_keys = seed_to_master_xprv(mnemonic_cstr, passphrase_cstr, network_cstr);
        let master_keys_cstr = unsafe { CStr::from_ptr(master_keys) };
        let master_keys: &str = match master_keys_cstr.to_str() {
            Err(_) => "Error",
            Ok(string) => &string,
        };

        let expected_xprv = "tprv8ZgxMBicQKsPemPpNMf8T3vsYavbM5izZ7LoWanGF1syCK1hMUw24yABcnkZXpGvBQWjirHMDTwsnTEBFH8wGtjAoZaSJQo3scM51H12898";

        assert_eq!(master_keys, expected_xprv);
        let account_index = "0";
        let xprv_cstr = CString::new(expected_xprv).unwrap().into_raw();
        let account_cstr = CString::new(account_index).unwrap().into_raw();
        let fingerprint = "ecf2c469";
        let hardened_path = "m/84h/1h/0h";
        let account_xprv = "tprv8fycmZ5gxLRvYG84dkGsa9Uks45SNTPZxJcWx5hY3owexdKwzRnqPkVoqb3s4iTcKgiMcoXQB9tJjWM5WSodZspH3j3xZeefsoyfuUX1bp8";
        let account_xpub = "tpubDCfeuy7w6i7bRj9rXPwTyZ8sS5bNXnaUXcDJEbjqU5k3o7aicpcRaF7g1iAHRApsHGsCMwqpYsvMZLaiNBX6DKq3FHnV5zN4cAf6ugQ44u4";
        let expected = String::from(fingerprint)
            + ":"
            + hardened_path
            + ":"
            + account_xprv
            + ":"
            + account_xpub;

        let result = derive_hardened(xprv_cstr, account_cstr);
        let result_cstr = unsafe { CStr::from_ptr(result) };
        let result: &str = match result_cstr.to_str() {
            Err(_) => "Error",
            Ok(string) => &string,
        };
        assert_eq!(result, &expected);

        let psbt = "cHNidP8BAKkCAAAAAoqqeP2daf1RU9VL9CaR7S4UJLcotwU9OOHv1J/I5JqPAAAAAAD9////VoiAqZfCMID69ugXnGUX0+Ij8yZ/BJfYnBoUquSWIpsBAAAAAP3///8CmjIKAAAAAAAiACBYBDj7ZrV3CNuMoebownkS83v+6sQ/5DjKm89A4g5KDGdLHQAAAAAAGXapFJ+aer1gDAyqA5g6d8jD344GLLL6iKwAAAAAAAEAcgIAAAABJnTOtLxBKeA1mqhGpu8rR7/YLPwMJyhuEw2fOzE8XoEBAAAAAP3///8C2lMOAAAAAAAWABQeuC3YiYhHLznMuAUJLpk6BQ6iwMAnCQAAAAAAF6kUdrhRPQ26tEnOcvKFz1Ztz8dHnPWHAAAAAAEBH9pTDgAAAAAAFgAUHrgt2ImIRy85zLgFCS6ZOgUOosAiBgMC0anznoOtyODTLDSJPpOwY5iyC0KhnuoQ0ktiZhCL/hjs8sRpVAAAgAEAAIAAAACAAQAAABAAAAAAAQBxAgAAAAEdfP0pBSgaeHczMNMwcpGIpoUriRrTakNgS5V8qgPF2QEAAAAA/v///wL6XrQcAQAAABYAFHuvCJ0yQugRPmqFIffpMz2f9gPFBysZAAAAAAAWABTlADAmT4AfSx7f0O0k9AOeWhZQ4fgmHgABAR8HKxkAAAAAABYAFOUAMCZPgB9LHt/Q7ST0A55aFlDhIgYCEqWzXFmYG7GPgsQYoVBPql8FcxahjtrzCq03sdD00/QY7PLEaVQAAIABAACAAAAAgAAAAAAtAAAAAAAA";
        let psbt_cstr = CString::new(psbt).unwrap().into_raw();
        let account_xprv_cstr = CString::new(account_xprv).unwrap().into_raw();
        let fingerprint_cstr = CString::new(fingerprint).unwrap().into_raw();

        let expected = "true:cHNidP8BAKkCAAAAAoqqeP2daf1RU9VL9CaR7S4UJLcotwU9OOHv1J/I5JqPAAAAAAD9////VoiAqZfCMID69ugXnGUX0+Ij8yZ/BJfYnBoUquSWIpsBAAAAAP3///8CmjIKAAAAAAAiACBYBDj7ZrV3CNuMoebownkS83v+6sQ/5DjKm89A4g5KDGdLHQAAAAAAGXapFJ+aer1gDAyqA5g6d8jD344GLLL6iKwAAAAAAAEAcgIAAAABJnTOtLxBKeA1mqhGpu8rR7/YLPwMJyhuEw2fOzE8XoEBAAAAAP3///8C2lMOAAAAAAAWABQeuC3YiYhHLznMuAUJLpk6BQ6iwMAnCQAAAAAAF6kUdrhRPQ26tEnOcvKFz1Ztz8dHnPWHAAAAAAEBH9pTDgAAAAAAFgAUHrgt2ImIRy85zLgFCS6ZOgUOosAiAgMC0anznoOtyODTLDSJPpOwY5iyC0KhnuoQ0ktiZhCL/kgwRQIhAJomnO6SPdHYwcYTsr6TFWkZh5S/sDVVNmoBJ3FwvIgOAiA1eyfIga93ib9qjrH3H7REyU/eVOOoQiLXpc+qhWko4wEiBgMC0anznoOtyODTLDSJPpOwY5iyC0KhnuoQ0ktiZhCL/hjs8sRpVAAAgAEAAIAAAACAAQAAABAAAAABBwABCGwCSDBFAiEAmiac7pI90djBxhOyvpMVaRmHlL+wNVU2agEncXC8iA4CIDV7J8iBr3eJv2qOsfcftETJT95U46hCItelz6qFaSjjASEDAtGp856Drcjg0yw0iT6TsGOYsgtCoZ7qENJLYmYQi/4AAQBxAgAAAAEdfP0pBSgaeHczMNMwcpGIpoUriRrTakNgS5V8qgPF2QEAAAAA/v///wL6XrQcAQAAABYAFHuvCJ0yQugRPmqFIffpMz2f9gPFBysZAAAAAAAWABTlADAmT4AfSx7f0O0k9AOeWhZQ4fgmHgABAR8HKxkAAAAAABYAFOUAMCZPgB9LHt/Q7ST0A55aFlDhIgICEqWzXFmYG7GPgsQYoVBPql8FcxahjtrzCq03sdD00/RHMEQCIA/zLyzlXtNxp4xbW9b6Ts5KM2mF4Ia2M42R4FU92DaeAiA2Ve5qFifRo1MMmTuYPT3GYSlT2vTlPUTKKoLTGL0W6AEiBgISpbNcWZgbsY+CxBihUE+qXwVzFqGO2vMKrTex0PTT9Bjs8sRpVAAAgAEAAIAAAACAAAAAAC0AAAABBwABCGsCRzBEAiAP8y8s5V7TcaeMW1vW+k7OSjNpheCGtjONkeBVPdg2ngIgNlXuahYn0aNTDJk7mD09xmEpU9r05T1EyiqC0xi9FugBIQISpbNcWZgbsY+CxBihUE+qXwVzFqGO2vMKrTex0PTT9AAAAA==";
        let signed = sign_solo_psbt(fingerprint_cstr, account_cstr, account_xprv_cstr, psbt_cstr);
        let signed_cstr = unsafe { CStr::from_ptr(signed) };
        let signed: &str = match signed_cstr.to_str() {
            Err(_) => "Error",
            Ok(string) => &string,
        };
        assert_eq!(signed, expected);


    }
}
