use std::io;

use flate2::read::GzDecoder;

const LEVEL1_GZIP: &[u8] = include_bytes!("../lessons/level1.txt.gz");
const LEVEL2_GZIP: &[u8] = include_bytes!("../lessons/level2.txt.gz");
const LEVEL3_GZIP: &[u8] = include_bytes!("../lessons/level3.txt.gz");
const LEVEL4_GZIP: &[u8] = include_bytes!("../lessons/level4.txt.gz");
const LEVEL5_GZIP: &[u8] = include_bytes!("../lessons/level5.txt.gz");

#[derive(Debug, Clone)]
pub(crate) struct WordList {
    pub(crate) name: String,
    pub(crate) words: Vec<String>,
}

pub(crate) fn load_word_lists() -> Vec<WordList> {
    vec![
        WordList {
            name: "Home Row - 8 keys".to_string(),
            words: load_words_from_embedded(LEVEL1_GZIP),
        },
        WordList {
            name: "Home Row - 10 keys".to_string(),
            words: load_words_from_embedded(LEVEL2_GZIP),
        },
        WordList {
            name: "Home Row + 8 keys".to_string(),
            words: load_words_from_embedded(LEVEL3_GZIP),
        },
        WordList {
            name: "Home Row + 8 more keys".to_string(),
            words: load_words_from_embedded(LEVEL4_GZIP),
        },
        WordList {
            name: "Full Alphabet".to_string(),
            words: load_words_from_embedded(LEVEL5_GZIP),
        },
    ]
}

fn load_words_from_embedded(source: &[u8]) -> Vec<String> {
    decompress_gzip(source)
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn decompress_gzip(compressed_data: &[u8]) -> String {
    let mut decoder = GzDecoder::new(compressed_data);
    let mut decoded_bytes = Vec::new();
    io::copy(&mut decoder, &mut decoded_bytes).expect("Failed to decompress");
    String::from_utf8(decoded_bytes).expect("Decoded bytes are not valid UTF-8")
}
