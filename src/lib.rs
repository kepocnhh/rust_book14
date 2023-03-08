//! `Rust` book lessons.
//! Chapter [14](https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html).

/// Returns the contained first value or computes it from a closure.
/// See: [`Option::unwrap_or_else`]
///
/// # Examples
///
/// ```
/// let list = vec![1, 2, 3];
/// let answer = rust_book14::get_first_or_else(&list, || &-1);
/// assert_eq!(1, *answer);
///
/// let list = vec![];
/// let answer = rust_book14::get_first_or_else(&list, || &-1);
/// assert_eq!(-1, *answer);
/// ```
pub fn get_first_or_else<'a, T, F: FnOnce() -> &'a T>(list: &'a Vec<T>, f: F) -> &'a T {
    if list.is_empty() {
        return f();
    }
    return &list[0];
    // return list.get(0).unwrap_or_else(f);
}

/// Returns the contained first value or a provided default.
/// See: [`Option::unwrap_or`]
///
/// # Examples
///
/// ```
/// let list = vec![1, 2, 3];
/// let answer = rust_book14::get_first_or(&list, &-1);
/// assert_eq!(1, *answer);
///
/// let list = vec![];
/// let answer = rust_book14::get_first_or(&list, &-1);
/// assert_eq!(-1, *answer);
/// ```
pub fn get_first_or<'a, T>(list: &'a Vec<T>, default: &'a T) -> &'a T {
    if list.is_empty() {
        return default;
    }
    return &list[0];
    // return list.get(0).unwrap_or(default);
}

/// [`Some(value)`]: Some
/// [`Err(err)`]: Err
///
/// Returns the contained first [`Some(value)`] or a provided [`Err(err)`].
/// See: [`Option::ok_or`]
///
/// # Errors
///
/// If the `list` is empty, an [`Err(err)`] will be returned.
///
/// # Examples
///
/// ```
/// let list = vec![1, 2, 3];
/// let answer = rust_book14::get_first_or_error(&list, "foo");
/// assert_eq!(Ok(&1), answer);
///
/// let list: Vec<u8> = vec![];
/// let answer = rust_book14::get_first_or_error(&list, "foo");
/// assert_eq!(Err("foo"), answer);
/// ```
pub fn get_first_or_error<T, U>(list: &Vec<T>, err: U) -> Result<&T, U> {
    if list.is_empty() {
        return Err(err);
    }
    return Ok(&list[0]);
    // return list.get(0).ok_or(err);
}

/// Returns the contained first value or panics with a `msg`.
/// See: [`Option::expect`]
///
/// # Panics
///
/// If the `list` is empty, panics with a `msg`.
///
/// # Examples
///
/// ```
/// let list = vec![1, 2, 3];
/// let answer = rust_book14::get_first_or_panic(&list, "foo");
/// assert_eq!(&1, answer);
///
/// let list: Vec<u8> = vec![];
/// let result = std::panic::catch_unwind(|| {
///     rust_book14::get_first_or_panic(&list, "foo")
/// });
/// assert!(result.is_err());
/// ```
pub fn get_first_or_panic<'a, T>(list: &'a Vec<T>, msg: &str) -> &'a T {
    if list.is_empty() {
        panic!("{msg}");
    }
    return &list[0];
    // return list.get(0).expect(msg);
}
