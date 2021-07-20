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
    battle_object_kind = FIGHTER_KIND_FOX, 
    animation = "throw_hi",
    animcmd = "game_throwhi")]
pub fn fox_throwhi(fighter : &mut L2CFighterCommon) {
    acmd!({
        FT_MOTION_RATE(FSM=0.8)
        if(is_excute)
        {
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=2.0, Angle=90, KBG=110, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=60, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=6)
            if(is_excute)
            {
                CHECK_FINISH_CAMERA(2, 20)
                //rust {
                //let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
                //lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.6);
                //lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 4.0, z: 0.0});
                //}
            }
            frame(Frame=7)
            if(is_excute)
            {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
            FT_MOTION_RATE(FSM=0.475)
            frame(Frame=15)
            if(ArticleModule::is_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER))
            {
                if(is_excute)
                {
                    ArticleModule::change_motion(*FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40{hash: hash40("close")}, false, 0.0)
                }
            }
            frame(Frame=18)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=21)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=24)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=27)
            if(ArticleModule::is_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER))
            {
                if(is_excute)
                {
                    ArticleModule::change_motion(*FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40{hash: hash40("close")}, false, 0.0)
                }
            }
            rust 
            {
            println!("FoxThrowHi");
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_FOX, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn fox_throwlw(fighter : &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=1.0, Angle=72, KBG=100, FKB=0, BKB=85, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=23)
            if(ArticleModule::is_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER))
            {
                if(is_excute)
            
                {
                    ArticleModule::change_motion(*FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40{hash: hash40("open")}, false, 0.0)
                }
            }
            frame(Frame=26)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=29)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=32)
            if(is_excute)
            {
                ArticleModule::generate_article(FIGHTER_FOX_GENERATE_ARTICLE_BLASTER_BULLET, false, 0)
            }
            frame(Frame=33)
            if(is_excute)
            {
            CHECK_FINISH_CAMERA(1, 1)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.6)
            //FighterCutInManager::set_throw_finish_offset(0, 6, 0)
            }
            frame(Frame=34)
            if(is_excute)
            {
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
            }
            frame(Frame=35)
            if(ArticleModule::is_exist(module_accessor, *FIGHTER_FOX_GENERATE_ARTICLE_BLASTER))
            {
                if(is_excute)
                {
                    ArticleModule::change_motion(*FIGHTER_FOX_GENERATE_ARTICLE_BLASTER, Hash40{hash: hash40("close")}, false, 0.0)
                }
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_FOX, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn fox_upair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=9)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=5.0, Angle=92, KBG=120, FKB=30, BKB=0, Size=5.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FOX_TAIL, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=5.0, Angle=92, KBG=120, FKB=30, BKB=0, Size=5.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FOX_TAIL, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("tail3"), Damage=5.0, Angle=92, KBG=120, FKB=30, BKB=0, Size=5.2, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_FOX_TAIL, Type=ATTACK_REGION_TAIL)
        }
        wait(Frames=2)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=13.0, Angle=88, KBG=108, FKB=0, BKB=30, Size=5.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=13.0, Angle=88, KBG=108, FKB=0, BKB=30, Size=7.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=13.0, Angle=88, KBG=108, FKB=0, BKB=30, Size=7.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}
pub fn install() 
{
    acmd::add_hooks!
    (
        fox_throwhi,
        fox_throwlw,
        fox_upair
    );
}   