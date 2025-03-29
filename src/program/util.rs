pub fn coalesce_2<T, F>(elements: &mut Vec<T>, merge: F)
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

pub fn coalesce_3<T, F>(elements: &mut Vec<T>, merge: F)
where
    T: Clone,
    F: Fn(&T, &T, &T) -> Option<T>,
{
    if elements.is_empty() {
        return;
    }

    let mut write = 0;
    let mut read = 0;

    while read + 2 < elements.len() {
        let current = &elements[read];
        let next = &elements[read + 1];
        let next_next = &elements[read + 2];

        if let Some(coalesced) = merge(current, next, next_next) {
            elements[write] = coalesced;
            read += 3;
        } else {
            elements[write] = current.clone();
            read += 1;
        }
        write += 1;
    }

    // Copy any remaining instructions
    while read < elements.len() {
        elements[write] = elements[read].clone();
        write += 1;
        read += 1;
    }

    elements.truncate(write);
}
