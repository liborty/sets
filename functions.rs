/// helper function to stringify a generic vector for display, without recourse to debug
pub fn writevec<T>(v:&[T]) -> String where T: std::fmt::Display {
    let mut s = String::from("[ ");
    for x in v { s.push_str(&x.to_string()); s.push_str(" ") };
    s.push_str("]");
    s
}
