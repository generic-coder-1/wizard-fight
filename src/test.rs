use itertools::Itertools;
use strum::IntoEnumIterator;

use crate::controller::model::spell::{Spell, SpellElement};

#[test]
fn spell_requirements() {
    assert_eq!(
        Spell::iter().map(|spell| spell.requirement()).collect_vec(),
        SpellElement::iter()
            .flat_map(|ele| { (1..=4).map(move |i| (ele, i)) })
            .collect_vec()
    )
}
