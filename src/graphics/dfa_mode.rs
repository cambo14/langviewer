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
use crate::graphics::connection;

use super::connection::{Connection, compute_arrow};

/// The radius of a DFA node when drawn on the canvas.
pub const NODE_SIZE: i32 = 2 << 4;
/// The size of the text label for a DFA node.
pub const NODE_TEXT_SIZE: iced::Pixels = iced::Pixels(16.0);
/// The maximum width of the text label for a DFA node.
pub const NODE_TEXT_MAXWIDTH: i32 = NODE_SIZE;


/// The main message enum for handling interactions in the DFA editor mode.
#[derive(Debug, Clone, Copy)]
pub enum Message{
   /// Add a node at the given point
   AddNode {
      /// The position to add the node at
      pos: iced::Point<f32>
   },

   /// Add a connection between two nodes with for the transition associated with given symbol
   AddCon {
      /// The starting point of the connection
      start: iced::Point<f32>,
      /// The ending point of the connection
      end: iced::Point<f32>,
      /// The symbol associated with the transition
      symbol: char
   },
}

/// The window in which to render the DFA editor
#[derive(Debug)]
pub struct DfaWindow{
   /// The current state of the DFA being edited, including its nodes and connections
   pub dfa: DfaInstance,
}

/// A single DFA containing nodes, edges, and an index 
/// for quick lookup of edges based on their start and end nodes and symbol
#[derive(Debug)]
pub struct DfaInstance {
   /// The nodes in the DFA, stored in an RTree for efficient spatial queries
   pub nodes: RTree<Node>,
   /// The edges of the DFA
   pub edges: RTree<Connection>,
}

impl canvas::Program<Message> for DfaWindow {
   fn draw(&self, _state: &Interaction, renderer: &Renderer, _theme: &Theme, bounds: Rectangle, _cursor: mouse::Cursor) -> Vec<canvas::Geometry> {
      let mut frame = canvas::Frame::new(renderer, bounds.size());
      for node in &self.dfa.nodes {
         let circle = canvas::Path::circle(node.pos, NODE_SIZE as f32);
         let text = get_node_text(node);
         frame.stroke(&circle,
            canvas::Stroke::default().with_color(Color::BLACK).with_width(2.0).with_line_join(canvas::LineJoin::Round));
         frame.fill_text(text);
      }
      for connection in self.dfa.edges.iter() {
         if let Some(path) = &connection.path {
            frame.stroke(path,
               canvas::Stroke::default().with_color(Color::BLACK).with_width(2.0).with_line_join(canvas::LineJoin::Round));
            let text = connection::connection_text(connection);
            frame.fill_text(text);
         }
         
         
      }

      vec![frame.into_geometry()]
   }

   /// Update the state of the DFA editor based on an [`Event`](enum@canvas::Event), such as mouse interactions
   /// and returns any resulting [`Action`](struct@canvas::Action) to be performed, such as redrawing the
   /// canvas or publishing a [`Message`].
   fn update(&self, interaction: &mut Interaction,
         event: &canvas::Event, bounds: Rectangle,
         cursor: mouse::Cursor) -> Option<canvas::Action<Message>>
   {

      let (exists, pos) = (cursor.position().is_some(),
         cursor.position().unwrap_or_default());
      let act_pos = iced::Point::new(pos.x, pos.y);
      let node: Option<&Node> = self.dfa.nodes.locate_within_distance(
         Node { pos: act_pos, .. },
         (NODE_SIZE.pow(2) << 1) as f32).next();


      match event {
         canvas::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            
            let message = 
               if exists && let Some(node) = node {
                  *interaction = Interaction::AddCon { init: *node };
                  None
               } else {
                  let nearest = self.dfa.nodes.nearest_neighbor_with_distance_2(
                     Node {pos: act_pos, .. } );
                  let float = if let Some((_, distance)) = nearest {
                        distance
                     } else {
                        f32::MAX
                     };
                  if float < ((NODE_SIZE.pow(2) << 3) as f32){  
                     return None;
                  }
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
                     
                  let message = {
                     *interaction = Interaction::None;
                     Some(Message::AddCon { start: start_node.pos, end: end_node.pos,
                        symbol: self.dfa.edges.size().to_string().chars().next().unwrap_or('?') }) //TODO: have a better way to determine symbol
                  };
                  Some(message.map(canvas::Action::publish)
                     .unwrap_or(canvas::Action::request_redraw()).and_capture(),)
               }else {
                  *interaction = Interaction::None;
                  None
               }// TODO
            } else {
               *interaction = Interaction::None;
               None// TODO
            }
         }
         _ => None
      }
}
   
   type State = Interaction;
   

}

impl DfaWindow {
   /// Implementation of [`ViewFn`](trait@iced::application::ViewFn) for the graphical instance
   /// generating the view based on the current DFA
   pub fn view(&self) -> iced::Element<'_, Message> {
      canvas::Canvas::new(self).width(iced::Fill).height(iced::Fill).into()
   }
}
impl DfaInstance {

   /// Implementation of [`UpdateFn`](trait@iced::application::UpdateFn) for the graphical instance
   /// handling messages and updating the state of the DFA editor accordingly
   pub fn update(&mut self, message: Message) {
      match message {
         Message::AddNode {pos} => {
            self.nodes.insert(Node { pos: iced::Point::new(pos.x, pos.y), index: Some(self.nodes.size()),
               is_accepting: self.nodes.size() == 0, is_initial: false });
         }

         Message::AddCon {start, end, symbol} => {
            let start_act = self.nodes.nearest_neighbor(
               Node { pos: start, index: None, is_accepting: false, is_initial: false });
            let end_act = self.nodes.nearest_neighbor(
               Node { pos: end, index: None, is_accepting: false, is_initial: false });
            if start_act.is_none() || end_act.is_none() {
               log::error!("Failed to find nodes for connection: start node at {:?} {}, end node at {:?} {}",
                  start, start_act.is_none(), end, end_act.is_none());
               return;
            }
            self.edges.insert(Connection {
               start: (start_act.unwrap().pos, start_act.unwrap().index.unwrap_or(0)),
               end: (end_act.unwrap().pos, end_act.unwrap().index.unwrap_or(0)),
               symbol,
               label_loc: iced::Point::new((start.x + end.x) / 2.0, (start.y + end.y) / 2.0),
               index: Some(self.edges.size()),
               path: None,
            });
            let edge_snap: Vec<_> = self.edges.iter().cloned().collect();
            let mut parallel: Vec<&Connection> = Vec::with_capacity(self.edges.size());
            let conn_size = self.edges.size();
            let mut paths: Vec<(usize, (iced::Point, canvas::Path))> = Vec::with_capacity(conn_size);
            for i in 0..conn_size {
               let conn: &Connection = &edge_snap[i];
               parallel.clear();
            
               for edge in edge_snap.iter().filter(
                  |e| (e.start.1 == conn.start.1 && e.end.1 == conn.end.1) ||
                  (e.start.1 == conn.end.1 && e.end.1 == conn.start.1))
               {
                  parallel.push(edge);
               }
               paths.push((i, (compute_arrow(
                  conn,
                  &parallel, &self.nodes, &self.edges))));
            }
            for (i, path) in paths{
               let conn = self.edges.iter_mut().nth(i).unwrap();
               conn.label_loc = path.0;
               conn.path = Some(path.1);
            }
         }
      }
   }
}

/// A helper function to generate a [canvas::Text]
/// object for a given node, displaying its index as "S{index}"
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
      max_width: NODE_TEXT_MAXWIDTH as f32,
      shaping: iced::widget::text::Shaping::Auto,
   }
}

/// A node in the DFA, represented as a point with additional metadata
/// such as whether it is an accepting or initial state
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Node {
   /// The position of the node on the canvas
   pub pos: iced::Point<f32>,
   /// The index of the node in the RTree, used for referencing in connections
   pub index: Option<usize> = None,
   /// Whether the node is an accepting state
   pub is_accepting: bool = false,
   /// Whether the node is an initial state
   pub is_initial: bool = false,
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


/// How the user is currently interacting with the DFA editor,
/// such as adding a connection by dragging from one node to another
#[derive(Debug, Default, PartialEq)]
pub enum Interaction {
   #[default]
   /// No interaction is currently happening
   None,

   /// User is adding a connection by dragging from one node to another,
   /// with the initial node stored in `init`
   AddCon{
      /// The starting point of the connection
      init: Node
   },
}