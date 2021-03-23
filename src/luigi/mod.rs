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
use smash::app::sv_battle_object::notify_event_msc_cmd;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn luigi_dair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({  
        frame(Frame=6)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=10)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=80, FKB=0, BKB=30, Size=8.0, X=0.0, Y=1.0, Z=2.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=6.0, Unk=false);
        }
        frame(Frame=14)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=3.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=5.2, X=4.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=18)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=24)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_hi",
    animcmd = "game_specialhi")]
pub fn luigi_specialhi(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=8)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=88, KBG=88, FKB=0, BKB=50, Size=4.4, X=1.2, Y=6.0, Z=7.0, X2=-1.2, Y2=6.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
        WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
        }
        wait(Frames=3)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=5.8, X=2.0, Y=2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=4.7, X=0.0, Y=4.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
        WorkModule::off_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
        }
        wait(Frames=1)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
        wait(Frames=1)
        if(is_excute){
        SA_SET(State=SITUATION_KIND_AIR)
        }
        frame(Frame=10)
        if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=24)
        if(is_excute){
        AttackModule::clear_all()
        }
    });
} 

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUIGI, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn luigi_specialairhi(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=6)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=30.0, Angle=90, KBG=80, FKB=0, BKB=40, Size=5.4, X=0.0, Y=6.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
        }
        wait(Frames=3)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=5.8, X=2.0, Y=2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
        ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=4.7, X=0.0, Y=4.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
        WorkModule::off_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
        }
        wait(Frames=1)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
        }
        wait(Frames=1)
        if(is_excute){
        SA_SET(State=SITUATION_KIND_AIR)
        }
        frame(Frame=10)
        if(is_excute){
        sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=24)
        if(is_excute){
        AttackModule::clear_all()
        }

    });
}

pub fn install() {
    acmd::add_hooks!(
        luigi_dair,
        luigi_specialhi,
        luigi_specialairhi
    );
}   