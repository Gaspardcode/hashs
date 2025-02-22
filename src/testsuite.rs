#[cfg(test)]
mod sha1 {
    use super::*;
    use crate::bits::Bits;
    use crate::sha::Sha1;

    macro_rules! sha1_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let mut sha:Sha1 = Sha1::new();
                    sha.update(&data.as_bytes());
                    let result = sha.digest_string();
                    assert_eq!(result, expected);
                }
            )*
        }
    }
    sha1_tests! {
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
    use super::*;
    use crate::bits::Bits;
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

                    let mut content = fs::read_to_string(path)
                        .expect("failed to read");

                    content.pop();
                    let ch = content.pop().unwrap();

                    if ch != '\r' {
                        content.push(ch);
                    }
                    
                    sha.update(&content.as_bytes());
                    let result = sha.digest_string();
                    
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    test_from_files! {
        /*
        lorem: (
        "src/testsuite/lorem.txt",
        "33fd1906bd900f30454ce0e7ec717c04bfcbe466fa3a081c2e5659c0074b9f4b"
        ),
        */
        verif: (
        "src/testsuite/verif.txt",
        "7b25bc0efdad46daec550347cf8f542d1ecca231ccb5c3877778a9b94cea498f"
        ),
        medium_size: (
        "src/testsuite/medium.txt",
        "5fcf5b38ef138ca8f83f827a6e4c323020bb9576a566fef793ce30a87f10ae16"
        ),
        medium_plus: (
        "src/testsuite/medium_plus.txt",
        "2997c402eb89d77fc28e08479542a2215bb6218a926b3c290ea07980f29b8b18"
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

/*
#[cfg(test)]
mod unit {
    use super::*;
    use crate::bits::Bits;

    macro_rules! rotR_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, rot_nb, expected) = $args;
                    let mut raw_bits = Bits::new(&data);
                    raw_bits.rotR(rot_nb);
                    assert_eq!(raw_bits.value, expected);
                }
            )*
        }
    }
    macro_rules! circular_rotR_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, rot_nb, expected) = $args;
                    let mut raw_bits = Bits::new(&data);
                    raw_bits.circular_rotR(rot_nb);
                    assert_eq!(raw_bits.value, expected);
                }
            )*
        }
    }
    
    rotR_tests! {
        rot_no_byte: (vec![], 8, vec![]),
        rot_empty: (vec![0,0,0], 8, vec![0,0,0]),
        rot_empty_not_round: (vec![0,0,0], 17, vec![0,0,0]),
        rot_single_quad: (vec![32], 2, vec![8]),
        rot_single_quad_bis: (vec![2 << 31], 31, vec![1]),
        rot_single_quad_too_much: (vec![32], 32, vec![0]),
        rot_two_quad: (vec![32, 32], 2, vec![8, 8]),
        rot_two_quad_tricky: (vec![1, 0], 1, vec![0, 1 << 31]),
        rot_two_quad_tricky_bis: (vec![31, 0], 2, vec![7, (1 << 31) + (1 << 30)]),
    }

    circular_rotR_tests! {
        circ_single_byte: (vec![32], 6, vec![1 << 31]),
        circ_empty_and_byte: (vec![64, 0, 0], 65, vec![0, 0, 32]),
        circ_empty_and_whole_rot: (vec![64, 0, 0], 128 , vec![0, 64, 0]),
        circ_two_bytes: (vec![32, 32], 6, vec![1 << 31, 1 << 31]),
        circ_two_bytes_bis: (vec![1, 0], 32, vec![0, 1]),
        circ_two_bytes_tricky: (vec![1, 0], 33, vec![1 << 31, 0]),
        circ_two_bytes_tricky_bis: (vec![31, 0], 2, vec![7, (1 << 31) + (1 << 30)]),
        circ_three_bytes_tricky: (vec![7, 7, 1],
                                  1,
                                  vec![(1 << 31) + 3, (1 << 31) + 3, 1 << 31]
                                 ),
    }
}
*/
