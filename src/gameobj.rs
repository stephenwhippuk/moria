pub enum GameObject{
    Weapon(Weapon),
    Armor(Armor),
    Key(Key),
    Consumable(Consumable),
    Switch(Switch)
}

impl GameObject{
    pub fn get_name(&self) -> String{
        match self{
            GameObject::Weapon(w) => w.name.clone(),
            GameObject::Armor(a) => a.name.clone(),
            GameObject::Key(k) => k.name.clone(),
            GameObject::Consumable(c) => c.name.clone(),
            GameObject::Switch(s) => s.name.clone()
        }
    }

    pub fn get_type(&self) -> String{
        match self{
            GameObject::Weapon(_) => "Weapon".to_string(),
            GameObject::Armor(_) => "Armor".to_string(),
            GameObject::Key(_) => "Key".to_string(),
            GameObject::Consumable(_) => "Consumable".to_string(),
            GameObject::Switch(_) => "Switch".to_string()
        }
    }

    fn serialise(&self) -> String{
        match self{
            GameObject::Weapon(w) => format!("{}: Damage: {}, Durability: {}", w.name, w.damage, w.durability),
            GameObject::Armor(a) => format!("{}: Defense: {}, Durability: {}", a.name, a.defense, a.durability),
            GameObject::Key(k) => format!("{}: Target ID: {}", k.name, k.target_id),
            GameObject::Consumable(c) => format!("{}: Effect: {}", c.name, c.effect),
            GameObject::Switch(s) => format!("{}: Target ID: {}", s.name, s.target_id)
        }
    }

    pub fn add_weapon(name: String, damage: i32, durability: i32) -> GameObject{
        GameObject::Weapon(Weapon{name, damage, durability, is_collectable : true})
    }

    pub fn add_armor(name: String, defense: i32, durability: i32) -> GameObject{
        GameObject::Armor(Armor{name, defense, durability, is_collectable : true})
    }

    pub fn add_key(name: String, target_id: i32, ) -> GameObject{
        GameObject::Key(Key{name, target_id, is_collectable : true})
    }
    pub fn add_consumable(name: String, effect: i32) -> GameObject{
        GameObject::Consumable(Consumable{name, effect, is_collectable : true})
    }
    pub fn add_switch(name: String, target_id: i32) -> GameObject{
        GameObject::Switch(Switch{name, target_id, is_collectable : false})
    }
    pub fn is_collectable(&self) -> bool{
        match self{
            GameObject::Weapon(w) => w.is_collectable,
            GameObject::Armor(a) => a.is_collectable,
            GameObject::Key(k) => k.is_collectable,
            GameObject::Consumable(c) => c.is_collectable,
            GameObject::Switch(s) => s.is_collectable
        }
    }
}

pub struct Weapon{
    is_collectable: bool,
    name: String,
    damage: i32,
    durability: i32
}

pub struct Armor{
    is_collectable: bool,
    name: String,
    defense: i32,
    durability: i32
}

pub struct Key{
    is_collectable: bool,
    name: String,
    target_id: i32
}

pub struct Consumable{
    is_collectable: bool,
    name: String,
    effect: i32
}

pub struct Switch{
    is_collectable: bool,
    name: String,
    target_id: i32
}
