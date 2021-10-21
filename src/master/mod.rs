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
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn master_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            //ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD)
            //as_hash__const(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, hash40("throw_lw"))
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40{hash:hash40("throw_lw")}, false, 0.0);
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.0, Angle=82, KBG=150, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=12)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=48, KBG=100, FKB=0, BKB=60, Size=6.0, X=0.0, Y=6.0, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(3, 0)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.2)
            //FighterCutInManager::set_throw_finish_offset(10, 1, 0)
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
        frame(Frame=52)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::app::ArticleOperationTarget(0))
        }
            
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn master_nair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, false, 0)
            //as_hash__const(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, hash40("attack_air_n"))
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, Hash40{hash:hash40("attack_air_n")}, false, 0.0);
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
            }
            frame(Frame=6)
            if(is_excute)
            {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=120, KBG=100, FKB=45, BKB=0, Size=5.5, X=-4.0, Y=-2.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=5.5, X=-4.0, Y=-2.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=94, KBG=100, FKB=45, BKB=0, Size=4.7, X=-4.0, Y=1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=2.5, Angle=367, KBG=100, FKB=40, BKB=0, Size=4.7, X=-4.0, Y=1.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.5, Angle=80, KBG=100, FKB=35, BKB=0, Size=5.0, X=0.0, Y=9.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=7, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_add_reaction_frame(ID=0, Frames=15.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=1, Frames=15.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=2, Frames=15.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=3, Frames=15.0, Unk=false)
            AttackModule::set_add_reaction_frame(ID=4, Frames=15.0, Unk=false)
            }
            frame(Frame=27)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            frame(Frame=28)
            if(is_excute)
            {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.5, Angle=55, KBG=150, FKB=0, BKB=44, Size=14.0, X=0.0, Y=9.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_OBJECT)
            }
            frame(Frame=29)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            frame(Frame=36)
            if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=53)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_BOW, smash::app::ArticleOperationTarget(0))
        }
            
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn master_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({ 
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, false, 0)
            //as_hash__const(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, hash40("attack_lw3"))
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, Hash40{hash:hash40("attack_lw3")}, false, 0.0);
            }
            frame(Frame=13)
            if(is_excute)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=96, KBG=55, FKB=0, BKB=67, Size=3.0, X=0.0, Y=2.8, Z=10.0, X2=0.0, Y2=2.8, Z2=12.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=8.0, Angle=106, KBG=55, FKB=0, BKB=67, Size=5.0, X=0.0, Y=2.8, Z=17.0, X2=0.0, Y2=2.8, Z2=25.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            }
            frame(Frame=16)
            if(is_excute)
            {
            AttackModule::clear_all()
            }
            frame(Frame=59)
            if(is_excute)
            {
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_SWORD, smash::app::ArticleOperationTarget(0))
            }

    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn master_dair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, false, 0)
            //as_hash__const(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, hash40("attack_air_lw"))
            ArticleModule::change_motion(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, Hash40{hash:hash40("attack_air_lw")}, false, 0.0);
            FighterAreaModuleImpl::enable_fix_jostle_area(3.0, 3.0)
        }
        frame(Frame=1)
        if(is_excute)
        {
            MotionModule::set_rate(1.25)
        }
        frame(Frame=2)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=22)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=19.0, Angle=58, KBG=61, FKB=0, BKB=87, Size=4.3, X=0.0, Y=2.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=19.0, Angle=72, KBG=56, FKB=0, BKB=97, Size=4.3, X=0.0, Y=2.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=19.0, Angle=58, KBG=61, FKB=0, BKB=87, Size=4.0, X=0.0, Y=-3.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=19.0, Angle=72, KBG=56, FKB=0, BKB=97, Size=4.0, X=0.0, Y=-3.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=23.0, Angle=270, KBG=86, FKB=0, BKB=30, Size=9.6, X=0.0, Y=13.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=20, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            ATTACK(ID=3, Part=0, Bone=hash40("haver"), Damage=23.0, Angle=270, KBG=78, FKB=0, BKB=100, Size=9.6, X=0.0, Y=13.5, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=20, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MASTER_AXE, Type=ATTACK_REGION_OBJECT)
            AttackModule::set_add_reaction_frame(ID=2, Frames=0.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=0.0, Unk=false);
        }
        wait(Frames=3)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=50)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=60)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_MASTER_GENERATE_ARTICLE_AXE, smash::app::ArticleOperationTarget(0))
        }
            
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn master_uptilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=9)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=92, KBG=83, FKB=0, BKB=66, Size=3.5, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=96, KBG=83, FKB=0, BKB=66, Size=4.5, X=0.0, Y=-1.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=96, KBG=83, FKB=0, BKB=66, Size=4.5, X=4.0, Y=-1.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=102, KBG=83, FKB=0, BKB=66, Size=4.5, X=8.0, Y=-1.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("sword1"), Damage=10.0, Angle=102, KBG=83, FKB=0, BKB=66, Size=4.5, X=12.0, Y=-1.5, Z=-2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=4, Frames=3.0, Unk=false);
        }
        frame(Frame=18)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_MASTER, 
    animation = "throw_hi",
    animcmd = "game_throwhi")]
pub fn master_throwhi(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
        {
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=85, KBG=100, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=14)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=6.0, Angle=76, KBG=110, FKB=0, BKB=60, Size=2.0, X=0.0, Y=-1.0, Z=1.0, X2=13.0, Y2=-1.0, Z2=1.0, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_catch_only_all(true, false)
            CHECK_FINISH_CAMERA(0, 30)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.7)
            //FighterCutInManager::set_throw_finish_offset(2, 8.5, 0)
        }
        frame(Frame=15)
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

pub fn install() 
{
    acmd::add_hooks!
    (
        master_nair,
        master_throwlw,
        master_dtilt,
        master_dair,
        master_uptilt,
        master_throwhi
    );
}   