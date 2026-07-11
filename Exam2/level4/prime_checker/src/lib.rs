#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}
pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb == 0 || nb == 1 {
        return None;
    }
    if nb == 2 {
        return Some(Ok(nb));
    }
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }
    let mut divider = 3;
    while divider * divider <= nb {
        if nb % divider == 0 {
            return Some(Err(PrimeErr::Divider(divider)));
        }
        divider += 2;
    }
    Some(Ok(nb))
}