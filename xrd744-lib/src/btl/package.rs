use allocator_api2::alloc::Allocator;
use bitflags::bitflags;
use opengfd::{
    gfw::{
        list::List,
        smartpointer::SmartPointer,
    },
    kernel::{
        allocator::GfdAllocator,
        task::Task as GfdTask
    }
};
use std::{
    ops::BitOrAssign,
    ptr::NonNull
};
use crate::btl::frequency::Frequency;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PackageState0 : u32 {
        const RUNNING = 1 << 0;
        const FIGHTING = 1 << 1;
        const LOADED_STAGE = 1 << 2;
        const LOADED_UNIT = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const GALLICA_TALK = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
        const FLAG16 = 1 << 16;
        const SWITCH_FORMATION = 1 << 17;
        const SWITCH_INTIMIDATION_FORMATION = 1 << 18;
        const SWITCH_PINCH_FORMATION = 1 << 19;
        const SWITCH_NORMAL_FORMATION = 1 << 20;
        const SWITCH_ADVANTAGEOUS_FORMATION = 1 << 21;
        const SWITCH_HOLD_UP_FORMATION = 1 << 22;
        const SWITCH_SUPPORT_TAKARA = 1 << 23;
        const INTERRUPTION = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const FLAG30 = 1 << 30;
        const FLAG31 = 1 << 31;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PackageState1 : u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
        const FLAG3 = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const IS_BOSS = 1 << 12;
        const FLAG13 = 1 << 13;
        const TIMEOVER_TERMINATION = 1 << 14;
        const FORCE_TERMINATION = 1 << 15;
        const FLAG16 = 1 << 16;
        const FLAG17 = 1 << 17;
        const FLAG18 = 1 << 18;
        const FLAG19 = 1 << 19;
        const FLAG20 = 1 << 20;
        const FLAG21 = 1 << 21;
        const FLAG22 = 1 << 22;
        const FLAG23 = 1 << 23;
        const FLAG24 = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const FLAG30 = 1 << 30;
        const FLAG31 = 1 << 31;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PackageState2 : u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
        const FLAG3 = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const OVERTURN = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
        const FLAG16 = 1 << 16;
        const FLAG17 = 1 << 17;
        const FLAG18 = 1 << 18;
        const FLAG19 = 1 << 19;
        const FLAG20 = 1 << 20;
        const FLAG21 = 1 << 21;
        const FLAG22 = 1 << 22;
        const FLAG23 = 1 << 23;
        const FLAG24 = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const FLAG30 = 1 << 30;
        const FLAG31 = 1 << 31;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct PackageState3 : u32 {
        const FLAG0 = 1 << 0;
        const FLAG1 = 1 << 1;
        const FLAG2 = 1 << 2;
        const FLAG3 = 1 << 3;
        const FLAG4 = 1 << 4;
        const FLAG5 = 1 << 5;
        const FLAG6 = 1 << 6;
        const FLAG7 = 1 << 7;
        const FLAG8 = 1 << 8;
        const FLAG9 = 1 << 9;
        const FLAG10 = 1 << 10;
        const FLAG11 = 1 << 11;
        const FLAG12 = 1 << 12;
        const FLAG13 = 1 << 13;
        const FLAG14 = 1 << 14;
        const FLAG15 = 1 << 15;
        const FLAG16 = 1 << 16;
        const FLAG17 = 1 << 17;
        const FLAG18 = 1 << 18;
        const FLAG19 = 1 << 19;
        const FLAG20 = 1 << 20;
        const FLAG21 = 1 << 21;
        const FLAG22 = 1 << 22;
        const FLAG23 = 1 << 23;
        const FLAG24 = 1 << 24;
        const FLAG25 = 1 << 25;
        const FLAG26 = 1 << 26;
        const FLAG27 = 1 << 27;
        const FLAG28 = 1 << 28;
        const FLAG29 = 1 << 29;
        const FLAG30 = 1 << 30;
        const FLAG31 = 1 << 31;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct PackageFlags {
    flag0: PackageState0,
    flag1: PackageState1,
    flag2: PackageState2,
    flag3: PackageState3,
}

impl BitOrAssign<PackageState0> for PackageFlags {
    fn bitor_assign(&mut self, rhs: PackageState0) {
        self.flag0 |= rhs;
    }
}

impl BitOrAssign<PackageState1> for PackageFlags {
    fn bitor_assign(&mut self, rhs: PackageState1) {
        self.flag1 |= rhs;
    }
}

impl BitOrAssign<PackageState2> for PackageFlags {
    fn bitor_assign(&mut self, rhs: PackageState2) {
        self.flag2 |= rhs;
    }
}

impl BitOrAssign<PackageState3> for PackageFlags {
    fn bitor_assign(&mut self, rhs: PackageState3) {
        self.flag3 |= rhs;
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum EncountType {
    #[default]
    Normal = 0,
    Event,
    Event2,
    Type3
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Preemptive {
    #[default]
    Normal = 0,
    Pinch,
    Chance,
    HoldUp
}

#[repr(C)]
#[derive(Debug)]
pub struct CompensationData<A = GfdAllocator>
where A: Allocator + Clone
{
    experience: u32,
    money: u32,
    extra_money: u32,
    fieldc: [u8; 0x1c],
    drop_item: List<usize, A>,
    talk_item: List<usize, A>,
    item3: List<usize, A>,
    max_level_of_enemy_killed: u32,
    total_level_of_enemy_killed: u32,
    number_of_enemy_killed: u32,
    field94: u16
}

#[repr(C)]
pub struct Package<A = GfdAllocator>
where A: Allocator + Clone
{
    _cpp_vtable: *const u8,
    scene: SmartPointer<usize, A>,
    camera: SmartPointer<usize, A>,
    phase: SmartPointer<usize, A>,
    order: SmartPointer<usize, A>,
    formation: SmartPointer<usize, A>,
    route: SmartPointer<usize, A>,
    camera_ctrl: SmartPointer<usize, A>,
    frequency: SmartPointer<Frequency, A>,
    player_list: List<SmartPointer<usize, A>, A>,
    enemy_list: List<SmartPointer<usize, A>, A>,
    action_list: List<SmartPointer<usize, A>, A>,
    voice_list: List<SmartPointer<usize, A>, A>,
    misc_units2: List<SmartPointer<usize, A>, A>,
    misc_units3: List<SmartPointer<usize, A>, A>,
    persona_effects: List<SmartPointer<usize, A>, A>,
    skill_timeline: List<SmartPointer<usize, A>, A>,
    object_list: List<SmartPointer<usize, A>, A>,
    event_list: List<SmartPointer<usize, A>, A>,
    misc_units4: List<SmartPointer<usize, A>, A>,
    flags: PackageFlags,
    encount_num: u32,
    encount_type: EncountType,
    field_major: u16,
    field_minor: u16,
    env_major: u16,
    env_minor: u16,
    env_sub: u16,
    preemptive: Preemptive,
    talk_money: u32,
    talk_item: u32,
    challenge_btl_298: [u32; 2],
    chanllenge_btl_2a0: u8,
    chanllenge_btl_2a1: u8,
    deltatime: f32,
    elapsed_time: f32,
    elapsed_real_time: f32,
    elapsed_turn: u32,
    elapsed_round: u32,
    field2b8: u32,
    ally_turns2: u32,
    enemy_turn: u32,
    ally_turns: u32,
    treasure_value: u32,
    unit_substitute_count: u16,
    unit_substitute_id: u16,
    is_unit_unstable: [u8; 5],
    resident_data: SmartPointer<usize, A>,
    screen_fade: SmartPointer<usize, A>,
    gui: SmartPointer<usize, A>,
    info_support: SmartPointer<usize, A>,
    script_cmd: SmartPointer<usize, A>,
    compensation: CompensationData,
    field410: u32,
    field414: u32,
    reinforcement_count: u32,
    blast_off_count: u16,
    jyokyo_help: SmartPointer<usize, A>,
    alert_effect_task: *mut u8,
    alert_effect_fade: [u8; 0x28],
    button_push_effect: SmartPointer<usize, A>,
    slow_effect: SmartPointer<usize, A>,
    rush_effect: SmartPointer<usize, A>,
    pinch_fil_effect: SmartPointer<usize, A>,
    field4f0_effect: SmartPointer<usize, A>,
    current_formation_effect: u32,
    one_more_board: SmartPointer<usize, A>,
    one_more_word: SmartPointer<usize, A>,
    one_more_state: u32,
    one_more_float: f32,
    one_more_ready: bool,
    switch_command_actor: SmartPointer<usize, A>,
    switch_command_count: u32,
    switch_command_float: f32,
    switch_command_ready: bool,
    switch_command_pc_id: u16,
    switch_general_actor: SmartPointer<usize, A>,
    switch_general_attack_step: u32,
    switch_general_attack_float: f32,
    switch_pursuit_actor: SmartPointer<usize, A>,
    switch_pursuit_attack_step: u32,
    switch_pursuit_attack_float: f32,
    switch_pursuit_ready: bool,
    switch_sp_attack_actor: SmartPointer<usize, A>,
    switch_sp_attack_attack_step: u32,
    switch_sp_attack_attack_float: f32,
    switch_sp_attack_ready: bool,
    switch_sp_attack_reveal_frame: SmartPointer<usize, A>,
    switch_sp_attack_wipe_in: SmartPointer<usize, A>,
    switch_sp_attack_wipe_in_step: u32,
    switch_sp_attack_wipe_in_float: f32,
    switch_sp_attack_wipe_out: SmartPointer<usize, A>,
    switch_sp_attack_wipe_out_step: u32,
    switch_sp_attack_wipe_out_float: f32,
    effect690: SmartPointer<usize, A>,
    effect690_ready: bool,
    sp_attack_wipe_close_result_mesh: *mut u8,
    sp_attack_wipe_close_result_node: *mut u8,
    sp_attack_wipe_state: u32,
    field6cc: u32,
    field6d0: u32,
    field6d4: u8,
    command_tex_state: u32,
    command_tex_fade_limit: f32,
    command_tex_fade_time: f32,
    field6e4: u32,
    switch_rush: SmartPointer<usize, A>,
    switch_rush_step: u32,
    switch_rush_float: f32,
    switch_rush_ready: bool,
    switch_result: SmartPointer<usize, A>,
    switch_result_step: u32,
    switch_result_float: f32,
    loop_result: SmartPointer<usize, A>,
    loop_result_step: u32,
    loop_result_float: f32,
    camera_shale: u32,
    cam_shake_amount_hori: [u8; 0x24],
    cam_shake_amount_vert: [u8; 0x24],
    total_units_created: u32,
    globals: [u32; 0x20],
    global_events: [u32; 0x20],
    uid_exists_bit: u32,
    field_battle_result: u8,
    event_script: *mut u8,
    action1: SmartPointer<usize, A>,
    action2: SmartPointer<usize, A>,
    action3: SmartPointer<usize, A>,
    action4: SmartPointer<usize, A>,
    action5: SmartPointer<usize, A>,
    boss: SmartPointer<usize, A>,
    boss_event_cutscene: SmartPointer<usize, A>,
    boss_cutscene_speed: f32,
    bgm_id: u32,
    command_texture: SmartPointer<usize, A>,
    sp_attack_close_white_texture: SmartPointer<usize, A>,
    talk_loader: SmartPointer<usize, A>,
    fielda10: f32,
    fielda14: u32,
    party_panel_task: Option<NonNull<GfdTask<A>>>,
    fielda20: usize,
    enemy_talk_task: Option<NonNull<GfdTask<A>>>,
    mission_task: Option<NonNull<GfdTask<A>>>,
    fielda38: usize,
    fielda40: u32,
    summon_count: u16,
    fielda46: [u8; 25],
    fielda5f: [u8; 49],
    time_since_danger_up: f32,
    fielda94: u32,
    fielda98: *mut u8,
    language: u32,
    fieldaa4: u8,
    members_tactics_state: [u16; 10],
    pc_persona_id: u16,
    bgm_cue_id: u32,
    atom_player_start_time: u64,
    fieldac8: u64,
    field_reshnd: [usize; 10],
    available_members: u32,
    fieldb24: u32,
    morgana_haru_showtime_car_decal: *mut u8,
    showtime_partner_id1: u16,
    showtime_partner_id2: u16,
    fieldb34: f32,
    fieldb38: u32,
    fieldb3c: bool,
    fieldb40: u32,
    fieldb44: u32,
    unstable_epl: SmartPointer<usize, A>,
    unstable_epl_phase: u32,
    unstable_epl_opacity: f32,
    fieldb70: u32,
    fieldb74: u32,
    non_owned_persona_percent: u32,
    fieldb7c: u32,
    showtime_cutscenes: [SmartPointer<usize, A>; 11],
    init_showtime_cutscenes: bool,
    fieldce4: u32,
    fieldce8: u32,
    blast_end_type: u32,
    blast_end_effect: SmartPointer<usize, A>,
    _allocator: A
}

impl<A> Package<A>
where A: Allocator + Clone
{
    pub fn get_frequency(&self) -> Option<&Frequency> {
        self.frequency.get_data_checked()
    }
    pub fn get_frequency_mut(&mut self) -> Option<&mut Frequency> {
        self.frequency.get_data_checked_mut()
    }
}