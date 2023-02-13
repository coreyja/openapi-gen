/// A very bad implementation of `make_ascii_titlecase` that only capitalizes the first letter
/// of the string and nothing else
pub(crate) fn make_ascii_titlecase(s: &mut str) {
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
}
