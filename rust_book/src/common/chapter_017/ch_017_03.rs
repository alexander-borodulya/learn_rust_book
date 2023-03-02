/// Initial implementation of the state pattern
///
/// Allow users to add text content only when a post is in the Draft state - based on explicit type check
pub mod v1;

/// Encoding States and Behavior as Types
pub mod v2;

/// Initial implementation of the state pattern
///
/// Allow users to add text content only when a post is in the Draft state
/// - based on hint:
/// - based on StackOverflow answer: https://stackoverflow.com/questions/57413949/object-orientated-rust-the-rust-book-chapter-17-blog
///
/// Hint:
///     have the state object responsible for what might change about the content but not responsible for modifying the Post.
pub mod v3;

/// TODO: Investigate in depth state pattern behaviour with:
///  - [ ] trait State
///  - [ ] self vs self mut conflict
///  - [ ] Compile error: behind reference
///  - [ ] Compile error: error[E0502]: cannot borrow `*self` as mutable because it is also borrowed as immutable
pub mod v4;
