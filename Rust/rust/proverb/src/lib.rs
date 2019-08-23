pub fn build_proverb(list: &[&str]) -> String {
    let mut arr = Vec::new();
    let arr_ref = &mut arr;
    if !list.is_empty() {
        for x in 0..list.len() - 1 {
            arr_ref.push(format!(
                "For want of a {} the {} was lost.",
                list[x],
                list[x + 1]
            ));
        }
        arr_ref.push(format!("And all for the want of a {}.", list[0]));
        arr.join("\n")
    } else {
        String::new()
    }
}
