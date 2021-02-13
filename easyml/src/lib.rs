#[cfg(feature = "translate")]
pub mod translate;

#[cfg(tests)]
mod tests {
  use super::*;

  #[test]
  fn is_working() {
    let lang = LanguagePair::new(Language::English, Language::English);

    assert_ne!(language, None)
  }
}
