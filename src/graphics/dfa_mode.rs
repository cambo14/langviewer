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

use iced::mouse::{self};
use iced::widget::{canvas};
use iced::{Color, Rectangle, Renderer, Theme};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Message{
    AddNode {pos: iced::Point<f32>},
    DelNode,
    AddCon,
    RemCon
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DfaWindow<'a> {
   pub dfa: &'a DfaInstance,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DfaInstance {
   pub nodes: Vec<Node>,
   pub edges: Vec<Edge>,
}

impl<'a> canvas::Program<Message> for DfaWindow<'a> {
   fn draw(&self, _state: &Interaction, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
      let mut frame = canvas::Frame::new(renderer, bounds.size());

      let circle = canvas::Path::circle(frame.center(), 15.0);

      frame.fill(&circle, Color::BLACK);

      vec![frame.into_geometry()]
   }

   fn update(&self, interaction: &mut Interaction,
         event: &canvas::Event, _bounds: Rectangle,
         cursor: mouse::Cursor) -> Option<canvas::Action<Message>>
   {
      match event {
         canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => {
            let (exists, pos) = (cursor.position().is_some(),
               cursor.position().unwrap_or(iced::Point::default()));
            let message = {
               *interaction = if exists && is_node(pos) {
                  Interaction::DelNode
               } else {
                  Interaction::AddNode
               };
               Some(Message::AddNode {pos})
            };

            Some(message.map(canvas::Action::publish)
               .unwrap_or(canvas::Action::request_redraw()).and_capture(),)
        }
         _ => None
      }
   }
   
   type State = Interaction;
   

}

impl <'a>DfaWindow<'a> {
   pub fn view(self) -> iced::Element<'a, Message> {
      canvas::Canvas::new(self).width(iced::Fill).height(iced::Fill).into()
   }
}

pub fn is_node(_pos: iced::Point<f32>) -> bool {
   return false;
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Node {
   pos: iced::Point<f32>,
   is_accepting: bool,
   is_initial: bool,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Edge {
   start: Option<usize>,
   end: usize,
   symbol: char,
}

#[allow(dead_code)]
#[derive(Default)]
pub enum Interaction {
   #[default]
   None,
   AddNode,
   DelNode,
   AddCon,
   RemCon
}