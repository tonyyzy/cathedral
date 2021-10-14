fn id<A>(a: A) -> A {
    a
}

fn compose<A, B, C>(a: impl Fn(A) -> B, b: impl Fn(B) -> C) -> impl Fn(A) -> C {
    move |x: A| b(a(x))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compose_respect_id() {
        let func1 = |x: i32| x + 1;
        let comp1 = compose(func1, id);
        let comp2 = compose(id, func1);
        assert_eq!(42, id(42));
        assert_eq!(comp1(41), comp2(41));
        assert_eq!(comp1(41), 42);
    }
}