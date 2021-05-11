#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 100;

mod mario;
mod falco;
mod jack;
mod sheik;
mod captain;
mod lucina;
mod cloud;
mod marth;
mod sonic;
mod ganon;
mod szerosuit;
mod edge;
mod inkling;
mod roy;
mod gekkouga;
mod wolf;
mod palutena;
mod link;
mod luigi;
mod kamui;
mod lucario;
mod master;
mod toonlink;
mod fox;
mod eflame;
mod elight;
mod pitb;
//mod custom;

#[skyline::main(name = "acmd_test")]
pub fn main() 
{
    mario::install();
    falco::install();
    jack::install();
    sheik::install();
    captain::install();
    lucina::install();
    cloud::install();
    marth::install();
    sonic::install();
    ganon::install();
    szerosuit::install();
    edge::install();
    inkling::install();
    roy::install();
    gekkouga::install();
    wolf::install();
    palutena::install();
    link::install();
    luigi::install();
    kamui::install();
    lucario::install();
    master::install();
    toonlink::install();
    fox::install();
    eflame::install();
    elight::install();
    pitb::install();
    //custom::install();
}