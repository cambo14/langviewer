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

/// This module is designed to handle the graphical implementation of dfa's
mod dfa_mode;

/// Manages connections between nodes in the DFA, including their rendering and interaction logic
mod connection;


use iced::Element;
use iced::widget::{
   button, center_x, column, container, operation, row, tooltip};

/// An identifier for the main window of the application
const WINDOW: &str = "window";

/// The different Messages for the overarching graphical instance
#[derive(Debug, Clone, Copy)]
enum Message {
   /// Message to switch to DFA creation mode
   DfaMode,

   /// Messages specific to the DFA editor, wrapped in a variant to be handled by the main graphical instance
   DfaMessage(dfa_mode::Message),
}

/// What is currently being displayed for the user to edit
/// 
/// # TODO
/// 
/// * Add NFA, Regex, and CFG editor modes
enum EditorMode {
   /// A DFA is currently being edited
   Dfa {
      /// The window containing the DFA editor
      dfa_win: Box<dfa_mode::DfaWindow>,
   },
   /// An empty canvas with no editor active
   Empty,
}

/// Initialise the graphical instance of an application and run it
pub fn initialise() -> iced::Result {
   iced::application(GraphicsInstance::new, GraphicsInstance::update, GraphicsInstance::view)
      .run()
}

/// The graphical instance of the application
pub struct GraphicsInstance {
   /// What is currently being displayed for the user to edit, such as a DFA editor or an empty canvas
   mode: EditorMode,
}

impl GraphicsInstance {

   /// Generate a new graphical instance with an empty editor and focuses on it
   fn new() -> (Self, iced::Task<Message>){
      (Self{
         mode: EditorMode::Empty,
      },
      operation::focus(WINDOW),)
   }

   /// Implementation of [`ViewFn`](trait@iced::application::ViewFn) for the graphical instance
   /// generating the view based on the current editor mode
   fn view(& self) -> Element<'_, Message>{
      let toolbar = row![
         toolbar_button("DFA Creation", "DFA Creation mode", Some(Message::DfaMode)),];
      let content: Element<Message> = match &self.mode {
         EditorMode::Dfa { dfa_win} => dfa_win.view().map(Message::DfaMessage),
         _ => iced::widget::text("").into(),
      };
      column![toolbar, content].into()
   }

   /// Implementation of [`UpdateFn`](trait@iced::application::UpdateFn) for the graphical instance
   /// handling messages and updating the state of the application accordingly
   fn update(& mut self, message: Message) -> iced::Task<Message>{
      match message{
         Message::DfaMode => {
            log::debug!("DFA Creation mode entered");
            self.mode = EditorMode::Dfa {
               dfa_win: Box::new(dfa_mode::DfaWindow {
                  dfa: dfa_mode::DfaInstance::default(),
               }),
            };
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

/// A helper function to generate a toolbar button with a tooltip and optional on_press message
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