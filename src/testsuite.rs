#[cfg(test)]
mod sha1 {
    
    use crate::sha::Sha1;

    macro_rules! functionnal_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let mut sha:Sha1 = Sha1::new();
                    sha.update(&data.as_bytes());
                    sha.digest();
                    let result = sha.digest_string();
                    assert_eq!(result, expected);
                }
            )*
        }
    }
    macro_rules! split_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let mut sha:Sha1 = Sha1::new();

                    data.as_bytes().chunks(12)
                        .for_each(|slice| sha.update(&slice));

                    sha.digest();
                    let result = sha.digest_string();
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    split_tests! {
        split: (
        "leteamsestpas laeton est lundi ca debug le sha256 en bien",
        "e2e0aa8a8a5e0c71c8c7f2222ac09b2d4ed2ac25"
        ),
        long: (
        "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "345cca26ce8b9db606c24ae853ec4010883262f8"
        ),
    }
    functionnal_tests! {
        arbitrary_text: (
        "leteamsestpas laeton est lundi ca debug le sha256 en bien",
        "e2e0aa8a8a5e0c71c8c7f2222ac09b2d4ed2ac25"
        ),
        Maj: (
        "MJUYBEVFSCVGUEIJHUYUGDUYGYUDG",
        "9c38480f8c0b4dff753cd3fcd76687bc595d283a"
        ),
        text: (
        "merklerootbro",
        "82cb2e362fbbeadc8154cacc16b6b19d0bad5db0"
        ),
        empty: (
        "",
        "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        ),
    }
}

#[cfg(test)]
mod sha256 {
    
    use crate::sha::Sha256;
    use std::fs;

    macro_rules! sha256_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let mut sha:Sha256 = Sha256::new();
                    sha.update(&data.as_bytes());
                    sha.digest();
                    let result = sha.digest_string();
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    macro_rules! test_from_files {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (path, expected) = $args;
                    let mut sha:Sha256 = Sha256::new();

                    let content = fs::read_to_string(path)
                        .expect("failed to read");

                    sha.update(&content.as_bytes());
                    sha.digest();

                    let result = sha.digest_string();

                    assert_eq!(result, expected);
                }
            )*
        }
    }

    // tool used for checking hash values
    // https://emn178.github.io/online-tools/sha256_checksum.html
    test_from_files! {
        lorem: (
        "src/testsuite/lorem.txt",
        "c47562e9a9c8d50170e6f9a56dae03ac37028560cbd9a70708a33241b38f2c06"
        ),
        verif: (
        "src/testsuite/verif.txt",
        "80b4486094df240d69161c616650633dadf587cfc1da7b47851baf30a711fe3c"
        ),
        medium_size: (
        "src/testsuite/medium.txt",
        "f09aebda3af1b9597248d028c39d292ab23e67785ed317582554d14efa207fc6"
        ),
        medium_plus: (
        "src/testsuite/medium_plus.txt",
        "36f68b00c246ad9f689b6987d31cef7994b4f0a8fb5fc2b6d9abb90ddb70462a"
        ),
    }

    sha256_tests! {
        emoji: (
        "✈️",
        "185beb968bd1a81d07ebcf82376642f7b29f1b5594b21fe9edee714efbdcaa44"
        ),
        text: (
        "merklerootbro",
        "7b25bc0efdad46daec550347cf8f542d1ecca231ccb5c3877778a9b94cea498f"
        ),
        empty: (
        "",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ),
        simple_small: (
        "abc",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        ),
        simple_chill: (
        "a",
        "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb"
        ),
        numbers: (
        "0987654321",
        "17756315ebd47b7110359fc7b168179bf6f2df3646fcc888bc8aa05c78b38ac1"
        ),
        simple_classic: (
        "abcde",
        "36bbe50ed96841d10443bcb670d6554f0a34b761be67ec9c4a8ad2c0c44ca42c"
        ),
        medium_chill: (
        "leteamsestpas la",
        "b3d5b293ef9bc2660c3598945f2b229c1710081513608534c06b26e647f4b12a"
        ),
        tricky_padding_1: (
        "cccc",
        "b6fbd675f98e2abd22d4ed29fdc83150fedc48597e92dd1a7a24381d44a27451"
        ),
        tricky_padding_length: (
        "cccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "f6c7b87acd114115d66897c8cb138c16a8b886673d1b93737f4918be472ea878"
        ),
        lots_of_c: (
        "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        "b9defaed1cf0009ea9e17a221356b92483696dbefc9954522348cd796814ed9b"
        ),
        arbitrary_length: (
        "leteamsestpas laeton est lundi ca debug le sha256 en bien",
        "2e6c522167c036d4091ab5d119a3a1c2963852358c08bade2816a1536aa996b5"
        ),
        lorem_ipsum: (
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pellentesque ipsum eu nunc gravida, ac porttitor tortor blandit. Phasellus posuere, leo eu volutpat efficitur, ante lectus bibendum lectus, vitae fermentum nisi sapien eu mi. Fusce sed euismod ligula. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aenean porttitor quam leo, eu fringilla magna pharetra eu. Praesent sagittis felis tortor, sit amet faucibus arcu posuere in. Curabitur mattis urna sed nunc scelerisque feugiat. Cras id lobortis tellus. Nullam fringilla quam eu maximus tincidunt. Sed placerat, nisl in eleifend vehicula, massa odio interdum nisl, vel varius lorem felis in nunc. Mauris laoreet diam vel felis tristique elementum. Pellentesque egestas purus orci, at bibendum justo cursus ac. Etiam malesuada, nulla hendrerit consequat faucibus, elit turpis semper neque, ut suscipit augue enim id lacus. Vestibulum et laoreet lorem. In semper orci vitae ligula vestibulum, consectetur pretium nisi luctus.",
        "54eb549e06fd33ff05765fb1c19ee8df6d58400e3c168dd3e3d5a962a8856504"
        ),
    }
}

mod md5 {

   use super::*; 
   use crate::Md5; 

    macro_rules! md5_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let mut md5:Md5 = Md5::new();
                    md5.update(&data.as_bytes());
                    md5.digest();
                    let result = md5.digest_string();
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    macro_rules! test_from_files {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (path, expected) = $args;
                    let mut md5:Md5 = Md5::new();

                    let content = fs::read_to_string(path)
                        .expect("failed to read");

                    md5.update(&content.as_bytes());
                    md5.digest();

                    let result = md5.digest_string();

                    assert_eq!(result, expected);
                }
            )*
        }
    }

    test_from_files! {}

    md5_tests! {
        fox: (
        "The quick brown fox jumps over the lazy dog",
        "9e107d9d372bb6826bd81d3542a419d6"
        ),
        empty: (
        "",
        "d41d8cd98f00b204e9800998ecf8427e"
        ),
        abc: (
        "abc",
        "900150983cd24fb0d6963f7d28e17f72"
        ),
        digits: (
        "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
        "57edf4a22be3c955ac49da2e2107b67a"
        ),
        message_digest: (
        "message digest",
        "f96b697d7cb7938d525a2f31aaf161d0"
        ),
        a:(
            "a",
            "0cc175b9c0f1b6a831c399e269772661"
        ),
        alphabet:(
            "abcdefghijklmnopqrstuvwxyz",
            "c3fcd3d76192e4007dfb496cca67e13b"
        ),
    }
}
