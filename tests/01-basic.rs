use enum_text_augment::EnumTextAugmentation;

#[derive(EnumTextAugmentation)]
enum TheEnum {
    FirstEnumValue,
    SecondEnumValue,
    ThirdEnumValue
}

fn main() {}

#[test]
fn test_function() {
    assert_eq!(TheEnum::FirstEnumValue.augmented_text(), "First enum value");
    assert_eq!(TheEnum::SecondEnumValue.augmented_text(), "Second enum value");
    assert_eq!(TheEnum::ThirdEnumValue.augmented_text(), "Third enum value");
}