extern crate premium_friday;

use premium_friday::*;

#[test]
fn test_is_premium_friday() {
    let p = PremiumFriday::new();
    assert!(p.is_premium_friday(2017, 2, 24).unwrap());
}

#[test]
fn test_is_not_premium_friday() {
    let p = PremiumFriday::new();
    assert!(!p.is_premium_friday(2017, 2, 23).unwrap());
    assert!(!p.is_premium_friday(2017, 2, 25).unwrap());
}

#[test]
fn test_is_not_last_friday() {
    let p = PremiumFriday::new();
    assert!(p.is_premium_friday(2017, 1, 27).unwrap());
    assert!(!p.is_premium_friday(2017, 3, 3).unwrap());
}

#[test]
fn test_is_premium_friday_in_range() {
    let p = PremiumFriday::new().set_start_date(2017, 2, 24);
    assert!(!p.is_premium_friday(2017, 1, 27).unwrap());
}
