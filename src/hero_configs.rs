/// Hero Configs
/// holds the different configs for heros used in this.
/// WIll gradually add to this ,but will mostly just be
/// characters I play.
///

pub struct HeroTimeout {
    pub e_key: u32,
    pub shift_key: u32,
}

pub trait HeroTrait {
    // gets the heros timeout
    fn get_timout() -> HeroTimeout;
}

pub struct HeroConfig<'a> {
    pub name: &'a str,
    pub e_key_timeout: u32,
    pub shift_key_timeout: u32,
}
