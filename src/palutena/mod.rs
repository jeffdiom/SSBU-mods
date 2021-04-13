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

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn palutena_nair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=4)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=5)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=367, KBG=100, FKB=0, BKB=0, Size=5.2, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=367, KBG=100, FKB=0, BKB=0, Size=5.2, X=0.0, Y=-5.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=2, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=100, KBG=100, FKB=65, BKB=0, Size=5.2, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=3, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=95, KBG=100, FKB=80, BKB=0, Size=5.2, X=0.0, Y=-5.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=7)
        if(is_excute)
        {
        ATTACK(ID=3, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=100, KBG=100, FKB=65, BKB=0, Size=6.2, X=0.0, Y=-5.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=24)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=367, KBG=100, FKB=65, BKB=0, Size=5.2, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=367, KBG=100, FKB=65, BKB=0, Size=5.2, X=0.0, Y=-5.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=2, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=100, KBG=100, FKB=65, BKB=0, Size=5.2, X=0.0, Y=5.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=3, Part=0, Bone=hash40("stick"), Damage=1.4, Angle=100, KBG=100, FKB=65, BKB=0, Size=5.2, X=0.0, Y=-5.4, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=0.3, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        AttackModule::set_add_reaction_frame(ID=0, Frames=13.0, Unk=false)
        AttackModule::set_add_reaction_frame(ID=1, Frames=13.0, Unk=false)
        }
        frame(Frame=28)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=29)
        if(is_excute)
        {
        ATTACK(ID=0, Part=0, Bone=hash40("stick"), Damage=5.2, Angle=55, KBG=155, FKB=0, BKB=35, Size=13.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        ATTACK(ID=1, Part=0, Bone=hash40("stick"), Damage=5.2, Angle=55, KBG=155, FKB=0, BKB=35, Size=13.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_MAGIC)
        }
        frame(Frame=33)
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
    battle_object_kind = FIGHTER_KIND_PALUTENA, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn palutena_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=75, KBG=20, FKB=0, BKB=90, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=24)
            if(is_excute)
            {
            CHECK_FINISH_CAMERA(7.0, 0.0)
            }
            frame(Frame=25)
            if(is_excute){
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            AttackModule::clear_all()
            }
    });
}

pub fn install() 
{
    acmd::add_hooks!
    (  
        palutena_nair,
        palutena_throwlw
    );
}   