pub fn do_it() -> Vec<&'static str> {
    let mut out = vec![];
    out.push("a::do_it");

    #[cfg(test)]
    out.push("a: cfg(test)");

    #[cfg(feature = "light")]
    out.push("a: cfg(feature = light)");

    #[cfg(feature = "full")]
    out.push("a: cfg(feature = full)");

    #[cfg(feature = "enable-mock-a")]
    out.push("a: cfg(feature = enable-mock-a)");

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        eprintln!("{:?}", crate::do_it());
    }
}
