/*  LangViewer - A tool for visualising languages, grammars, and automata.
Copyright (C) 2026 Campbell Rowland>

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */


use std::path::PathBuf;

/// This module is designed to handle the graphical implementation of dfa's
mod dfa_mode;

mod connection;
use iced::Element;
use iced::wgpu::naga::FastHashMap;
use iced::widget::{
   button, center_x, column, container, operation, row, tooltip};
use rstar::RTree;

const WINDOW: &str = "window";

#[derive(Debug, Clone, Copy)]
enum Message {
   DfaMode,
   DfaMessage(dfa_mode::Message),
}

/// What is currently being displayed for the user to edit
enum EditorMode {
   Dfa {dfa_win: dfa_mode::DfaWindow},
   Nfa,
   Regex,
   Cfg,
   Empty,
}

pub fn initialise() -> iced::Result {
   iced::application(GraphicsInstance::new, GraphicsInstance::update, GraphicsInstance::view)
      .run()
}

/// The graphical instance of the application
pub struct GraphicsInstance{
   file: Option<PathBuf>,
   mode: EditorMode,
}

impl GraphicsInstance{
   fn new() -> (Self, iced::Task<Message>){
      (Self{
         file: None,
         mode: EditorMode::Empty,
      },
      operation::focus(WINDOW),)
   }

   fn view(& self) -> Element<'_, Message>{
      let toolbar = row![
         toolbar_button("DFA Creation", "DFA Creation mode", Some(Message::DfaMode)),];
      let content: Element<Message> = match &self.mode {
         EditorMode::Dfa { dfa_win} => dfa_win.view().map(Message::DfaMessage),
         _ => iced::widget::text("").into(),
      };
      column![toolbar, content].into()
   }

   fn update(& mut self, message: Message) -> iced::Task<Message>{
      match message{
         Message::DfaMode => {
            log::debug!("DFA Creation mode entered");
            self.mode = EditorMode::Dfa { dfa_win: dfa_mode::DfaWindow { dfa: dfa_mode::DfaInstance { 
               nodes: RTree::new(), edges: Vec::new(), edge_index: FastHashMap::default() } } };
         }
         Message::DfaMessage(dfa_msg) => {
            if let EditorMode::Dfa {dfa_win} = & mut self.mode {
               dfa_win.dfa.update(dfa_msg);
            }
         }
      }
      iced::Task::none()
   }

}

fn toolbar_button<'a, Message:Clone + 'a>(
      content: impl Into<Element<'a, Message>>,
      label: &'a str,
      on_press: Option<Message>,
) -> Element<'a, Message>{
      let but = button(center_x(content).width(iced::Length::Fill))
         .padding(10)
         .width(iced::Length::Shrink);

      if let Some(on_press) = on_press{
         tooltip(but.on_press(on_press), label, tooltip::Position::FollowCursor,)
         .style(container::bordered_box).into()
      } else {
         but.style(button::secondary).into()
      }
}