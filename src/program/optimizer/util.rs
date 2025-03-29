pub fn coalesce<T, F>(elements: &mut Vec<T>, merge: F)
where
    T: Clone,
    F: Fn(&T, &T) -> Option<T>,
{
    if elements.is_empty() {
        return;
    }

    let mut write = 0;
    for read in 1..elements.len() {
        let current = &elements[write];
        let next = &elements[read];

        if let Some(coalesced) = merge(current, next) {
            elements[write] = coalesced;
        } else {
            write += 1;
            elements[write] = elements[read].clone();
        }
    }

    elements.truncate(write + 1);
}
