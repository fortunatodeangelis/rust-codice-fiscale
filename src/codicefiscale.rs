pub mod codicefiscale {

    extern crate serde_json;
    use std::fs::File;
    use std::convert::TryInto;
    use std::collections::HashMap;

    pub fn calculate (name: &str, lastname: &str, birtdate: &str, gender: &str, birthprovince: &str, birthplace: &str) -> String  {
 

        let mut result = String::new();

        let month: Vec<char>= vec!['A', 'B', 'C', 'D', 'E', 'H', 'L', 'M', 'P', 'R', 'S', 'T'];
        let vocals: Vec<char>= vec!['A', 'E', 'I', 'O', 'U'];
        let consonants: Vec<char>= vec!['B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z'];
        let alphabet: Vec<char>= vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        let alphabet_number: Vec<char>= vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
        
        let file = File::open("src/comuni.json")
        .expect("file should open read only");
        let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    
        let code = format!("{}_{}", birthprovince, birthplace);
    
        let mut matrix: HashMap<String, i32> = HashMap::new();
        matrix.insert(String::from("01"), 1);
        matrix.insert(String::from("00"), 0);
        matrix.insert(String::from("11"), 0);
        matrix.insert(String::from("10"), 1);
        matrix.insert(String::from("21"), 5);
        matrix.insert(String::from("20"), 2);
        matrix.insert(String::from("31"), 7);
        matrix.insert(String::from("30"), 3);
        matrix.insert(String::from("41"), 9);
        matrix.insert(String::from("40"), 4);
        matrix.insert(String::from("51"), 13);
        matrix.insert(String::from("50"), 5);
        matrix.insert(String::from("61"), 15);
        matrix.insert(String::from("60"), 6);
        matrix.insert(String::from("71"), 17);
        matrix.insert(String::from("70"), 7);
        matrix.insert(String::from("81"), 19);
        matrix.insert(String::from("80"), 8);
        matrix.insert(String::from("91"), 21);
        matrix.insert(String::from("90"), 9);
        matrix.insert(String::from("101"), 1);
        matrix.insert(String::from("100"), 0);
        matrix.insert(String::from("111"), 0);
        matrix.insert(String::from("110"), 1);
        matrix.insert(String::from("121"), 5);
        matrix.insert(String::from("120"), 2);
        matrix.insert(String::from("131"), 7);
        matrix.insert(String::from("130"), 3);
        matrix.insert(String::from("141"), 9);
        matrix.insert(String::from("140"), 4);
        matrix.insert(String::from("151"), 13); 
        matrix.insert(String::from("150"), 5);
        matrix.insert(String::from("161"), 15); 
        matrix.insert(String::from("160"), 6);
        matrix.insert(String::from("171"), 17); 
        matrix.insert(String::from("170"), 7);
        matrix.insert(String::from("181"), 19); 
        matrix.insert(String::from("180"), 8);
        matrix.insert(String::from("191"), 21); 
        matrix.insert(String::from("190"), 9);
        matrix.insert(String::from("201"), 2);
        matrix.insert(String::from("200"), 10); 
        matrix.insert(String::from("211"), 4);
        matrix.insert(String::from("210"), 11); 
        matrix.insert(String::from("221"), 18); 
        matrix.insert(String::from("220"), 12); 
        matrix.insert(String::from("231"), 20); 
        matrix.insert(String::from("230"), 13); 
        matrix.insert(String::from("241"), 11); 
        matrix.insert(String::from("240"), 14); 
        matrix.insert(String::from("251"), 3);
        matrix.insert(String::from("250"), 15); 
        matrix.insert(String::from("261"), 6);
        matrix.insert(String::from("260"), 16); 
        matrix.insert(String::from("271"), 8);
        matrix.insert(String::from("270"), 17); 
        matrix.insert(String::from("281"), 12); 
        matrix.insert(String::from("280"), 18); 
        matrix.insert(String::from("291"), 14); 
        matrix.insert(String::from("290"), 19); 
        matrix.insert(String::from("301"), 16); 
        matrix.insert(String::from("300"), 20); 
        matrix.insert(String::from("311"), 10); 
        matrix.insert(String::from("310"), 21); 
        matrix.insert(String::from("321"), 22); 
        matrix.insert(String::from("320"), 22); 
        matrix.insert(String::from("331"), 25); 
        matrix.insert(String::from("330"), 23); 
        matrix.insert(String::from("341"), 24); 
        matrix.insert(String::from("340"), 24); 
        matrix.insert(String::from("351"), 23); 
        matrix.insert(String::from("350"), 25);
    
    
        let born_year: i32 = birtdate[2..4].parse::<i32>().unwrap();
        let mut born_month: usize = birtdate[5..7].parse::<usize>().unwrap();
        born_month = born_month -1;
        let mut born_day: i32 = birtdate[8..10].parse::<i32>().unwrap();
    
        
        if gender == "F" {
            born_day = born_day + 40;
        }
    
        result.push_str(&format_lastname(lastname, &consonants, &vocals)); 
        result.push_str(&format_name(name, &consonants, &vocals)); 
        result.push_str(&born_year.to_string()); 
        result.push_str(&month[born_month].to_string()); 
        result.push_str(&born_day.to_string()); 
        result.push_str(&&json[&code].to_string()[1..5]); 
        
        
    
        let mut control_code = 0;
    
        let partial_code: Vec<char> = result.to_uppercase().chars().collect();
        for n in 0..15 {
            let index: usize = alphabet_number.iter().position(|&r| r == partial_code[n]).unwrap();
            let string_val: usize = (n + 1) % 2;
            let matrix_cod = format!("{}{}", index, string_val);
            for (key, val) in matrix.iter() {
                if key == &matrix_cod {
                    control_code = control_code + val;
                }
            }
        }
    
        let index_alphabet: usize = (&control_code % 26).try_into().unwrap();
        result.push_str(&alphabet[index_alphabet].to_string()); 
        result.to_string()
    }


    fn format_name(name: &str, consonants: &Vec<char>, vocals: &Vec<char>) -> String  {

        let mut count = 0;
        let mut string = String::new();
    
        // Prendo il nome è divido lettera per lettera fino a cotruire un vec
        let name_vect: Vec<char> = name.to_uppercase().chars().collect();
    
        while string.len() < 4 && (count + 1 <= name_vect.len()) {
            if consonants.iter().any(|&i| i== name_vect[count]) {
                string.push_str(&name_vect[count].to_string()); 
            }
            count = count + 1;
        }
    
    
        if string.len() > 3 {
            // Identifico il carattere alla posizione 1
            let char: char = match string.chars().nth(1) {
                Some(a) => a,
                _ => unreachable!(),
            };
            // Rimpiazzo il carattere con uno vuoto
            string = string.replace(char, "");
        }
    
        count = 0;
        while string.len() < 3 && (count + 1 <= name_vect.len()) {
            if vocals.iter().any(|&i| i== name_vect[count]) {
                string.push_str(&name_vect[count].to_string()); 
            }
            count = count + 1;
        }
        string.push_str("XXX"); 
        string[..3].to_string()
    }

    fn format_lastname(lastname: &str, consonants: &Vec<char>, vocals: &Vec<char>) -> String  {

        let mut count = 0;
        let mut string = String::new();
    
        // Prendo il cognome è divido lettera per lettera fino a cotruire un vec
        let lastname_vect: Vec<char> = lastname.to_uppercase().chars().collect();
    
        while string.len() < 4 && (count + 1 <= lastname_vect.len()) {
            if consonants.iter().any(|&i| i== lastname_vect[count]) {
                string.push_str(&lastname_vect[count].to_string()); 
            }
            count = count + 1;
        }
    
        count = 0;
        while string.len() < 3 && (count + 1 <= lastname_vect.len()) {
            if vocals.iter().any(|&i| i== lastname_vect[count]) {
                string.push_str(&lastname_vect[count].to_string()); 
            }
            count = count + 1;
        }
        string.push_str("XXX"); 
        string[..3].to_string()
    }
}