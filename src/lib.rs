use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(n: u32) -> Vec<u32> {
    let mut fib_arr = Vec::new();
    if n == 0 {
        return fib_arr;
    } else if n == 1 {
        fib_arr.push(0);
        return fib_arr;
    } else if n == 2 {  
        fib_arr.push(0);
        fib_arr.push(1);
        return fib_arr;
    }
    fib_arr.push(0);
    fib_arr.push(1);
    for i in 2..n {
        let next_fib = fib_arr[(i - 1) as usize] + fib_arr[(i - 2) as usize];
        fib_arr.push(next_fib);
    }
    return fib_arr;
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(n: u32) -> bool {
    return n.to_string().chars().eq(n.to_string().chars().rev());
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(n: usize, a: &[i32]) -> Option<i32> {
    if n >= a.len() {
        return None;
    }
    let mut sorted_vec = a.to_vec();
    sorted_vec.sort_by(|x, y| y.cmp(x));
    return Some(sorted_vec[n]);
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(s: &str) -> String {
    for c in s.chars() {
        let count = s.matches(c).count();
        if count == s.chars().map(|x| s.matches(x).count()).max().unwrap() {
            return c.to_string();
        }
    }
    return "".to_string();
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    _arr1: &[String],
    _arr2: &[String],
) -> Option<HashMap<String, String>> {
    if _arr1.len() != _arr2.len() {
        return None;
    }
    let mut map = HashMap::new();
    for i in 0.._arr1.len() {
        map.insert(_arr1[i].clone(), _arr2[i].clone());
    }
    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut vec: Vec<(String, String)> = map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    vec
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(
        &mut self,
        name: String,
        number: String,
        is_listed: bool,
    ) -> bool {
        // 1. Name must be unique
        if self.entries.iter().any(|entry| entry.name == name) {
            return false;
        }

        // 2. Validate number format manually: NNN-NNN-NNNN
        // Total length must be 12: 3 digits, dash, 3 digits, dash, 4 digits
        if number.len() != 12 {
            return false;
        }

        let chars: Vec<char> = number.chars().collect();

        // Check dashes
        if chars[3] != '-' || chars[7] != '-' {
            return false;
        }

        // Check digits
        if !chars[..3].iter().all(|c| c.is_ascii_digit()) ||
           !chars[4..7].iter().all(|c| c.is_ascii_digit()) ||
           !chars[8..].iter().all(|c| c.is_ascii_digit())
        {
            return false;
        }

        // 3. Listed number must be unique
        if is_listed && self.entries.iter().any(|entry| entry.number == number && entry.is_listed) {
            return false;
        }

        self.entries.push(PhoneEntry { name, number, is_listed });
        true
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    /// chungus
    /// Otherwise returns None.
    pub fn lookup(&self, name: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.name == name && entry.is_listed {
                return Some(entry.number.clone());
            }
        }
        None
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        for entry in &self.entries {
            if entry.number == num && entry.is_listed {
                return Some(entry.name.clone());
            }
        }
        None
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        let mut names = Vec::new();
        for entry in &self.entries {
            if entry.number.starts_with(areacode) {
                names.push(entry.name.clone());
            }
        }
        names
    }
}