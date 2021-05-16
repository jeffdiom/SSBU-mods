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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "catch",
    animcmd = "game_catch")]
pub fn jack_catch(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute){
        GrabModule::set_rebound(CanCatchRebound=true)
        }
        frame(Frame=6)
        if(is_excute)
        {
        CATCH(ID=0, Bone=hash40("top"), Size=6.3, X=0.0, Y=8.1, Z=4.5, X2=0.0, Y2=8.1, Z2=10.65, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
        CATCH(ID=1, Bone=hash40("top"), Size=3.0, X=0.0, Y=8.1, Z=2.85, X2=0.0, Y2=8.1, Z2=10.65, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
        }
        game_CaptureCutCommon()
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "catch_dash",
    animcmd = "game_catchdash")]
pub fn jack_catchdash(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(Frame=7)
        if(is_excute)
        {
		GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=8)
        if(is_excute)
        {
		CATCH(ID=0, Bone=hash40("top"), Size=6.3, X=0.0, Y=6.6, Z=4.0, X2=0.0, Y2=6.6, Z2=11.9, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
		CATCH(ID=1, Bone=hash40("top"), Size=3.0, X=0.0, Y=6.6, Z=2.7, X2=0.0, Y2=6.6, Z2=11.9, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		game_CaptureCutCommon()
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "catch_turn",
    animcmd = "game_catchturn")]
pub fn jack_catchturn(fighter: &mut L2CFighterCommon) {
    acmd!({
		frame(Frame=7)
        if(is_excute)
        {
		GrabModule::set_rebound(CanCatchRebound=true)
		}
		frame(Frame=8)
        if(is_excute)
        {
		CATCH(ID=0, Bone=hash40("top"), Size=6.3, X=0.0, Y=6.6, Z=-7.0, X2=0.0, Y2=6.6, Z2=-18.700001, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_GA)
		CATCH(ID=1, Bone=hash40("top"), Size=3.0, X=0.0, Y=6.6, Z=-5.35, X2=0.0, Y2=6.6, Z2=-18.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
		}
		game_CaptureCutCommon()
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "throw_lw",
    animcmd = "game_throwlw")]
pub fn jack_throwlw(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=81, KBG=105, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            else
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=81, KBG=95, FKB=0, BKB=55, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
        }
        frame(Frame=22)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(12.8, 0.0)
        }
        frame(Frame=23)
        if(is_excute)
        {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT),
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP),
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
        frame(Frame=25)
        FT_MOTION_RATE(FSM=0.8)
        frame(Frame=36)
        FT_MOTION_RATE(FSM=1.0)
        rust 
        {
            println!("JackThrowLw");
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "throw_hi",
    animcmd = "game_throwhi")]
pub fn jack_throwhi(fighter: &mut L2CFighterCommon) {
        acmd!({
            if(is_excute)
            {
                if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
                {
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=90, KBG=72, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                
                }
                else
                {
                    //KBG = 60 -> 62
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=90, KBG=62, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                }    
            }
            frame(Frame=15)
            if(is_excute)
            {
                CHECK_FINISH_CAMERA(3.0, 10.0)
            }
            frame(Frame=16)
            if(is_excute)
            {
                ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"),
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
                WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                rust 
                {
                    println!("JackThrowHi");
                }
            }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "throw_b",
    animcmd = "game_throwb")]
pub fn jack_throwb(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=135, KBG=70, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            
            }
            else
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=131, KBG=66, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=40, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }    
        }
        frame(Frame=13)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(-7.0, 0.0)
        }
        frame(Frame=14)
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "throw_f",
    animcmd = "game_throwf")]
pub fn jack_throwf(fighter: &mut L2CFighterCommon) {
    acmd!({
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=10.0, Angle=45, KBG=65, FKB=0, BKB=65, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
            else
            {
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=45, KBG=65, FKB=0, BKB=65, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
            }
        }
        frame(Frame=8)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(15.0, 0.0)
        }
        frame(Frame=9)
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
pub fn jack_upair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=1.0, Angle=80, KBG=80, FKB=15, BKB=20, Size=6.0, X=4.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=1.0, Angle=367, KBG=100, FKB=15, BKB=30, Size=6.0, X=4.7, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=4, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=20.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=20.0, Unk=false);
        }
        frame(Frame=19)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=20)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
            {
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {
					ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=80, KBG=125, FKB=0, BKB=50, Size=8.0, X=4.5, Y=0.0, Z=0.0, X2=1.5, Y2=0.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
					ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=116, FKB=0, BKB=50, Size=8.0, X=0.0, Y=20.0, Z=1.5, X2=0.0, Y2=25.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
                else 
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=4.0, Angle=80, KBG=125, FKB=0, BKB=50, Size=8.0, X=4.5, Y=0.0, Z=0.0, X2=1.5, Y2=0.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
					ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=116, FKB=0, BKB=50, Size=8.0, X=0.0, Y=20.0, Z=1.5, X2=0.0, Y2=25.0, Z2=0.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)    
				}
			}
            else 
            {
	           ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=3.0, Angle=80, KBG=160, FKB=0, BKB=50, Size=6.0, X=4.8, Y=0.0, Z=0.0, X2=2.5, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
        }
        frame(Frame=24)
        if(is_excute)
        {
            AttackModule::clear_all()
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
pub fn jack_dair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            MotionModule::set_rate(1.05)
        }
        frame(Frame=3)
        if(is_excute) 
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_excute)         
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=60, FKB=80, BKB=40, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=60, FKB=80, BKB=40, Size=3.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
                }   
                else
                {
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=60, FKB=80, BKB=40, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=60, FKB=80, BKB=40, Size=3.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
                }
            }
            else
            {
                ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=100, FKB=0, BKB=40, Size=3.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=100, FKB=0, BKB=40, Size=4.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            }   
        }
        frame(Frame=15)
        if(is_excute) 
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true) 
            {
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK)) 
                {
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=270, KBG=96, FKB=0, BKB=35, Size=8.0, X=0.0, Y=4.0, Z=1.0, X2=0.0, Y2=-4.0, Z2=1.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=50, FKB=0, BKB=50, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=50, FKB=0, BKB=50, Size=3.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2.0)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
                    AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
                }
                else
                {
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=8.0, Angle=270, KBG=96, FKB=0, BKB=35, Size=8.0, X=0.0, Y=4.0, Z=1.0, X2=0.0, Y2=-4.0, Z2=1.0, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=5, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=50, FKB=0, BKB=50, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=38, KBG=50, FKB=0, BKB=50, Size=3.0, X=0.0, Y=2.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=3, ShieldstunMul=2.0)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=2, ShieldstunMul=1.5)
                    ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.5)
                    AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
                }
            }
        }
        frame(Frame=16)
        if(is_excute)
        {
            AttackModule::clear(ID=4, false)
        }
        frame(Frame=19)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        rust 
        {
            println!("JackAttackAirLw");
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
pub fn jack_nair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=5)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=12)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {          
                    ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=3, Bone=hash40("throw"), Damage=4.0, Angle=46, KBG=139, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=3, Bone=hash40("throw"), Damage=4.0, Angle=46, KBG=139, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)            
                    AttackModule::set_add_reaction_frame(ID=0, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=1, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=2, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=3, Frames=1.0, Unk=false);
                }
                else
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=3, Bone=hash40("throw"), Damage=4.0, Angle=46, KBG=139, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=3, Bone=hash40("throw"), Damage=4.0, Angle=46, KBG=139, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_PUNCH)            
                    AttackModule::set_add_reaction_frame(ID=0, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=1, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=2, Frames=1.0, Unk=false);
                    AttackModule::set_add_reaction_frame(ID=3, Frames=1.0, Unk=false);
                }
            }
            else       
            {  
                ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=7.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                //ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=8.0, Angle=46, KBG=93, FKB=0, BKB=35, Size=4.0, X=0.0, Y=2.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(ID=0, Frames=1.5, Unk=false);
                //AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            }
        }            
        frame(Frame=30)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        rust 
        {
            println!("JackAttackAirN");
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
pub fn jack_bair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute)
        {   
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }  
                else
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }            
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=0.0, Y=3.3, Z=-6.0, X2=0.0, Y2=10.7, Z2=-8.4, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=0.0, Y=4.7, Z=-11.1, X2=0.0, Y2=11.1, Z2=-14.2, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=4.0, X=0.0, Y=4.0, Z=-3.0, X2=0.0, Y2=8.5, Z2=-5.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            } 
        }
        frame(Frame=8)
        if(is_excute)
        {   
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {   
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
                else
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=-1.0, Y=2.5, Z=1.0, X2=-3.0, Y2=-0.5, Z2=8.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=0.0, Y=-3.0, Z=2.5, X2=-4.5, Y2=-6.0, Z2=10.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=4.0, X=0.0, Y=4.5, Z=-6.5, X2=0.0, Y2=7.5, Z2=-5.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        frame(Frame=9)
        if(is_excute)
        {   
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {   
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
                else
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=35, KBG=72, FKB=0, BKB=51, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=7.5, Z=-10.0, X2=0.0, Y2=5.5, Z2=-8.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=3, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=94, FKB=0, BKB=58, Size=6.0, X=0.0, Y=10.5, Z=-12.5, X2=0.0, Y2=15.0, Z2=-12.5, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=4, Part=1, Bone=hash40("top"), Damage=9.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                    //ATTACK(ID=5, Part=2, Bone=hash40("top"), Damage=13.0, Angle=35, KBG=98, FKB=0, BKB=76, Size=6.0, X=0.0, Y=16.5, Z=-12.0, X2=0.0, Y2=20.5, Z2=-9.0, Hitlag=1.4, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=-1.0, Y=2.5, Z=1.0, X2=-3.0, Y2=-0.5, Z2=8.0, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("knife"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=3.5, X=0.0, Y=-3.0, Z=2.5, X2=-4.5, Y2=-6.0, Z2=10.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=9.0, Angle=43, KBG=90, FKB=0, BKB=55, Size=4.0, X=0.0, Y=4.5, Z=-6.5, X2=0.0, Y2=7.5, Z2=-5.5, Hitlag=0.8, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
        }
        wait(Frames=1)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=30)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
pub fn jack_fair(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=4)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=7)
        if(is_excute)
        {
            // ATTACK(ID=2) is the Akechi hitbox
            ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.0, X=3.2, Y=0.0, Z=0.0, X2=3.2, Y2=-0.5, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.0, Angle=78, KBG=50, FKB=0, BKB=73, Size=5.0, X=3.2, Y=0.0, Z=0.0, X2=3.2, Y2=0.0, Z2=1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=367, KBG=5, FKB=0, BKB=70, Size=5.0, X=4.2, Y=-0.7, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=80, KBG=50, FKB=0, BKB=73, Size=5.0, X=4.2, Y=-0.7, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=2.0, Angle=367, KBG=25, FKB=0, BKB=70, Size=5.0, X=0.0, Y=8.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=2.0, Angle=78, KBG=50, FKB=0, BKB=73, Size=5.0, X=0.0, Y=8.5, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=4, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=5, Frames=3.0, Unk=false);
        }
        frame(Frame=8)
        if(is_excute)
        {
            AttackModule::clear(ID=4, false)
            AttackModule::clear(ID=5, false)
        }
        frame(Frame=9)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=12)
        if(is_excute)
        {   
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {   
                if (ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK))
                {   
                    AttackModule::clear(ID=1, false)
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=118, FKB=0, BKB=40, Size=7.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=90, FKB=0, BKB=50, Size=7.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=90, FKB=0, BKB=50, Size=8.0, X=0.0, Y=18.0, Z=5.0, X2=0.0, Y2=17.0, Z2=6.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                else
                {
                    AttackModule::clear(ID=1, false)
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=118, FKB=0, BKB=40, Size=7.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.9, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=90, FKB=0, BKB=50, Size=7.0, X=0.0, Y=13.5, Z=5.0, X2=0.0, Y2=13.5, Z2=7.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=1, Bone=hash40("top"), Damage=8.0, Angle=34, KBG=90, FKB=0, BKB=50, Size=8.0, X=0.0, Y=18.0, Z=5.0, X2=0.0, Y2=17.0, Z2=6.5, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }   
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=5.0, Angle=361, KBG=125, FKB=0, BKB=46, Size=5.0, X=4.4, Y=-0.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("legl"), Damage=5.0, Angle=361, KBG=125, FKB=0, BKB=46, Size=5.0, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
        }
        wait(Frames=4)
        if(is_excute)
        {
            AttackModule::clear_all()
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
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_lw",
    animcmd = "game_speciallw")]
pub fn jack_speciallw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_INVINCIBLE), false)
        }
        frame(Frame=3)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_INVINCIBLE), false)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
        frame(Frame=11)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_NORMAL), false);
        }
        frame(Frame=36)
        if(is_excute)
        {
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
pub fn jack_specialairlw(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_INVINCIBLE), false)
        }
        frame(Frame=3)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_INVINCIBLE), false)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_ON, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
        frame(Frame=11)
        if(is_excute)
        {
            HitModule::set_status_all(app::HitStatus(*HIT_STATUS_NORMAL), false)
        }
        frame(Frame=36)
        if(is_excute)
        {
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_SHIELD, 0, FIGHTER_JACK_SHIELD_GROUP_KIND_SPECIAL_LW)
            sv_module_access::shield(MA_MSC_CMD_SHIELD_OFF, COLLISION_KIND_REFLECTOR, FIGHTER_JACK_REFLECTOR_KIND_SPECIAL_LW, FIGHTER_REFLECTOR_GROUP_EXTEND)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_lw_counter",
    animcmd = "game_speciallwcounter")]
pub fn jack_speciallwcounter(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_INVINCIBLE)
        }
        frame(Frame=5)
        if(is_excute)
        {
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=50, FKB=0, BKB=80, Size=25.0, X=0.0, Y=10.5, Z=-5.0, X2=0.0, Y2=10.5, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_force_reaction(1, true, false)
        }
        frame(Frame=10)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_lw_counter",
    animcmd = "game_specialairlwcounter")]
pub fn jack_specialairlwcounter(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_INVINCIBLE)
        }
        frame(Frame=5)
        if(is_excute)
        {
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=12.0, Angle=361, KBG=50, FKB=0, BKB=80, Size=25.0, X=0.0, Y=10.5, Z=-5.0, X2=0.0, Y2=10.5, Z2=10.0, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_ENERGY)
            AttackModule::set_force_reaction(0, true, false)
            AttackModule::set_force_reaction(1, true, false)
        }
        frame(Frame=10)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_NORMAL)
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_lw_attack",
    animcmd = "game_speciallwattack")]
pub fn jack_speciallwattack(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=8)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=50, KBG=100, FKB=82, BKB=0, Size=15.0, X=0.0, Y=10.0, Z=-3.0, X2=0.0, Y2=10.0, Z2=3.0, Hitlag=1.25, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=4)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_NORMAL)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_lw_attack",
    animcmd = "game_specialairlwattack")]
pub fn jack_specialairlwattack(fighter: &mut L2CFighterCommon) {
    acmd!({
        frame(Frame=1)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_XLU)
        }
        frame(Frame=8)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=50, KBG=100, FKB=82, BKB=0, Size=15.0, X=0.0, Y=10.0, Z=-3.0, X2=0.0, Y2=10.0, Z2=3.0, Hitlag=1.25, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=1.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_NONE)
        }
        wait(Frames=4)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=15)
        if(is_excute)
        {
            WHOLE_HIT(HIT_STATUS_NORMAL)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
pub fn jack_uptilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        if(is_excute)
        {
            MotionModule::set_rate(1.5)
        }
        frame(Frame=9)
        if(is_excute)
        {
            MotionModule::set_rate(1.0)
        }
        frame(Frame=10)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=120, KBG=100, FKB=150, BKB=0, Size=4.0, X=0.0, Y=4.0, Z=4.0, X2=0.0, Y2=8.0, Z2=7.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=105, KBG=100, FKB=150, BKB=0, Size=4.0, X=-1.0, Y=0.0, Z=0.0, X2=1.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=4.0, Angle=110, KBG=100, FKB=60, BKB=0, Size=4.0, X=-1.0, Y=0.0, Z=0.0, X2=1.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=110, KBG=100, FKB=165, BKB=0, Size=3.5, X=0.0, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=115, KBG=100, FKB=60, BKB=0, Size=3.5, X=0.0, Y=1.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=125, KBG=100, FKB=165, BKB=0, Size=3.5, X=0.0, Y=3.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=6, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=115, KBG=100, FKB=60, BKB=0, Size=3.5, X=0.0, Y=3.3, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        frame(Frame=12)
        if(is_excute)
        {
            AttackModule::clear(ID=0, false)
            AttackModule::clear(ID=1, false)
            AttackModule::clear(ID=2, false)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=4.0, Angle=130, KBG=100, FKB=20, BKB=0, Size=4.0, X=0.0, Y=23.0, Z=1.0, X2=0.0, Y2=20.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=4, Part=0, Bone=hash40("top"), Damage=4.0, Angle=367, KBG=100, FKB=20, BKB=0, Size=3.0, X=0.0, Y=23.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=5, Part=0, Bone=hash40("top"), Damage=4.0, Angle=135, KBG=100, FKB=0, BKB=0, Size=4.0, X=0.0, Y=13.0, Z=7.5, X2=0.0, Y2=18.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::clear(ID=6, false)
        }
        frame(Frame=13)
        if(is_excute)
        {
            ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=1.0, Angle=367, KBG=20, FKB=0, BKB=20, Size=6.0, X=0.0, Y=1.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            AttackModule::set_add_reaction_frame(ID=3, Frames=1.0, Unk=false);
            AttackModule::clear(ID=4, false)
            AttackModule::clear(ID=5, false)
        }
        frame(Frame=14)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {   
                ATTACK(ID=0, Part=1, Bone=hash40("top"), Damage=0.7, Angle=367, KBG=20, FKB=0, BKB=50, Size=8.0, X=0.0, Y=25.0, Z=-1.0, X2=0.0, Y2=25.0, Z2=1.0, Hitlag=0.3, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=1.0, Angle=367, KBG=20, FKB=0, BKB=20, Size=6.0, X=0.0, Y=1.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.3, SDI=0.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::set_add_reaction_frame(ID=0, Frames=1.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=3, Frames=1.0, Unk=false);
            }
        }
        frame(Frame=23)
        if(is_excute)
        {    
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {   
                ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=4.0, Angle=90, KBG=200, FKB=0, BKB=30, Size=6.0, X=0.0, Y=1.6, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_ELEC, Type=ATTACK_REGION_SWORD)
                AttackModule::clear(ID=0, false)
            }
            else
            {
                ATTACK(ID=3, Part=0, Bone=hash40("knife"), Damage=1.0, Angle=90, KBG=270, FKB=0, BKB=35, Size=6.0, X=0.0, Y=1.7, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                AttackModule::clear(ID=0, false)
            }
        }
        frame(Frame=28)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_dash",
    animcmd = "game_attackdash")]
pub fn jack_dashattack(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=6)
        if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=35, KBG=95, FKB=70, BKB=0, Size=4.0, X=0.0, Y=4.0, Z=4.0, X2=3.0, Y2=9.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=40, KBG=95, FKB=80, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=50, KBG=95, FKB=50, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=2.0, Angle=25, KBG=95, FKB=40, BKB=0, Size=4.0, X=0.0, Y=4.0, Z=4.0, X2=3.0, Y2=9.0, Z2=5.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("kneer"), Damage=2.0, Angle=10, KBG=95, FKB=30, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=2.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=15, KBG=95, FKB=25, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=2, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=3, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=4, Frames=3.0, Unk=false);
            AttackModule::set_add_reaction_frame(ID=5, Frames=3.0, Unk=false);
        }
        wait(Frames=2)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=14)
        if(is_excute)
        {
            FighterAreaModuleImpl::enable_fix_jostle_area(3.5, 4.0)
        }
        frame(Frame=15)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=8.0, Z=7.0, X2=0.0, Y2=8.0, Z2=12.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=8.0, Z=7.0, X2=0.0, Y2=8.0, Z2=12.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.75)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.75)
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.0, Y=0.0, Z=0.0, X2=5.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
        }
        frame(Frame=17)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=10.0, Z=7.0, X2=0.0, Y2=10.0, Z2=13.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=10.0, Z=7.0, X2=0.0, Y2=10.0, Z2=13.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.75)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.75)
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.0, Y=0.0, Z=0.0, X2=5.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)           
            }
        }
        frame(Frame=19)
        if(is_excute)
        { 
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=11.0, Z=7.0, X2=0.0, Y2=11.0, Z2=11.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=361, KBG=70, FKB=0, BKB=80, Size=6.0, X=0.0, Y=11.0, Z=7.0, X2=0.0, Y2=11.0, Z2=11.0, Hitlag=1.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.75)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.75)
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.0, Y=0.0, Z=0.0, X2=5.0, Y2=0.0, Z2=0.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("footl"), Damage=6.0, Angle=361, KBG=85, FKB=0, BKB=60, Size=3.8, X=0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
        }
        frame(Frame=25)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
pub fn jack_dtilt(fighter: &mut L2CFighterCommon) {
    acmd!
    ({
        frame(Frame=1)
        if(is_excute)
        {
            MotionModule::set_rate(1.12)
        }
        frame(Frame=7)
        if(is_excute)
        {
            JostleModule::set_status(false)
        }
        frame(Frame=8)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=80, KBG=61, FKB=0, BKB=55, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=80, KBG=91, FKB=0, BKB=70, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.3)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.3)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=80, KBG=63, FKB=0, BKB=75, Size=3.6, X=-4.0, Y=0.0, Z=1.5, X2=5.0, Y2=0.0, Z2=1.5, Hitlag=0.75, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
                AttackModule::set_attack_height_all(smash::app::AttackHeight(*ATTACK_HEIGHT_LOW), false)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            }
        }
        frame(Frame=10)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=82, KBG=61, FKB=0, BKB=55, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=82, KBG=91, FKB=0, BKB=70, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.3)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.3)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=82, KBG=63, FKB=0, BKB=75, Size=3.6, X=-4.0, Y=0.0, Z=1.5, X2=5.0, Y2=0.0, Z2=1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            }
        }
        frame(Frame=12)
        if(is_excute)
        {
            if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
            {
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=85, KBG=61, FKB=0, BKB=55, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=1, Bone=hash40("top"), Damage=8.0, Angle=85, KBG=91, FKB=0, BKB=70, Size=4.5, X=0.0, Y=3.2, Z=0.0, X2=0.0, Y2=3.2, Z2=8.5, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.3)
                ATK_SET_SHIELD_SETOFF_MUL(ID=1, ShieldstunMul=1.3)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
                AttackModule::set_add_reaction_frame(ID=1, Frames=3.0, Unk=false);
            }
            else
            {
                ATTACK(ID=0, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=85, KBG=63, FKB=0, BKB=75, Size=3.6, X=-4.0, Y=0.0, Z=1.5, X2=5.0, Y2=0.0, Z2=1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=0.5)
                AttackModule::set_add_reaction_frame(ID=0, Frames=3.0, Unk=false);
            }
        }
        frame(Frame=18)
        if(is_excute)
        {
            MotionModule::set_rate(1.4)
        }
        frame(Frame=20)
        if(is_excute)
        {
            AttackModule::clear_all()
            JostleModule::set_status(true)
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_n_barrage_lw",
    animcmd = "game_specialairndown")]
pub fn jack_specialairndown(fighter: &mut L2CFighterCommon) {
    acmd! 
    ({
        frame(Frame=1)
        for(20 Iterations)
        {
            if(is_excute)
            {
                if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=270, KBG=80, FKB=0, BKB=20, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-32.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.2, Angle=45, KBG=0, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=-10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.2, Angle=55, KBG=0, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.2, Angle=45, KBG=0, FKB=0, BKB=30, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 5, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(3, 5, 4)
                    AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false);
                    ControlModule::set_rumble(Hash40::new_raw(0x10f5122f08), 2, false, false)
                }
                else
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=50, KBG=80, FKB=0, BKB=25, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-25.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=1.2, Angle=45, KBG=0, FKB=0, BKB=29, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-50.0, Z2=-10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=1.2, Angle=55, KBG=0, FKB=0, BKB=29, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-50.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=1.2, Angle=45, KBG=0, FKB=0, BKB=29, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-50.0, Z2=10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 5, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(3, 5, 4)
                    AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false);
                    ControlModule::set_rumble(Hash40::new_raw(0x10f5122f08), 2, false, false)
                }
            }
            wait(Frames=1)
            if(is_excute)
            {
                AttackModule::clear_all()
            }
            wait(Frames=3)
            if(is_excute)
            {
                if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.6, Angle=270, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-32.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.8, Angle=45, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=-10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.8, Angle=55, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.8, Angle=45, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 5, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(3, 5, 4)
                    AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false);
                    ControlModule::set_rumble(Hash40::new_raw(0x10f5122f08), 2, false, false)
                }
            }
            wait(Frames=1)
            if(is_excute)
            {
                AttackModule::clear_all()
            }
            wait(Frames=3)
            if(is_excute)
            {
                if (WorkModule::is_flag(module_accessor,*FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE)==true)
                {
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.4, Angle=270, KBG=0, FKB=0, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-32.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.2, Angle=45, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=-10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.2, Angle=55, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=0.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.2, Angle=45, KBG=0, FKB=0, BKB=0, Size=6.0, X=0.0, Y=0.0, Z=0.0, X2=0.0, Y2=-60.0, Z2=10.0, Hitlag=0.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_jack_bullet"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_JACK_SHOT, Type=ATTACK_REGION_OBJECT)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(0, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(1, 5, 4)
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(2, 5, 4, hash40("jack_gun_hit2"), hash40("se_jack_special_n02"))
                    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(3, 5, 4)
                    AttackModule::set_add_reaction_frame(ID=0, Frames=2.0, Unk=false);
                    ControlModule::set_rumble(Hash40::new_raw(0x10f5122f08), 2, false, false)
                }
            }
            wait(Frames=1)
            if(is_excute)
            {
                AttackModule::clear_all()
            }
            wait(Frames=3)
        }
    });     
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_hi_throw",
    animcmd = "game_specialhithrow")]
pub fn jack_specialhigrab(fighter: &mut L2CFighterCommon) {
    acmd! 
    ({
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0)
            ArticleModule::change_motion(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40{hash:hash40("special_hi_throw")}, false, 0.0);
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=80, KBG=40, FKB=0, BKB=75, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=4)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(-20, 10)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
        }
        frame(Frame=6)
        if(is_excute)
        {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
        frame(Frame=8)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, smash::app::ArticleOperationTarget(0))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_hi_throw",
    animcmd = "game_specialairhithrow")]
pub fn jack_specialairhigrab(fighter: &mut L2CFighterCommon) {
    acmd! 
    ({
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0)
            ArticleModule::change_motion(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40{hash:hash40("special_hi_throw")}, false, 0.0);
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=12.0, Angle=80, KBG=40, FKB=0, BKB=75, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
        }
        frame(Frame=4)
        if(is_excute)
        {
            CHECK_FINISH_CAMERA(-20, 10)
            //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
        }
        frame(Frame=6)
        if(is_excute)
        {
            ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), 
            WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
        }
        frame(Frame=8)
        if(is_excute)
        {
            ArticleModule::remove_exist(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, smash::app::ArticleOperationTarget(0))
        }
    });
}

#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER, 
    battle_object_kind = FIGHTER_KIND_JACK, 
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
pub fn jack_specialairhihit(fighter: &mut L2CFighterCommon) {
    acmd! 
    ({
        if(is_excute)
        {
            ArticleModule::generate_article(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0)
        }
        frame(Frame=8)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR)
        }
        frame(Frame=12)
        if(is_excute)
        {
            WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
            WorkModule::on_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE)
        }
        frame(Frame=13)
        if(is_excute)
        {
            GroundModule::select_cliff_hangdata(FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO)
            WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE)
        }
        frame(Frame=15)
        if(is_excute)
        {
            WorkModule::off_flag(Flag=FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK)
        }
        frame(Frame=19)
        if(is_excute)
        {
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(0))
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            sv_animcmd::ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
            AreaModule::reset_area(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
            sv_animcmd::ENABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
            frame(Frame=20)
            if(is_excute)
        {
            ATTACK(ID=0, Part=0, Bone=hash40("throw"), Damage=5.0, Angle=60, KBG=56, FKB=0, BKB=76, Size=5.5, X=1.5, Y=2.0, Z=2.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
                
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=60, KBG=56, FKB=0, BKB=76, Size=5.5, X=0.0, Y=22.0, Z=3.2, X2=0.0, Y2=38.0, Z2=10.6, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, 
            Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_OBJECT)
        }
        frame(Frame=21)
        if(is_excute)
        {
            AttackModule::clear(ID=1, false)
        }
        frame(Frame=23)
        if(is_excute)
        {
            sv_animcmd::UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD)
        }
        frame(Frame=24)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=25)
        if(is_excute)
        {
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
        }
        frame(Frame=26)
        if(is_excute)
        {
            AttackModule::clear_all()
        }
        frame(Frame=27)
        if(is_excute)
        {
            ArticleModule::change_status(FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(0))
            sv_module_access::search(MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL)
            sv_animcmd::UNABLE_AREA(FIGHTER_JACK_AREA_KIND_ITEM_CATCH)
        }
    });
}

pub fn install() {
    acmd::add_hooks!(
        jack_catch,
        jack_catchdash,
        jack_catchturn,
        jack_throwlw,
        jack_throwhi,
        jack_upair,
        jack_dair,
        jack_nair,
        jack_bair,
        jack_fair,
        jack_speciallw,
        jack_specialairlw,
        jack_speciallwcounter,
        jack_specialairlwcounter,
        jack_speciallwattack,
        jack_specialairlwattack,
        jack_throwb,
        jack_throwf,
        jack_uptilt,
        jack_dashattack,
        jack_dtilt,
        jack_specialairndown,
        jack_specialhigrab,
        jack_specialairhigrab,
        jack_specialairhihit
    );
}   