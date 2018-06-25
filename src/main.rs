
fn matches(member: [u8; 4]) -> [(u8, u8); 6]
{
    let mut result: [(u8, u8); 6] = [(0, 0); 6];
    let mut i = 0;
    let mut j = 1;
    let mut count = 0;

    while i < 4 {
        j = i + 1;
        while j < 4 {
            result[count] = (member[i], member[j]);
            count = count + 1;
            j = j + 1;
        }
        i = i + 1;
    }

    result
}

fn main() {
    let a = [1,2,3,4];
    let combinition = matches(a);
    for (t1, t2) in &combinition {
        println!("{} vs {}", t1, t2);
    }
}
