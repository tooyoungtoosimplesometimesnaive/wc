use std::collections::HashMap;

fn matches(member: [u8; 4]) -> [(u8, u8); 6]
{
    let mut result: [(u8, u8); 6] = [(0, 0); 6];
    let mut i = 0;
    let mut count = 0;

    while i < 4 {
        let mut j = i + 1;
        while j < 4 {
            result[count] = (member[i], member[j]);
            count = count + 1;
            j = j + 1;
        }
        i = i + 1;
    }

    result
}

fn find_match(total_combinition: [(u8, u8); 6],
              m: (u8, u8)) -> bool {

    for (t1, t2) in &total_combinition {
        let (m1, m2) = m;
        if (*t1 == m1 && *t2 == m2) || (*t1 == m2 && *t2 == m1) {
            return true;
        }
    }
    false
}

fn main() {
    let a = [1,2,3,4];
    let combinition = matches(a);
    for (t1, t2) in &combinition {
        println!("{} vs {}", t1, t2);
    }
    let mut map = HashMap::new();
    map.insert("Mexico", 1);
    map.insert("Germany", 2);
    map.insert("Sweden", 3);
    map.insert("South Korea", 4);

    let current_result = vec![(("Mexico", 1), ("Germany", 0)),
        (("Mexico", 2), ("South Korea", 1)),
        (("Germany", 2), ("Sweden", 1)),
        (("Sweden", 1), ("South Korea", 0))];

    for ((team_1, score_1), (team_2, score_2)) in current_result {
        if let Some(n1) = map.get(team_1) {
            if let Some(n2) = map.get(team_2) {
        let n_1 = *n1 as u8;
        let n_2 = *n2 as u8;
        println!("match: {}", find_match(combinition, (n_1, n_2)));
        }
        }
    }
}
