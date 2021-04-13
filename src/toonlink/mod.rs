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
use smash::resource::Hash40 as OtherHash40;
use smash::app::sv_module_access::shield;

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK, 
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
pub fn toonlink_dsmash(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=4)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=9)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=173, KBG=0, FKB=95, BKB=92, Size=4.5, X=0.0, Y=3.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=173, KBG=0, FKB=65, BKB=90, Size=4.5, X=0.0, Y=3.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=6.0, Angle=170, KBG=0, FKB=150, BKB=100, Size=4.5, X=0.0, Y=3.0, Z=17.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=170, KBG=0, FKB=120, BKB=100, Size=4.5, X=0.0, Y=3.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=6.0, Angle=160, KBG=0, FKB=100, BKB=80, Size=2.9, X=0.0, Y=3.0, Z=5.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=12.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=12.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=6.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=6.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=4, Frames=6.0, Unk=false);
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=2)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=17)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=110, FKB=0, BKB=30, Size=5.1, X=0.0, Y=5.0, Z=-18.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=110, FKB=0, BKB=30, Size=5.1, X=0.0, Y=5.0, Z=-13.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=110, FKB=0, BKB=30, Size=5.1, X=0.0, Y=5.5, Z=-8.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=13.0, Angle=42, KBG=110, FKB=0, BKB=30, Size=5.1, X=0.0, Y=6.0, Z=-4.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
        }
        wait(Frames=3)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=24)
        FT_MOTION_RATE(FSM=0.88)
        frame(Frame=50)
        FT_MOTION_RATE(FSM=1)
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn toonlink_dashattack(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        FT_MOTION_RATE(FSM=0.7)
        frame(Frame=8)
        FT_MOTION_RATE(FSM=1.1)
        if(is_excute)
        {
        ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=90, KBG=100, FKB=0, BKB=45, Size=4.2, X=5.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=8.0, Angle=80, KBG=100, FKB=0, BKB=45, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=6.0, Angle=75, KBG=70, FKB=0, BKB=70, Size=3.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=5.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=5.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=7.0, Unk=false);
        }
        wait(Frames=4)
        FT_MOTION_RATE(FSM=0.62)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=40)
        FT_MOTION_RATE(FSM=1)

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn toonlink_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=100, KBG=110, FKB=0, BKB=55, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=22)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=361, KBG=150, FKB=0, BKB=60, Size=5.0, X=0.0, Y=2.4, Z=7.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_ELBOW)
            AttackModule::set_catch_only_all(true, false)
        }
        frame(Frame=23)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(-6, 4)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
            //FighterCutInManager::set_throw_finish_offset(0, 0, 0)
        }
        frame(Frame=24)
        if(is_excute)
        {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn toonlink_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=9)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=7.5, Angle=85, KBG=20, FKB=0, BKB=80, Size=2.5, X=6.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=7.5, Angle=85, KBG=20, FKB=0, BKB=80, Size=3.0, X=1.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.5, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=7.0, Angle=361, KBG=88, FKB=0, BKB=30, Size=3.0, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
        }
        wait(Frames=3)
        if(is_excute)
        {
        AttackModule::clear_all()
        }

    });
}  

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_TOONLINK, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn toonlink_bair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=6)
            if(is_excute)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("sword2"), Damage=11.0, Angle=60, KBG=117, FKB=0, BKB=24, Size=4.0, X=6.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("sword2"), Damage=11.0, Angle=60, KBG=117, FKB=0, BKB=24, Size=5.0, X=2.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=11.0, Angle=60, KBG=117, FKB=0, BKB=24, Size=4.0, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_TOONLINK_HIT, Type=ATTACK_REGION_SWORD)
            }
            frame(Frame=10)
            if(is_excute)
            {
                AttackModule::clear_all()
            }
            FT_MOTION_RATE(FSM=1.65)
            frame(Frame=24)
            if(is_excute)
            {
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=39)
            if(is_excute)
            {
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
    });
}




pub fn install() 
{
    acmd::add_hooks!
    (
       toonlink_dsmash,
       toonlink_throwlw,
       toonlink_dashattack,
       toonlink_dtilt,
       toonlink_bair
    );
}   