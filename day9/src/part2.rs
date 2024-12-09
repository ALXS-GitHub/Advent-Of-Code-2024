use std::cmp::min;
use std::collections::BTreeMap;


fn parse_input(input_line: String) -> (i64, BTreeMap<i64, (i64, i64)>, BTreeMap<i64, i64>) {
    let mut total_space = 0;
    let mut indent = 0;
    let mut spaces = BTreeMap::new();
    let mut free_spaces = BTreeMap::new();
    let mut id_counter = 0;

    let chars: Vec<char> = input_line.chars().collect();

    for i in 0..chars.len() {
        if i % 2 == 1 {
            let value = chars[i].to_digit(10).unwrap() as i64;
            free_spaces.insert(indent, value);
            indent += value;
        } else {
            let value = chars[i].to_digit(10).unwrap() as i64;
            spaces.insert(id_counter, (value, indent));
            total_space += value;
            indent += value;
            id_counter += 1;
        }
    }

    return (total_space, spaces, free_spaces);

}

fn fill_free_spaces(line: String, spaces: &mut BTreeMap<i64, (i64, i64)>, free_spaces: &mut BTreeMap<i64, i64>, total_space: i64) -> BTreeMap<i64, (i64, i64)> {
    let mut chars: Vec<char> = line.chars().collect();
    // let mut final_spaces = Vec::new();
    let mut id_counter: i64 = 0;
    let mut spaces_copy = spaces.clone();

    for (key, (value, indent)) in spaces.iter_mut().rev() {

        let mut modif: (i64, i64, i64) = (-1, -1, -1);

        for (free_indent, space) in free_spaces.iter_mut() {
            if space >= value && free_indent < indent {
                *space -= *value;
                let new_indent = free_indent;
                modif = (*space, *new_indent + *value, free_indent.clone());
                // if *space == 0 {
                //     free_spaces.remove(free_indent);
                // } else {
                //     free_spaces.insert(*free_indent, *space);
                // }
                spaces_copy.insert(*key, (*value, *new_indent));
                break;
            }
        }

        if modif.0 == 0 {
            free_spaces.remove(&modif.2);
        } else if modif.0 != -1 {
            free_spaces.remove(&modif.2);
            free_spaces.insert(modif.1, modif.0);
        }

    }

    return spaces_copy.clone();
}

fn get_result(final_spaces: BTreeMap<i64, (i64, i64)>) -> i64 {
    let mut result = 0;
    
    for (key, (value, indent)) in final_spaces.iter() {
        let mut indent_copy = *indent;
        for _ in 0..*value {
            result += *key * indent_copy;
            indent_copy += 1;
        }
    }

    return result;
}


pub fn part2(input: &Vec<String>) -> i64 {
    let line = input[0].clone();
    let (total_space, mut spaces, mut free_spaces) = parse_input(line.clone());
    let final_spaces = fill_free_spaces(line.clone(), &mut spaces.clone(), &mut free_spaces.clone(), total_space);
    let result = get_result(final_spaces);

    return result;
}