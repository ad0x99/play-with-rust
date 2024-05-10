pub fn sized_types_and_sized_trait() {
    // Generic function definition
    fn _generic1<T>(_t: T) {}

    // is actually treated as though we had written this:
    fn _generic2<T: Sized>(_t: T) {}

    // By default, generic functions will work only on types that have a known size at compile time. However, you can use the following special syntax to relax this restriction:
    fn _generic3<T: ?Sized>(_t: &T) {}
}
