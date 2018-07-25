// FIXME: Make me pass! Diff budget: 10 lines.
// Do not `use` any items.

// Do not change the following two lines.
#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
struct IntWrapper(isize);

impl Ord for IntWrapper {
    fn cmp(&self, other: &IntWrapper) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl Eq for IntWrapper {}

pub fn main() {
    assert_eq!(std::cmp::max(1usize, 3), 3);
    assert_eq!(std::cmp::max(1u8, 3), 3);
    assert_eq!(std::cmp::max(1u8, 3), 3);
    assert_eq!(
        std::cmp::max(IntWrapper(120), IntWrapper(248)),
        IntWrapper(248)
    );
}
