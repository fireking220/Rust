pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ret: Vec<(usize, usize)> = Vec::new();
    if input.is_empty() || input[0].is_empty() {
        return ret;
    } else {
        let mut d: Vec<Vec<u64>> = Vec::new();
        //find max in rows and push to d
        for i in 0..input.len() {
            let m: &u64 = input[i].iter().max().unwrap();
            let a: Vec<u64> = input[i].iter().filter(|&x| x == m).cloned().collect();
            d.push(a);
        }

        let mut n: Vec<Vec<u64>> = Vec::new();
        //find min in collums and push to n
        for i in 0..input[0].len() {
            let mut b: Vec<u64> = Vec::new();
            for j in 0..input.len() {
                b.push(input[j][i]);
            }
            let min: &u64 = b.iter().min().unwrap();
            b = b.iter().filter(|&x| x == min).cloned().collect();
            n.push(b);
        }

        //find saddle points in input
        for i in 0..input.len() {
            for j in 0..input[0].len() {
                if d[i].contains(&input[i][j]) && n[j].contains(&input[i][j]) {
                    ret.push((i as usize, j as usize));
                }
            }
        }
    }
    ret
}
