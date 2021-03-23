use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CFighterCommon;
use smash::lua2cpp::L2CFighterBase;
use smash::cpp::l2c_value::LuaConst;
use acmd::{acmd, acmd_func};
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, lua_bind::*, *};
use smash::phx::Vector3f;
use smash::phx::Hash40;
use std::env;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn captain_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
        {
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=6.0, Angle=72, KBG=40, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=14)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(13, 3)
        }
        frame(Frame=21)
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
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn captain_dair(fighter: &mut L2CFighterCommon) {
    acmd!({  
        frame(Frame=4)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            JostleModule::set_status(false)
        }
        frame(Frame=16)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=14.0, Angle=270, KBG=100, FKB=0, BKB=10, Size=6.64, X=0.0, Y=-5.2, Z=0.8, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=16.6666666667, Angle=270, KBG=100, FKB=0, BKB=40, Size=5.86, X=0.0, Y=1.0, Z=0.8, X2=0.0, Y2=10.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=5, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=1.5, Unk=false);
        }
        wait(Frames=4)
        if(is_excute)
        {
            AttackModule::clear_all()
            JostleModule::set_status(true)
        }
        frame(Frame=38)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn captain_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!({  
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=10)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=11)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=82, KBG=25, FKB=0, BKB=75, Size=4.8, X=6.5, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=12.0, Angle=80, KBG=25, FKB=0, BKB=75, Size=4.8, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=78, KBG=25, FKB=0, BKB=75, Size=3.5, X=0.0, Y=3.5, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATK_SET_SHIELD_SETOFF_MUL_arg5(ID1=0, ID2=1, ID3=2, ShieldstunMul=0.6)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false)
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }    
        wait(Frames=4)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn captain_fair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=5)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=14)
        if(is_excute)
        {
        WorkModule::set_int(0, FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_INT_CRITICAL_ATTACK_ID)
        ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=22.0, Angle=32, KBG=70, FKB=0, BKB=30, Size=6.0, X=4.4, Y=-0.2, Z=-1.0, X2=4.4, Y2=0.3, Z2=1.0, Hitlag=1.2, SDI=1.2, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=13, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=361, KBG=80, FKB=0, BKB=35, Size=3.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2.0)
        }
        wait(Frames=5)
        if(is_excute)
        {
        WorkModule::off_flag(Flag=FIGHTER_CAPTAIN_STATUS_ATTACK_AIR_WORK_FLAG_CRITICAL)
        ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=3.0, Angle=361, KBG=80, FKB=0, BKB=35, Size=4.7, X=4.4, Y=-0.2, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
        ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=80, FKB=0, BKB=35, Size=3.0, X=0.0, Y=6.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KNEE)
        }
        frame(Frame=32)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute)
        {
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn captain_dashattack(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=7)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=68, KBG=67, FKB=0, BKB=75, Size=6.2, X=0.0, Y=10.0, Z=12.0, X2=0.0, Y2=8.0, Z2=12.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.5)
        }
        wait(Frames=3)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=68, KBG=60, FKB=0, BKB=75, Size=5.0, X=0.0, Y=10.0, Z=12.0, X2=0.0, Y2=6.0, Z2=12.0, Hitlag=1.25, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_BODY)
        ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=2.5)
        }
        wait(Frames=7)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn captain_upair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
            {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=7)
            if(is_excute)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=70, KBG=100, FKB=0, BKB=15, Size=5.5, X=3.2, Y=2.1, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=70, KBG=100, FKB=0, BKB=15, Size=6.0, X=6.2, Y=0.9, Z=-0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=1, Frames=2.0, Unk=false);
            }
            wait(Frames=4)
            if(is_excute)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=9.0, Angle=30, KBG=80, FKB=0, BKB=12, Size=5.5, X=3.2, Y=2.1, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=8.0, Angle=30, KBG=80, FKB=0, BKB=12, Size=6.0, X=6.2, Y=0.9, Z=-0.4, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                AttackModule::set_add_reaction_frame(ID=0, Frames=1.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=1, Frames=1.0, Unk=false);
            }
            wait(Frames=2)
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
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn captain_uptilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=17)
        FT_MOTION_RATE(FSM=1.0)
        if(is_excute)
        {
        sv_animcmd::HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
        ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=100, FKB=0, BKB=35, Size=6.0, X=7.0, Y=-1.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=12.0, Angle=280, KBG=90, FKB=0, BKB=22, Size=3.5, X=7.0, Y=-1.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=12.0, Angle=280, KBG=100, FKB=0, BKB=35, Size=4.0, X=5.0, Y=0.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        ATTACK(ID=3, Part=0, Bone=hash40("legr"), Damage=12.0, Angle=280, KBG=100, FKB=0, BKB=35, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
        AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
        AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
        }
        frame(Frame=21)
        if(is_excute){
        sv_animcmd::HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
        AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_CAPTAIN, 
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
pub fn captain_upsmash(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=1.0)
        frame(Frame=9)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=22)
        if(is_excute)
        {
            sv_animcmd::HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=110, KBG=90, FKB=150, BKB=0, Size=5.7, X=0.0, Y=7.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=96, KBG=90, FKB=80, BKB=0, Size=6.0, X=0.0, Y=17.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=12.0, Angle=80, KBG=90, FKB=5, BKB=10, Size=5.0, X=0.0, Y=21.0, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=12.0, Angle=80, KBG=90, FKB=0, BKB=0, Size=5.0, X=0.0, Y=29.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            //AttackModule::set_vec_target_pos(2, Hash40::new("top"), 5.0, 29.0, 7.5)
            rust 
            {
                let vector = smash::phx::Vector2f{x: 5.0, y: 28.0};
                let top_hash = smash::phx::Hash40::new("top");
                AttackModule::set_vec_target_pos(module_accessor, 2, top_hash, &vector, 7.5 as u32, false);
                println!("UpSmash frame 22");
            }
        }
        frame(Frame=23)
        if(is_excute)
        {
            sv_animcmd::HIT_NODE(hash40("kneer"), HIT_STATUS_XLU)
            AttackModule::clear(ID=1, false)
        }
        frame(Frame=24)
        if(is_excute)
        {
            sv_animcmd::HIT_NODE(hash40("kneer"), HIT_STATUS_NORMAL)
            AttackModule::clear_all()
            //HitModule::set_status_all(HIT_STATUS_NORMAL)
        }
        frame(Frame=28)
        if(is_excute)
        {
            sv_animcmd::HIT_NODE(hash40("kneel"), HIT_STATUS_XLU)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=85, KBG=80, FKB=0, BKB=70, Size=6.0, X=0.0, Y=29.0, Z=5.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=14.0, Angle=85, KBG=81, FKB=0, BKB=70, Size=5.5, X=0.0, Y=21.0, Z=2.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=14.0, Angle=70, KBG=88, FKB=0, BKB=70, Size=5.0, X=0.0, Y=16.0, Z=0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=30)
        if(is_excute)
        {
            //HitModule::set_status_all(HIT_STATUS_NORMAL)
            sv_animcmd::HIT_NODE(hash40("kneel"), HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }

    });
}


pub fn install() 
{
    acmd::add_hooks!
    (
        captain_throwlw,
        captain_dair,
        captain_dtilt,
        captain_fair,
        captain_upair,
        captain_dashattack,
        captain_uptilt,
        captain_upsmash
    );
}   