fn main() {
    eprintln!("{:#?}", c::do_it());
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        eprintln!("{:?}", c::do_it());
    }
}
