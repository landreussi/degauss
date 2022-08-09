/// Full compatibility: A new schema is fully compatible if it’s both backward and forward compatible.
#[cfg(test)]
mod full_compat {

    use apache_avro::Schema;
    use degauss::prelude::*;

    #[test]
    fn adding_a_field_with_default_is_a_backward_and_a_forward_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Full);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn transitively_adding_a_field_without_a_default_is_not_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Full);
        assert_eq!(dc.validate(&schemas), true);
    }

    #[test]
    fn transitively_removing_a_field_without_a_default_is_not_a_compatible_change() {
        let schemas = vec![
            Schema::parse_file("tests/data/schema3.avsc").unwrap(),
            Schema::parse_file("tests/data/schema2.avsc").unwrap(),
            Schema::parse_file("tests/data/schema1.avsc").unwrap(),
        ];
        let dc = DegaussCheck(DegaussCompatMode::Full);
        assert_eq!(dc.validate(&schemas), true);
    }
}
