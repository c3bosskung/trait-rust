#[derive(Debug, PartialEq)]
pub struct Player {
    name: String,
    age: u32,
    class: String,
    hp: u32,
    walk_style: String,
    spacial_skill: String,
    status_online: bool,
}

pub trait DetailPlayer {
    fn info_player(&self) -> String;
    fn is_online(&self) -> bool;
}

impl DetailPlayer for Player {
    fn info_player(&self) -> String {
        return format!("Name: {}, Age: {}, Class: {}, Hp: {}, walkStyle: {}, spacial_skill: {}, status_online: {}", 
        self.name, self.age, self.class, self.hp, self.walk_style, self.spacial_skill, self.status_online);
    }
    fn is_online(&self) -> bool {
        return true;
    }
}

pub struct MagicalSkillCoolDown {
    fissure_wave: u8,
    psyche_of_aad_sphera: u8,
    emergency_escape: u8,
    fire_ball: u8,
}

pub struct TankSkillCoolDown {
    titan_syndrome: u8,
    smack_down: u8,
    fierce_strike: u8,
    unstoppable: u8,
}

pub trait CoolDownCheck {
    fn is_available(&self) -> String;
}

impl CoolDownCheck for MagicalSkillCoolDown {
    fn is_available(&self) -> String {
        let fissure_wave_bool: bool;
        let psyche_of_aad_sphera_bool: bool;
        let emergency_escape_bool: bool;
        let fireball_bool: bool;
        // Please refactor here
        
        if self.fissure_wave != 0 {
            fissure_wave_bool = false;
        } else {
            fissure_wave_bool = true;
        }
        if self.psyche_of_aad_sphera != 0 {
            psyche_of_aad_sphera_bool = false;
        } else {
            psyche_of_aad_sphera_bool = true;
        }
        if self.emergency_escape !=0 {
            emergency_escape_bool = false;
        } else {
            emergency_escape_bool = true;
        }
        if self.fire_ball != 0 {
            fireball_bool = false;
        } else {
            fireball_bool = true;
        }
        return format!("Magical: fissure_wave: {}, psyche_of_aad_sphera: {}, emergency_escape: {}, Fireball: {}",
    fissure_wave_bool, psyche_of_aad_sphera_bool, emergency_escape_bool, fireball_bool);
    }
}

impl CoolDownCheck for TankSkillCoolDown {
    fn is_available(&self) -> String {
        let titan_syndrome_bool: bool;
        let smack_down_bool: bool;
        let fierce_strike_bool: bool;
        let unstoppable_bool: bool;

        if self.titan_syndrome != 0 {
            titan_syndrome_bool = false;
        } else {
            titan_syndrome_bool = true;
        }
        if self.smack_down != 0 {
            smack_down_bool = false;
        } else {
            smack_down_bool = true;
        }
        if self.fierce_strike != 0 {
            fierce_strike_bool = false;
        } else {
            fierce_strike_bool = true;
        }
        if self.unstoppable != 0 {
            unstoppable_bool = false;
        } else {
            unstoppable_bool = true;
        }

        return format!("Tank: TitanSyndrome: {}, SmackDown: {}, FierceStrike: {}, Unstoppable: {}"
        , titan_syndrome_bool, smack_down_bool, fierce_strike_bool, unstoppable_bool);
    }
}

fn main() {
    let info1 = Player {
        name: "Kira".to_string(),
        age: 20,
        class: "Tank".to_string(),
        hp: 300,
        walk_style: "normal".to_string(),
        spacial_skill: "Unstoppable".to_string(),
        status_online: true
    };
    println!("{}", info1.info_player());

    let info2 = Player {
        name: "Light".to_string(),
        age: 2000,
        class: "Magical".to_string(),
        hp: 150,
        walk_style: "Fly with bloom".to_string(),
        spacial_skill: "FireBall".to_string(),
        status_online: false
    };
    println!("{}", info2.info_player());

    print!("\n");

    let skill_magic = MagicalSkillCoolDown {
        fissure_wave: 20,
        psyche_of_aad_sphera: 18,
        emergency_escape: 0,
        fire_ball: 0,
    };
    println!("{}", skill_magic.is_available());

    let skill_tank = TankSkillCoolDown {
        titan_syndrome: 20,
        smack_down: 0,
        fierce_strike: 0,
        unstoppable: 12,
    };
    println!("{}", skill_tank.is_available());    
}


