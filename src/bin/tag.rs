use std::env;
use viterbi_pos_tagger::read_corpus;
use viterbi_pos_tagger::get_words;
use viterbi_pos_tagger::all_data_vec;
use viterbi_pos_tagger::tagger;


pub fn main() {
    // set up argument parsing and assign filename to first arg
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read file
    let file_data = read_corpus(filename);

    // get words
    let word_list = get_words(&file_data);

    let all_data = all_data_vec(word_list);

    // setting up data to pass to main function
    let test_sentences_input = vec!["lamb at test", "another sentence!"];

    // tagging
    tagger(test_sentences_input, all_data);
}

