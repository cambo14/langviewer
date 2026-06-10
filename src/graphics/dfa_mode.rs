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

use iced::mouse::{self};
use iced::widget::text::LineHeight;
use iced::widget::{canvas};
use iced::{Color, Rectangle, Renderer, Theme};
use rstar::{Point, RTree};

const NODE_SIZE: i32 = 2 << 4;
const NODE_TEXT_SIZE: iced::Pixels = iced::Pixels{0: 16.0};
const NODE_TEXT_MAXWWIDTH: i32 = NODE_SIZE;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Message{
    AddNode {pos: iced::Point<f32>},
    DelNode,
    AddCon {start: Node, end: Node, symbol: char},
    RemCon
}

#[derive(Debug)]
pub struct DfaWindow<'a> {
   pub dfa: &'a DfaInstance,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct DfaInstance {
   pub nodes: RTree<Node>,
   pub edges: Vec<Edge>,
}

impl<'a> canvas::Program<Message> for DfaWindow<'a> {
   fn draw(&self, _state: &Interaction, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
      let mut frame = canvas::Frame::new(renderer, bounds.size());
      for node in &self.dfa.nodes {
         let circle = canvas::Path::circle(node.pos, NODE_SIZE as f32);
         let text = get_node_text(&node);
         frame.stroke(&circle,
            canvas::Stroke::default().with_color(Color::BLACK).with_width(2.0).with_line_join(canvas::LineJoin::Round));
         frame.fill_text(text);
      }

      vec![frame.into_geometry()]
   }

   fn update(&self, interaction: &mut Interaction,
         event: &canvas::Event, bounds: Rectangle,
         cursor: mouse::Cursor) -> Option<canvas::Action<Message>>
   {

      let (exists, pos) = (cursor.position().is_some(),
         cursor.position().unwrap_or(iced::Point::default()));
      let act_pos = iced::Point::new(pos.x - bounds.x, pos.y - bounds.y);
      let node: Option<&Node> = self.dfa.nodes.locate_within_distance( //TODO: have different radius for node selection and connection creation
         Node { pos: act_pos, index: None, is_accepting: false, is_initial: false },
         ((NODE_SIZE * NODE_SIZE) << 2) as f32).last();

      match event {
         canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            
            let message = if exists && node.is_some() {
                  *interaction = Interaction::AddCon { init: *node.unwrap() };
                  None
               } else {
                  *interaction = Interaction::None;
                  Some(Message::AddNode {pos: iced::Point::new(pos.x - bounds.x, pos.y - bounds.y)})
            };
            Some(message.map(canvas::Action::publish)
            .unwrap_or(canvas::Action::request_redraw()).and_capture(),)
         }

            
         canvas::Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
            if let Interaction::AddCon { init: init_node } = *interaction {
               if let Some(end_node) = node {
                  // Find the original start node in the RTree to get a &'a Node reference
                  let start_node = self.dfa.nodes.locate_within_distance(
                     Node { pos: init_node.pos, index: None, is_accepting: false, is_initial: false },
                     0.1).last().unwrap_or(end_node);
                     
                  log::debug!("Adding connection from {:?} to {:?}", start_node, end_node);
                  let message = {
                     *interaction = Interaction::None;
                     Some(Message::AddCon { start: *start_node, end: *end_node, symbol: '\0' })
                  };
                  Some(message.map(canvas::Action::publish)
                     .unwrap_or(canvas::Action::request_redraw()).and_capture(),)
               }else {
                  *interaction = Interaction::None;
                  return None;
               }// TODO
            } else {
               *interaction = Interaction::None;
               return None; // TODO
            }
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
impl DfaInstance {
   pub fn update(&mut self, message: Message) {
      match message {
         Message::AddNode {pos} => {
            log::debug!("Adding node at position {:?}", pos);
            self.nodes.insert(Node { pos: iced::Point::new(pos.x, pos.y), index: Some(self.nodes.size()), is_accepting: false, is_initial: false });
         }

         Message::AddCon {start, end, symbol} => {
            log::debug!("Adding connection from {:?} to {:?} with symbol '{}'", start, end, symbol);
            self.edges.push(Edge { start: start.index, end: end.index.unwrap_or(0), symbol });
         }
         _ => {}
      }
   }
}

fn get_node_text(node: &Node) -> canvas::Text{
   canvas::Text {
      content: format!("S{}", node.index.unwrap_or(0)),
      position: iced::Point::new(node.pos.x, node.pos.y),
      color: Color::BLACK,
      size: NODE_TEXT_SIZE,
      font: iced::Font::DEFAULT,
      align_x: iced::widget::text::Alignment::Center,
      align_y: iced::alignment::Vertical::Center,
      line_height: LineHeight::Relative(1.0),
      max_width: NODE_TEXT_MAXWWIDTH as f32,
      shaping: iced::widget::text::Shaping::Auto,
   }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Node {
   pos: iced::Point<f32>,
   index: Option<usize>,
   is_accepting: bool,
   is_initial: bool,
}


impl Point for Node {

    fn generate(mut generator: impl FnMut(usize) -> Self::Scalar) -> Self {
      Node {
         pos: iced::Point::new(generator(0), generator(1)),
         index: None,
         is_accepting: false,
         is_initial: false,
      }
    }
    fn nth(&self, index: usize) -> Self::Scalar {
      match index {
         0 => self.pos.x,
         1 => self.pos.y,
         _ => unreachable!(),
      }
    }
   fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
      match index {
         0 => &mut self.pos.x,
         1 => &mut self.pos.y,
         _ => unreachable!(),
      }
   }
    
   type Scalar = f32;
    
   const DIMENSIONS: usize = 2;
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct Edge {
   start: Option<usize>,
   end: usize,
   symbol: char,
}

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq)]
pub enum Interaction {
   #[default]
   None,
   AddNode,
   DelNode,
   AddCon{init: Node},
   RemCon,
}