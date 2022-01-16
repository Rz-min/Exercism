
#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (is_sublist(a, b), is_sublist(b, a)) {
        (true, true) => Equal,
        (true, false) => Sublist,
        (false, true) => Superlist,
        _ =>  Unequal,
    }

}

pub fn is_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    a.len() <= b.len() && (0..b.len() - a.len() + 1)
        .any(|i| &a[..] == &b[i..a.len() + i])
}