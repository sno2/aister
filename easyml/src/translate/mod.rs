#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Language {
  Catalan,
  English,
  French,
  German,
  Italian,
  Portuguese,
  Russian,
  Spanish,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// A language pair used for translating different languages.
pub struct LanguagePair(Language, Language);

impl LanguagePair {
  /// Checks to see whether a given language pair is valid.
  const fn is_valid_pair(l1: Language, l2: Language) -> bool {
    use Language::*;
    match l1 {
      English => match l2 {
        French | Spanish | Portuguese | Italian | Catalan | German => true,
        _ => false,
      },
      French => match l2 {
        German => true,
        _ => false,
      },
      _ => false,
    }
  }

  /// Returns a [`Option<LanguagePair>`] which can have a value depending on if the language pair is valid.
  pub const fn new(l1: Language, l2: Language) -> Option<Self> {
    if LanguagePair::is_valid_pair(l1, l2) || LanguagePair::is_valid_pair(l2, l1) {
      Some(Self(l1, l2))
    } else {
      None
    }
  }
}
