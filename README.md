# Viterbi POS Tagger 
An implementation of the Viterbi algorithm for part-of-speech (POS) tagging in Rust with no external dependencies.

## How to Use
This package is straightforward to use: 
* First, load your training data (as a tagged corpus) from a text file
* Next, convert that data to a list of words and a vector you can run into the tagging algorithm
* Finally, pass a vector of your text to tag into the main tagging function

```rust
pub fn main() {
    use std::env;
    use pos_tagger::read_corpus;
    use pos_tagger::get_words;
    use pos_tagger::all_data_vec;
    use pos_tagger::tagger;

    // set up argument parsing and assign filename to first arg
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // read file
    let file_data = read_corpus(filename);

    // get words and put into a vector 
    let word_list = get_words(&file_data);
    let all_data = all_data_vec(word_list);

    // providing our sentences to to pass to the main tagging function
    let test_sentences_input = vec!["lamb at test"];
    
    // tagging our input
    tagger(test_sentences_input, all_data);
}
```
Then, simply run your file containing the function above with the filename as an argument. The tagged text will be returned as a vector of tuples:

```
cargo run --bin tag_words test_file.txt
// example output: [("the", "AT"), ("grand", "JJ"), ("jury", "NN"), ("commented", "VBD")]
```

There is also a helper function to split your tagged corpus into a smaller training corpus, if needed. The following function chunks your vec of words into two vecs, with one the first 20% of the corpus and the remaining 80% in the other:

```rust

use pos_tagger::training_and_validation_data_split

let all_data = training_and_validation_data_split(word_list);
let training_data = all_data.0;
let remaining_data = all_data.1;
```

## Corpus Formatting
Training corpora should be formatted similarly to the Brown Corpus: with each word separated from its tag by an underscore, like so: "the_AT Fulton_NP County_NN Grand_JJ Jury_NN said_VBD Friday_NR an_AT investigation_NN of_IN Atlanta's_NP$ recent_JJ primary_NN election_NN produced_VBD no_AT evidence_NN that_CS any_DTI irregularities_NNS took_VBD place_NN ._."

The complete Brown Corpus is included with the package.


