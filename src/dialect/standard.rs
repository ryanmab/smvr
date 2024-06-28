use crate::dialect::DialectParser;

pub struct Standard;

impl DialectParser for Standard {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::component::PartType;
    use alloc::vec;

    #[test]
    fn should_move_to_minor_from_major() {
        let result =
            Standard::parse_byte(&b'.', (PartType::Major, &vec![b'1']), &[b'1', b'2']).unwrap();

        let next_type = result;

        assert_eq!(next_type, Some(PartType::Minor))
    }

    #[test]
    fn should_move_to_patch_from_minor() {
        let result =
            Standard::parse_byte(&b'.', (PartType::Minor, &vec![b'1', b'1']), &[b'0']).unwrap();

        let next_type = result;

        assert_eq!(next_type, Some(PartType::Patch))
    }

    #[test]
    fn should_move_to_prerelease_from_patch() {
        let result = Standard::parse_byte(&b'-', (PartType::Patch, &vec![b'0']), &[b'a']).unwrap();

        let next_type = result;

        assert_eq!(next_type, Some(PartType::Prerelease))
    }

    #[test]
    fn should_move_to_build_from_patch() {
        let result = Standard::parse_byte(&b'+', (PartType::Patch, &vec![b'0']), &[b'a']).unwrap();

        let next_type = result;

        assert_eq!(next_type, Some(PartType::BuildMetadata))
    }
}
