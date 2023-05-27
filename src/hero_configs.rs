use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Hero Configs
/// holds the different configs for heros used in this.
/// WIll gradually add to this ,but will mostly just be
/// characters I play. will eventually make a gui for this,
/// but its going to be terminal based for now.

pub struct HeroTimeout {
    pub e_key: u32,
    pub shift_key: u32,
}

pub trait HeroTrait {
    // gets the heros timeout
    fn get_timout() -> HeroTimeout;
}

pub struct HeroConfig {
    pub name: &'static str,
    pub e_key_timeout: u32,
    pub shift_key_timeout: u32,
}

static HERO_MAP: Lazy<HashMap<String, HeroConfig>> = Lazy::new(|| {
    let all_heros = vec![
        HeroConfig {
            name: "ana",
            e_key_timeout: 10,
            shift_key_timeout: 14,
        },
        HeroConfig {
            name: "kiriko",
            e_key_timeout: 14,
            shift_key_timeout: 14,
        },
    ];
    let mut map = HashMap::<String, HeroConfig>::new();
    for hero in all_heros.into_iter() {
        map.insert(hero.name.to_string(), hero);
    }
    map
});
