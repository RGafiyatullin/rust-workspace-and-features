pub fn do_it() -> Vec<&'static str> {
    let mut out = a::do_it();
    out.push("b::do_it");

    #[cfg(test)]
    out.push("b: cfg(test)");

    #[cfg(feature = "light")]
    out.push("b: cfg(feature = light)");

    #[cfg(feature = "full")]
    out.push("b: cfg(feature = full)");

    #[cfg(feature = "enable-mock-b")]
    out.push("b: cfg(feature = enable-mock-b)");
    
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        eprintln!("{:?}", crate::do_it());
    }
}
