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

fn find_match(combinition: Vec<(u8, u8)>,
              m: (u8, u8)) -> Option<u8> {

    for (t1, t2) in &combinition {
        let (m1, m2) = m;
        if (*t1 == m1 && *t2 == m2) || (*t1 == m2 && *t2 == m1) {
            return true;
        }
    }
    false
}

fn get_remaining_matches(total_combinition: [(u8, u8); 6],
                         current_combinitions: Vec<(u8, u8)>) -> Vec<(u8, u8)> {
    let mut diff: Vec<(u8, u8)> = Vec::new();
    for pair in &total_combinition {
        if !find_match(current_combinitions, pair) {
            diff.push(pair);
        }
    }
    diff
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

    let current_result: Vec<((&str, u8), (&str, u8))> = vec![
        (("Mexico", 1), ("Germany", 0)),
        (("Mexico", 2), ("South Korea", 1)),
        (("Germany", 2), ("Sweden", 1)),
        (("Sweden", 1), ("South Korea", 0))];


    // GF, GA, Pts
    let mut scores: [(u8, u8, u8); 4] = [
        (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0)];

    for ((team_1, score_1), (team_2, score_2)) in current_result {

        if let Some(n1) = map.get(team_1) {
        if let Some(n2) = map.get(team_2) {
            let n_1 = *n1 as usize - 1; // team 1 index in the scores array
            let n_2 = *n2 as usize - 1; // team 2 index in the scores array

            let (mut t1_gf, mut t1_ga, mut t1_pts) = scores[n_1];
            let (mut t2_gf, mut t2_ga, mut t2_pts) = scores[n_2];
            t1_gf = t1_gf + score_1;
            t1_ga = t1_ga + score_2;
            t2_gf = t2_gf + score_2;
            t2_ga = t2_ga + score_1;

            if score_1 > score_2 {
                t1_pts += 3;
            } else if score_1 < score_2 {
                t2_pts += 3;
            } else {
                t1_pts += 1;
                t2_pts += 1;
            }

            // Update the scores
            scores[n_1] = (t1_gf, t1_ga, t1_pts);
            scores[n_2] = (t2_gf, t2_ga, t2_pts);

            let n_1 = *n1 as usize - 1; // team 1 index
            let n_2 = *n2 as usize - 1; // team 2 index
            // println!("match: {}", find_match(combinition, (n_1, n_2)));
        }}
    }

    for s in &scores {
        let (a, b, c) = s;
        println!("{}, {}, {}", a, b, c);
    }
}
