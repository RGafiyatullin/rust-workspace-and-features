pub fn do_it() -> Vec<&'static str> {
    let mut out = b::do_it();
    out.push("c::do_it");

    #[cfg(test)]
    out.push("c: cfg(test)");

    #[cfg(feature = "light")]
    out.push("c: cfg(feature = light)");

    #[cfg(feature = "full")]
    out.push("c: cfg(feature = full)");

    #[cfg(feature = "enable-mock-c")]
    out.push("c: cfg(feature = enable-mock-c)");

    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        eprintln!("{:?}", crate::do_it());
    }
}
