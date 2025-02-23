pub fn validate_ticket_format(ticket: &str) -> bool {
    regex::Regex::new(r"^[A-Za-z]{2,10}-[0-9]{1,6}$")
        .unwrap()
        .is_match(ticket.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ticket_format() {
        assert!(validate_ticket_format("PROJ-123"));
        assert!(validate_ticket_format("ABC-1"));
        assert!(!validate_ticket_format("proj-123"));
        assert!(!validate_ticket_format("PROJ123"));
        assert!(!validate_ticket_format("123-PROJ"));
    }
}
