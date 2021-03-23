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
    battle_object_kind = FIGHTER_KIND_ELIGHT, 
    animation = "catch",
    animcmd = "game_catch")]
pub fn elight_catch(fighter: &mut L2CFighterCommon) {
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
        CATCH(ID=0, Bone=hash40("top"), Size=6.6, X=0.0, Y=8.5, Z=4.0, X2=0.0, Y2=8.5, Z2=11.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=8.5, Z=2.35, X2=0.0, Y2=8.5, Z2=10.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        }
        //methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()
        wait(Frames=2)
        if(is_excute)
        {
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_ELIGHT, 
    animation = "catch_dash",
    animcmd = "game_catchdash")]
pub fn elight_catchdash(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=11)
        if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=12)
        if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=6.6, X=0.0, Y=6.6, Z=3.0, X2=0.0, Y2=6.6, Z2=11.5, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        CATCH(ID=1, Bone=hash40("top"), Size=1.5, X=0.0, Y=6.6, Z=1.5, X2=0.0, Y2=6.6, Z2=13.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        }
        //methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()
        wait(Frames=2)
        if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_ELIGHT, 
    animation = "catch_turn",
    animcmd = "game_catchturn")]
pub fn elight_catchturn(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.5)
        frame(Frame=3)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=12)
        if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=13)
        if(is_excute){
        CATCH(ID=0, Bone=hash40("top"), Size=6.6, X=0.0, Y=6.6, Z=-6.0, X2=0.0, Y2=6.6, Z2=-14.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=6.6, Z=-4.35, X2=0.0, Y2=6.6, Z2=-16.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        }
        //methodlua2cpp::L2CFighterAnimcmdGameCommon::game_CaptureCutCommon()
        wait(Frames=2)
        if(is_excute){
        sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
        WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
        GrabModule::set_rebound(CanCatchRebound=false)
        }
    });
}

pub fn install() 
{
    acmd::add_hooks!
    (
        elight_catch,
        elight_catchdash,
        elight_catchturn,
        //elight_throwlw
    );
}   