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
    battle_object_kind = FIGHTER_KIND_EDGE, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn edge_nair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=3)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=15)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=60, KBG=72, FKB=0, BKB=64, Size=16.0, X=0.0, Y=10.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_MAGIC)
            AttackModule::set_add_reaction_frame(ID=0, Frames=1.5, Unk=false);
        }
        wait(Frames=2)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=36)
        if(is_excute)
        {
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_EDGE, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn edge_dair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED)
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(0.0, 1.0, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(1.0, 4.0, 8.0, 3.0)
            }
            frame(Frame=5)
            if(is_excute)
            {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            FT_MOTION_RATE(FSM=1.2)
            frame(Frame=15)
            FT_MOTION_RATE(FSM=1)
            frame(Frame=16)
            if(is_excute)
            {
            FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 4.0)
            }
            frame(Frame=17)
            if(is_excute)
            {
            WorkModule::on_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            SET_SPEED_EX(0.0, -4.2, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::off_flag(Flag=FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK)
            ATTACK(ID=1, Part=0, Bone=hash40("0x075bd7e757"), Damage=15.0, Angle=58, KBG=65, FKB=0, BKB=75, Size=5.5, X=20.0, Y=0.0, Z=2.8, X2=20.0, Y2=0.0, Z2=2.8, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=0, Part=0, Bone=hash40("0x075bd7e757"), Damage=15.0, Angle=270, KBG=90, FKB=0, BKB=20, Size=11.0, X=20.0, Y=0.0, Z=2.8, X2=20.0, Y2=-9.0, Z2=2.8, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("0x075bd7e757"), Damage=10.0, Angle=48, KBG=73, FKB=0, BKB=58, Size=6.0, X=11.0, Y=0.0, Z=1.5, X2=19.5, Y2=0.0, Z2=2.8, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=22)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("0x075bd7e757"), Damage=10.0, Angle=48, KBG=73, FKB=0, BKB=58, Size=4.0, X=9.5, Y=0.0, Z=1.5, X2=21.0, Y2=-9.0, Z2=2.8, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("0x075bd7e757"), Damage=10.0, Angle=48, KBG=73, FKB=0, BKB=58, Size=4.0, X=9.5, Y=0.0, Z=1.5, X2=21.0, Y2=-9.0, Z2=2.8, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("0x075bd7e757"), Damage=10.0, Angle=48, KBG=73, FKB=0, BKB=58, Size=4.0, X=9.5, Y=0.0, Z=1.5, X2=21.0, Y2=-9.0, Z2=2.8, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_sting"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=38)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            frame(Frame=45)
            if(is_excute)
            {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
            }
            frame(Frame=53)
            if(is_excute)
            {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
    });
}



pub fn install() {
    acmd::add_hooks!(
        edge_nair,
        //edge_dair

    );
}   