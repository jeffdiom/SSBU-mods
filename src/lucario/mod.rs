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
    battle_object_kind = FIGHTER_KIND_LUCARIO, 
    animation = "special_s",
    animcmd = "game_specials")]
pub fn lucario_specials(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(2.0, 5.0)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=8)
        if(is_excute)
        {
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=9)
        if(is_excute)
        {
            CATCH(ID=0, Bone=hash40("top"), Size=4.0, X=0.0, Y=6.0, Z=5.3, X2=0.0, Y2=6.0, Z2=18.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            CATCH(ID=1, Bone=hash40("top"), Size=4.0, X=0.0, Y=6.0, Z=7.4, X2=0.0, Y2=6.0, Z2=18.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
        }
        wait(Frames=1)
        if(is_excute)
        {
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
        frame(Frame=24)
        IS_GENERATABLE_ARTICLE(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG)
        if(is_excute)
        {    
            if (WorkModule::is_flag(module_accessor,*FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG)==true)
            {
                ArticleModule::generate_article(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, true, 0)
            }
            else
            {
                ArticleModule::generate_article(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0)

            }
        }
            
    });
}


#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUCARIO, 
    animation = "special_air_s",
    animcmd = "game_specialairs")]
pub fn lucario_specialairs(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
            {
            FighterAreaModuleImpl::enable_fix_jostle_area(9.0, 5.0)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=6.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, 
            Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            frame(Frame=24)
            IS_GENERATABLE_ARTICLE(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG)
            if(is_excute)
            { 
                if (WorkModule::is_flag(module_accessor,*FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG)==true)
                {
                    ArticleModule::generate_article(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, true, 0)
                }
                else
                {
                    ArticleModule::generate_article(FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, 0)
                }
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUCARIO, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn lucario_dair(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        if(is_excute)
            {
            FighterAreaModuleImpl::enable_fix_jostle_area_xy(3.0, 3.0, 8.0, 2.0)
            }
            frame(Frame=2)
            if(is_excute){
            WorkModule::get_float(FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_ATTACK_AIR_LW_SPEED)
            SET_SPEED_EX(0.15, KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN)
            WorkModule::on_flag(Flag=FIGHTER_LUCARIO_ATTACK_AIR_STATUS_WORK_ID_FLAG_DEC_SPEED)
            KineticModule::suspend_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=4)
            if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=30, FKB=0, BKB=20, Size=3.8, X=0.0, Y=-2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=367, KBG=100, FKB=50, BKB=0, Size=4.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=150, KBG=100, FKB=25, BKB=0, Size=4.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=2)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=11)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=270, KBG=100, FKB=0, BKB=60, Size=6.8, X=0.0, Y=-2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=6.0, Angle=270, KBG=100, FKB=0, BKB=60, Size=5.6, X=0.0, Y=2.2, Z=0.3, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_aura"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_LUCARIO, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=2)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=24)
            if(is_excute){
            KineticModule::resume_energy(FIGHTER_KINETIC_ENERGY_ID_CONTROL)
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_LUCARIO, 
    animation = "catch",
    animcmd = "game_catch")]
pub fn lucario_catch(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=5)
        if(is_excute)
        {
            GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=6)
        if(is_excute)
        {
            CATCH(ID=0, Bone=hash40("top"), Size=5.2, X=0.0, Y=8.5, Z=4.0, X2=0.0, Y2=8.5, Z2=9.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
            CATCH(ID=1, Bone=hash40("top"), Size=1.9, X=0.0, Y=8.5, Z=2.1, X2=0.0, Y2=8.5, Z2=11.6, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        game_CaptureCutCommon()
        wait(Frames=3)
        if(is_excute)
        {
            sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
            GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        lucario_specials,
        lucario_specialairs,
        lucario_dair,
        lucario_catch
    );
}   