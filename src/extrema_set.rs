pub trait ReferenceToAnElement<T> {
    fn ref_any(&self) -> &T;
}

impl<T> ReferenceToAnElement<T> for Vec<T> {
    fn ref_any(&self) -> &T {
        &self[0]
    }
}

/// Implementation guts for `min_set`, `min_set_by`, and `min_set_by_key`.
pub fn min_set_impl<R, I, K, F, L>(mut it: I,
                                mut key_for: F,
                                mut lt: L) -> Option<R>
    where I: Iterator,
          F: FnMut(&I::Item) -> K,
          L: FnMut(&I::Item, &I::Item, &K, &K) -> bool,
          R: std::iter::FromIterator<I::Item> + std::iter::Extend<I::Item> + ReferenceToAnElement<I::Item>,
{
    let (mut result, mut current_key) = match it.next() {
        None => return None,
        Some(element) => {
            let key = key_for(&element);
            (Some(element).into_iter().collect::<R>(), key)
        }
    };

    for element in it {
        let key = key_for(&element);
        if lt(&element, result.ref_any(), &key, &current_key) {
            result = Some(element).into_iter().collect::<R>();
            current_key = key;
        } else if !lt(result.ref_any(), &element, &current_key, &key) {
            result.extend(Some(element).into_iter());
        }
    }

    Some(result)
}

/// Implementation guts for `ax_set`, `max_set_by`, and `max_set_by_key`.
pub fn max_set_impl<R, I, K, F, L>(it: I,
                                key_for: F,
                                mut lt: L) -> Option<R>
    where I: Iterator,
          F: FnMut(&I::Item) -> K,
          L: FnMut(&I::Item, &I::Item, &K, &K) -> bool,
          R: std::iter::FromIterator<I::Item> + std::iter::Extend<I::Item> + ReferenceToAnElement<I::Item>,
{
    min_set_impl(it, key_for, |it1, it2, key1, key2| lt(it2, it1, key2, key1))
}


