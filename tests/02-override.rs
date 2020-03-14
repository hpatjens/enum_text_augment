use enum_text_augment::EnumTextAugmentation;

#[derive(EnumTextAugmentation)]
enum TheEnum {
    #[enum_text_augmentation("First")]
    FirstEnumValue,
    #[enum_text_augmentation("Second")]
    SecondEnumValue,
    #[enum_text_augmentation("Third")]
    ThirdEnumValue
}

fn main() {}

#[test]
fn test_function() {
    assert_eq!(TheEnum::FirstEnumValue.augmented_text(), "First");
    assert_eq!(TheEnum::SecondEnumValue.augmented_text(), "Second");
    assert_eq!(TheEnum::ThirdEnumValue.augmented_text(), "Third");
}