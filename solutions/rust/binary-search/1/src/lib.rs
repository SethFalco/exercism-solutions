pub fn find<A, K>(array: A, key: K) -> Option<usize>
where
    A: AsRef<[K]>,
    K: Ord + PartialEq<K>
{
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut start = 0;
    let mut end = array.len();

    loop {
        let middle = (start + end) / 2;
        let val = &array[middle];

        if val == &key {
            return Some(middle);
        }

        if end - start == 1 {
            return None;
        }

        if val > &key {
            end = middle;
            continue;
        }

        start = middle;
    }
}
