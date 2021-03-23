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
//use smash::cpp::root::app::ArticleOperationTarget

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn kamui_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({  
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.9)
        frame(Frame=5)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=5.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=4.0, X=-1.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=7.5, Angle=100, KBG=80, FKB=0, BKB=50, Size=4.0, X=-1.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
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
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn kamui_uptilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.88)
        frame(Frame=7)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=4.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=8.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=9.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=5.0, X=0.0, Y=12.0, Z=4.0, X2=0.0, Y2=12.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
        }
        wait(Frames=2)
        if(is_excute)
        {
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=6.0, Angle=88, KBG=53, FKB=0, BKB=65, Size=4.0, X=0.0, Y=22.0, Z=-2.0, X2=0.0, Y2=22.0, Z2=2.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            //AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            //AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
        }
        wait(Frames=8)
        if(is_excute)
        {
        AttackModule::clear_all()
        }

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn kamui_upair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.88)
        frame(Frame=4)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=12.5, Angle=78, KBG=87, FKB=0, BKB=50, Size=6.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=12.5, Angle=78, KBG=87, FKB=0, BKB=50, Size=6.5, X=0.0, Y=4.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=12.5, Angle=78, KBG=87, FKB=0, BKB=50, Size=6.5, X=0.0, Y=8.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=9)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn kamui_fair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.85)
        frame(Frame=9)
        if(is_excute)
        {
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=60, KBG=87, FKB=0, BKB=40, Size=5.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=60, KBG=87, FKB=0, BKB=40, Size=5.0, X=-1.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=9.0, Angle=60, KBG=87, FKB=0, BKB=40, Size=5.0, X=-1.0, Y=8.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=4)
        if(is_excute)
        {
        AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute)
        {
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=59)
        if(is_excute)
        {
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn kamui_bair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.85)
        frame(Frame=6)
        if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=13)
        if(is_excute){
        SET_SPEED_EX(1.25, 0.25, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=120, FKB=0, BKB=40, Size=8.5, X=0.0, Y=10.0, Z=-12.5, X2=0.0, Y2=11.0, Z2=-17.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=3)
        if(is_excute){
        ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=40, Size=6.7, X=0.0, Y=10.7, Z=-12.5, X2=0.0, Y2=12.2, Z2=-21.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_BODY)
        }
        wait(Frames=3)
        if(is_excute){
        AttackModule::clear_all()
        }
        frame(Frame=42)
        if(is_excute){
        WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn kamui_nair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.82)
        if(is_excute)
            {
            ArticleModule::generate_article(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, false, 0)
            ArticleModule::change_motion(FIGHTER_KAMUI_GENERATE_ARTICLE_SPEARHAND, Hash40{hash:hash40("attack_air_n")}, false, 0.0);
            }
            frame(Frame=6)
            if(is_excute)
            {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=1, Bone=hash40("handl"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=6.0, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=1, Bone=hash40("handl"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=4.5, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=1, Bone=hash40("handl"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=4.0, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=5.0, X=0.0, Y=4.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("haver"), Damage=7.0, Angle=70, KBG=105, FKB=0, BKB=45, Size=4.0, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute)
            {
            ATTACK(ID=0, Part=1, Bone=hash40("handl"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=6.0, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=1, Bone=hash40("handl"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=4.5, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=1, Bone=hash40("handl"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=4.0, X=7.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=5.0, X=0.0, Y=4.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("haver"), Damage=5.5, Angle=70, KBG=115, FKB=0, BKB=50, Size=4.0, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=12)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            frame(Frame=45)
            if(is_excute)
            {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "throw_hi",
    animcmd = "game_throwhi")]
pub fn kamui_throwhi(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0)
            ArticleModule::set_visibility_whole(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, smash::app::ArticleOperationTarget(0))
            ArticleModule::change_motion(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40{hash:hash40("throw_hi")}, false, 0.0);
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=85, KBG=200, FKB=0, BKB=30, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=5)
        if(is_excute)
        {
            VisibilityModule::set_whole(false)
            ArticleModule::set_visibility_whole(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, smash::app::ArticleOperationTarget(0))
        }
        frame(Frame=12)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=85, KBG=160, FKB=0, BKB=40, Size=8.0, X=0.0, Y=20.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(1, 27)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
        }
        frame(Frame=13)
        if(is_excute)
        {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.7)
        frame(Frame=40)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, smash::app::ArticleOperationTarget(0))
            VisibilityModule::set_whole(true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn kamui_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        FT_MOTION_RATE(FSM=0.95)
        if(is_excute)
            {
            ArticleModule::generate_article(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0)
            ArticleModule::set_visibility_whole(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, smash::app::ArticleOperationTarget(0))
            ArticleModule::change_motion(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40{hash:hash40("throw_lw")}, false, 0.0);
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=4.0, Angle=72, KBG=150, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=5)
            if(is_excute)
            {
            VisibilityModule::set_whole(false)
            ArticleModule::set_visibility_whole(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, smash::app::ArticleOperationTarget(0))
            }
            frame(Frame=26)
            if(is_excute)
            {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=150, FKB=0, BKB=40, Size=7.0, X=0.0, Y=4.0, Z=7.0, X2=0.0, Y2=4.0, Z2=-6.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(3, 0)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.1)
            }
            frame(Frame=27)
            if(is_excute)
            {
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                AttackModule::clear_all()
            }
            FT_MOTION_RATE(FSM=0.7)
            frame(Frame=45)
            if(is_excute)
            {
            ArticleModule::remove_exist(FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, smash::app::ArticleOperationTarget(0))
            VisibilityModule::set_whole(true)
            }

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_KAMUI, 
    animation = "catch",
    animcmd = "game_catch")]
pub fn kamui_catch(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=6)
        if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=7)
        if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=5.5, X=0.0, Y=7.5, Z=4.0, X2=0.0, Y2=7.5, Z2=10.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        CATCH(ID=1, Bone=hash40("top"), Size=1.55, X=0.0, Y=7.5, Z=2.45, X2=0.0, Y2=7.5, Z2=10.25, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        game_CaptureCutCommon()
        wait(Frames=2)
        if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_WEAPON, 
    battle_object_kind = WEAPON_KIND_KAMUI_RYUSENSYA, 
    animation = "shot_max",
    animcmd = "game_shotmax")]
pub fn kamui_waterball(fighter: &mut L2CFighterBase) {
    acmd!
    ({
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=55, KBG=90, FKB=0, BKB=40, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_NONE)
        }
    });
}



pub fn install() 
{
    acmd::add_hooks!
    (
        kamui_dtilt,
        kamui_fair,
        kamui_upair,
        kamui_uptilt,
        kamui_bair,
        kamui_nair,
        kamui_throwhi,
        kamui_throwlw,
        kamui_catch,
        kamui_waterball
    );
}   