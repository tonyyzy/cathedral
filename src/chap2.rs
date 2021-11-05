use std::{collections::HashMap, hash::Hash};

fn memoize<T, A, B>(func: T) -> impl FnMut(A) -> B
where
    T: Fn(A) -> B,
    A: Eq + Hash + Clone,
    B: Clone,
{
    let mut cache: HashMap<A, B> = HashMap::new();

    return move |a| {
        if let Some(result) = cache.get(&a) {
            (*result).clone()
        } else {
            let result = func(a.clone());
            cache.insert(a, result.clone());
            result
        }
    };
}
