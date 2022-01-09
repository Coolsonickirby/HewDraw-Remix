
use super::*;


#[acmd_script( agent = "gamewatch", script = "game_throwf" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 68, 40, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON);
        WorkModule::set_float(boma, 50.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, -110.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, 120.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, -30.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, 135.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, -70.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 14, 6);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    
}

#[acmd_script( agent = "gamewatch", script = "game_throwb" , category = ACMD_GAME , low_priority)]
unsafe fn game_throwb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 60, 40, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON);
        WorkModule::set_float(boma, -50.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, 110.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, -120.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, 30.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        REVERSE_LR(fighter);
        WorkModule::set_float(boma, 135.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        WorkModule::set_float(boma, -70.0, *FIGHTER_GAMEWATCH_STATUS_THROW_WORK_FLOAT_STOCK_ICON_ROTATE);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        CHECK_FINISH_CAMERA(fighter, 12, 3);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAMEWATCH_STATUS_THROW_FLAG_SET_STOCK_ICON);
        ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(boma, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        game_throwf,
        game_throwb,
    );
}
