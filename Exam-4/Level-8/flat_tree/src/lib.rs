use std::collections::BTreeSet;

pub fn flatten_tree<T: Clone>(tree: &BTreeSet<T>) -> Vec<T> {
    tree.iter().cloned().collect()
}



#[cfg(test)]
mod tests {
    use flat_tree::*;
    use std::collections::BTreeSet;

    #[test]
    fn it_works() {
        assert_eq!(flatten_tree(&BTreeSet::from([3, 0, 9, 10])), [0, 3, 9, 10]);
    }

    #[test]
    fn test_with_str() {
        assert_eq!(
            flatten_tree(&BTreeSet::from(["Slow", "kill", "will", "Horses"])),
            ["Horses", "Slow", "kill", "will"]
        );
    }
}