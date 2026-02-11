#[cfg(test)]
mod tests {
    use iso_currency_lib::Currency;
    
    #[test]
    fn check_numeric_type() {
        let c = Currency::from_code("USD").unwrap();
        let n = c.numeric();
        
        // This will show the type in the error message
        let _: u8 = n;
    }
}
