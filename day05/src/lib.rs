mod parser;

#[derive(PartialEq, Eq, Debug)]
pub struct RawMapping {
    dest_start: u32,
    source_start: u32,
    range_length: u32
}
impl RawMapping {
    fn new(d: u32, s: u32, l: u32) -> Self {
        RawMapping {
            dest_start: d,
            source_start: s,
            range_length: l
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
