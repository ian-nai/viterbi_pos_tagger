use std::fs;

pub fn read_corpus(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    return contents;
}

pub fn get_words(string_to_analyze: &str) -> Vec<String> {
    let mut word_list: Vec<String> = Vec::new();
    for split_word in string_to_analyze.split([' ']) {
        let split_word = split_word.replace(&[',', '.', ';', ':'][..], "");
        word_list.push(split_word.to_string());
    }
    return word_list;
}

pub fn training_and_validation_data_split(word_list_2: Vec<String>) ->  (Vec<(String, String)>, Vec<(String, String)>) {    
     let mut tagged_words_map: Vec<(String, String)> = Vec::new();
     for n in word_list_2 {
         let parts = n.split("_");
         let mut temp_vec: Vec<String> = Vec::new();
         for part in parts {
             temp_vec.push(part.to_string());
         }
         if temp_vec.len() == 2 {
            let temp_tuple = (temp_vec[0].clone(), temp_vec[1].clone());
            tagged_words_map.push(temp_tuple);
         }
     }
    
    let training_data_num = ((tagged_words_map.len() as f64) * 0.8) as i64;
    let practice_data_num = ((tagged_words_map.len() as i64) - training_data_num) as i64;
    let mut chunk_array: Vec<&[(String, String)]> = Vec::new();

    let v_slices: Vec<&[(String, String)]> = tagged_words_map.chunks(training_data_num as usize).collect();
        for v in v_slices {
            chunk_array.push(v);
        }
    let training_data = chunk_array[0].to_vec();
    let prac_data = chunk_array[1].to_vec();

    return (training_data, prac_data);
}

pub fn all_data_vec(word_list: Vec<String>) ->  Vec<(String, String)> {    
    let mut tagged_words_map: Vec<(String, String)> = Vec::new();
    for n in word_list {
        let parts = n.split("_");
        let mut temp_vec: Vec<String> = Vec::new();
        for part in parts {
            temp_vec.push(part.to_string());
        }
        if temp_vec.len() == 2 {
           let temp_tuple = (temp_vec[0].clone(), temp_vec[1].clone());
           tagged_words_map.push(temp_tuple);
        }
    }

   return tagged_words_map;
}

// compute emission probability 
pub fn word_given_tag(word: &str, tag: &str, train_bag: Vec<(String, String)>) -> (i32, i32) {
    let mut tag_list = vec![];
    let mut w_given_tag_list = vec![];
    for el in train_bag {
        if el.1 == tag {
            tag_list.push(el);
        }
    }
    let count_tag = tag_list.len() as i32; // total number of times the passed tag occurred in train_bag
    
    for el in tag_list {
        if el.0 == word {
            w_given_tag_list.push(el);
        }
    }
    // calculating the total number of times the passed word occurred as the passed tag
    let count_w_given_tag = w_given_tag_list.len() as i32;
    return (count_w_given_tag, count_tag);
}

// compute transition probability
pub fn tag2_given_tag1(tag1: &str, tag2: &str, train_bag: Vec<(String, String)>) -> (i32, i32) {
    let mut tags = vec![];
    let mut count_tag1 = 0;
    for pair in train_bag {
        tags.push(pair.1); 
    }

    for t in &tags {
        if t == tag1 {
            count_tag1 += 1;
        }
    }

    let mut count_tag2_tag1 = 0;
    for index in 0..(tags.len() - 1) {
        if tags[index] == tag1 && tags[index+1] == tag2 {
            count_tag2_tag1 += 1
        }
    }
    return (count_tag2_tag1, count_tag1);

}

// the main tagging function
pub fn tagger(input: Vec<&str>, training_data: Vec<(String, String)>) -> Vec<(String, String)> {    
    let mut tags_set = vec![];
    for pair in &training_data {
        tags_set.push(pair.1.clone());
    }
    tags_set.sort();
    tags_set.dedup();
    let array_length = tags_set.len();
    
     // setting up our 2d array
     let mut grid_raw = vec![0.0; array_length * array_length];
     let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(array_length).collect();
     // our final 2d array stored in the grid variable
     let grid = grid_base.as_mut_slice();

    for (i, t1) in tags_set.iter().enumerate() {
        for (j, t2) in tags_set.iter().enumerate() {
            grid[i][j] = tag2_given_tag1(t2, t1, training_data.clone()).0 as f64/tag2_given_tag1(t2, t1, training_data.clone()).1 as f64;
        }
    };
    
    // splitting our input sentences into words
    let mut tokenized_sents = vec![];
    for sent in input {
        let sent_parts = sent.split(" ");
        let collection: Vec<&str> = sent_parts.collect();
        tokenized_sents.push(collection);
    };
    
    // initializing an initial state
    let mut state: Vec<String> = vec!["NN".to_string()];
    
    // iterating through the words in our input sentences and finding the probability for each tag given our training data
    let mut tagged_vec: Vec<(String, String)> = Vec::new();
    for elem in tokenized_sents {
        for (key, word) in elem.iter().enumerate() {
            let mut p: Vec<f64> = vec![];
            for tag in &tags_set {
                let tag_index = tags_set.iter().position(|r| *r == *tag).unwrap() as usize;
                let last_state = state.last().unwrap();
                let state_index = tags_set.iter().position(|r| *r == *last_state).unwrap() as usize;
                let transition_p = grid[state_index][tag_index];
                let emission_p = word_given_tag(elem[key], tag, training_data.clone()).0 as f64/word_given_tag(elem[key], tag, training_data.clone()).1 as f64;
                let state_probability = emission_p * transition_p;
                p.push(state_probability);

            let pmax_value = p.iter().fold(std::f64::MIN, |a,b| a.max(*b));
            let pmax_index = p.iter().position(|&r| r == pmax_value).unwrap();
            let state_max = &tags_set[pmax_index];
            state.push(state_max.to_string());
            }; 
            let last_state2 = state.last().unwrap().clone();
            tagged_vec.push((word.to_string(), last_state2));
        };
    };
    
    println!("{:?}", tagged_vec);
    return tagged_vec;
}