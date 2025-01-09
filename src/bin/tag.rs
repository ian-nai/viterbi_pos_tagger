use std::env;
use viterbi_pos_tagger::read_corpus;
use viterbi_pos_tagger::get_words;
use viterbi_pos_tagger::all_data_vec;
use viterbi_pos_tagger::tagger;
use viterbi_pos_tagger::lines_from_file;


pub fn main() {
    // set up argument parsing and assign our training data and input text to first and second args
    let args: Vec<String> = env::args().collect();
    let training_filename = &args[1];
    let input_text_filename = &args[2];

    // read file
    let training_file_data = read_corpus(training_filename);

    // get words
    let word_list = get_words(&training_file_data);

    let all_data = all_data_vec(word_list);

    // getting text data from our text file
    let file_lines = lines_from_file(input_text_filename);

    // you can also pass your own String like so:
    //let input_text = vec!["Lamb is a test."];
    
    // tagging
    let file_lines_processed: Vec<&str> = file_lines.iter().map(|x| &**x).collect();
    tagger(file_lines_processed, all_data);
}
