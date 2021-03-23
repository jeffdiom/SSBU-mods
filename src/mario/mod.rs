use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use acmd::{acmd, acmd_func};
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::*, *};
use smash::phx::Vector3f;
use smash::phx::Hash40;
use std::env;
use smash::app::sv_module_access::shield;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn mario_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(16)
        if(is_execute) 
        {            
            ATTACK(ID=0, Part=0, Bone=hash40("arml"), 
                // changed Damage 12.0 -> 19.0
                Damage=25.0, Angle=280, KBG=60, FKB=0, BKB=15, 
                // Changed Size 3.0 -> 113.0
                Size=5.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
            
            // Inline Rust code inside ACMD macro
            rust 
            {
                println!("Fair frame 16");
            }
            sv_kinetic_energy::add_speed(1.5);
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn mario_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        for(5 Iterations)
        {
            if(is_excute)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.4, Angle=94, KBG=15, FKB=0, BKB=50, Size=4.0, X=0.0, Y=-0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.4, Angle=94, KBG=15, FKB=0, BKB=25, Size=8.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
            }
            wait(Frames=1)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            wait(Frames=1)
        }
        frame(Frame=23)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.5, Angle=75, KBG=100, FKB=0, BKB=80, Size=12.0, X=0.0, Y=6.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.8, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=1)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=33)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MARIO, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn mario_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=70, KBG=90, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=17)
            if(is_excute)
            {
            CHECK_FINISH_CAMERA(4, 0)
            }
            frame(Frame=18)
            if(is_excute)
            {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_MARIO_FIREBALL, 
    animation = "regular",
    animcmd = "game_regular")]
pub fn mario_fireball(fighter: &mut L2CFighterBase) {
    acmd!
    ({
        if(is_excute)
            {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=20, FKB=0, BKB=40, Size=3.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_ice"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MARIO_FIREBALL, Type=ATTACK_REGION_NONE)
            AttackModule::enable_safe_pos()
            }
            frame(Frame=5)
            if(is_excute)
            {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=15, FKB=0, BKB=40, Size=3.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2.5, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_ice"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MARIO_FIREBALL, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=30)
            if(is_excute)
            {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=10, FKB=0, BKB=40, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_SPEED, SetWeight=false, ShieldDamage=-2, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_ice"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MARIO_FIREBALL, Type=ATTACK_REGION_NONE)
            }
    });
}


pub fn install() {
    acmd::add_hooks!(
        mario_fair,
        mario_dair,
        mario_throwlw,
        mario_fireball
    );
}
