/* BSD 3-Clause License

Copyright (c) 2026, Campbell Rowland

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.
  
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE. */



use std::path::PathBuf;

mod dfa_mode;
use iced::Element;
use iced::widget::{
   button, center_x, column, container, operation, row, tooltip};

const WINDOW: &str = "window";

#[derive(Debug, Clone, Copy)]
enum Message {
   DfaMode
}

enum EditorMode {
   Dfa {nodes: Vec<dfa_mode::Node>, edges: Vec<dfa_mode::Edge>},
   Nfa,
   Regex,
   Cfg,
   Empty,
}

pub fn initialise() -> iced::Result {
   iced::application(GraphicsInstance::new, GraphicsInstance::update, GraphicsInstance::view)
      .run()
}

pub struct GraphicsInstance{
   file: Option<PathBuf>,
}

impl GraphicsInstance{
   fn new() -> (Self, iced::Task<Message>){
      (Self{
         file: None
      },
      operation::focus(WINDOW),)
   }

   fn view(& self) -> Element<'_, Message>{
      let toolbar = row![
         toolbar_button("DFA Creation", "DFA Creation mode", Some(Message::DfaMode)),];
      column![toolbar].into() //TODO: add DFAWindow
   }

   fn update(&mut self, message: Message) -> iced::Task<Message>{
      match message{
         Message::DfaMode => {
            log::debug!("DFA Creation mode entered");
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
      let but = button(center_x(content).width(25));

      if let Some(on_press) = on_press{
         tooltip(but.on_press(on_press), label, tooltip::Position::FollowCursor,)
         .style(container::bordered_box).into()
      } else {
         but.style(button::secondary).into()
      }
}